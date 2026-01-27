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
| [007](007-production-ready-cleanup.md) | Production-Ready Cleanup and Code Quality | ✅ Completed | 2025-01-26 |
| [008](008-formal-specification-validation.md) | Formal Specification Validation Framework | ✅ Completed | 2025-01-26 |
| [009](009-property-based-testing.md) | Property-Based Testing Integration | ✅ Completed | 2025-01-26 |
| [010](010-property-based-formal-verification.md) | Property-Based Formal Verification | ✅ Completed | 2025-01-26 |
| [013](013-complete-formal-verification-implementation.md) | Complete Formal Verification Implementation | ✅ Completed | 2025-01-26 |
| [014](014-tri-vector-signal-validation.md) | Tri-Vector Signal Validation | ✅ Completed | 2025-01-26 |
| [015](015-ghost-intent-search-validation.md) | Ghost Intent Search Validation | ✅ Completed | 2025-01-26 |
| [016](016-modular-z3-verification-architecture.md) | Modular Z3 Verification Architecture | ✅ Completed | 2025-01-26 |
| [017](017-rossnet-scoring-validation.md) | RossNet Scoring Validation | ✅ Completed | 2025-01-26 |
| [018](018-hebbian-learning-constraint-validation.md) | Hebbian Learning Constraint Validation | ✅ Completed | 2025-01-26 |
| [019](019-anti-drift-protocol-verification.md) | Anti-Drift Protocol Verification | ✅ Completed | 2025-01-26 |
| [020](020-reference-md-formal-verification-challenge.md) | Reference.md Formal Verification Challenge | ✅ Completed | 2025-01-26 |
| [021](021-test-compilation-errors-remediation.md) | Test Compilation Errors Remediation | ✅ Completed | 2025-01-26 |
| [022](022-pest-parser-migration-for-robustness.md) | Pest Parser Migration for Robustness | ✅ Completed | 2026-01-27 |
| [023](023-deep-verification-architecture-for-semantic-security.md) | Deep Verification Architecture for Semantic Security | ✅ Completed | 2026-01-27 |
| [024](024-adversarial-resistance-framework.md) | Adversarial Resistance Framework | ✅ Completed | 2026-01-27 |
| [025](025-security-assessment-update-to-existing-adrs.md) | Security Assessment Update to Existing ADRs | ✅ Completed | 2026-01-27 |
| [026](026-security-hardening-implementation-roadmap.md) | Security Hardening Implementation Roadmap | ✅ Completed | 2026-01-27 |
| [027](027-canonical-ast-architecture-completion.md) | **Canonical AST Architecture Completion** | ✅ **COMPLETED** | **2026-01-27** |

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