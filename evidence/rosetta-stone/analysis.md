# AISP Rosetta Stone Analysis

This document analyzes different AISP document formats and their validation scores, demonstrating the progression from minimal to full specifications.

## Validation Results Summary

| File | Size | Tier | δ (Semantic) | ρ (Pure) | Status |
|------|------|------|--------------|----------|--------|
| bronze.aisp | 168B | ◊ Silver | 0.58 | 1.87 | ✓ Valid |
| minimal.aisp | 195B | ◊⁺ Gold | 0.64 | 2.20 | ✓ Valid |
| prose-minimal.aisp | 254B | ◊⁺ Gold | 0.67 | 1.89 | ✓ Valid |
| standard.aisp | 476B | ◊⁺⁺ Platinum | 1.00 | 2.68 | ✓ Valid |
| prose-standard.aisp | 427B | ◊⁺⁺ Platinum | 1.00 | 2.79 | ✓ Valid |
| prose-full.aisp | 2.3KB | ◊⁺⁺ Platinum | 1.00 | 1.43 | ✓ Valid |
| spec.aisp | 8.2KB | ◊⁺⁺ Platinum | 1.00 | 0.34 | ✓ Valid |

## Tier Thresholds

| Tier | Symbol | Semantic Density (δ) | Description |
|------|--------|---------------------|-------------|
| Reject | ⊘ | δ < 0.20 | Invalid, insufficient formalization |
| Bronze | ◊⁻ | δ ≥ 0.20 | Minimal valid AISP |
| Silver | ◊ | δ ≥ 0.40 | Basic formal specification |
| Gold | ◊⁺ | δ ≥ 0.60 | Well-structured specification |
| Platinum | ◊⁺⁺ | δ ≥ 0.75 | Complete, proof-carrying spec |

## Semantic Density Formula

```
δ = (blockScore × 0.4) + (bindingScore × 0.6)

blockScore = blocksFound / 5  (required blocks: Ω, Σ, Γ, Λ, Ε)
bindingScore = min(totalBindings / 20, 1.0)

totalBindings = definitions(≜) + assignments(≔) + quantifiers(∀∃)
              + lambdas(λ) + implications(⇒⇔→↔) + setOps(∈⊆∩∪∅)
```

## Prose-to-AISP Conversion Modes

The `@aisp/convert` package provides three conversion modes:

### 1. Minimal Mode (0.5-1x tokens)
Direct Rosetta mapping only:
```
Prose: "For all users, if admin then allow access"
AISP:  ∀ users, if admin⇒allow access
```

### 2. Standard Mode (1.5-2x tokens)
Adds header and evidence block:
```aisp
𝔸5.1.auth@2026-01-16
γ≔auth

⟦Λ:Funcs⟧{
  ∀ users, if admin⇒allow access
}

⟦Ε⟧⟨δ≜0.70;τ≜◊⁺⟩
```

### 3. Full Mode (4-8x tokens)
Complete specification with all blocks and proofs:
```aisp
𝔸5.1.auth@2026-01-16
γ≔auth.definitions
ρ≔⟨auth,types,rules⟩

⟦Ω:Meta⟧{ ... }
⟦Σ:Types⟧{ ... }
⟦Γ:Rules⟧{ ... }
⟦Λ:Funcs⟧{ ... }
⟦Χ:Errors⟧{ ... }  // optional
⟦Θ:Proofs⟧{ ... }  // optional
⟦Ε⟧⟨δ≜0.82;φ≜100;τ≜◊⁺⁺;⊢valid;∎⟩
```

## Rosetta Stone Mappings

### Simple Mappings

| Prose | Code | AISP |
|-------|------|------|
| "x is defined as 5" | `const x = 5` | `x≜5` |
| "for all x in S, P holds" | `S.every(x => P(x))` | `∀x∈S:P(x)` |
| "there exists unique x" | - | `∃!x:f(x)≡0` |
| "A implies B" | `if(A){ B }` | `A⇒B` |
| "f maps i to o" | `(i) => o` | `f≜λi.o` |

### Core Symbols Quick Reference

| Symbol | Meaning | Category |
|--------|---------|----------|
| ≜ | defined as | Ω:Transmuter |
| ≔ | assignment | Ω:Transmuter |
| ∀ | for all | ∀:Quantifier |
| ∃ | exists | ∀:Quantifier |
| ⇒ | implies | Ω:Transmuter |
| λ | lambda | Ω:Transmuter |
| ∈ | element of | Γ:Topologic |
| ⊆ | subset | Γ:Topologic |
| ⟦⟧ | block | ⟦⟧:Delimiter |
| 𝔸 | AISP header | ⟦⟧:Delimiter |

## File Descriptions

### bronze.aisp
Minimal Bronze-tier document with bare essentials. Shows the minimum structure required for validation.

### minimal.aisp
Slightly enhanced minimal document with type annotations and explicit quantification.

### prose-minimal.aisp
Example of minimal prose conversion with added AISP structure to meet validation requirements.

### standard.aisp
Standard complexity document with well-defined types, rules, and functions.

### prose-standard.aisp
Standard prose conversion showing RBAC (Role-Based Access Control) domain.

### prose-full.aisp
Full specification with complete type system, rules, functions, error handling, and proofs.

### spec.aisp
Complete Rosetta Stone reference document with all mapping examples from the AISP 5.1 specification.

## Key Observations

1. **All 5 required blocks** significantly impact semantic density (40% weight)
2. **Binding operators** (≜, ≔, ∀, λ, ⇒, ∈) drive the remaining 60%
3. **Pure density (ρ)** measures AISP symbol concentration but doesn't affect tier
4. **Larger documents** may have lower pure density but still achieve Platinum if well-structured
5. **Comments and prose** in AISP files don't negatively impact validation

## Validator Usage

```bash
# Basic validation
npx aisp-validator validate document.aisp

# Detailed report
npx aisp-validator validate document.aisp --long

# JSON output for automation
npx aisp-validator validate document.aisp --json

# Prose conversion
npx @aisp/convert minimal "your prose here"
npx @aisp/convert standard "your prose here"
npx @aisp/convert full "your prose here"
npx @aisp/convert auto "your prose here"
```

## Conclusion

The AISP validation system rewards:
- **Complete structure**: All 5 required blocks
- **Formal semantics**: Rich use of binding operators
- **Precision**: Clear type definitions and rules
- **Proofs**: Optional but enhances documentation

Documents achieving Platinum tier (◊⁺⁺) demonstrate production-ready specifications suitable for AI-to-AI communication with guaranteed <2% ambiguity.
