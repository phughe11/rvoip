# Media Server Core Implementation Plan

## Goal
Implement a basic Media Server capability that can act as an RTP endpoint for playing audio and mixing conferences.
It leverages `rvoip-media-core` for the heavy lifting (RTP handling, codecs).

## Architecture

### `MediaServerEngine`
The central component that manages:
- **Endpoints**: Sources and sinks of media (Files, Text-to-Speech, Conference Rooms).
- **Sessions**: Active media sessions connected to SIP dialogs.

### Components
1.  **Endpoint Trait**:
    - `generate_packet()`: Source of audio.
    - `receive_packet()`: Sink for audio.
2.  **FilePlayer**:
    - Reads WAV/Raw files.
    - Packets them into RTP.
    - Handles timing.
3.  **ConferenceRoom** (Future):
    - Mixes audio from N sources.
    - Distributes to N sinks.

## Phase 1: MVP (WAV Player)
- `MediaServerEngine` initializes `MediaEngine`.
- Provide method: `play_file(session_id, file_path)`.
- This requires:
    - Opening the file.
    - decoding headers.
    - Determining packetization (e.g. 20ms chunks).
    - Feeding `MediaEngine` or `RtpSession` directly.

## Integration Strategy
- `media-server-core` creates `MediaSession`s via `MediaEngine`.
- It acts as the "Application" driving the `MediaSession`.

## Proposed API
```rust
let server = MediaServerEngine::new().await?;
let session = server.create_session(config).await?;
server.play_file(session.id(), "welcome.wav").await?;
```
