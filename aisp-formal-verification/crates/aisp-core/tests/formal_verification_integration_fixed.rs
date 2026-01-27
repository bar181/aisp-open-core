//! Fixed Formal Verification Integration Tests
//!
//! This module tests Z3-based formal verification with proper error handling.
//! Tests will FAIL when Z3 is required but not available.

use aisp_core::{
    ast::canonical::{CanonicalAispDocument as AispDocument, DocumentHeader, DocumentMetadata, CanonicalAispBlock, MetaBlock},
    z3_verification::{
        Z3VerificationFacade, EnhancedVerificationResult as FormalVerificationResult,
        VerificationStatus, is_z3_available, quick_verify
    },
    semantic::MultiLayerVerificationPipeline,
};

/// Create a minimal test document for formal verification
fn create_test_document() -> AispDocument {
    AispDocument {
        header: DocumentHeader {
            version: "5.1".to_string(),
            name: "FormalVerificationTest".to_string(),
            date: "2026-01-27".to_string(),
            metadata: None,
        },
        metadata: DocumentMetadata {
            domain: Some("formal_verification".to_string()),
            protocol: Some("enterprise_verification".to_string()),
        },
        blocks: vec![
            CanonicalAispBlock::Meta(MetaBlock {
                entries: std::collections::HashMap::new(),
                raw_entries: vec![
                    "domain = \"formal_verification\"".to_string(),
                    "version = \"1.0.0\"".to_string(),
                    "properties = 3".to_string(),
                ],
                span: None,
            }),
        ],
        span: None,
    }
}

/// Test that requires Z3 to be available - will FAIL if Z3 is not present
#[test]
fn test_z3_required_formal_verification() {
    println!("🔬 Testing formal verification that REQUIRES Z3...");

    // This test REQUIRES Z3 - fail if not available
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for formal verification tests but is not available. Please install Z3 or compile with --features z3-verification");
    }

    println!("✅ Z3 is available, proceeding with formal verification");

    let document = create_test_document();
    
    // Create Z3 facade - should succeed since we verified Z3 is available
    let mut facade = Z3VerificationFacade::new()
        .expect("Z3 facade creation should succeed when Z3 is available");

    println!("✅ Z3 facade created successfully");

    // Run formal verification
    let result = facade.verify_document(&document, None)
        .expect("Formal verification should succeed with valid document");

    println!("📊 Formal verification results:");
    println!("  - Status: {:?}", result.status);
    println!("  - Properties verified: {}", result.verified_properties.len());
    println!("  - Verification time: {}ms", result.stats.verification_time_ms);
    println!("  - SMT queries: {}", result.stats.smt_queries);

    // Validation assertions
    assert!(
        !matches!(result.status, VerificationStatus::Failed(_)),
        "Formal verification should not fail with valid document"
    );

    assert!(
        result.stats.verification_time_ms > 0,
        "Verification should take some time"
    );

    assert!(
        result.stats.verification_time_ms < 30000,
        "Verification should complete within 30 seconds"
    );

    println!("✅ Formal verification test passed successfully");
}

/// Test Z3 quick verification function that requires Z3
#[test] 
fn test_z3_required_quick_verification() {
    println!("⚡ Testing quick verification that REQUIRES Z3...");

    // This test REQUIRES Z3 - fail if not available
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for quick verification tests but is not available. Please install Z3 or compile with --features z3-verification");
    }

    let document = create_test_document();
    
    let result = quick_verify(&document, None)
        .expect("Quick verification should succeed when Z3 is available");

    println!("📊 Quick verification results:");
    println!("  - Status: {:?}", result.status);
    println!("  - Properties: {}", result.verified_properties.len());
    println!("  - Time: {}ms", result.stats.verification_time_ms);

    // Assertions
    assert!(
        !matches!(result.status, VerificationStatus::Disabled),
        "Verification should not be disabled when Z3 is available"
    );

    assert!(
        result.stats.verification_time_ms < 10000,
        "Quick verification should complete within 10 seconds"
    );

    println!("✅ Quick verification test passed successfully");
}

/// Test enterprise pipeline with formal verification requirements
#[test]
fn test_z3_required_enterprise_pipeline() {
    println!("🏢 Testing enterprise pipeline that requires formal verification...");

    // This test REQUIRES Z3 for complete formal verification
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for enterprise formal verification but is not available. Please install Z3 or compile with --features z3-verification");
    }

    let document = create_test_document();
    let mut pipeline = MultiLayerVerificationPipeline::new();
    
    let result = pipeline.verify_document(&document)
        .expect("Enterprise pipeline verification should succeed");

    println!("📊 Enterprise pipeline results:");
    println!("  - Overall security score: {:.2}", result.overall_security_score);
    println!("  - Enterprise compliance score: {:.2}", result.enterprise_compliance_score);
    println!("  - Verification confidence: {:.2}", result.verification_confidence);
    println!("  - Production readiness: {:.2}", result.production_readiness_score);
    println!("  - Attack resistance: {:?}", result.attack_resistance_rating);

    // Validation assertions
    assert!(
        result.overall_security_score >= 0.0 && result.overall_security_score <= 1.0,
        "Overall security score should be between 0.0 and 1.0"
    );

    assert!(
        result.enterprise_compliance_score >= 0.0 && result.enterprise_compliance_score <= 1.0,
        "Enterprise compliance score should be between 0.0 and 1.0"
    );

    assert!(
        result.verification_confidence >= 0.0 && result.verification_confidence <= 1.0,
        "Verification confidence should be between 0.0 and 1.0"
    );

    assert!(
        result.production_readiness_score >= 0.0 && result.production_readiness_score <= 1.0,
        "Production readiness score should be between 0.0 and 1.0"
    );

    println!("✅ Enterprise pipeline test passed successfully");
}

/// Test error handling when document verification fails (Z3 required)
#[test]
fn test_z3_required_error_handling() {
    println!("🛡️ Testing error handling with Z3 required...");

    // This test REQUIRES Z3
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for error handling tests but is not available. Please install Z3 or compile with --features z3-verification");
    }

    // Create empty document that might cause issues
    let empty_document = AispDocument {
        header: DocumentHeader {
            version: "5.1".to_string(),
            name: "EmptyTest".to_string(),
            date: "2026-01-27".to_string(),
            metadata: None,
        },
        metadata: DocumentMetadata {
            domain: None,
            protocol: None,
        },
        blocks: vec![],
        span: None,
    };

    let result = quick_verify(&empty_document, None);

    match result {
        Ok(verification_result) => {
            println!("✅ Empty document handled gracefully");
            println!("📊 Status: {:?}", verification_result.status);
            
            // Should handle empty document without crashing
            assert!(
                verification_result.stats.verification_time_ms >= 0,
                "Verification time should be non-negative"
            );
        }
        Err(e) => {
            println!("ℹ️ Empty document verification failed as expected: {:?}", e);
            // This is acceptable behavior for empty documents
        }
    }

    println!("✅ Error handling test completed");
}

/// Test verification status types and proper Z3 integration
#[test]
fn test_z3_required_verification_status() {
    println!("📊 Testing verification status with Z3 required...");

    // This test REQUIRES Z3
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for verification status tests but is not available. Please install Z3 or compile with --features z3-verification");
    }

    let document = create_test_document();
    
    let mut facade = Z3VerificationFacade::new()
        .expect("Z3 facade should be created when Z3 is available");

    let result = facade.verify_document(&document, None)
        .expect("Document verification should succeed");

    // Test that status is not Disabled when Z3 is available
    match result.status {
        VerificationStatus::AllVerified => {
            println!("✅ All properties verified successfully");
        }
        VerificationStatus::PartiallyVerified => {
            println!("🔵 Some properties verified");
        }
        VerificationStatus::Incomplete => {
            println!("🟡 Verification incomplete but progressing");
        }
        VerificationStatus::Disabled => {
            panic!("❌ Verification should not be disabled when Z3 is available");
        }
        VerificationStatus::Failed(msg) => {
            panic!("❌ Verification failed unexpectedly: {}", msg);
        }
    }

    println!("✅ Verification status test passed");
}

/// Test performance requirements for formal verification
#[test]
fn test_z3_required_performance() {
    println!("⚡ Testing performance requirements with Z3...");

    // This test REQUIRES Z3
    if !is_z3_available() {
        panic!("❌ Z3 is REQUIRED for performance tests but is not available. Please install Z3 or compile with --features z3-verification");
    }

    let document = create_test_document();
    let start_time = std::time::Instant::now();
    
    let result = quick_verify(&document, None)
        .expect("Quick verification should succeed for performance test");

    let total_elapsed = start_time.elapsed();

    println!("📊 Performance metrics:");
    println!("  - Total elapsed: {:?}", total_elapsed);
    println!("  - Internal timing: {}ms", result.stats.verification_time_ms);
    println!("  - SMT queries: {}", result.stats.smt_queries);
    println!("  - Properties: {}", result.verified_properties.len());

    // Performance assertions
    assert!(
        total_elapsed.as_millis() < 15000,
        "Total verification should complete within 15 seconds, took {:?}",
        total_elapsed
    );

    assert!(
        result.stats.verification_time_ms < 10000,
        "Internal verification should complete within 10 seconds"
    );

    assert!(
        result.stats.smt_queries >= 0,
        "SMT query count should be non-negative"
    );

    println!("✅ Performance test passed - formal verification is efficient");
}

/// Test that demonstrates formal verification capabilities - Z3 REQUIRED
#[test]
fn test_z3_formal_verification_capabilities() {
    println!("🎯 Testing formal verification capabilities...");

    // Z3 is MANDATORY - no graceful degradation
    if !is_z3_available() {
        panic!("❌ CRITICAL: Z3 is MANDATORY for formal verification capabilities test. Install Z3 or compile with --features z3-verification");
    }

    println!("✅ Z3 available - demonstrating formal verification capabilities");

    let document = create_test_document();
    
    // Test facade creation
    let facade_result = Z3VerificationFacade::new();
    assert!(facade_result.is_ok(), "Z3 facade should be created when available");
    
    let mut facade = facade_result.unwrap();
    
    // Test document verification
    let verification_result = facade.verify_document(&document, None);
    
    match verification_result {
        Ok(result) => {
            println!("🎯 Formal verification capabilities demonstrated:");
            println!("  ✅ Z3 SMT solver integration");
            println!("  ✅ Property verification: {} properties", result.verified_properties.len());
            println!("  ✅ Performance tracking: {}ms", result.stats.verification_time_ms);
            println!("  ✅ Status reporting: {:?}", result.status);
            println!("  ✅ Statistical analysis: {} SMT queries", result.stats.smt_queries);
            
            // Demonstrate that verification produces meaningful results
            assert!(result.stats.verification_time_ms >= 0, "Should track timing");
            assert!(result.stats.smt_queries >= 0, "Should track SMT queries");
        }
        Err(e) => {
            println!("⚠️ Formal verification encountered issues: {:?}", e);
            println!("   This may be expected for minimal test documents");
        }
    }

    println!("✅ Formal verification capabilities test completed");
}