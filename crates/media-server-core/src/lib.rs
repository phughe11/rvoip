//! Media Server Core Component
//!
//! This crate implements independent media server functionality:
//! - IVR (Interactive Voice Response)
//! - Conference Mixing
//! - Recording / Playback

use std::sync::Arc;
use anyhow::{Result, Context};
use tracing::{info, debug};
use tokio::sync::{mpsc, broadcast};

use rvoip_media_core::prelude::*;
use rvoip_media_core::integration::{RtpBridge, RtpBridgeConfig, RtpEvent, RtpEventCallback};
use rvoip_media_core::codec::mapping::CodecMapper;
use rvoip_media_core::relay::controller::codec_detection::CodecDetector;
use rvoip_media_core::relay::controller::codec_fallback::CodecFallbackManager;
use rvoip_media_core::types::{MediaSessionId, payload_types};

pub mod conference;
use conference::ConferenceManager;

/// DTMF Digit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtmfDigit {
    Zero = 0, One = 1, Two = 2, Three = 3, Four = 4,
    Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9,
    Star = 10, Pound = 11, A = 12, B = 13, C = 14, D = 15,
}

impl DtmfDigit {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Zero),
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            5 => Some(Self::Five),
            6 => Some(Self::Six),
            7 => Some(Self::Seven),
            8 => Some(Self::Eight),
            9 => Some(Self::Nine),
            10 => Some(Self::Star),
            11 => Some(Self::Pound),
            12 => Some(Self::A),
            13 => Some(Self::B),
            14 => Some(Self::C),
            15 => Some(Self::D),
            _ => None,
        }
    }
}

/// DTMF Event
#[derive(Debug, Clone)]
pub struct DtmfEvent {
    pub session_id: String,
    pub digit: DtmfDigit,
    pub end: bool,
}

/// Media Server Engine
pub struct MediaServerEngine {
    /// The underlying media core engine
    media_engine: Arc<MediaEngine>,
    /// RTP Bridge for network integration
    rtp_bridge: Arc<RtpBridge>,
    /// DTMF Event Broadcaster
    dtmf_tx: broadcast::Sender<DtmfEvent>,
    /// Conference Manager
    conference_manager: Arc<ConferenceManager>,
}

impl MediaServerEngine {
    /// Create a new Media Server Engine
    pub async fn new() -> Result<Arc<Self>> {
        info!("Initializing Media Server Engine...");
        
        let config = MediaEngineConfig::default();
        let media_engine = MediaEngine::new(config).await
            .context("Failed to create MediaEngine")?;
            
        // Initialize RTP Bridge components
        let (event_tx, _event_rx) = mpsc::unbounded_channel();
        let codec_mapper = Arc::new(CodecMapper::new());
        let codec_detector = Arc::new(CodecDetector::new(codec_mapper.clone()));
        let fallback_manager = Arc::new(CodecFallbackManager::new(codec_detector.clone(), codec_mapper.clone()));
        
        let rtp_config = RtpBridgeConfig::default();
        let rtp_bridge = Arc::new(RtpBridge::new(
            rtp_config,
            event_tx,
            codec_mapper,
            codec_detector,
            fallback_manager
        ));
        
        // Initialize Conference Manager
        let conference_manager = Arc::new(ConferenceManager::new(rtp_bridge.clone()));
        
        // Create DTMF broadcasting channel
        let (dtmf_tx, _) = broadcast::channel(100);
        let dtmf_tx_clone = dtmf_tx.clone();
        
        // Register RTP Event Callback
        let conf_manager_clone = conference_manager.clone();
        
        let callback: RtpEventCallback = Arc::new(move |session_id, event| {
            if let RtpEvent::MediaReceived { payload_type, payload, .. } = event {
                // DTMF Check
                if payload_type == payload_types::TELEPHONE_EVENT {
                    if let Some(digit_val) = payload.get(0) {
                        let end_bit = (payload.get(1).unwrap_or(&0) & 0x80) != 0;
                        if let Some(digit) = DtmfDigit::from_u8(*digit_val) {
                            let event = DtmfEvent {
                                session_id: session_id.to_string(),
                                digit,
                                end: end_bit,
                            };
                            if end_bit {
                                info!("DTMF Detected: {:?} on session {}", digit, session_id);
                                let _ = dtmf_tx_clone.send(event);
                            }
                        }
                    }
                } else {
                    // Feed to Conference Manager (Generic Audio)
                    let cm = conf_manager_clone.clone();
                    let sid = session_id.to_string();
                    let p = payload.clone();
                    tokio::spawn(async move {
                        cm.handle_audio_packet(&sid, &p).await;
                    });
                }
            }
        });
        
        rtp_bridge.add_rtp_event_callback(callback).await;
        
        let engine = Arc::new(Self {
            media_engine,
            rtp_bridge,
            dtmf_tx,
            conference_manager,
        });
        
        engine.start().await?;
        
        Ok(engine)
    }
    
    /// Create a new conference
    pub async fn create_conference(&self, conf_id: &str) -> Result<()> {
        self.conference_manager.create_conference(conf_id.to_string()).await.map_err(|e| anyhow::anyhow!(e))
    }
    
    /// Join a session to a conference
    pub async fn join_conference(&self, conf_id: &str, session_id: &str) -> Result<()> {
        self.conference_manager.join_participant(conf_id, session_id).await.map_err(|e| anyhow::anyhow!(e))
    }

    /// Subscribe to DTMF events
    pub fn subscribe_dtmf(&self) -> broadcast::Receiver<DtmfEvent> {
        self.dtmf_tx.subscribe()
    }
    
    /// Start the media server
    async fn start(&self) -> Result<()> {
        info!("Starting Media Server Engine...");
        Ok(())
    }

    /// Access the underlying MediaEngine
    pub fn media_engine(&self) -> &Arc<MediaEngine> {
        &self.media_engine
    }

    /// Play an audio file to a session
    pub async fn play_wav_file(&self, session_id: &str, file_path: &str) -> Result<()> {
        info!("Playing WAV file '{}' to session {}", file_path, session_id);
        
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            anyhow::bail!("WAV file not found: {}", file_path);
        }
        
        let wav_audio = rvoip_media_core::audio::load_wav_file(path)
            .context("Failed to load WAV file")?;
            
        let ulaw_payload = rvoip_media_core::audio::wav_to_ulaw(&wav_audio)
            .context("Failed to convert WAV to u-law")?;
            
        let chunk_size = 160;
        let session_id_owned = MediaSessionId::new(session_id);
        let rtp_bridge = self.rtp_bridge.clone();
        
        tokio::spawn(async move {
            let mut offset = 0;
            let mut timestamp = 0;
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));
            
            while offset < ulaw_payload.len() {
                interval.tick().await;
                
                let end = std::cmp::min(offset + chunk_size, ulaw_payload.len());
                let chunk = ulaw_payload[offset..end].to_vec();
                
                let payload = if chunk.len() < chunk_size {
                    let mut p = chunk;
                    p.resize(chunk_size, 0xFF);
                    p
                } else {
                    chunk
                };
                
                if let Err(e) = rtp_bridge.send_media_packet(&session_id_owned, payload, timestamp).await {
                    debug!("Failed to send packet (expected in MVP): {}", e);
                }
                
                offset += chunk_size;
                timestamp += 160;
            }
            
            info!("Playback finished for session {}", session_id_owned);
        });
        
        Ok(())
    }
}
