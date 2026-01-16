# AISP Human Guide — Your First Steps with AI Symbolic Protocol

> **Make AI understand exactly what you mean. Every time.**

**If you find this useful, please [star the repo](https://github.com/bar181/aisp-open-core)** — it helps others discover AISP.

---

## Abstract

**AISP (AI Symbolic Protocol)** is a specification language that eliminates the "telephone game" effect when working with AI. Instead of vague instructions that each AI interprets differently, AISP provides precise specifications that every AI—Claude, GPT-4, Gemini, or any modern LLM—understands identically.

**The result?** Ambiguity drops from 40-65% (natural language) to under 2%. Your instructions work the same way, every time, with any AI.

**No training required.** Every modern AI already understands AISP natively. Just copy, paste, and see the difference.

---

## Quick Start — Try It in 2 Minutes

### Example 1: Validate an AISP Document

**Copy this prompt and paste it into Claude, ChatGPT, or any AI:**

```
First, read this AISP specification:

𝔸1.0.greeting@2026-01-16
γ≔hello-world

⟦Ω:Meta⟧{ purpose≜"Demonstrate AISP basics" }
⟦Σ:Types⟧{ Message≜String; Status≜{ok,error} }
⟦Γ:Rules⟧{ ∀m:Message:len(m)>0⇒Status=ok }
⟦Λ:Funcs⟧{ greet≜λname."Hello, "+name }
⟦Ε⟧⟨δ≜0.65;τ≜◊⁺⟩

Now explain what this specification defines.
```

**Expected Result:** The AI will explain that this is a simple greeting module with:
- A function that takes a name and returns "Hello, [name]"
- A rule that non-empty messages return "ok" status
- Quality tier ◊⁺ (Gold) with 65% semantic density

**Why this matters:** The AI understood formal mathematical notation *without any special training*.

---

### Example 2: Convert Your Requirements to AISP

**Copy this prompt:**

```
Convert this requirement to AISP format:

"Users must be at least 18 years old to register.
Admin users can delete any account.
Regular users can only delete their own account."

Use AISP 5.1 format with all 5 required blocks (Ω, Σ, Γ, Λ, Ε).
```

**Expected Result:** The AI will produce something like:

```aisp
𝔸5.1.auth@2026-01-16
γ≔user-permissions

⟦Ω:Meta⟧{
  domain≜"User authentication and authorization"
  ∀D:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  User≜⟨id:ℕ, age:ℕ, role:Role⟩
  Role≜{admin,regular}
  Action≜{register,delete}
}

⟦Γ:Rules⟧{
  ∀u:User:canRegister(u)⇔u.age≥18
  ∀u:User:u.role=admin⇒canDelete(u,∀account)
  ∀u:User:u.role=regular⇒canDelete(u,u.account)
}

⟦Λ:Funcs⟧{
  canRegister≜λu.u.age≥18
  canDelete≜λ(u,target).u.role=admin∨target=u.account
}

⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩
```

**Why this matters:** Your vague English requirements became precise, unambiguous rules that any AI will interpret identically.

---

### Example 3: Generate Code from AISP

**Copy this prompt:**

```
Generate a JavaScript implementation from this AISP spec:

𝔸1.0.calculator@2026-01-16
γ≔basic-math

⟦Ω:Meta⟧{ purpose≜"Four basic operations" }
⟦Σ:Types⟧{ Num≜ℝ; Op≜{add,sub,mul,div} }
⟦Γ:Rules⟧{ ∀x,y:Num:div(x,y)⇒y≠0 }
⟦Λ:Funcs⟧{
  add≜λ(x,y).x+y
  sub≜λ(x,y).x-y
  mul≜λ(x,y).x×y
  div≜λ(x,y).x÷y
}
⟦Ε⟧⟨δ≜0.70;τ≜◊⁺⟩

Include error handling for division by zero.
```

**Expected Result:** Clean JavaScript code with the exact functions specified and proper error handling.

---

## What is AISP?

Think of AISP as **a recipe language for AI**.

**The Problem with Natural Language:**

| You Say | Chef 1 Thinks | Chef 2 Thinks | Chef 3 Thinks |
|---------|---------------|---------------|---------------|
| "Add some salt" | 1 pinch | 1 teaspoon | 1 tablespoon |
| "Cook until golden" | 5 minutes | 10 minutes | 15 minutes |
| "Serve with garnish" | Parsley | Lemon | Nothing |

**Result:** Three completely different dishes from one recipe.

**The Same Problem with AI:**

When you give instructions in natural language, each AI interprets vague words differently. String multiple AI agents together (very common in modern systems), and the original meaning gets lost—like a game of telephone.

**AISP Fixes This:**

| AISP Says | Every AI Understands |
|-----------|----------------------|
| `salt≜5g` | Exactly 5 grams |
| `time≜10min` | Exactly 10 minutes |
| `garnish≜parsley:2g` | Exactly 2g parsley |

**Result:** Identical output from any AI, every time.

---

## The 5 Required Blocks

Every AISP document needs these 5 blocks:

| Block | Name | What It Does | Example |
|-------|------|--------------|---------|
| `⟦Ω⟧` | **Meta** | Defines the document's purpose and constraints | `purpose≜"User auth system"` |
| `⟦Σ⟧` | **Types** | Defines the data types used | `User≜⟨name:String,age:ℕ⟩` |
| `⟦Γ⟧` | **Rules** | Defines the business rules | `∀u:canVote(u)⇔u.age≥18` |
| `⟦Λ⟧` | **Functions** | Defines the operations | `validate≜λuser.user.age≥0` |
| `⟦Ε⟧` | **Evidence** | Quality metrics | `⟨δ≜0.75;τ≜◊⁺⁺⟩` |

**Memory trick:** **Ω**mega starts it, **Σ**igma defines types, **Γ**amma sets rules, **Λ**ambda has functions, **Ε**vidence proves quality.

> **Advanced:** Complex documents may include optional blocks: `⟦Χ⟧` (Errors), `⟦Θ⟧` (Proofs), `⟦ℭ⟧` (Categories). See [reference.md](./reference.md) for details.

---

## What Does "Ambiguity < 2%" Mean?

When you write "add some salt," different people interpret it differently. AISP measures this:

- **40-65% ambiguity** = Natural language (lots of interpretation variance)
- **5-15% ambiguity** = Code (better, but comments and naming vary)
- **< 2% ambiguity** = AISP (98%+ identical interpretation)

The formula: `Ambig = 1 - (unique parses / total parses)`

Lower is better. AISP documents mathematically guarantee alignment.

---

## Quality Tiers — How Good Is Your Spec?

AISP documents are scored by **semantic density (δ)** — how much precise meaning is packed into the document.

| Tier | Symbol | Score | Meaning |
|------|--------|-------|---------|
| **Platinum** | ◊⁺⁺ | δ ≥ 0.75 | Production-ready, fully specified |
| **Gold** | ◊⁺ | δ ≥ 0.60 | High quality, minor gaps |
| **Silver** | ◊ | δ ≥ 0.40 | Usable, some interpretation needed |
| **Bronze** | ◊⁻ | δ ≥ 0.20 | Draft quality, significant gaps |
| **Reject** | ⊘ | δ < 0.20 | Not valid AISP |

**Goal:** Aim for ◊⁺ (Gold) or higher for production use.

---

## Common Symbols — Your Rosetta Stone

| English | Code | AISP |
|---------|------|------|
| "x is defined as 5" | `const x = 5` | `x≜5` |
| "for all x in set S" | `S.every(x => ...)` | `∀x∈S` |
| "there exists an x" | `S.some(x => ...)` | `∃x∈S` |
| "if A then B" | `if(A) { B }` | `A⇒B` |
| "function that takes x, returns y" | `(x) => y` | `λx.y` |
| "x equals y" | `x === y` | `x≡y` |
| "x is element of S" | `S.includes(x)` | `x∈S` |
| "true" / "success" | `true` | `⊤` |
| "false" / "crash" | `false` | `⊥` |
| "compose f and g" | `f(g(x))` | `f∘g` |
| "success case" | `Result.Ok` | `⊕` |
| "failure case" | `Result.Err` | `⊖` |

---

## Validate Your Documents

### Using npm (Node.js)

```bash
# Install
npm install aisp-validator

# Validate a file
npx aisp-validator validate your-spec.aisp

# Get detailed output
npx aisp-validator validate your-spec.aisp --long
```

### Using Rust

```toml
# Cargo.toml
[dependencies]
aisp = "0.1"
```

```rust
use aisp::{validate, Tier};

let result = validate(your_document);
println!("Valid: {}, Tier: {}", result.valid, result.tier.symbol());
```

---

## Real Discovery: AISP and Creative Writing

**The hardest test we could imagine:** Could AISP—a mathematical specification language—successfully constrain *creative fiction*? Everyone said it couldn't work. Creativity needs ambiguity, interpretation, the human touch. You can't reduce storytelling to equations.

**We tried it anyway.**

We wrote an AISP specification for a horror story about AI disruption. Not a prompt—a *specification*. It locked the plot points, character arc, emotional journey, and key moments while leaving stylistic execution open.

Then we gave the same spec to Claude and GPT-4. No coordination between them. Single-shot generation.

**The Results:**

| Metric | Result |
|--------|--------|
| Semantic alignment (plot, arc, theme) | **98%** |
| Stylistic variance (voice, rhythm) | **38%** |
| Cross-model replication | **Confirmed** |

**Both AIs produced the same story.** Same character (Marcus Chen, 23-year architect). Same plot beats. Same ending line. But completely different *voices*—one read like structured literary prose, the other like a published short story.

**What we learned:** Meaning and voice are separable. You can lock *what* the content says while letting *how* it's expressed vary freely. This has massive implications for brand consistency, regulated industries, and multi-agent creative systems.

> *"The limit isn't at the boundary of creative work. It's somewhere further out—possibly much further."*

**See the full experiment:** [`evidence/creative-short-story/`](./evidence/creative-short-story/)

---

## More Evidence to Explore

| Experiment | What It Shows | Location |
|------------|---------------|----------|
| **Tic-Tac-Toe** | AISP vs natural language specs: +26% quality, 0 ambiguities | [`evidence/tic-tac-toe/`](./evidence/tic-tac-toe/) |
| **Rosetta Stone** | Examples at every quality tier (Bronze → Platinum) | [`evidence/rosetta-stone/`](./evidence/rosetta-stone/) |
| **Creative Fiction** | 98% alignment in creative writing across AI models | [`evidence/creative-short-story/`](./evidence/creative-short-story/) |

---

## Get Involved

**AISP is just getting started.**

We've proven the concept works—from technical specifications to creative writing. But there's so much more to explore:

- **Multi-agent coordination** — Swarms of AI agents working from shared specs
- **Regulated industries** — Healthcare, finance, legal content with guaranteed compliance
- **Brand systems** — Perfect consistency across any AI, agency, or market
- **Education** — Teaching AI to teach consistently
- **Research** — Where else does specification-driven generation apply?

### Sponsorship & Partnership

We're building an **AISP Elite Team** of researchers, engineers, and organizations who want to push the boundaries of what's possible.

**Interested in:**
- Early access to new research and tools
- Collaborative experiments in your domain
- Sponsoring specific use case development
- Academic partnerships

**Contact:**
- **Bradley Ross** — [GitHub @bar181](https://github.com/bar181) | [LinkedIn](https://linkedin.com/in/bradaross)
- **Email:** Research inquiries welcome

**Support the Project:**
- **Star the repo:** [github.com/bar181/aisp-open-core](https://github.com/bar181/aisp-open-core) — helps others discover AISP
- **Join the discussion:** [GitHub Issue #1](https://github.com/bar181/aisp-open-core/issues/1) — introduce yourself and share ideas
- Try AISP in your projects and share results
- Report issues and suggest improvements
- Spread the word

---

## Resources

| Resource | Description |
|----------|-------------|
| [AI_GUIDE.md](./AI_GUIDE.md) | Complete AISP 5.1 Platinum specification (copy this to any AI) |
| [reference.md](./reference.md) | Full symbol glossary and reference |
| [evidence/](./evidence/) | All validation experiments and results |
| [validator/](./validator/) | npm package for validating AISP documents |
| [aisp-rust/](./aisp-rust/) | Rust crate for validation |

### Published Packages

| Package | Registry | Install |
|---------|----------|---------|
| aisp-validator | npm | `npm install aisp-validator` |
| aisp | crates.io | `aisp = "0.1"` |

---

## Quick Reference Card

```
AISP Document Structure:
========================
𝔸[version].[name]@[date]     ← Header
γ≔[context]                   ← Context identifier

⟦Ω:Meta⟧{ ... }              ← Foundation/metadata
⟦Σ:Types⟧{ ... }             ← Type definitions
⟦Γ:Rules⟧{ ... }             ← Business rules
⟦Λ:Funcs⟧{ ... }             ← Functions
⟦Ε⟧⟨δ≜...;τ≜...⟩            ← Evidence/quality

Key Symbols:
============
≜  defined as       ∀  for all        ∃  exists
⇒  implies          λ  lambda         ∈  element of
≡  equals           ⊤  true           ⊥  false

Quality Tiers:
==============
◊⁺⁺ Platinum (≥0.75)    ◊⁺ Gold (≥0.60)
◊   Silver (≥0.40)      ◊⁻ Bronze (≥0.20)
⊘   Reject (<0.20)
```

---

## Final Thought

> *"AISP is like giving every AI the same dictionary, the same grammar book, and the same understanding of your intent. The telephone game ends here."*

**Ready to try it?** Copy the [AI_GUIDE.md](./AI_GUIDE.md) into your favorite AI and start experimenting. You'll be surprised how much it already understands.

---

**Author:** Bradley Ross — Harvard ALM Candidate | CS50 Teaching Fellow | [GitHub @bar181](https://github.com/bar181)

**License:** MIT
