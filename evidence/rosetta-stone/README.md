# AISP Rosetta Stone Evidence

This folder contains reference examples demonstrating the AISP 5.1 specification's Rosetta Stone mappings between prose, code, and AISP formal notation.

## Contents

| File | Purpose | Tier |
|------|---------|------|
| [spec.aisp](spec.aisp) | Complete Rosetta Stone reference | ◊⁺⁺ Platinum |
| [prose-full.aisp](prose-full.aisp) | Full RBAC specification example | ◊⁺⁺ Platinum |
| [prose-standard.aisp](prose-standard.aisp) | Standard complexity example | ◊⁺⁺ Platinum |
| [standard.aisp](standard.aisp) | Math operations example | ◊⁺⁺ Platinum |
| [prose-minimal.aisp](prose-minimal.aisp) | Minimal prose conversion | ◊⁺ Gold |
| [minimal.aisp](minimal.aisp) | Minimal valid AISP | ◊⁺ Gold |
| [bronze.aisp](bronze.aisp) | Bare minimum example | ◊ Silver |
| [analysis.md](analysis.md) | Validation scores and analysis | - |

## Quick Validation

```bash
# Validate all examples
cd validator
for f in ../evidence/rosetta-stone/*.aisp; do
  node bin/cli.js validate "$f" --json
done

# Detailed report for main spec
node bin/cli.js validate ../evidence/rosetta-stone/spec.aisp --long
```

## Key Rosetta Mappings

### Prose → AISP
| English | AISP |
|---------|------|
| "x is defined as 5" | `x≜5` |
| "for all x in S" | `∀x∈S` |
| "A implies B" | `A⇒B` |
| "lambda x returns y" | `λx.y` |

### Code → AISP
| JavaScript | AISP |
|------------|------|
| `const x = 5` | `x≜5` |
| `S.every(x => P(x))` | `∀x∈S:P(x)` |
| `if(A){ B }` | `A⇒B` |
| `(x) => y` | `λx.y` |

## Source

These examples are derived from the [AISP 5.1 Platinum Specification](../../AI_GUIDE.md) (§Rosetta).

## License

MIT OR Apache-2.0
