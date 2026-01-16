# AISP Validator Test Results
Generated: 2026-01-15T18:22:35+00:00
Version: 0.2.1

## Test Files

| File | Level | δ (Semantic) | ρ (Pure) | Tier |
|------|-------|--------------|----------|------|
| tic-tac-toe-existing | existing | 1.000 | 1.857 | ◊⁺⁺ Platinum |
| tic-tac-toe-full | full | 1.000 | 2.436 | ◊⁺⁺ Platinum |
| tic-tac-toe-minimal | minimal | 0.760 | 2.059 | ◊⁺⁺ Platinum |
| tic-tac-toe-standard | standard | 1.000 | 2.176 | ◊⁺⁺ Platinum |

## Detailed Results

### tic-tac-toe-existing.aisp
```

AISP Density Debug
==================

Tier: ◊⁺⁺ Platinum
Semantic (δ): 1.000
Pure (ρ):     1.857

Semantic Score Breakdown:
  Block Score:   100.0% (weight: 40%)
  Binding Score: 100.0% (weight: 60%)

Blocks Found: 5/5
  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧

Semantic Operators: 28
  ≜ definitions:  12
  ≔ assignments:  1
  ∀∃ quantifiers: 6
  λ lambdas:      3
  ⇒⇔ implications: 2
  ∈⊆∩∪ set ops:   4

Pure Density Breakdown:
  AISP Symbols: 52
  Total Tokens: 28

Formulas:
  δ = (1.00 × 0.4) + (1.00 × 0.6) = 1.000
  ρ = 52 ÷ 28 = 1.857
```

### tic-tac-toe-full.aisp
```

AISP Density Debug
==================

Tier: ◊⁺⁺ Platinum
Semantic (δ): 1.000
Pure (ρ):     2.436

Semantic Score Breakdown:
  Block Score:   100.0% (weight: 40%)
  Binding Score: 100.0% (weight: 60%)

Blocks Found: 5/5
  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧

Semantic Operators: 51
  ≜ definitions:  13
  ≔ assignments:  13
  ∀∃ quantifiers: 8
  λ lambdas:      4
  ⇒⇔ implications: 5
  ∈⊆∩∪ set ops:   8

Pure Density Breakdown:
  AISP Symbols: 95
  Total Tokens: 39

Formulas:
  δ = (1.00 × 0.4) + (1.00 × 0.6) = 1.000
  ρ = 95 ÷ 39 = 2.436
```

### tic-tac-toe-minimal.aisp
```

AISP Density Debug
==================

Tier: ◊⁺⁺ Platinum
Semantic (δ): 0.760
Pure (ρ):     2.059

Semantic Score Breakdown:
  Block Score:   100.0% (weight: 40%)
  Binding Score: 60.0% (weight: 60%)

Blocks Found: 5/5
  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧

Semantic Operators: 12
  ≜ definitions:  6
  ≔ assignments:  1
  ∀∃ quantifiers: 2
  λ lambdas:      1
  ⇒⇔ implications: 1
  ∈⊆∩∪ set ops:   1

Pure Density Breakdown:
  AISP Symbols: 35
  Total Tokens: 17

Formulas:
  δ = (1.00 × 0.4) + (0.60 × 0.6) = 0.760
  ρ = 35 ÷ 17 = 2.059
```

### tic-tac-toe-standard.aisp
```

AISP Density Debug
==================

Tier: ◊⁺⁺ Platinum
Semantic (δ): 1.000
Pure (ρ):     2.176

Semantic Score Breakdown:
  Block Score:   100.0% (weight: 40%)
  Binding Score: 100.0% (weight: 60%)

Blocks Found: 5/5
  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧

Semantic Operators: 41
  ≜ definitions:  16
  ≔ assignments:  2
  ∀∃ quantifiers: 9
  λ lambdas:      4
  ⇒⇔ implications: 3
  ∈⊆∩∪ set ops:   7

Pure Density Breakdown:
  AISP Symbols: 74
  Total Tokens: 34

Formulas:
  δ = (1.00 × 0.4) + (1.00 × 0.6) = 1.000
  ρ = 74 ÷ 34 = 2.176
```

