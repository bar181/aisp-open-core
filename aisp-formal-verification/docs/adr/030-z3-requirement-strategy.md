# ADR-030: Z3 Requirement Strategy for Production Systems

**Status:** 🔄 PROPOSED  
**Date:** 2026-01-27  
**Supersedes:** Graceful degradation patterns in formal verification  

## Context

The current AISP formal verification system implements **graceful degradation** where Z3 formal verification is optional, with tests skipping when Z3 is unavailable. For enterprise production systems, this raises the question: should formal verification be **mandatory** or **optional**?

## Problem Statement

**Current Architecture:**
```rust
if !is_z3_available() {
    println!("⚠️ SKIPPING: Z3 formal verification test");
    return; // Graceful skip
}
```

**Question:** Should production systems require strict Z3 availability for formal verification capabilities?

## Decision

**ACCEPT**: **Strict Z3 Requirements** for ALL builds. No graceful degradation accepted.

## Architecture Strategy

### 🏗️ **Strict Z3 Requirement Strategy**

#### **All Builds: Mandatory Z3**
```rust
// ALL builds require Z3 - no exceptions
if !is_z3_available() {
    compile_error!("❌ CRITICAL: Z3 is MANDATORY for AISP formal verification");
}
```

#### **Runtime Enforcement**
```rust
// Runtime verification that Z3 is properly configured
if !is_z3_available() {
    panic!("❌ FATAL: Z3 is REQUIRED but not available - cannot proceed");
}
```

### 🚀 **Implementation Plan**

#### **1. Mandatory Z3 Architecture**
```toml
# Cargo.toml
[features]
default = ["verification"]
verification = ["z3-sys"]  # Always required
```

#### **2. Strict Compilation Strategy**
```rust
// Z3 verification is MANDATORY for ALL builds
#[cfg(not(feature = "verification"))]
compile_error!("Z3 verification is MANDATORY - must compile with --features verification");

// All verification functions require Z3
fn verify_with_z3() { 
    assert!(is_z3_available(), "❌ FATAL: Z3 must be available");
    // Formal verification with Z3
}
```

#### **3. Makefile Updates**
```makefile
# All builds require Z3
build: ## Build with mandatory Z3 verification
	@cd aisp-formal-verification && cargo build --features verification

# Remove non-Z3 build options
# build-no-z3: REMOVED - not supported

# All deployment requires Z3
deploy: build test-z3 ## Deployment pipeline with mandatory Z3
	@echo "✅ Build complete with mandatory formal verification"
```

## Rationale

### **Why Strict Z3 for Production?**

#### **Security Assurance**
- **Formal verification** is critical for enterprise security compliance
- **Mathematical proofs** of correctness cannot be optional in production
- **Zero tolerance** for security verification gaps

#### **Regulatory Compliance**
- **ISO27001, NIST, GDPR** often require formal verification capabilities
- **Audit requirements** mandate consistent security verification
- **Compliance frameworks** expect deterministic security assessment

#### **Operational Excellence**  
- **Fail-fast deployment** prevents runtime surprises
- **Consistent behavior** across all production environments
- **Dependency validation** at build time vs runtime discovery

#### **Risk Management**
- **Attack surface** is unpredictable without formal verification
- **Threat modeling** requires mathematical correctness proofs
- **Incident response** needs reliable security assessment tools

### **Why Graceful for Development?**

#### **Developer Productivity**
- **Local development** shouldn't require full enterprise setup
- **Rapid iteration** enables faster feature development
- **CI/CD flexibility** for different environment capabilities

#### **Testing Strategy**
- **Unit tests** can run without Z3 for basic functionality
- **Integration tests** require Z3 for complete verification
- **Smoke tests** validate core logic without formal proving

## Implementation

### **Phase 1: Feature Flag Introduction**
```rust
// crates/aisp-core/src/z3_verification/facade.rs
pub fn new() -> Result<Self, VerificationError> {
    #[cfg(feature = "strict-z3")]
    {
        if !is_z3_available() {
            return Err(VerificationError::Z3Required(
                "Z3 is MANDATORY for production formal verification".to_string()
            ));
        }
    }
    
    #[cfg(not(feature = "strict-z3"))]
    {
        if !is_z3_available() {
            return Err(VerificationError::Z3Unavailable(
                "Z3 not available - formal verification disabled".to_string()
            ));
        }
    }
    
    // Z3 initialization...
}
```

### **Phase 2: Test Architecture Update**
```rust
// Strict production tests
#[cfg(feature = "strict-z3")]
#[test]
fn test_production_requires_z3() {
    assert!(is_z3_available(), "❌ PRODUCTION REQUIRES Z3");
}

// Development flexibility tests
#[cfg(not(feature = "strict-z3"))]
#[test] 
fn test_graceful_z3_handling() {
    if !is_z3_available() {
        println!("⚠️ Z3 not available - skipping formal verification");
        return;
    }
    // Full verification when available
}
```

### **Phase 3: Deployment Pipeline**
```yaml
# .github/workflows/production.yml
- name: Production Build
  run: |
    make setup  # Ensure Z3 installation
    make build-prod  # Strict Z3 requirements
    make test-z3    # Full formal verification
```

## Consequences

### ✅ **Positive Impacts**

#### **Security Assurance**
- **Guaranteed** formal verification in production
- **Predictable** security verification behavior
- **Compliance** with enterprise security standards

#### **Development Flexibility**  
- **Local development** remains productive
- **CI/CD** can adapt to environment capabilities
- **Testing** can occur at multiple verification levels

#### **Operational Excellence**
- **Clear deployment requirements** prevent configuration surprises
- **Fail-fast** behavior catches missing dependencies early
- **Consistent** production behavior across environments

### ⚠️ **Considerations**

#### **Deployment Complexity**
- **Production environments** must have Z3 properly configured
- **Container images** need Z3 dependencies included
- **Infrastructure** requirements become more specific

#### **Development Setup**
- **New developers** may encounter setup complexity
- **CI environments** need proper Z3 configuration
- **Documentation** must clearly explain requirements

## Migration Plan

### **Week 1: Feature Flag Implementation**
- Add `strict-z3` feature flag
- Update conditional compilation blocks
- Maintain backward compatibility

### **Week 2: Test Architecture Update**  
- Split tests into production/development categories
- Update formal verification test suite
- Validate both modes work correctly

### **Week 3: Documentation and Deployment**
- Update Makefile with production commands
- Create deployment documentation
- Establish enterprise build pipeline

### **Week 4: Rollout**
- Enable strict mode for production builds
- Monitor deployment success
- Gather feedback and iterate

## Related ADRs

- **ADR-003**: Formal Verification Test Architecture
- **ADR-029**: Production-Ready Architecture Completion
- **ADR-025**: Security Assessment Update

---

**Decision made by:** Software Architect  
**Implementation priority:** HIGH (Production Security)  
**Risk level:** LOW (Maintains development flexibility)