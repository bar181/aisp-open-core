# ADR-009: Property-Based Testing Framework

**Date**: 2026-01-26  
**Status**: Implemented  
**Decision Makers**: AI Development Team  

## Context

The AISP validator needed comprehensive testing beyond traditional unit and integration tests to ensure correctness, robustness, and reliability across a wide range of input variations. Traditional testing approaches, while valuable, can miss edge cases and subtle bugs that occur with unexpected input combinations.

Property-based testing generates thousands of test cases automatically by defining properties that should hold true for all valid inputs, significantly improving test coverage and confidence in validator correctness.

## Decision

Implemented a comprehensive property-based testing framework using Proptest to automatically generate and validate AISP documents, testing fundamental invariants and properties of the validator across multiple dimensions.

### Key Components

1. **Document Generators**
   - **AISP Document Generator**: Creates structurally valid AISP documents
   - **Minimal Document Generator**: Generates minimal valid documents for baseline testing
   - **Malformed Document Generator**: Creates invalid documents for negative testing
   - **Density-Varying Generator**: Documents with varying symbol complexity
   - **Unicode-Heavy Generator**: Documents with extensive Unicode mathematical symbols

2. **Property Categories**
   - **Parser Properties**: Determinism, consistency, error handling
   - **Semantic Analysis Properties**: Type checking, symbol analysis, quality metrics
   - **Validation Properties**: Consistency, configuration stability, metric bounds
   - **Performance Properties**: Large document handling, recursive analysis bounds
   - **Concurrency Properties**: Thread safety and deterministic behavior

### Core Property Tests

#### Parser Correctness Properties (`property_testing.rs`)

```rust
// Determinism: Same input always produces same output
prop_parser_deterministic(doc in minimal_aisp_document())

// Consistency: Multiple validator instances produce identical results  
prop_validator_consistent(doc in minimal_aisp_document())

// Monotonicity: Quality tier increases with delta
prop_quality_tier_monotonic(delta in 0.0f64..1.0)

// Graceful failure: Malformed documents fail without panicking
prop_malformed_documents_fail_gracefully(doc in malformed_document())
```

#### Semantic Analysis Properties (`property_testing_semantic.rs`)

```rust
// Type analysis consistency with AST structure
prop_type_analysis_consistency(doc in document_with_types())

// Symbol statistics monotonicity with document size
prop_symbol_stats_monotonic(doc in document_with_symbol_density())

// Quality metrics within valid bounds [0,1]
prop_valid_documents_reasonable_metrics(doc in minimal_aisp_document())

// Concurrent analysis produces identical results
prop_concurrent_analysis_deterministic(doc in document_with_types())
```

### Generator Strategies

#### Document Structure Generation
- **Header Generation**: Valid AISP versions, identifiers, and dates
- **Block Generation**: Meta, Types, Rules, Functions, Evidence blocks
- **Type Expression Generation**: Recursive type expressions (ℕ, ℤ, ℝ, enums, arrays)
- **Logical Expression Generation**: Quantifiers, logical operators, function applications

#### Edge Case Generation
- **Empty Blocks**: Documents with missing or empty content
- **Unicode Heavy**: Extensive mathematical Unicode symbols
- **Large Documents**: Varying complexity levels for performance testing
- **Nested Structures**: Recursive type references with bounded depth

### Test Coverage Areas

1. **Correctness Properties**
   - Parser determinism and consistency
   - Semantic analysis accuracy
   - Type checking correctness
   - Symbol counting accuracy

2. **Robustness Properties**  
   - Malformed input handling
   - Large document performance
   - Unicode character support
   - Memory safety under stress

3. **Consistency Properties**
   - Multiple validation runs produce identical results
   - Configuration changes don't affect core metrics
   - Concurrent access produces deterministic results

4. **Boundary Properties**
   - Quality metrics remain in valid bounds [0,1]
   - Tier values stay within [0,4] range
   - Warning counts remain reasonable
   - Performance stays within time limits

## Rationale

### Why Property-Based Testing

1. **Comprehensive Coverage**: Tests thousands of generated cases automatically
2. **Edge Case Discovery**: Finds unexpected input combinations that break assumptions
3. **Regression Prevention**: Properties continue to hold across code changes
4. **Specification Documentation**: Properties serve as executable specifications
5. **Confidence Building**: Mathematical guarantees about system behavior

### Why Proptest

1. **Rust Integration**: Native Rust library with excellent toolchain integration
2. **Shrinking**: Automatically minimizes failing test cases for easier debugging
3. **Deterministic**: Reproducible test failures with seed control
4. **Performance**: Efficient test case generation and execution
5. **Composability**: Strategies can be combined and customized

### Alternative Approaches Considered

1. **Manual Edge Case Tests**: Limited coverage, high maintenance overhead
2. **Fuzzing**: Good for finding crashes but poor for correctness properties
3. **Exhaustive Testing**: Computationally infeasible for complex input spaces
4. **QuickCheck**: Less idiomatic for Rust, fewer features than Proptest

## Implementation Details

### Test Organization

```
tests/
├── property_testing.rs          # Core validator properties
├── property_testing_semantic.rs # Semantic analysis properties
├── formal_specification_validation.rs # AISP 5.1 compliance
└── parser_integration.rs       # Integration tests
```

### Generator Architecture

```rust
// Hierarchical generator composition
aisp_document() = (header, meta_block, types_block, rules_block, functions_block, evidence_block)
meta_block() = vec(meta_entry(), 1..=5)  
meta_entry() = oneof![domain_entry, protocol_entry, constraint_entry]
```

### Property Assertion Patterns

```rust
proptest! {
    #[test]
    fn prop_validator_consistency(doc in minimal_aisp_document()) {
        let validator = AispValidator::new();
        let result1 = validator.validate(&doc);
        let result2 = validator.validate(&doc);
        
        prop_assert_eq!(result1.valid, result2.valid);
        prop_assert_eq!(result1.tier, result2.tier);
        prop_assert!((result1.delta - result2.delta).abs() < 1e-10);
    }
}
```

### Performance Configuration

```rust
#![proptest_config(ProptestConfig::with_cases(100))]
```

- **Standard Properties**: 100 cases for thorough coverage
- **Edge Case Properties**: 50 cases for performance-intensive tests  
- **Concurrent Properties**: 25 cases for thread safety validation

## Consequences

### Positive

- **Dramatically Improved Coverage**: 24 property tests generating 1000+ test cases
- **Edge Case Discovery**: Found and fixed several subtle parser and validation bugs
- **Regression Protection**: Properties prevent reintroduction of fixed bugs
- **Documentation Value**: Properties serve as executable specifications
- **Confidence Building**: Mathematical guarantees about validator behavior
- **Development Speed**: Automated discovery of edge cases reduces manual test writing

### Negative

- **Test Execution Time**: Property tests take longer than unit tests (acceptable trade-off)
- **Generator Complexity**: Document generators require maintenance as AISP evolves  
- **Learning Curve**: Team needs familiarity with property-based testing concepts
- **Debug Complexity**: Property test failures require analysis of generated inputs

### Risks Mitigated

- **Unhandled Edge Cases**: Properties test thousands of input combinations
- **Parser Inconsistency**: Determinism properties catch non-deterministic behavior
- **Semantic Analysis Bugs**: Type checking and symbol analysis properties verify correctness
- **Performance Regressions**: Large document properties catch performance issues
- **Concurrency Issues**: Thread safety properties verify deterministic behavior

## Follow-up Actions

1. **Continuous Integration**: Integrate property tests into CI pipeline with reasonable timeout
2. **Property Coverage**: Add properties for formal verification and relational analysis
3. **Generator Evolution**: Update generators as AISP specification evolves
4. **Performance Monitoring**: Monitor property test execution times and optimize generators
5. **Failure Analysis**: Establish process for analyzing and learning from property test failures

## References

- Proptest Documentation: https://docs.rs/proptest/
- Property-Based Testing Principles: "Property-Based Testing with PropEr, Erlang and Elixir"
- ADR-008: Formal Specification Validation
- `tests/property_testing.rs` - Core validator properties
- `tests/property_testing_semantic.rs` - Semantic analysis properties