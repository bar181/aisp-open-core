# AISP Validator

[![npm version](https://img.shields.io/npm/v/aisp-validator.svg)](https://www.npmjs.com/package/aisp-validator)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Validate AISP 5.1 documents** — AI Symbolic Protocol with <2% ambiguity.

## Install

```bash
npm install aisp-validator
```

## CLI Usage

```bash
# Validate a document
npx aisp-validator validate spec.aisp

# Detailed output with JSON
npx aisp-validator validate spec.aisp --long
npx aisp-validator validate spec.aisp --json
```

**Output:**
```
✓ VALID
  Tier: ◊⁺⁺ Platinum
  Semantic (δ): 1.000
  Pure (ρ):     1.857
```

## Node.js Usage

```javascript
import AISP from 'aisp-validator';

await AISP.init();

const result = AISP.validate(`
𝔸1.0.example@2026-01-16
γ≔test

⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩
`);

console.log(result.valid);    // true
console.log(result.tier);     // '◊⁺⁺'
console.log(result.tierName); // 'Platinum'
console.log(result.delta);    // 0.82
```

## Browser Usage

```html
<script type="module">
import AISP from 'aisp-validator/browser';

await AISP.init('/path/to/aisp.wasm');
const result = AISP.validate(source);
</script>
```

## Quality Tiers

| Tier | Symbol | δ Threshold | Description |
|------|--------|-------------|-------------|
| Platinum | ◊⁺⁺ | ≥ 0.75 | Production ready |
| Gold | ◊⁺ | ≥ 0.60 | Pre-production |
| Silver | ◊ | ≥ 0.40 | Development |
| Bronze | ◊⁻ | ≥ 0.20 | Draft |
| Reject | ⊘ | < 0.20 | Invalid |

## Required Blocks

Every AISP document needs 5 blocks:

| Block | Purpose |
|-------|---------|
| `⟦Ω⟧` | Meta/Foundation |
| `⟦Σ⟧` | Types/Glossary |
| `⟦Γ⟧` | Rules/Inference |
| `⟦Λ⟧` | Functions |
| `⟦Ε⟧` | Evidence |

## API

| Method | Returns | Description |
|--------|---------|-------------|
| `AISP.init()` | Promise | Initialize WASM (call once) |
| `AISP.validate(src)` | Object | Full validation result |
| `AISP.isValid(src)` | boolean | Quick valid check |
| `AISP.getTier(src)` | string | Tier symbol only |
| `AISP.getDensity(src)` | number | Semantic density δ |
| `AISP.validateFile(path)` | Object | Validate file (Node.js) |

### Validation Result

```javascript
{
  valid: true,          // Document passes validation
  tier: '◊⁺⁺',          // Quality tier symbol
  tierName: 'Platinum', // Tier name
  tierValue: 4,         // Numeric (0-4)
  delta: 0.82,          // Semantic density [0,1]
  pureDensity: 1.85,    // Symbol concentration
  ambiguity: 0.01,      // Must be <0.02
  errorCode: 0          // 0 = success
}
```

## What is AISP?

**AI Symbolic Protocol** is a formal specification language for AI-to-AI communication:

- **<2% Ambiguity** — Eliminates interpretation errors
- **Formal Semantics** — Based on type theory
- **Machine-Readable** — Designed for AI agents
- **Human-Auditable** — Clear structure for review

## Links

- [AI Guide](https://github.com/bar181/aisp-open-core/blob/main/AI_GUIDE.md) — Full AISP 5.1 spec
- [Human Guide](https://github.com/bar181/aisp-open-core/blob/main/HUMAN_GUIDE.md) — Introduction
- [GitHub](https://github.com/bar181/aisp-open-core)

## Author

Bradley Ross — [GitHub @bar181](https://github.com/bar181)

## License

MIT
