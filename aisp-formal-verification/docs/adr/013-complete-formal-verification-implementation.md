# ADR 013: Complete Formal Verification Implementation

## Status
Accepted - 2026-01-26

## Context

Analysis of the current `aisp-formal-verification` implementation reveals:

**✅ Working Components:**
- Comprehensive formal verification architecture (ADR-009)
- Z3 integration design (ADR-005) 
- Relational analysis (17/17 constraints satisfied)
- Temporal analysis (LTL/CTL formula detection)
- Quality assessment (δ=1.000, ambiguity=0.000 measured)

**❌ Implementation Gaps:**
- Z3 compilation issues preventing formal verification
- Proof construction uses placeholder logic instead of actual theorem proving
- Parser fails on enumeration syntax in test documents
- Ambiguity calculation claims vs. actual measurement inconsistency

**🔍 Test Results Analysis:**
```
File: test_document.aisp
  Status: ✗ Invalid
  Quality: ◊⁺⁺ Platinum (δ=1.000, ambiguity=0.000)
  Formal Verification:
    Status: Incomplete
    Properties: 0/0 proven
    Time: 0ms
```

The framework is present but verification is not executing actual proofs.

## Decision

Complete the formal verification implementation through targeted fixes:

### 1. Z3 Integration Completion

**Problem:** Z3 compilation fails due to missing system dependencies
```
wrapper.h:1:10: fatal error: 'z3.h' file not found
```

**Solution:** Implement robust Z3 integration with fallbacks
```rust
// aisp-core/Cargo.toml
[features]
default = ["std", "serde", "z3-verification"]  # Enable Z3 by default
z3-verification = ["dep:z3"]
z3-static = ["z3-verification", "z3/static"]   # Static linking option

[dependencies]
z3 = { version = "0.12", optional = true, features = ["static-link-z3"] }
```

**Build Script for Z3 Setup:**
```rust
// build.rs
fn main() {
    #[cfg(feature = "z3-verification")]
    {
        if let Err(_) = pkg_config::find_library("z3") {
            // Fall back to bundled Z3 or provide installation instructions
            println!("cargo:warning=Z3 not found, using fallback verification");
        }
    }
}
```

### 2. Genuine Proof Construction

**Current Issue:** Placeholder proof generation
```rust
// Current: Always succeeds with hardcoded steps
fn try_direct_proof(&mut self, property: &PropertyFormula) -> AispResult<ProofSteps> {
    let steps = vec![
        ProofStep { conclusion: "Property assumed for direct proof".to_string(), ... }
    ];
    Ok(ProofSteps(steps))  // Always succeeds!
}
```

**Solution:** Implement actual logical derivation
```rust
fn try_direct_proof(&mut self, property: &PropertyFormula) -> AispResult<ProofSteps> {
    let mut proof_context = ProofContext::new();
    let mut derivation_steps = Vec::new();
    
    // Actual proof construction
    match &property.structure {
        FormulaStructure::Universal(var, body) => {
            // Universal introduction: prove for arbitrary variable
            let arbitrary_var = proof_context.introduce_variable(var);
            let instantiated_body = body.substitute(var, &arbitrary_var);
            let body_proof = self.prove_formula(&instantiated_body)?;
            
            derivation_steps.push(ProofStep {
                rule_name: "UNIVERSAL_INTRODUCTION".to_string(),
                premises: body_proof.conclusion_formulas(),
                conclusion: format!("∀{}.{}", var, body),
                justification: format!("Arbitrary {} satisfies {}", var, body),
                dependencies: body_proof.step_indices(),
            });
        }
        
        FormulaStructure::Implication(antecedent, consequent) => {
            // Implication introduction: assume antecedent, derive consequent
            proof_context.assume(antecedent.clone());
            let consequent_proof = self.prove_formula(consequent)?;
            
            derivation_steps.push(ProofStep {
                rule_name: "IMPLICATION_INTRODUCTION".to_string(),
                premises: vec![format!("{}", antecedent)],
                conclusion: format!("{} → {}", antecedent, consequent),
                justification: "Assuming antecedent, proved consequent".to_string(),
                dependencies: consequent_proof.step_indices(),
            });
        }
        
        _ => return Err(AispError::VerificationFailed(
            format!("Direct proof not implemented for formula type: {:?}", property.structure)
        )),
    }
    
    // Validate proof correctness
    self.validate_proof_steps(&derivation_steps)?;
    Ok(ProofSteps(derivation_steps))
}
```

### 3. Parser Enumeration Fix

**Current Issue:** Parser fails on enumeration syntax
```
Parse error at line 13, column 25: Expected ',' or '}' in enumeration
```

**Solution:** Fix enumeration parsing in types_parser.rs
```rust
impl TypesParser {
    fn parse_enumeration(&mut self) -> AispResult<TypeExpression> {
        self.lexer.expect_token(TokenType::LeftBrace)?;
        let mut variants = Vec::new();
        
        loop {
            if self.lexer.check_token(&TokenType::RightBrace) {
                break;
            }
            
            let variant = self.lexer.expect_identifier()?;
            variants.push(variant);
            
            // Handle both comma-separated and space-separated variants
            if self.lexer.check_token(&TokenType::Comma) {
                self.lexer.next_token(); // consume comma
            } else if !self.lexer.check_token(&TokenType::RightBrace) {
                // Allow space separation without commas for AISP compatibility
                continue;
            }
        }
        
        self.lexer.expect_token(TokenType::RightBrace)?;
        Ok(TypeExpression::Enumeration(variants))
    }
}
```

### 4. Ambiguity Measurement Implementation

**Current Status:** Claims `ambiguity=0.000` but calculation unclear

**Solution:** Implement measurable ambiguity calculation
```rust
pub struct AmbiguityAnalyzer {
    parser_cache: HashMap<String, ParseResult>,
    interpretation_cache: HashMap<String, Vec<SemanticInterpretation>>,
}

impl AmbiguityAnalyzer {
    /// Calculate ambiguity as: Ambig(D) = 1 - |Parse_unique(D)| / |Parse_total(D)|
    pub fn calculate_ambiguity(&mut self, document: &str) -> AispResult<f64> {
        // Parse with multiple interpretation strategies
        let parsing_strategies = vec![
            ParsingStrategy::Strict,
            ParsingStrategy::Permissive, 
            ParsingStrategy::Backtracking,
        ];
        
        let mut unique_interpretations = HashSet::new();
        let mut total_interpretations = 0;
        
        for strategy in parsing_strategies {
            let interpretations = self.parse_with_strategy(document, strategy)?;
            total_interpretations += interpretations.len();
            
            for interpretation in interpretations {
                let semantic_hash = self.semantic_hash(&interpretation);
                unique_interpretations.insert(semantic_hash);
            }
        }
        
        if total_interpretations == 0 {
            return Ok(1.0); // Maximum ambiguity if unparseable
        }
        
        let ambiguity = 1.0 - (unique_interpretations.len() as f64) / (total_interpretations as f64);
        
        // Enforce AISP invariant: Ambig(D) < 0.02
        if ambiguity >= 0.02 {
            return Err(AispError::ValidationError(
                format!("Document ambiguity {:.3} exceeds maximum allowed 0.02", ambiguity)
            ));
        }
        
        Ok(ambiguity)
    }
}
```

## Implementation Plan

### Phase 1: Infrastructure (Week 1)
- [ ] Fix Z3 build system and dependencies
- [ ] Resolve parser enumeration syntax issues  
- [ ] Enable Z3 by default with fallback verification

### Phase 2: Core Verification (Week 2)
- [ ] Replace placeholder proof construction with genuine logical derivation
- [ ] Implement natural deduction rules (modus ponens, universal introduction, etc.)
- [ ] Add SMT formula generation for Z3 integration

### Phase 3: Measurement & Testing (Week 3)
- [ ] Implement quantifiable ambiguity calculation
- [ ] Create comprehensive formal verification test suite
- [ ] Performance optimization and memory management

### Phase 4: Integration & Documentation (Week 4)
- [ ] End-to-end verification workflow testing
- [ ] Update documentation to match actual capabilities
- [ ] Benchmark against reference specification claims

## Success Metrics

**Technical Validation:**
- [ ] All formal verification claims in documentation are implementationally validated
- [ ] `Ambig(D) < 0.02` measurably enforced and calculated
- [ ] Formal verification finds actual logical errors in test specifications
- [ ] Z3 integration works without build failures

**Performance Targets:**
- [ ] Formal verification completes in < 30 seconds for complex documents
- [ ] Proof generation produces valid mathematical derivations
- [ ] Parser handles all AISP syntax without enumeration errors

**Quality Assurance:**
- [ ] Test suite covers all formal verification code paths
- [ ] Documentation matches actual implementation capabilities
- [ ] CI/CD pipeline includes formal verification testing

## Related ADRs
- [ADR-005](005-z3-native-integration.md): Z3 Native Integration
- [ADR-009](../ADR-009-formal-verification-architecture.md): Formal Verification Architecture
- [ADR-002](002-formal-methods-framework.md): Formal Methods Framework