# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for RVOIP.

## What is an ADR?

An ADR is a document that captures an important architectural decision made along with its context and consequences.

## ADR Format

Each ADR follows this structure:

```markdown
# ADR-XXXX: Title

**Status**: Proposed | Accepted | Deprecated | Superseded  
**Date**: YYYY-MM-DD  
**Deciders**: Names or roles  
**Technical Story**: Link to issue/discussion

## Context

What is the issue that we're seeing that is motivating this decision or change?

## Decision

What is the change that we're proposing and/or doing?

## Consequences

What becomes easier or more difficult to do because of this change?

### Positive

- Benefit 1
- Benefit 2

### Negative

- Drawback 1
- Drawback 2

### Neutral

- Other impact 1

## Alternatives Considered

What other options did we consider?

### Alternative 1

Description and why it was rejected.

### Alternative 2

Description and why it was rejected.

## References

- Links to relevant documentation
- Links to related ADRs
- External resources
```

## Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [ADR-0001](adr-0001-three-session-core-versions.md) | Three Session-Core Versions Strategy | Accepted | 2026-01-11 |
| [ADR-0002](adr-0002-async-architecture.md) | Async-First Architecture with Tokio | Accepted | 2024-XX-XX |
| [ADR-0003](adr-0003-memory-safety-rust.md) | Memory Safety with Rust | Accepted | 2024-XX-XX |

## Creating a New ADR

1. Copy the template above
2. Number it sequentially (check the index)
3. Write the ADR
4. Submit as PR for review
5. Update this index when merged

## Guidelines

- **Be Specific**: Describe concrete technical decisions, not general principles
- **Include Context**: Explain why this decision was necessary
- **List Alternatives**: Show what options were considered
- **Document Consequences**: Be honest about trade-offs
- **Update Status**: Mark as superseded when replaced
- **Link Related ADRs**: Cross-reference related decisions

## Status Definitions

- **Proposed**: Under discussion, not yet decided
- **Accepted**: Decision made and being implemented
- **Deprecated**: No longer recommended, but still in use
- **Superseded**: Replaced by a newer ADR (link to it)

## More Information

- [Architecture Decision Records](https://adr.github.io/)
- [Joel Parker Henderson's ADR template](https://github.com/joelparkerhenderson/architecture-decision-record)
