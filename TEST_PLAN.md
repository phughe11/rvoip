# RVOIP End-to-End Test Plan

## Overview
This document outlines the testing strategy to verify the integrity and functionality of the RVOIP system, specifically focusing on the interactions between `b2bua-core`, `proxy-core`, `media-server-core`, and `sbc-core`.

## Test Scenarios

### 1. Simple Proxy Call
**Objective**: Verify `proxy-core` stateful routing and DNS logic.
**Setup**:
- 1 Proxy Instance (`rvoip-proxy-core`).
- 2 Clients (`alice@domain.com`, `bob@domain.com`).
- DNS/Location Mock.
**Steps**:
1. Alice sends INVITE `bob@domain.com`.
2. Proxy consults `LocationService`: `bob` -> `192.168.1.50:5060`.
3. Proxy creates ClientTransaction to `192.168.1.50`.
4. Bob receives INVITE.
5. Bob checks `Via` header (should ensure visibility).

### 2. IVR Auto-Attendant (Media Server)
**Objective**: Verify `media-server-core` playback & DTMF handling.
**Setup**:
- B2BUA instance controlling call.
- Media Server connected via `RtpBridge`.
**Steps**:
1. Alice calls `ivr@service`.
2. B2BUA accepts call (200 OK).
3. B2BUA instructs MediaServer: `play_wav("welcome.wav", session_id)`.
4. RTP Stream verified (Sequence numbers incrementing).
5. Alice sends DTMF '1' (RFC 2833).
6. MediaServer broadcasts `DtmfEvent(Digit::One)`.
7. B2BUA receives Event -> triggers "Sales Queue" logic.

### 3. SBC Security Enforcement
**Objective**: Verify `sbc-core` topology hiding and policy.
**Setup**:
- SBC placed in front of Proxy/B2BUA.
**Steps**:
1. External INVITE arrives.
2. SBC checks Rate Limit (pass).
3. SBC strips `Server` and `User-Agent` headers.
4. Internal components process request.
5. Response is generated.
6. SBC strips internal `Via` information (Topology Hiding).

## Integration Test Suite (Proposed)
Create `tests/system_integration.rs` in `rvoip-server`:

```rust
#[tokio::test]
async fn test_full_ivr_flow() {
    // 1. Init Components
    let media = MediaServerEngine::new().await.unwrap();
    let b2bua = B2buaEngine::new().await.unwrap();
    
    // 2. Mock Call Setup
    let session_id = "test-session";
    
    // 3. Play Audio
    media.play_wav_file(session_id, "test.wav").await.unwrap();
    
    // 4. Inject DTMF Packet (RTP Payload 101)
    // Verify B2BUA receives generic event
}
```

## Manual Verification Checklist
- [ ] Run `cargo build` on workspace.
- [ ] Verify `IMPLEMENTATION_PROGRESS.md` matches code state.
- [ ] Check `Cargo.toml` workspace members are correct.
