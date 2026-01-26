use aisp_core::validator::AispValidator;

fn main() {
    let document = r#"
𝔸5.1.formal-test@2026-01-26

⟦Ω:Meta⟧{
  domain≜formal-testing
}

⟦Σ:Types⟧{
  Unit≜{unit}
}

⟦Γ:Rules⟧{
  ∀x:Unit→Valid(x)
}

⟦Λ:Functions⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.85;φ≜120;τ≜◊⁺⁺⟩
"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);
    
    println!("Valid: {}", result.valid);
    println!("Tier: {:?}", result.tier);
    println!("Delta: {}", result.delta);
    println!("Ambiguity: {}", result.ambiguity);
    
    if let Some(error) = &result.error {
        println!("Error: {}", error);
    }
    
    if let Some(analysis) = &result.semantic_analysis {
        println!("Warnings count: {}", analysis.warnings.len());
        for warning in &analysis.warnings {
            println!("Warning: {}", warning);
        }
    }
}