# ADR-002: Z3 SMT Solver Integration Strategy

## Status
Accepted

## Context
The current Z3 integration is conditional and uses stub implementations when disabled. For production formal verification, we need genuine Z3 integration that provides actual mathematical guarantees.

## Decision
Implement mandatory Z3 integration with graceful degradation for unsupported platforms.

### Core Integration Strategy

#### 1. Default Z3 Enablement
```toml
[dependencies]
z3 = { version = "0.12", features = ["static-link-z3"] }
```
- Z3 enabled by default in all builds
- Static linking to avoid runtime dependencies
- Conditional compilation only for unsupported platforms

#### 2. AISP-to-SMT Translation
Implement bidirectional translation between AISP constructs and Z3 SMT-LIB:

**Type System Mapping:**
- `BasicType::Natural` → Z3 `Int` with `>= 0` constraints
- `BasicType::Boolean` → Z3 `Bool`
- `TypeExpression::Enumeration` → Z3 algebraic datatypes
- `TypeExpression::Function` → Z3 uninterpreted functions

**Logical Formula Translation:**
- Universal quantifiers `∀x:P(x)` → Z3 `(forall ((x Type)) P)`
- Existential quantifiers `∃x:P(x)` → Z3 `(exists ((x Type)) P)`
- Logical connectives directly mapped to Z3 operators

#### 3. Verification Workflow
```rust
pub enum VerificationResult {
    Proven(ProofCertificate),
    Disproven(Counterexample),  
    Unknown(TimeoutReason),
    Error(VerificationError),
}
```

**Satisfiability Checking:**
1. Convert AISP constraints to SMT-LIB
2. Assert negation for validity checking
3. Query Z3 solver with timeout
4. Generate proof certificates or counterexamples

**Model Generation:**
- Extract satisfying assignments for Satisfiable results
- Provide concrete counterexamples for invalid properties
- Support witness extraction for existential proofs

#### 4. Fallback Strategy
For platforms where Z3 unavailable:
- Simplified verification using built-in theorem prover
- Clear warnings about reduced verification capabilities
- Graceful degradation rather than compilation failure

### Formula Generation Examples

**AISP Rule:**
```aisp
∀move:ValidMove(board,pos)⇔board[pos]=Empty
```

**Generated SMT-LIB:**
```smt
(declare-fun ValidMove (Board Pos) Bool)
(declare-fun board-access (Board Pos) Cell)
(declare-const Empty Cell)

(assert (forall ((board Board) (pos Pos))
  (= (ValidMove board pos)
     (= (board-access board pos) Empty))))
```

### Performance Considerations
- **Timeout Management**: 30-second default per query
- **Incremental Solving**: Reuse Z3 context across related queries  
- **Memory Limits**: 1GB limit for SMT solver process
- **Parallelization**: Independent property verification in parallel

## Consequences

### Positive
- Genuine mathematical verification capabilities
- Industry-standard SMT solver reliability
- Concrete counterexamples for debugging
- Performance optimizations through incremental solving

### Negative
- Increased binary size (~50MB for static Z3)
- Platform dependency limitations
- Additional complexity in CI/CD pipeline

### Risk Mitigation
- Static linking eliminates runtime dependencies
- Comprehensive test suite across platforms
- Fallback verification for unsupported environments
- Performance monitoring and optimization

## Implementation Checklist
- [x] Update Cargo.toml with mandatory Z3 dependency
- [x] Implement AISP-to-SMT translation layer
- [x] Replace stub Z3Verifier with genuine implementation  
- [x] Add timeout and resource limit management
- [x] Implement proof certificate generation
- [x] Create comprehensive Z3 integration tests
- [x] Fix formal verification test compilation errors
- [x] Implement Z3 requirement enforcement in tests
- [ ] Document Z3 version compatibility requirements

## Update 2026-01-27: Test Integration Completion

### Formal Verification Test Suite
Successfully implemented comprehensive Z3 integration tests with strict requirement enforcement:

**Test Files:**
- `formal_verification_integration.rs` - Complete Z3 integration tests
- `formal_verification_integration_fixed.rs` - Z3 requirement enforcement
- `formal_verification_comprehensive.rs` - Enterprise pipeline testing
- `formal_verification_simple.rs` - Basic Z3 functionality tests

**Key Features:**
- **Mandatory Z3 Enforcement**: Tests fail with clear error messages when Z3 is required but unavailable
- **Comprehensive Coverage**: Basic properties, temporal logic, mathematical proofs, concurrent systems
- **Error Handling**: Graceful timeout and resource limit handling
- **Performance Testing**: SMT formula generation and verification timing
- **Enterprise Integration**: Multi-layer verification pipeline testing

**Z3 Requirement Pattern:**
```rust
if !is_z3_available() {
    panic!("❌ Z3 is REQUIRED for [test] but is not available. Please install Z3 or compile with --features z3-verification");
}
```

This ensures production-ready formal verification that explicitly fails when mathematical guarantees cannot be provided.