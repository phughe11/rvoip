# Conference Mixing Implementation Plan

## Goal
Enable multi-party audio conferencing by leveraging `AudioMixer` from `media-core`.

## Components
1.  **ConferenceManager**: Orchestrates multiple conferences.
2.  **ConferenceRoom**: Represents a single conference, holding an `AudioMixer` and a map of `MediaSessionId`.
3.  **Mixing Loop**: A task that runs every 20ms:
    - Collects audio from all participants.
    - Feeds to `AudioMixer`.
    - Sends mixed audio back to participants via `RtpBridge` (or directly to Session).

## Integration
- `MediaServerEngine` will hold `Arc<ConferenceManager>`.
- `B2BUA` will control conferences via `MediaServerEngine` API:
    - `create_conference() -> conf_id`
    - `add_participant(conf_id, session_id)`
    - `remove_participant(conf_id, session_id)`

## Implementation Steps
1.  Create `crates/media-server-core/src/conference.rs`.
2.  Implement `ConferenceManager` and `ConferenceRoom`.
3.  Expose API in `MediaServerEngine`.
4.  Update `IMPLEMENTATION_PROGRESS.md`.
