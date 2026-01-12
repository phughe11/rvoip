## MODIFIED Requirements

### Requirement: Session Management Architecture
The RVOIP stack SHALL use session-core-v3 as the single session management implementation for all endpoint-facing components.

#### Scenario: Call center uses v3 coordinator
- **WHEN** CallCenterEngine is initialized
- **THEN** it SHALL create a UnifiedCoordinator from session-core-v3
- **AND** it SHALL NOT create a legacy SessionCoordinator from session-core v1

#### Scenario: Client library uses SimplePeer
- **WHEN** a VoIP client is created via client-core
- **THEN** it SHALL use SimplePeer from session-core-v3
- **AND** it SHALL NOT depend on session-core v1 types

### Requirement: Server Component Layering
Server-side components SHALL follow the architectural layering defined in LIBRARY_DESIGN_ARCHITECTURE.md.

#### Scenario: B2BUA does not depend on SBC
- **GIVEN** the b2bua-core crate
- **WHEN** its dependencies are examined
- **THEN** it SHALL NOT have a dependency on sbc-core
- **AND** it SHALL only depend on dialog-core, media-core, and infra-common

#### Scenario: SBC optionally uses B2BUA
- **GIVEN** the sbc-core crate
- **WHEN** B2BUA mode is enabled
- **THEN** it MAY depend on b2bua-core
- **AND** this dependency SHALL be optional

## ADDED Requirements

### Requirement: Unified Event Model
All session events SHALL use the session-core-v3 event types.

#### Scenario: Call state events use v3 types
- **WHEN** a call state changes (e.g., Ringing, Connected, Terminated)
- **THEN** the event SHALL be represented using types from session-core-v3::api::events
- **AND** legacy event types from session-core v1 SHALL NOT be used

### Requirement: Deprecation Path for v1
Components that previously exposed session-core v1 types SHALL provide deprecation notices.

#### Scenario: Deprecated type usage warning
- **WHEN** a user imports a deprecated v1 type from the rvoip facade
- **THEN** a deprecation warning SHALL be emitted
- **AND** the warning SHALL reference the v3 replacement type
