# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records for the AISP Pure Rust implementation.

## ADR Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [001](001-pure-rust-architecture.md) | Pure Rust Architecture for AISP Validation | Accepted | 2025-01-26 |
| [002](002-formal-methods-framework.md) | Formal Methods Framework with Mathematical Foundations | Accepted | 2025-01-26 |
| [003](003-rocq-integration.md) | Rocq-of-Rust Integration for Mechanized Proofs | Accepted | 2025-01-26 |
| [004](004-modular-srp-architecture.md) | Single Responsibility Principle Modular Architecture | Accepted | 2025-01-26 |
| [005](005-z3-native-integration.md) | Native Z3 Integration for SMT Solving | Accepted | 2025-01-26 |
| [006](006-garden-inspired-verification.md) | Garden-Inspired Formal Verification Methodology | Accepted | 2025-01-26 |

## ADR Template

We follow the format:
- **Status**: Proposed | Accepted | Deprecated | Superseded
- **Context**: The issue motivating this decision
- **Decision**: The change being proposed or has been made
- **Consequences**: The positive and negative outcomes

## ADR Process

1. Create new ADR with incremental number
2. Document decision rationale and context
3. Review with technical stakeholders
4. Update status and implementation notes
5. Reference from relevant code documentation