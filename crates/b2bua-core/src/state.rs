//! B2BUA State Definitions

/// High-level state of a B2BUA call
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum B2buaState {
    /// Initial state, no call established
    Idle,
    /// Incoming INVITE received, processing
    IncomingCall,
    /// Outgoing INVITE sent to leg B
    Dialing,
    /// Both legs connected
    Bridged,
    /// Call is being transferred
    Transferring,
    /// Call is ending
    Terminating,
    /// Call has ended
    Terminated,
}
