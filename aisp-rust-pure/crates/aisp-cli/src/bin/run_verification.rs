//! AISP Formal Verification Runner
//!
//! Demonstrates the complete formal verification workflow including
//! invariant discovery, satisfiability checking, and proof generation.

use aisp_core::{
    ast::{AispDocument, DocumentHeader, DocumentMetadata, AispBlock, TypesBlock, TypeDefinition, TypeExpression, BasicType, Span, Position},
    formal_verification::{FormalVerifier, VerificationConfig, VerificationMethod},
    invariant_discovery::InvariantDiscovery,
    satisfiability_checker::SatisfiabilityChecker,
    parser_new::AispParser,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 AISP Formal Verification System");
    println!("===================================\n");

    // Test 1: Simple document verification
    println!("📋 Test 1: Simple AISP Document Verification");
    run_simple_verification()?;

    // Test 2: Complex document with multiple types
    println!("\n📋 Test 2: Complex Document with Multiple Types");
    run_complex_verification()?;

    // Test 3: Parser integration test
    println!("\n📋 Test 3: Full Parser Integration Test");
    run_parser_integration_test()?;

    // Test 4: Performance analysis
    println!("\n📋 Test 4: Performance Analysis");
    run_performance_analysis()?;

    // Test 5: Verification method comparison
    println!("\n📋 Test 5: Verification Method Comparison");
    run_method_comparison()?;

    println!("\n✅ All verification tests completed successfully!");
    Ok(())
}

fn run_simple_verification() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test document
    let document = create_simple_test_document();
    
    println!("   📄 Document: {}", document.header.name);
    println!("   📅 Version: {}", document.header.version);
    
    // Create verifier with default configuration
    let mut verifier = FormalVerifier::new();
    
    // Run verification
    let start_time = Instant::now();
    let result = verifier.verify_document(&document)?;
    let verification_time = start_time.elapsed();
    
    // Display results
    println!("   ⏱️  Verification time: {:?}", verification_time);
    println!("   🔍 Status: {:?}", result.status);
    println!("   📊 Invariants processed: {}", result.statistics.invariants_processed);
    println!("   ✅ Invariants verified: {}", result.statistics.invariants_verified);
    println!("   🧮 Proofs generated: {}", result.statistics.proofs_generated);
    
    if !result.verified_invariants.is_empty() {
        println!("   🎯 First verified invariant: {}", result.verified_invariants[0].invariant.name);
        println!("   🏗️  Proof method: {:?}", result.verified_invariants[0].verification_method);
        println!("   💪 Confidence: {:.2}%", result.verified_invariants[0].verification_confidence * 100.0);
    }
    
    Ok(())
}

fn run_complex_verification() -> Result<(), Box<dyn std::error::Error>> {
    // Create a complex test document
    let document = create_complex_test_document();
    
    println!("   📄 Document: {}", document.header.name);
    println!("   🔢 Type blocks: {}", document.blocks.len());
    
    // Create verifier with custom configuration
    let config = VerificationConfig {
        total_timeout: Duration::from_secs(60),
        proof_timeout: Duration::from_secs(10),
        enabled_methods: vec![
            VerificationMethod::DirectProof,
            VerificationMethod::SmtSolverVerification,
            VerificationMethod::AutomatedProof,
        ],
        proof_confidence_threshold: 0.7,
        parallel_verification: true,
        worker_threads: 2,
        enable_proof_cache: true,
        ..Default::default()
    };
    
    let mut verifier = FormalVerifier::with_config(config);
    
    // Run verification
    let result = verifier.verify_document(&document)?;
    
    // Display detailed results
    println!("   🔍 Status: {:?}", result.status);
    println!("   📊 Statistics:");
    println!("      • Total time: {:?}", result.statistics.total_time);
    println!("      • Invariants processed: {}", result.statistics.invariants_processed);
    println!("      • Invariants verified: {}", result.statistics.invariants_verified);
    println!("      • Average proof time: {:?}", result.statistics.avg_proof_time);
    println!("      • Peak memory usage: {} bytes", result.statistics.memory_stats.peak_usage);
    
    // Show method distribution
    println!("   🛠️  Method distribution:");
    for (method, count) in &result.statistics.method_distribution {
        println!("      • {:?}: {} proofs", method, count);
    }
    
    // Show proof complexity analysis
    if !result.proofs.is_empty() {
        let avg_complexity = result.proofs.iter()
            .map(|p| p.complexity.complexity_rating as f64)
            .sum::<f64>() / result.proofs.len() as f64;
        println!("   🧠 Average proof complexity: {:.1}/10", avg_complexity);
        
        let simple_proofs = result.proofs.iter().filter(|p| p.complexity.is_simple()).count();
        let complex_proofs = result.proofs.iter().filter(|p| p.complexity.is_complex()).count();
        println!("   📈 Proof complexity distribution: {} simple, {} complex", simple_proofs, complex_proofs);
    }
    
    Ok(())
}

fn run_parser_integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let aisp_text = r#"𝔸5.1.VerificationDemo@2026-01-26

⟦Σ:Types⟧{
  Counter≜ℕ
  Status≜{Active,Inactive,Pending}
  Balance≜ℕ
  Flag≜𝔹
}

⟦Γ:Rules⟧{
  ∀c:Counter→c≥0
  ∀s:Status→s∈{Active,Inactive,Pending}
  ∀b:Balance→b≥0
}

⟦Λ:Funcs⟧{
  increment≜λx:ℕ.x+1
  reset≜λx:ℕ.0
}
"#;

    println!("   📝 Parsing AISP document...");
    
    // Parse the document
    let mut parser = AispParser::new(aisp_text.to_string());
    let document = parser.parse()?;
    
    println!("   ✅ Document parsed successfully");
    println!("   📄 Name: {}", document.header.name);
    println!("   📅 Date: {}", document.header.date);
    
    // Run full verification workflow
    println!("   🔬 Running formal verification...");
    
    let mut verifier = FormalVerifier::new();
    let result = verifier.verify_document(&document)?;
    
    println!("   📊 Verification Results:");
    println!("      • Status: {:?}", result.status);
    println!("      • Processed: {} invariants", result.statistics.invariants_processed);
    println!("      • Verified: {} invariants", result.statistics.invariants_verified);
    println!("      • Generated: {} proofs", result.statistics.proofs_generated);
    
    // Show individual proof details
    for (i, proof) in result.proofs.iter().enumerate().take(3) {
        println!("   🧮 Proof {} ({}): {} steps, complexity {}/10", 
                 i + 1, 
                 proof.id[..8].to_string() + "...", 
                 proof.complexity.steps,
                 proof.complexity.complexity_rating);
    }
    
    if result.proofs.len() > 3 {
        println!("   ... and {} more proofs", result.proofs.len() - 3);
    }
    
    Ok(())
}

fn run_performance_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("   🚀 Running performance analysis...");
    
    // Create documents of varying complexity
    let simple_doc = create_simple_test_document();
    let complex_doc = create_complex_test_document();
    let large_doc = create_large_test_document();
    
    let documents = vec![
        ("Simple", simple_doc),
        ("Complex", complex_doc), 
        ("Large", large_doc),
    ];
    
    for (name, document) in documents {
        println!("   📊 Testing {} document:", name);
        
        let mut verifier = FormalVerifier::new();
        let start_time = Instant::now();
        
        match verifier.verify_document(&document) {
            Ok(result) => {
                let total_time = start_time.elapsed();
                println!("      • Total time: {:?}", total_time);
                println!("      • Invariants: {}/{} verified", 
                         result.statistics.invariants_verified,
                         result.statistics.invariants_processed);
                println!("      • Memory peak: {} KB", 
                         result.statistics.memory_stats.peak_usage / 1024);
                
                if !result.proofs.is_empty() {
                    let avg_proof_time = result.statistics.avg_proof_time;
                    println!("      • Avg proof time: {:?}", avg_proof_time);
                }
            }
            Err(e) => {
                println!("      • Failed: {}", e);
            }
        }
    }
    
    Ok(())
}

fn run_method_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔄 Comparing verification methods...");
    
    let document = create_simple_test_document();
    
    let methods_to_test = vec![
        ("Direct Proof", vec![VerificationMethod::DirectProof]),
        ("SMT Verification", vec![VerificationMethod::SmtSolverVerification]),
        ("Automated Proof", vec![VerificationMethod::AutomatedProof]),
        ("Hybrid Approach", vec![
            VerificationMethod::DirectProof,
            VerificationMethod::SmtSolverVerification,
            VerificationMethod::AutomatedProof,
        ]),
    ];
    
    for (name, methods) in methods_to_test {
        println!("   🧪 Testing {}:", name);
        
        let config = VerificationConfig {
            enabled_methods: methods,
            proof_timeout: Duration::from_secs(5),
            ..Default::default()
        };
        
        let mut verifier = FormalVerifier::with_config(config);
        let start_time = Instant::now();
        
        match verifier.verify_document(&document) {
            Ok(result) => {
                let verification_time = start_time.elapsed();
                println!("      • Time: {:?}", verification_time);
                println!("      • Success rate: {}/{}", 
                         result.statistics.invariants_verified,
                         result.statistics.invariants_processed);
                
                if !result.proofs.is_empty() {
                    let avg_complexity = result.proofs.iter()
                        .map(|p| p.complexity.complexity_rating as f64)
                        .sum::<f64>() / result.proofs.len() as f64;
                    println!("      • Avg complexity: {:.1}/10", avg_complexity);
                }
            }
            Err(e) => {
                println!("      • Failed: {}", e);
            }
        }
    }
    
    Ok(())
}

// Helper functions to create test documents

fn create_simple_test_document() -> AispDocument {
    let mut types = HashMap::new();
    types.insert("Counter".to_string(), TypeDefinition {
        name: "Counter".to_string(),
        type_expr: TypeExpression::Basic(BasicType::Natural),
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    });

    AispDocument {
        header: DocumentHeader {
            version: "5.1".to_string(),
            name: "SimpleVerificationTest".to_string(),
            date: "2026-01-26".to_string(),
            metadata: None,
        },
        metadata: DocumentMetadata {
            domain: None,
            protocol: None,
        },
        blocks: vec![
            AispBlock::Types(TypesBlock {
                definitions: types,
                span: Span { 
                    start: Position { line: 1, column: 1, offset: 0 }, 
                    end: Position { line: 1, column: 1, offset: 0 } 
                },
            }),
        ],
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    }
}

fn create_complex_test_document() -> AispDocument {
    let mut types = HashMap::new();
    
    types.insert("Counter".to_string(), TypeDefinition {
        name: "Counter".to_string(),
        type_expr: TypeExpression::Basic(BasicType::Natural),
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    });
    
    types.insert("Status".to_string(), TypeDefinition {
        name: "Status".to_string(),
        type_expr: TypeExpression::Enumeration(vec![
            "Active".to_string(),
            "Inactive".to_string(),
            "Pending".to_string(),
        ]),
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    });
    
    types.insert("Flag".to_string(), TypeDefinition {
        name: "Flag".to_string(),
        type_expr: TypeExpression::Basic(BasicType::Boolean),
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    });

    AispDocument {
        header: DocumentHeader {
            version: "5.1".to_string(),
            name: "ComplexVerificationTest".to_string(),
            date: "2026-01-26".to_string(),
            metadata: None,
        },
        metadata: DocumentMetadata {
            domain: None,
            protocol: None,
        },
        blocks: vec![
            AispBlock::Types(TypesBlock {
                definitions: types,
                span: Span { 
                    start: Position { line: 1, column: 1, offset: 0 }, 
                    end: Position { line: 1, column: 1, offset: 0 } 
                },
            }),
        ],
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    }
}

fn create_large_test_document() -> AispDocument {
    let mut types = HashMap::new();
    
    // Create many types to test scalability
    for i in 0..10 {
        types.insert(format!("Counter{}", i), TypeDefinition {
            name: format!("Counter{}", i),
            type_expr: TypeExpression::Basic(BasicType::Natural),
            span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
        });
        
        types.insert(format!("Flag{}", i), TypeDefinition {
            name: format!("Flag{}", i),
            type_expr: TypeExpression::Basic(BasicType::Boolean),
            span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
        });
        
        types.insert(format!("Status{}", i), TypeDefinition {
            name: format!("Status{}", i),
            type_expr: TypeExpression::Enumeration(vec![
                format!("State{}A", i),
                format!("State{}B", i),
                format!("State{}C", i),
            ]),
            span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
        });
    }

    AispDocument {
        header: DocumentHeader {
            version: "5.1".to_string(),
            name: "LargeScaleVerificationTest".to_string(),
            date: "2026-01-26".to_string(),
            metadata: None,
        },
        metadata: DocumentMetadata {
            domain: None,
            protocol: None,
        },
        blocks: vec![
            AispBlock::Types(TypesBlock {
                definitions: types,
                span: Span { 
                    start: Position { line: 1, column: 1, offset: 0 }, 
                    end: Position { line: 1, column: 1, offset: 0 } 
                },
            }),
        ],
        span: Span { 
            start: Position { line: 1, column: 1, offset: 0 }, 
            end: Position { line: 1, column: 1, offset: 0 } 
        },
    }
}