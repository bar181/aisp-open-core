# ADR-010: Property-Based Testing for Formal Verification

**Date**: 2026-01-26  
**Status**: Implemented  
**Decision Makers**: AI Development Team  

## Context

The formal verification components of the AISP validator (property extraction, theorem proving, and Z3 integration) required comprehensive testing beyond traditional unit tests. These modules handle complex mathematical structures, formal logic, and automated theorem proving - areas where edge cases and subtle bugs can have significant impact on correctness.

Property-based testing was identified as the optimal approach to validate formal verification components because:

1. **Complexity**: Formal verification involves complex logical structures that are difficult to test exhaustively with manual test cases
2. **Edge Cases**: Mathematical formulas and proof structures have many edge cases that traditional testing might miss
3. **Integration**: Multiple formal verification components need to work together correctly
4. **Correctness**: Formal verification systems must be highly reliable as they validate other systems

## Decision

Implemented comprehensive property-based testing for formal verification components using Proptest, focusing on automated generation of AISP documents with formal properties and validation of correctness invariants across all formal verification modules.

### Implementation Components

#### 1. **Document Generators**

```rust
// Formal documents with complete AISP structure
fn formal_document() -> impl Strategy<Value = String>

// Complex type documents with advanced type systems
fn complex_type_document() -> impl Strategy<Value = String>

// Temporal documents with temporal logic operators
fn temporal_document() -> impl Strategy<Value = String>
```

**Generator Features**:
- **Type Generation**: Basic types (ℕ, ℤ, ℝ, 𝔹), enumerations, arrays, tuples, function types
- **Rule Generation**: Logical expressions with quantifiers, temporal operators, arithmetic constraints
- **Function Generation**: Lambda expressions with proper type annotations
- **Evidence Generation**: Validity metrics, proof certificates, temporal specifications

#### 2. **Property Categories**

**Core Properties**:
- **Property Extraction Determinism**: Same input always produces same extracted properties
- **Complexity Bounds**: All complexity metrics stay within reasonable bounds
- **Statistics Accuracy**: Extraction statistics match actual extracted properties
- **Formula Well-formedness**: Extracted formulas have valid logical structure

**Formal Verification Properties**:
- **Theorem Prover Consistency**: Multiple prover instances produce identical results
- **Z3 Integration Graceful Handling**: Z3 verification never crashes on valid documents
- **Context Consistency**: Property contexts accurately reflect document content
- **Integration Coherence**: Property extraction and verification work together correctly

**Performance Properties**:
- **Large Document Handling**: Performance remains acceptable for complex documents
- **Temporal Property Processing**: Temporal logic extraction completes within bounds
- **Memory Safety**: No memory leaks or excessive resource consumption

#### 3. **Test Structure**

```
tests/property_testing_formal.rs
├── Core Properties (9 tests, 100 cases each)
├── Edge Cases (4 tests, 25-50 cases each)
└── Integration Tests (cross-component validation)
```

### Key Property Tests

#### Property Extraction Correctness

```rust
#[test]
fn prop_property_extraction_deterministic(doc in formal_document()) {
    let mut extractor1 = PropertyExtractor::new();
    let mut extractor2 = PropertyExtractor::new();
    
    let props1 = extractor1.extract_properties(&parsed1)?;
    let props2 = extractor2.extract_properties(&parsed2)?;
    
    // Same number and types of properties
    prop_assert_eq!(props1.len(), props2.len());
    prop_assert_eq!(types1, types2);
}
```

#### Complexity Bounds Validation

```rust
#[test]
fn prop_property_complexity_bounds(doc in formal_document()) {
    for property in &properties {
        prop_assert!(property.complexity.quantifier_depth <= 10);
        prop_assert!(property.complexity.difficulty_score <= 20);
        prop_assert!(property.complexity.variable_count <= 20);
    }
}
```

#### Z3 Integration Safety

```rust
#[test]
fn prop_z3_verifier_graceful_handling(doc in formal_document()) {
    let result = verifier.verify_document(&parsed_doc, None, None);
    
    prop_assert!(result.is_ok(), "Z3 should not crash");
    // Verify status validity and bounded execution time
}
```

### Test Configuration

```rust
#![proptest_config(ProptestConfig::with_cases(100))]  // Standard tests
#![proptest_config(ProptestConfig::with_cases(50))]   // Complex tests  
#![proptest_config(ProptestConfig::with_cases(25))]   // Edge cases
```

**Test Case Volumes**:
- **Standard Properties**: 100 cases for thorough coverage
- **Complex Properties**: 50 cases for computationally intensive tests
- **Edge Cases**: 25 cases for resource-intensive scenarios
- **Total Cases Generated**: ~1,000 automated test cases per run

## Rationale

### Why Property-Based Testing for Formal Verification

1. **Mathematical Rigor**: Formal verification deals with mathematical structures requiring mathematical testing approaches
2. **Edge Case Discovery**: Automated generation finds corner cases that manual tests miss
3. **Combinatorial Coverage**: Tests combinations of types, operators, and structures that would be impractical to write manually
4. **Regression Prevention**: Properties continue to hold as formal verification code evolves
5. **Confidence Building**: Mathematical guarantees about system behavior under diverse inputs

### Why Proptest Over Alternatives

1. **Rust Integration**: Native Rust library with excellent type system integration
2. **Shrinking**: Automatically minimizes failing test cases for easier debugging
3. **Compositional Generators**: Complex document structures built from simple generators
4. **Deterministic Reproduction**: Failed tests can be reproduced with seeds
5. **Performance**: Efficient generation and execution for complex mathematical structures

### Integration with Existing Testing

**Complements Unit Tests**: Property-based tests validate invariants while unit tests verify specific behaviors

**Complements Integration Tests**: Property tests verify cross-component behavior under diverse inputs

**Enhances Regression Testing**: Properties prevent reintroduction of fixed issues across a wide input space

## Implementation Details

### Test File Organization

```
crates/aisp-core/tests/property_testing_formal.rs
├── Document Generators (formal_document, complex_type_document, temporal_document)
├── Core Property Tests (9 tests)
├── Edge Case Module (formal_edge_cases)
└── Integration Validation
```

### Generator Architecture

```rust
// Hierarchical composition
formal_document() = (header, meta_block, types_block, rules_block, functions_block, evidence_block)
types_block() = vec(type_definition(), 1..=5)
type_definition() = (type_name(), type_expression())
type_expression() = recursive(basic_type | enumeration | array | tuple | function_type)
```

### Property Assertion Patterns

```rust
proptest! {
    #[test]
    fn prop_formula_structure_wellformed(doc in formal_document()) {
        match &property.formula.structure {
            FormulaStructure::Atomic(atomic) => {
                prop_assert!(!atomic.predicate.is_empty());
            }
            FormulaStructure::Universal(quantifier, _) => {
                prop_assert!(!quantifier.variable.is_empty());
            }
            // ... other cases
        }
    }
}
```

### Error Handling Strategy

**Graceful Degradation**: Tests verify that malformed inputs fail gracefully without panicking

**Resource Bounds**: Performance tests ensure computation completes within reasonable time limits

**Memory Safety**: Tests verify no memory leaks or excessive allocation under stress

## Consequences

### Positive

- **Dramatically Improved Coverage**: 13 property tests generating 1,000+ test cases automatically
- **Mathematical Correctness**: Properties verify fundamental mathematical invariants of formal verification
- **Edge Case Discovery**: Found and prevented several subtle bugs in property extraction and formula handling
- **Integration Validation**: Comprehensive testing of property extraction ↔ Z3 verification integration
- **Regression Prevention**: Properties prevent reintroduction of fixed formal verification bugs
- **Confidence in Formal Methods**: Mathematical guarantees about formal verification system correctness

### Negative

- **Increased Test Execution Time**: Property tests take longer than unit tests (acceptable trade-off for coverage)
- **Generator Complexity**: Formal document generators require maintenance as AISP specification evolves
- **Debug Complexity**: Property test failures require analysis of generated formal mathematical structures
- **Learning Curve**: Team familiarity with property-based testing and formal verification concepts

### Risks Mitigated

- **Formal Verification Bugs**: Properties catch errors in property extraction, theorem proving, and verification
- **Integration Failures**: Tests verify that formal verification components work correctly together
- **Performance Regressions**: Large document tests catch performance degradation in formal verification
- **Edge Case Failures**: Automated generation tests mathematical edge cases that manual tests would miss
- **Specification Drift**: Properties ensure formal verification remains correct as AISP specification evolves

## Test Results

**All 13 Property Tests Pass**:
- 9 core property tests (100 cases each) = 900 test cases
- 4 edge case tests (25-50 cases each) = ~125 test cases
- Total: ~1,025 automatically generated test cases per run

**Test Categories Verified**:
- Property extraction correctness and determinism
- Complexity metric bounds and consistency
- Z3 integration safety and performance
- Formula structure well-formedness
- Context consistency with document content
- Large document performance characteristics
- Edge case and error handling

## Follow-up Actions

1. **Continuous Integration**: Integrate property-based formal verification tests into CI pipeline
2. **Property Coverage Expansion**: Add properties for additional formal verification scenarios as they're implemented
3. **Performance Monitoring**: Monitor property test execution times and optimize generators as needed
4. **Generator Evolution**: Update document generators as AISP specification evolves
5. **Failure Analysis**: Establish process for analyzing and learning from property test failures in formal verification

## References

- Proptest Documentation: https://docs.rs/proptest/
- "Property-Based Testing with PropEr, Erlang and Elixir" - Mathematical property testing principles
- ADR-009: Property-Based Testing Framework (core strategy)
- ADR-008: Formal Specification Validation (formal verification foundation)
- `tests/property_testing_formal.rs` - Complete implementation
- `crates/aisp-core/src/property_extractor.rs` - Property extraction under test
- `crates/aisp-core/src/z3_integration.rs` - Z3 integration under test