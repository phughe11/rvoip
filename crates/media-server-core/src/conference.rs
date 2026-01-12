use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tracing::{info, warn, error};

use rvoip_media_core::types::conference::{
    ConferenceMixingConfig, ParticipantId, AudioStream, ConferenceResult
};
use rvoip_media_core::processing::audio::mixer::AudioMixer;
use rvoip_media_core::integration::RtpBridge;
use rvoip_media_core::types::MediaSessionId;
use rvoip_media_core::codec::audio::{G711Codec, AudioCodec};

/// ID for a conference room
pub type ConferenceId = String;

/// Manages active conferences
pub struct ConferenceManager {
    rooms: RwLock<HashMap<ConferenceId, Arc<ConferenceRoom>>>,
    rtp_bridge: Arc<RtpBridge>,
}

impl ConferenceManager {
    pub fn new(rtp_bridge: Arc<RtpBridge>) -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
            rtp_bridge,
        }
    }

    pub async fn create_conference(&self, conf_id: ConferenceId) -> ConferenceResult<()> {
        let room = ConferenceRoom::new(conf_id.clone(), self.rtp_bridge.clone()).await?;
        self.rooms.write().await.insert(conf_id.clone(), Arc::new(room));
        info!("Created conference room: {}", conf_id);
        Ok(())
    }

    pub async fn join_participant(&self, conf_id: &str, session_id: &str) -> ConferenceResult<()> {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(conf_id) {
            room.add_participant(session_id).await?;
            info!("Participant {} joined conference {}", session_id, conf_id);
        } else {
            // Error handling omitted for brevity in this snippet
            warn!("Conference {} not found", conf_id);
        }
        Ok(())
    }
    
    /// Handle incoming RTP audio for mixing
    pub async fn handle_audio_packet(&self, session_id: &str, payload: &[u8]) {
        // Find if this session is in any conference
        let rooms = self.rooms.read().await;
        for room in rooms.values() {
            if room.has_participant(session_id).await {
                room.process_input(session_id, payload).await;
                break;
            }
        }
    }
}

/// A single conference room
pub struct ConferenceRoom {
    id: ConferenceId,
    mixer: Arc<AudioMixer>,
    rtp_bridge: Arc<RtpBridge>,
    participants: RwLock<Vec<String>>, // session_ids
    mixing_task: Mutex<Option<JoinHandle<()>>>,
    codec: Mutex<G711Codec>, // For MVP, assume G.711
}

impl ConferenceRoom {
    pub async fn new(id: ConferenceId, rtp_bridge: Arc<RtpBridge>) -> ConferenceResult<Self> {
        let config = ConferenceMixingConfig::default();
        let mixer = Arc::new(AudioMixer::new(config).await?);
        
        let room = Self {
            id: id.clone(),
            mixer: mixer.clone(),
            rtp_bridge: rtp_bridge.clone(),
            participants: RwLock::new(Vec::new()),
            mixing_task: Mutex::new(None),
            codec: Mutex::new(G711Codec::mu_law(8000, 1).expect("Failed to create G711 codec")),
        };
        
        // Start mixing loop
        let mixer_clone = mixer.clone();
        let bridge_clone = rtp_bridge.clone();
        let room_id = id.clone();

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut codec = G711Codec::mu_law(8000, 1).expect("Failed to create codec for mixer");

            loop {
                interval.tick().await;
                let start = std::time::Instant::now();
                
                // 1. Perform mixing
                // Pass empty slice as inputs are managed internally by AudioStreamManager
                match mixer_clone.mix_participants(&[]).await {
                    Ok(mixed_frames) => {
                        for (pid, frame) in mixed_frames {
                            // 2. Encode
                            if let Ok(encoded) = codec.encode(&frame) {
                                // 3. Send back to participant via RTP bridge
                                // Convert ParticipantId to MediaSessionId
                                let session_id = MediaSessionId::new(pid.to_string());
                                let _ = bridge_clone.send_media_packet(
                                    &session_id, 
                                    encoded,
                                    frame.timestamp
                                ).await;
                            }
                        }
                    },
                    Err(e) => {
                        error!("Mixing error in room {}: {:?}", room_id, e);
                    }
                }

                let elapsed = start.elapsed();
                if elapsed > std::time::Duration::from_millis(15) {
                    warn!("Mixing loop took too long in room {}: {:?}", room_id, elapsed);
                }
            }
        });
        
        *room.mixing_task.lock().await = Some(task);
        
        Ok(room)
    }

    pub async fn has_participant(&self, session_id: &str) -> bool {
        self.participants.read().await.contains(&session_id.to_string())
    }

    pub async fn add_participant(&self, session_id: &str) -> ConferenceResult<()> {
        let mut parts = self.participants.write().await;
        if !parts.contains(&session_id.to_string()) {
            parts.push(session_id.to_string());
            
            // Register with AudioMixer
            let stream = AudioStream::new(ParticipantId::new(session_id), 8000, 1);
            self.mixer.add_audio_stream(ParticipantId::new(session_id), stream).await?;
        }
        Ok(())
    }

    pub async fn process_input(&self, session_id: &str, payload: &[u8]) {
        // 1. Decode
        let mut codec = self.codec.lock().await;
        // G711Codec `decode` expects payload and returns AudioFrame
        if let Ok(frame) = codec.decode(payload) {
             // 2. Feed to Mixer
             // AudioMixer::process_audio_frame expects &ParticipantId
             let pid = ParticipantId::new(session_id);
             let _ = self.mixer.process_audio_frame(&pid, frame).await;
        }
    }
}
