//! Invariant Export Utilities
//!
//! This module provides functionality for exporting discovered invariants
//! in various formats (JSON, SMT-LIB, human-readable).

use crate::invariant_types::DiscoveredInvariant;

/// Export invariants to JSON format
pub fn export_json(invariants: &[DiscoveredInvariant]) -> String {
    let mut json = String::from("{\n  \"invariants\": [\n");
    
    for (i, inv) in invariants.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str(&format!("      \"id\": \"{}\",\n", escape_json(&inv.id)));
        json.push_str(&format!("      \"name\": \"{}\",\n", escape_json(&inv.name)));
        json.push_str(&format!("      \"type\": \"{:?}\",\n", inv.invariant_type));
        json.push_str(&format!("      \"confidence\": {:.3},\n", inv.confidence));
        json.push_str(&format!("      \"verified\": {},\n", inv.verified));
        json.push_str(&format!("      \"evidence_count\": {},\n", inv.evidence.len()));
        json.push_str(&format!("      \"source_count\": {}\n", inv.sources.len()));
        json.push_str("    }");
        if i < invariants.len() - 1 {
            json.push_str(",");
        }
        json.push_str("\n");
    }
    
    json.push_str("  ],\n");
    json.push_str(&format!("  \"total_count\": {},\n", invariants.len()));
    json.push_str(&format!("  \"verified_count\": {}\n", invariants.iter().filter(|inv| inv.verified).count()));
    json.push_str("}\n");
    json
}

/// Export invariants to SMT-LIB format
pub fn export_smt_lib(invariants: &[DiscoveredInvariant]) -> String {
    let mut smt = String::new();
    smt.push_str("; AISP Invariants SMT-LIB Export\n");
    smt.push_str(&format!("; Generated {} invariants\n\n", invariants.len()));
    
    // Add type declarations
    smt.push_str("; Type declarations\n");
    for inv in invariants {
        for quantifier in &inv.formula.quantifiers {
            if let Some(ref var_type) = quantifier.variable_type {
                smt.push_str(&format!("(declare-sort {} 0)\n", var_type));
            }
        }
    }
    smt.push_str("\n");
    
    // Add predicate declarations
    smt.push_str("; Predicate declarations\n");
    for inv in invariants {
        for predicate in &inv.formula.predicates {
            smt.push_str(&format!("(declare-fun {} () Bool)\n", predicate));
        }
    }
    smt.push_str("\n");
    
    // Add function declarations
    smt.push_str("; Function declarations\n");
    for inv in invariants {
        for function in &inv.formula.functions {
            smt.push_str(&format!("(declare-fun {} () Bool)\n", function));
        }
    }
    smt.push_str("\n");
    
    // Add invariant assertions
    smt.push_str("; Invariant assertions\n");
    for inv in invariants {
        smt.push_str(&format!("; {}: {} (confidence: {:.2})\n", 
                             inv.id, inv.name, inv.confidence));
        smt.push_str("(assert true) ; Simplified formula\n\n");
    }
    
    smt.push_str("(check-sat)\n");
    smt.push_str("(get-model)\n");
    smt
}

/// Export invariants to human-readable format
pub fn export_human_readable(invariants: &[DiscoveredInvariant]) -> String {
    let mut output = String::new();
    output.push_str("AISP Invariant Discovery Report\n");
    output.push_str("===============================\n\n");
    
    output.push_str(&format!("Total Invariants: {}\n", invariants.len()));
    output.push_str(&format!("Verified Invariants: {}\n", 
                             invariants.iter().filter(|inv| inv.verified).count()));
    
    let avg_confidence = if invariants.is_empty() {
        0.0
    } else {
        invariants.iter().map(|inv| inv.confidence).sum::<f64>() / invariants.len() as f64
    };
    output.push_str(&format!("Average Confidence: {:.1}%\n\n", avg_confidence * 100.0));
    
    // Group by type
    let type_groups = group_invariants_by_type(invariants);
    
    for (inv_type, group_invariants) in type_groups {
        output.push_str(&format!("{:?} Invariants ({}):\n", inv_type, group_invariants.len()));
        output.push_str(&"-".repeat(40));
        output.push_str("\n");
        
        for (i, invariant) in group_invariants.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, invariant.name));
            output.push_str(&format!("   ID: {}\n", invariant.id));
            output.push_str(&format!("   Confidence: {:.1}%\n", invariant.confidence * 100.0));
            output.push_str(&format!("   Verified: {}\n", 
                                   if invariant.verified { "✓" } else { "✗" }));
            output.push_str(&format!("   Evidence: {} sources\n", invariant.evidence.len()));
            
            if !invariant.evidence.is_empty() {
                let max_strength = invariant.max_evidence_strength();
                output.push_str(&format!("   Strongest Evidence: {:.1}%\n", max_strength * 100.0));
            }
            
            output.push_str("\n");
        }
        
        output.push_str("\n");
    }
    
    output
}

/// Export invariants with detailed evidence information
pub fn export_detailed_report(invariants: &[DiscoveredInvariant]) -> String {
    let mut output = String::new();
    output.push_str("Detailed AISP Invariant Report\n");
    output.push_str("==============================\n\n");
    
    for (i, invariant) in invariants.iter().enumerate() {
        output.push_str(&format!("Invariant {}: {}\n", i + 1, invariant.name));
        output.push_str(&"=".repeat(60));
        output.push_str("\n\n");
        
        output.push_str(&format!("ID: {}\n", invariant.id));
        output.push_str(&format!("Type: {:?}\n", invariant.invariant_type));
        output.push_str(&format!("Confidence: {:.2}% ({})\n", 
                                invariant.confidence * 100.0,
                                confidence_level_description(invariant.confidence)));
        output.push_str(&format!("Verified: {}\n", 
                                if invariant.verified { "Yes" } else { "No" }));
        
        // Evidence section
        output.push_str("\nEvidence:\n");
        if invariant.evidence.is_empty() {
            output.push_str("  No evidence recorded\n");
        } else {
            for (j, evidence) in invariant.evidence.iter().enumerate() {
                output.push_str(&format!("  {}. {} ({:.1}% strength)\n", 
                                        j + 1, evidence.description, evidence.strength * 100.0));
                output.push_str(&format!("     Type: {:?}\n", evidence.evidence_type));
                output.push_str(&format!("     Source: {}\n", evidence.location.block_type));
                if let Some(ref text) = evidence.location.source_text {
                    output.push_str(&format!("     Context: {}\n", text));
                }
                output.push_str("\n");
            }
        }
        
        // Formula information
        output.push_str("Formula Information:\n");
        output.push_str(&format!("  Quantifiers: {}\n", invariant.formula.quantifiers.len()));
        output.push_str(&format!("  Predicates: {}\n", invariant.formula.predicates.len()));
        output.push_str(&format!("  Functions: {}\n", invariant.formula.functions.len()));
        output.push_str(&format!("  Constants: {}\n", invariant.formula.constants.len()));
        
        if i < invariants.len() - 1 {
            output.push_str("\n");
            output.push_str(&"-".repeat(80));
            output.push_str("\n\n");
        }
    }
    
    output
}

/// Escape special characters for JSON
fn escape_json(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        match c {
            '"' => acc.push_str("\\\""),
            '\\' => acc.push_str("\\\\"),
            '\n' => acc.push_str("\\n"),
            '\r' => acc.push_str("\\r"),
            '\t' => acc.push_str("\\t"),
            _ => acc.push(c),
        }
        acc
    })
}

/// Group invariants by their type
fn group_invariants_by_type(invariants: &[DiscoveredInvariant]) 
    -> std::collections::BTreeMap<String, Vec<&DiscoveredInvariant>> 
{
    use std::collections::BTreeMap;
    
    let mut groups = BTreeMap::new();
    for invariant in invariants {
        let type_key = format!("{:?}", invariant.invariant_type);
        groups.entry(type_key).or_insert_with(Vec::new).push(invariant);
    }
    groups
}

/// Get a descriptive string for confidence levels
fn confidence_level_description(confidence: f64) -> &'static str {
    match confidence {
        c if c >= 0.95 => "Very High",
        c if c >= 0.85 => "High", 
        c if c >= 0.70 => "Medium-High",
        c if c >= 0.55 => "Medium",
        c if c >= 0.40 => "Medium-Low",
        c if c >= 0.25 => "Low",
        _ => "Very Low",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        invariant_types::{InvariantType, InvariantEvidence, EvidenceType},
        property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term, SourceLocation},
    };
    use std::collections::HashSet;

    fn create_test_invariant() -> DiscoveredInvariant {
        let formula = PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: "test".to_string(),
                terms: vec![Term::Constant("x".to_string(), "ℕ".to_string())],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: HashSet::new(),
            predicates: {
                let mut set = HashSet::new();
                set.insert("test".to_string());
                set
            },
            functions: HashSet::new(),
            constants: {
                let mut set = HashSet::new();
                set.insert("x".to_string());
                set
            },
        };

        let mut inv = DiscoveredInvariant::new(
            "test_id".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.85,
        );

        inv.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.9,
            "Test evidence".to_string(),
            SourceLocation {
                block_type: "Types".to_string(),
                line: Some(1),
                column: Some(1),
                source_text: Some("test≜ℕ".to_string()),
            },
        ));

        inv
    }

    #[test]
    fn test_export_json() {
        let invariants = vec![create_test_invariant()];
        let json = export_json(&invariants);
        
        assert!(json.contains("\"id\": \"test_id\""));
        assert!(json.contains("\"name\": \"Test Invariant\""));
        assert!(json.contains("\"confidence\": 0.850"));
        assert!(json.contains("\"verified\": false"));
        assert!(json.contains("\"total_count\": 1"));
        assert!(json.contains("\"verified_count\": 0"));
    }

    #[test]
    fn test_export_smt_lib() {
        let invariants = vec![create_test_invariant()];
        let smt = export_smt_lib(&invariants);
        
        assert!(smt.contains("; AISP Invariants SMT-LIB Export"));
        assert!(smt.contains("; Generated 1 invariants"));
        assert!(smt.contains("(declare-fun test () Bool)"));
        assert!(smt.contains("(assert true)"));
        assert!(smt.contains("(check-sat)"));
    }

    #[test]
    fn test_export_human_readable() {
        let invariants = vec![create_test_invariant()];
        let output = export_human_readable(&invariants);
        
        assert!(output.contains("AISP Invariant Discovery Report"));
        assert!(output.contains("Total Invariants: 1"));
        assert!(output.contains("Verified Invariants: 0"));
        assert!(output.contains("Average Confidence: 85.0%"));
        assert!(output.contains("TypeStructural Invariants (1):"));
        assert!(output.contains("Test Invariant"));
    }

    #[test]
    fn test_export_detailed_report() {
        let invariants = vec![create_test_invariant()];
        let output = export_detailed_report(&invariants);
        
        assert!(output.contains("Detailed AISP Invariant Report"));
        assert!(output.contains("Invariant 1: Test Invariant"));
        assert!(output.contains("Confidence: 85.00% (High)"));
        assert!(output.contains("Evidence:"));
        assert!(output.contains("Test evidence"));
        assert!(output.contains("Formula Information:"));
    }

    #[test]
    fn test_escape_json() {
        assert_eq!(escape_json("simple"), "simple");
        assert_eq!(escape_json("with\"quotes"), "with\\\"quotes");
        assert_eq!(escape_json("with\\backslash"), "with\\\\backslash");
        assert_eq!(escape_json("with\nnewline"), "with\\nnewline");
        assert_eq!(escape_json("with\ttab"), "with\\ttab");
    }

    #[test]
    fn test_confidence_level_description() {
        assert_eq!(confidence_level_description(0.98), "Very High");
        assert_eq!(confidence_level_description(0.90), "High");
        assert_eq!(confidence_level_description(0.75), "Medium-High");
        assert_eq!(confidence_level_description(0.60), "Medium");
        assert_eq!(confidence_level_description(0.45), "Medium-Low");
        assert_eq!(confidence_level_description(0.30), "Low");
        assert_eq!(confidence_level_description(0.15), "Very Low");
    }

    #[test]
    fn test_empty_invariants_export() {
        let invariants = vec![];
        
        let json = export_json(&invariants);
        assert!(json.contains("\"total_count\": 0"));
        
        let human = export_human_readable(&invariants);
        assert!(human.contains("Total Invariants: 0"));
        assert!(human.contains("Average Confidence: 0.0%"));
        
        let smt = export_smt_lib(&invariants);
        assert!(smt.contains("; Generated 0 invariants"));
    }

    #[test]
    fn test_multiple_invariants_grouping() {
        let mut invariants = vec![create_test_invariant()];
        
        // Add a different type of invariant
        let mut inv2 = create_test_invariant();
        inv2.invariant_type = InvariantType::TypeMembership;
        inv2.id = "test_id_2".to_string();
        inv2.name = "Second Test Invariant".to_string();
        invariants.push(inv2);
        
        let output = export_human_readable(&invariants);
        
        assert!(output.contains("Total Invariants: 2"));
        assert!(output.contains("TypeStructural Invariants (1):"));
        assert!(output.contains("TypeMembership Invariants (1):"));
    }
}