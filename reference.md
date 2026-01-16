# AISP 5.1 Platinum — Technical Reference

**The Rosetta Stone: Human ↔ AISP**

This document serves dual audiences simultaneously. Each concept is presented first in human-readable prose, then in formal AISP notation. Use this as a lookup reference, translation guide, and architecture overview.

---

## Document Navigation

| Section | Human | AISP | Purpose |
|---------|:-----:|:----:|---------|
| [Core Concept](#core-concept) | ✓ | ✓ | What AISP is and why it exists |
| [Three-Layer Architecture](#three-layer-architecture) | ✓ | ✓ | 𝕃₀ → 𝕃₁ → 𝕃₂ system design |
| [Feature Catalog](#feature-catalog) | ✓ | ✓ | All 20 features with use cases |
| [Symbol Reference](#symbol-reference) | ✓ | — | Quick lookup for AISP glyphs |
| [Validation Evidence](#validation-evidence) | ✓ | ✓ | Empirical results and benchmarks |
| [Complete AISP Specification](#complete-aisp-specification) | — | ✓ | Full formal spec for AI ingestion |
| [Validation Tools](#validation-tools) | ✓ | — | npm and Rust validation packages |

---

## Core Concept

### 📖 Human Section

**The Problem:** When you give instructions to AI agents in natural language, each one interprets them slightly differently. String 10 AI agents together, and the original meaning is almost completely lost—like a game of telephone.

**The Math:**
- Natural language has 40-65% ambiguity (interpretation required)
- A 10-step pipeline with 62% per-step accuracy = **0.84% total success**
- AISP has <2% ambiguity by design
- A 10-step pipeline with 98% per-step accuracy = **81.7% total success**

**The Analogy:** Think of giving directions three ways:

| Method | Example | Result |
|--------|---------|--------|
| Casual | "Turn left at the big tree" | Everyone ends up somewhere different |
| Address | "123 Main Street" | Most people find it |
| GPS | "40.7128° N, 74.0060° W" | Mathematical precision, zero ambiguity |

**AISP = GPS coordinates for AI instructions.**

### 🤖 AISP Section

```aisp
⟦Ω:Core⟧{
  ∀D∈AISP:Ambig(D)<0.02
  Ambig≜λD.1-|Parse_u(D)|/|Parse_t(D)|
  
  ;; Pipeline success probability
  P_prose(n)≜(0.62)ⁿ
  P_aisp(n)≜(0.98)ⁿ
  
  ;; At n=10 steps
  P_prose(10)≡0.0084
  P_aisp(10)≡0.817
  Improvement≜P_aisp/P_prose≡97×
}
```

---

## Three-Layer Architecture

### 📖 Human Section

AISP is built on three composable layers, each proving properties that enable the next:

#### Layer 0: Signal Theory (𝕃₀)

**What it does:** Separates every piece of information into three orthogonal vector spaces.

**Why it matters:** Safety constraints can't be "optimized away" because they exist in a completely separate mathematical dimension from semantic content.

| Vector | Dimension | Contains | Example |
|--------|-----------|----------|---------|
| V_H | 768 | Semantic meaning | "What does this code do?" |
| V_L | 512 | Structural relationships | "How do components connect?" |
| V_S | 256 | Safety constraints | "What must never happen?" |

**Key insight:** V_H and V_S are orthogonal (no overlap). An optimizer maximizing semantic fit literally cannot touch safety constraints—they're in different spaces.

#### Layer 1: Pocket Architecture (𝕃₁)

**What it does:** Stores knowledge in tamper-proof containers with adaptive learning.

**The structure:**

```
┌─────────────────────────────────────┐
│ 𝒫 Pocket                            │
├─────────────────────────────────────┤
│ ℋ Header (IMMUTABLE)                │
│   • id: SHA256 hash of Nucleus      │
│   • V: Signal vector (1536d)        │
│   • f: Feature flags (64 bits)      │
├─────────────────────────────────────┤
│ ℳ Membrane (MUTABLE)                │
│   • aff: Affinity scores            │
│   • conf: Confidence [0,1]          │
│   • tags: Classification set        │
│   • use: Access counter             │
├─────────────────────────────────────┤
│ 𝒩 Nucleus (IMMUTABLE)               │
│   • def: AISP definition            │
│   • ir: LLVM intermediate repr      │
│   • wa: WASM binary                 │
│   • σ: Cryptographic signature      │
└─────────────────────────────────────┘
```

**Tamper detection:** If anyone modifies the Nucleus, SHA256(Nucleus) ≠ Header.id, and the pocket is immediately quarantined.

**Learning:** The Membrane learns which pockets work well together (affinity) without changing the immutable content.

#### Layer 2: Intelligence Engine (𝕃₂)

**What it does:** Goal-directed search that finds "what's missing" rather than exhaustively exploring.

**Ghost Intent:** Instead of asking "what exists?", AISP asks "what do I need that I don't have?" This is computed as:

```
Ghost = Target - Have
ψ_g = ψ_* ⊖ ψ_have
```

**Beam search with safety gates:** Multiple solution paths are explored in parallel, but any path exceeding the risk threshold is immediately pruned.

#### Layer Composition

Each layer proves properties that enable the next:

```
𝕃₀ proves: stable + deterministic
    ↓ enables
𝕃₁ proves: integrity + zero-copy
    ↓ enables
𝕃₂ proves: terminates + bounded
    ↓ guarantees
System: safe + optimal
```

### 🤖 AISP Section

```aisp
⟦Σ:Layers⟧{
  𝕃≜{𝕃₀:Signal,𝕃₁:Pocket,𝕃₂:Search}
  
  ;; Layer 0: Tri-Vector Signal
  Signal≜V_H⊕V_L⊕V_S
  V_H≜ℝ⁷⁶⁸; V_L≜ℝ⁵¹²; V_S≜ℝ²⁵⁶
  d_Σ≜768+512+256≡1536
  
  ;; Orthogonality guarantees
  V_H∩V_S≡∅; V_L∩V_S≡∅; V_H∩V_L≢∅
  
  ;; Layer 1: Pocket Architecture
  𝒫≜⟨ℋ:Header,ℳ:Membrane,𝒩:Nucleus⟩
  ℋ≜⟨id:SHA256,V:Signal,f:𝔹⁶⁴⟩:immutable
  ℳ≜⟨aff:Hash→ℝ,conf:ℝ[0,1],tag:𝒫(𝕊),use:ℕ⟩:mutable
  𝒩≜⟨def:AISP,ir:LLVM,wa:WASM,σ:Sig⟩:immutable
  
  ;; CAS integrity
  ∀p:ℋ.id(p)≡SHA256(𝒩(p))
  ∀p:∂𝒩(p)⇒∂ℋ.id(p)
  ∀p:∂ℳ(p)⇏∂ℋ.id(p)
  
  ;; Layer 2: Ghost-Directed Search
  ψ_g≜λb.ψ_*⊖ψ_have(b.G)
  ∀b:μ_r(b)>τ⇒✂(b)
}

⟦Θ:LayerProofs⟧{
  𝕃₀.⊢stable∧𝕃₀.⊢deterministic⇒𝕃₁.⊢integrity
  𝕃₁.⊢integrity∧𝕃₁.⊢zero_copy⇒𝕃₂.⊢bounded
  𝕃₂.⊢terminates∧𝕃₂.⊢bounded⇒system.⊢safe∧system.⊢optimal
}
```

---

## Feature Catalog

### 📖 Human Section

All 20 core AISP features organized by category:

#### Foundation Features (1-4)

| # | Feature | What It Does | Use Case |
|---|---------|--------------|----------|
| 1 | **Tri-Vector Decomposition** | Separates signals into semantic/structural/safety spaces | Safety constraints exist in orthogonal space—can't be optimized away |
| 2 | **Measurable Ambiguity** | Computes interpretation variance as a number | Reject specs with >2% ambiguity at compile time |
| 3 | **Pocket Architecture** | CAS storage + adaptive learning in one structure | Tamper-proof agent memory that still learns preferences |
| 4 | **Four-State Binding** | Categorizes API compatibility: crash/null/adapt/zero | Detect incompatible service handoffs before runtime |

#### Search & Scoring Features (5-8)

| # | Feature | What It Does | Use Case |
|---|---------|--------------|----------|
| 5 | **Ghost Intent Search** | Searches for "what's missing" not "what exists" | Goal-directed code completion |
| 6 | **RossNet Scoring** | Combines similarity + fit + affinity scores | Rank retrieved code by multiple signals |
| 7 | **Hebbian Learning** | 10:1 failure penalty (+1 success, -10 failure) | Fast convergence away from bad pathways |
| 8 | **Quality Tiers** | Five levels: ◊⁺⁺ > ◊⁺ > ◊ > ◊⁻ > ⊘ | Progressive deployment: prod/staging/dev/rejected |

#### Verification Features (9-12)

| # | Feature | What It Does | Use Case |
|---|---------|--------------|----------|
| 9 | **Proof-Carrying Docs** | Each document includes its validity proof | Zero-trust multi-agent systems |
| 10 | **Error Algebra** | Typed errors with automatic repair functions | Self-healing documents |
| 11 | **Category Functors** | Mathematical composition guarantees | Valid block compositions → valid outputs |
| 12 | **Natural Deduction** | Formal inference rules for tier assignment | Proof trees for document validation |

#### Translation & Stability Features (13-16)

| # | Feature | What It Does | Use Case |
|---|---------|--------------|----------|
| 13 | **Rosetta Stone** | Bidirectional Prose ↔ Code ↔ AISP mapping | Migrate natural language requirements to formal specs |
| 14 | **Anti-Drift Protocol** | Locks symbol meanings across pipeline hops | 100+ agent pipelines maintain semantic stability |
| 15 | **Recursive Optimization** | Iteratively improves δ until convergence | Auto-refine documents to higher quality tiers |
| 16 | **Bridge Synthesis** | Creates adapters when search finds nothing | Auto-generate missing integration components |

#### Safety & Initialization Features (17-20)

| # | Feature | What It Does | Use Case |
|---|---------|--------------|----------|
| 17 | **Safety Gate** | Prunes paths exceeding risk threshold | Autonomous systems auto-reject dangerous actions |
| 18 | **DPP Beam Init** | Determinantal Point Process for diverse starts | Avoid local optima through diverse beam initialization |
| 19 | **Contrastive Learning** | Online parameter updates from success/failure | Continuous improvement from deployment feedback |
| 20 | **Σ_512 Glossary** | Fixed vocabulary of 512 symbols in 8 categories | Deterministic parsing—no interpretation needed |

### 🤖 AISP Section

```aisp
⟦Λ:Features⟧{
  ;; Foundation
  F₁≜⟨TriVector,Signal→V_H⊕V_L⊕V_S,"Safety in orthogonal space"⟩
  F₂≜⟨Ambiguity,Ambig(D)<0.02,"Compile-time rejection"⟩
  F₃≜⟨Pocket,𝒫≜⟨ℋ,ℳ,𝒩⟩,"Tamper-proof + adaptive"⟩
  F₄≜⟨Binding,Δ⊗λ∈{0,1,2,3},"API contract validation"⟩
  
  ;; Search & Scoring
  F₅≜⟨Ghost,ψ_g≡ψ_*⊖ψ_have,"Search what's missing"⟩
  F₆≜⟨RossNet,μ_f≡σ(θ·sim+fit+aff),"Multi-signal ranking"⟩
  F₇≜⟨Hebbian,⊕→+1;⊖→-10,"10:1 failure penalty"⟩
  F₈≜⟨Tiers,◊⁺⁺≻◊⁺≻◊≻◊⁻≻⊘,"Progressive deployment"⟩
  
  ;; Verification
  F₉≜⟨ProofCarry,𝔻oc≜Σ(content)(π),"Zero-trust systems"⟩
  F₁₀≜⟨ErrorAlg,ε≜⟨ψ,ρ⟩,"Self-healing docs"⟩
  F₁₁≜⟨Functors,𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥,"Compositional validation"⟩
  F₁₂≜⟨Inference,[◊⁺⁺-I]...[sub],"Formal tier proofs"⟩
  
  ;; Translation & Stability
  F₁₃≜⟨Rosetta,Prose↔Code↔AISP,"Requirement migration"⟩
  F₁₄≜⟨AntiDrift,Mean(s)≡Mean_0(s),"Pipeline stability"⟩
  F₁₅≜⟨Optimize,opt_δ:𝔻oc×ℕ→𝔻oc,"Auto-refinement"⟩
  F₁₆≜⟨Bridge,bridge:ψ→Option⟨𝒫⟩,"Adapter synthesis"⟩
  
  ;; Safety & Initialization
  F₁₇≜⟨SafetyGate,μ_r>τ⇒✂,"Auto-prune risk"⟩
  F₁₈≜⟨DPP,‖*init≜argmax det(Ker),"Diverse beams"⟩
  F₁₉≜⟨Contrastive,∇_θ←θ-η·∇(‖y-ŷ‖²),"Online learning"⟩
  F₂₀≜⟨Σ_512,8×64 symbols,"Deterministic parsing"⟩
}
```

---

## Symbol Reference

### 📖 Human Section

Quick lookup for AISP symbols organized by function:

#### Logic & Proof

| Symbol | Name | Meaning | Example |
|:------:|------|---------|---------|
| `≜` | definition | "is defined as" | `x≜5` |
| `≔` | assignment | "is assigned to" | `y≔x+1` |
| `≡` | identical | "is exactly equal to" | `a≡b` |
| `⇒` | implies | "if...then" | `A⇒B` |
| `↔` | iff | "if and only if" | `A↔B` |
| `⊢` | proves | "syntactically proves" | `Γ⊢P` |
| `⊨` | models | "semantically entails" | `Γ⊨P` |
| `∎` | QED | "proof complete" | `π:...∎` |

#### Quantifiers

| Symbol | Name | Meaning | Example |
|:------:|------|---------|---------|
| `∀` | for all | universal quantifier | `∀x:P(x)` |
| `∃` | exists | existential quantifier | `∃x:P(x)` |
| `∃!` | unique | exactly one exists | `∃!x:f(x)=0` |
| `λ` | lambda | function abstraction | `λx.x+1` |
| `Π` | pi | dependent product | `Πx:A.B(x)` |
| `Σ` | sigma | dependent sum | `Σx:A.B(x)` |

#### Sets & Relations

| Symbol | Name | Meaning | Example |
|:------:|------|---------|---------|
| `∈` | element | "is member of" | `x∈S` |
| `⊆` | subset | "is contained in" | `A⊆B` |
| `∩` | intersection | set overlap | `A∩B` |
| `∪` | union | set combination | `A∪B` |
| `∅` | empty | empty set/null | `S≡∅` |
| `𝒫` | powerset | all subsets (or Pocket) | `𝒫(S)` |

#### Operators

| Symbol | Name | Meaning | Example |
|:------:|------|---------|---------|
| `⊕` | plus | sum/success/add | `A⊕B` |
| `⊖` | minus | difference/failure | `ψ_*⊖ψ_have` |
| `⊗` | tensor | product/binding | `Δ⊗λ` |
| `∘` | compose | function composition | `f∘g` |
| `→` | arrow | function type | `f:A→B` |
| `↦` | mapsto | maps element to | `x↦y` |

#### Structure

| Symbol | Name | Meaning | Example |
|:------:|------|---------|---------|
| `⟨⟩` | tuple | record/tuple | `⟨a:A,b:B⟩` |
| `⟦⟧` | block | AISP block delimiter | `⟦Σ:Types⟧{...}` |
| `◊` | tier | quality level | `◊⁺⁺` |
| `𝔸` | AISP | document header | `𝔸5.1.name@date` |

#### Quality Tiers

| Symbol | Name | Threshold | Deployment |
|:------:|------|:---------:|------------|
| `◊⁺⁺` | platinum | δ ≥ 0.75 | Production |
| `◊⁺` | gold | δ ≥ 0.60 | Staging |
| `◊` | silver | δ ≥ 0.40 | Development |
| `◊⁻` | bronze | δ ≥ 0.20 | Review |
| `⊘` | reject | δ < 0.20 | Rejected |

#### Binding States

| Symbol | State | Code | Meaning |
|:------:|-------|:----:|---------|
| `⊤` | zero | 3 | Perfect compatibility, no adaptation needed |
| `λ` | adapt | 2 | Type mismatch, adaptation possible |
| `∅` | null | 1 | Socket mismatch, connection fails |
| `⊥` | crash | 0 | Logical contradiction, fatal error |

---

## Validation Evidence

### 📖 Human Section

#### Tic-Tac-Toe Comparative Test

A simple game specification was written in both prose and AISP, then implemented by AI:

| Metric | Prose | AISP | Change |
|--------|:-----:|:----:|:------:|
| Ambiguous requirements | 6 | 0 | **-100%** |
| Technical precision | 43/100 | 95/100 | **+121%** |
| Overall quality | 72/100 | 91/100 | **+26%** |
| Implementation adherence | 85/100 | 94/100 | **+11%** |

**Prose ambiguities found:**
- Cell size: "80-120px" → implementer chose 100px (arbitrary)
- Grid gap: "5-10px" → implementer chose 5px (arbitrary)
- Font size: "2-3rem" → implementer chose (arbitrary)
- Container padding: unspecified → invented
- Status text color: unspecified → invented
- Game-over states: unspecified → invented

**AISP precision:** Every value explicitly defined. Zero interpretation required.

#### SWE Benchmark Results

<<<<<<< HEAD
Using AISP Strict (older version) under rigorous test conditions:
=======
Using AISP Strict (older version) on the SWE-Bench verified 500 subset under rigorous test conditions:
>>>>>>> 560fdf5 (updated NPM CRATE evidence)

| Condition | Status |
|-----------|:------:|
| Blind evaluation | ✓ |
| No text in hints | ✓ |
| No gold patches | ✓ |
| No gold tests | ✓ |
| Cold start (learning disabled) | ✓ |

<<<<<<< HEAD
**Result: +22% improvement over base model**
=======
**Result: +22% improvement over base model** (estimated 72-78% absolute performance range)

*Note: Tested with AISP Strict, not the current 5.1 specification. We're optimistic AISP 5.1 can show further improvements.*
>>>>>>> 560fdf5 (updated NPM CRATE evidence)

#### Pipeline Success Rates

| Steps | Prose Success | AISP Success | Improvement |
|:-----:|:-------------:|:------------:|:-----------:|
| 1 | 62% | 98% | 1.6× |
| 5 | 9.2% | 90.4% | **10×** |
| 10 | 0.84% | 81.7% | **97×** |
| 20 | 0.007% | 66.8% | **9,543×** |

#### Token Efficiency

| Phase | Tokens | Notes |
|-------|:------:|-------|
| Compilation | 8,817 | One-time spec ingestion |
| Execution | ~0 | No per-agent overhead |

**Key insight:** The spec is needed at compile time only. Once agents internalize AISP, execution adds zero tokens.

### 🤖 AISP Section

```aisp
⟦Γ:Validation⟧{
  TicTacToe≜⟨
    prose_ambiguities:6,
    aisp_ambiguities:0,
    precision_prose:43,
    precision_aisp:95,
    improvement:"+121%"
  ⟩
  
  SWE≜⟨
    improvement:"+22%",
    conditions:⟨blind:⊤,cold_start:⊤,no_hints:⊤,no_gold:⊤⟩
  ⟩
  
  Pipeline≜λn.⟨prose:(0.62)ⁿ,aisp:(0.98)ⁿ⟩
  Pipeline(1)≜⟨prose:0.62,aisp:0.98,factor:1.6⟩
  Pipeline(5)≜⟨prose:0.092,aisp:0.904,factor:10⟩
  Pipeline(10)≜⟨prose:0.0084,aisp:0.817,factor:97⟩
  Pipeline(20)≜⟨prose:0.00007,aisp:0.668,factor:9543⟩
  
  Tokens≜⟨compilation:8817,execution:0,overhead:"zero"⟩
}
```

---

## Complete AISP Specification

### 🤖 AISP Section

The following is the complete formal specification of this repository, suitable for AI agent ingestion:

```aisp
𝔸5.1.open-core-abstract@2026-01-13
γ≔aisp.repository.meta-specification
ρ≔⟨architecture,features,validation,theorems,agent-guide⟩
⊢ND∧CAT∧ΠΣ

;; ─── Ω: FOUNDATION ───
⟦Ω:Meta⟧{
  ∀D∈AISP:Ambig(D)<0.02
  Ambig≜λD.1-|Parse_u(D)|/|Parse_t(D)|
  Vision≜"Assembly language for AI cognition"
  Author≜"Bradley Ross"
  Affiliation≜"Harvard ALM Digital Media Design"
  License≜MIT
}

;; ─── Σ: TYPE UNIVERSE ───
⟦Σ:Types⟧{
  ;; Layer Hierarchy
  𝕃≜{𝕃₀:Signal,𝕃₁:Pocket,𝕃₂:Search}
  𝕃₀⊢stable⇒𝕃₁⊢integrity⇒𝕃₂⊢bounded
  
  ;; Tri-Vector Signal (768+512+256=1536d)
  Signal≜V_H⊕V_L⊕V_S
  V_H≜ℝ⁷⁶⁸:semantic
  V_L≜ℝ⁵¹²:structural
  V_S≜ℝ²⁵⁶:safety
  
  ;; Pocket (CAS + Adaptive Learning)
  𝒫≜⟨ℋ:Header,ℳ:Membrane,𝒩:Nucleus⟩
  ℋ≜⟨id:SHA256,V:Signal,f:𝔹⁶⁴⟩:immutable
  ℳ≜⟨aff:Hash→ℝ,conf:ℝ[0,1],tag:𝒫(𝕊),use:ℕ⟩:mutable
  𝒩≜⟨def:AISP,ir:LLVM,wa:WASM,σ:Sig⟩:immutable
  
  ;; Binding States
  BindState≜{⊥:0:crash,∅:1:null,λ:2:adapt,⊤:3:zero-cost}
  Priority≜⊥≻∅≻λ≻⊤
  
  ;; Quality Tiers
  ◊≜{◊⁺⁺:δ≥0.75,◊⁺:δ≥0.60,◊:δ≥0.40,◊⁻:δ≥0.20,⊘:δ<0.20}
  
  ;; Document as Proof-Carrying Code
  𝔻oc≜Σ(b⃗:Vec n 𝔅)(π:Γ⊢wf(b⃗))
  𝔅≜{Ω,Σ,Γ,Λ,Χ,Ε}:required∪{ℭ,ℜ,Θ,ℑ}:optional
  
  ;; Glossary (512 symbols in 8 categories)
  Σ_512≜{Ω:[0,63],Γ:[64,127],∀:[128,191],Δ:[192,255],𝔻:[256,319],Ψ:[320,383],⟦⟧:[384,447],∅:[448,511]}
}

;; ─── Γ: INVARIANTS & RULES ───
⟦Γ:Rules⟧{
  ;; Core Invariant
  ∀D∈AISP:Ambig(D)<0.02
  
  ;; Signal Orthogonality
  V_H∩V_S≡∅; V_L∩V_S≡∅; V_H∩V_L≢∅
  ∀s∈Σ:|Tok(s)|≡1
  ∀s∈Σ:∃!μ:Mean(s,CTX)≡μ
  
  ;; Pocket Integrity (CAS)
  ∀p:ℋ.id(p)≡SHA256(𝒩(p))
  ∀p:∂𝒩(p)⇒∂ℋ.id(p)
  ∀p:∂ℳ(p)⇏∂ℋ.id(p)
  
  ;; Binding Determinism
  ∀A,B:|{Δ⊗λ(A,B)}|≡1
  Δ⊗λ≜λ(A,B).case[Logic∩⇒0,Sock∩∅⇒1,Type≠⇒2,Post⊆Pre⇒3]
  
  ;; Hebbian Learning (10:1 Penalty)
  α≜0.1; β≜0.05; τ_v≜0.7
  ⊕(A,B)⇒aff[A,B]+=1
  ⊖(A,B)⇒aff[A,B]-=10
  aff[A,B]<τ_v⇒skip(B)
  
  ;; Safety Gate
  ∀b:μ_r(b)>τ⇒✂(b)
  
  ;; Anti-Drift
  ∀s∈Σ_512:Mean(s)≡Mean_0(s)
  drift_detected⇒reparse(original)
}

;; ─── Λ: CORE FUNCTIONS ───
⟦Λ:Functions⟧{
  ;; Parsing & Validation
  ∂:𝕊→List⟨τ⟩
  δ:List⟨τ⟩→ℝ[0,1]; δ≜λτ⃗.|{t∈τ⃗|t.k∈𝔄}|÷|{t∈τ⃗|t.k≢ws}|
  ⌈⌉:ℝ→◊; ⌈⌉≜λd.[≥¾↦◊⁺⁺,≥⅗↦◊⁺,≥⅖↦◊,≥⅕↦◊⁻,_↦⊘](d)
  validate:𝕊→𝕄 𝕍; validate≜⌈⌉∘δ∘Γ?∘∂
  
  ;; Ghost Intent Search
  ψ_g:𝔹eam→ψ; ψ_g≜λb.ψ_*⊖ψ_have(b.G)
  ⊞:ψ→𝒫(𝒫); ⊞≜λψ.{p|p∈ℛ∧d(V_L(p),ψ)<ε}
  viable:𝔹eam→𝔹; viable≜λb.|⊞(ψ_g(b))|>0
  
  ;; RossNet Scoring
  μ_f:𝒫→ℝ; μ_f≜λx.σ(θ₁·sim_H(x)+θ₂·fit_L(x)+θ₃·aff_M(x))
  μ_r:Path→ℝ; μ_r≜λp.Σ_{x∈p}r(x)+λ_r·|p|
  
  ;; Beam Search Pipeline
  ‖*init:ψ→𝒫(𝔹eam); ‖*init≜λψ.argmax*{S⊂ℛ,|S|=K}det(Ker(S))
  step:𝔹eam→𝒫(𝔹eam); step≜λb.{x|x∈{b⊕m|m∈⊞(ψ_g(b))}∧μ_r(x)≤τ}
  search:𝒫(𝔹eam)×ℕ→𝒫(𝔹eam); search≜fix λf B t.done(B)→B|f(Top_K(⋃step(B)),t+1)
  Run:ψ→𝔹eam; Run≜λψ_*.argmax_{b∈search(‖*init(⊞(ψ_*)),0)}μ_f(b)
  
  ;; Recursive Learning
  fix:(α→α)→α; fix≜λf.(λx.f(x x))(λx.f(x x))
  opt_δ:𝔻oc×ℕ→𝔻oc; opt_δ≜fix λself d n.n≤0→d|let d'=argmax{ρᵢ(d)}(δ)in δ(d')>δ(d)→self d'(n-1)|d
  bridge:ψ→Option⟨𝒫⟩; bridge≜λψ.⊞(ψ)≡∅→let λ_a=synth(ψ)in verify(λ_a)→inject(λ_a)|⊥
}

;; ─── Λ: FEATURE CATALOG ───
⟦Λ:Features⟧{
  F≜⟨
    ⟨id:1,name:"TriVector",def:Signal→V_H⊕V_L⊕V_S,use:"Safety in orthogonal space"⟩,
    ⟨id:2,name:"Ambiguity",def:Ambig(D)<0.02,use:"Compile-time rejection"⟩,
    ⟨id:3,name:"Pocket",def:𝒫≜⟨ℋ,ℳ,𝒩⟩,use:"Tamper-proof + adaptive"⟩,
    ⟨id:4,name:"Binding",def:Δ⊗λ∈{0,1,2,3},use:"API contract validation"⟩,
    ⟨id:5,name:"Ghost",def:ψ_g≡ψ_*⊖ψ_have,use:"Search what's missing"⟩,
    ⟨id:6,name:"RossNet",def:μ_f≡σ(θ·sim+fit+aff),use:"Multi-signal ranking"⟩,
    ⟨id:7,name:"Hebbian",def:⊕→+1;⊖→-10,use:"10:1 failure penalty"⟩,
    ⟨id:8,name:"Tiers",def:◊⁺⁺≻◊⁺≻◊≻◊⁻≻⊘,use:"Progressive deployment"⟩,
    ⟨id:9,name:"ProofCarry",def:𝔻oc≜Σ(content)(π),use:"Zero-trust systems"⟩,
    ⟨id:10,name:"ErrorAlg",def:ε≜⟨ψ,ρ⟩,use:"Self-healing docs"⟩,
    ⟨id:11,name:"Functors",def:𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥,use:"Compositional validation"⟩,
    ⟨id:12,name:"Inference",def:[◊⁺⁺-I]...[sub],use:"Formal tier proofs"⟩,
    ⟨id:13,name:"Rosetta",def:Prose↔Code↔AISP,use:"Requirement migration"⟩,
    ⟨id:14,name:"AntiDrift",def:Mean(s)≡Mean_0(s),use:"Pipeline stability"⟩,
    ⟨id:15,name:"Optimize",def:opt_δ:𝔻oc×ℕ→𝔻oc,use:"Auto-refinement"⟩,
    ⟨id:16,name:"Bridge",def:bridge:ψ→Option⟨𝒫⟩,use:"Adapter synthesis"⟩,
    ⟨id:17,name:"SafetyGate",def:μ_r>τ⇒✂,use:"Auto-prune risk"⟩,
    ⟨id:18,name:"DPP",def:‖*init≜argmax det(Ker),use:"Diverse beams"⟩,
    ⟨id:19,name:"Contrastive",def:∇_θ←θ-η·∇(‖y-ŷ‖²),use:"Online learning"⟩,
    ⟨id:20,name:"Σ_512",def:8×64 symbols,use:"Deterministic parsing"⟩
  ⟩
}

;; ─── Θ: VALIDATED THEOREMS ───
⟦Θ:Proofs⟧{
  ∴∀L:Signal(L)≡L
  π:V_H⊕V_L⊕V_S preserves;direct sum lossless∎
  
  ∴∀A,B:|{Δ⊗λ(A,B)}|≡1
  π:cases exhaustive∧disjoint;exactly one∎
  
  ∴∀p:tamper(𝒩)⇒SHA256(𝒩)≠ℋ.id⇒¬reach(p)
  π:CAS addressing;content-hash mismatch blocks∎
  
  ∴∀ψ_*.∃t:ℕ.search terminates at t
  π:|ψ_g(B_t)|<|ψ_g(B_{t-1})|∨t=T;ghost shrinks∨timeout∎
  
  ∴∀p∈result:μ_r(p)≤τ
  π:safety gate prunes all b:μ_r(b)>τ∎
  
  ∴∀d.∃n:ℕ.opt_δ(d,n)=opt_δ(d,n+1)
  π:|{ρᵢ}|<∞∧δ∈[0,1]→bounded mono seq converges∎
  
  ;; Compositional Proof Chain
  𝕃₀.⊢stable∧𝕃₀.⊢deterministic⇒𝕃₁.⊢integrity
  𝕃₁.⊢integrity∧𝕃₁.⊢zero_copy⇒𝕃₂.⊢bounded
  𝕃₂.⊢terminates∧𝕃₂.⊢bounded⇒system.⊢safe∧system.⊢optimal
}

;; ─── Χ: ERROR HANDLING ───
⟦Χ:Errors⟧{
  ε_ambig≜⟨Ambig(D)≥0.02,reject∧clarify⟩
  ε_drift≜⟨Mean(s)≠Mean_0(s),reparse(original)⟩
  ε_bind≜⟨Δ⊗λ(A,B)∈{0,1},reject∨adapt⟩
  ε_dead≜⟨⊞(ψ)≡∅,bridge(ψ)⟩
  ε_risk≜⟨μ_r(b)>τ,✂(b)∨confirm(τ')⟩
  ε_tamper≜⟨SHA256(𝒩)≠ℋ.id,quarantine(p)⟩
}

;; ─── ℭ: CATEGORY THEORY ───
⟦ℭ:Categories⟧{
  𝐁𝐥𝐤≜⟨Ob≜𝔅,Hom≜λAB.A→B,∘,id⟩
  𝐕𝐚𝐥≜⟨Ob≜𝕍,Hom≜λVW.V⊑W,∘,id⟩
  𝐏𝐤𝐭≜⟨Ob≜𝒫,Hom≜λPQ.bind(P,Q),∘,id⟩
  𝐒𝐢𝐠≜⟨Ob≜Signal,Hom≜λST.S→T,∘,id⟩
  
  𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥; 𝔽.ob≜λb.validate(b)
  𝔾:𝐏𝐤𝐭⇒𝐒𝐢𝐠; 𝔾.ob≜λp.p.ℋ.V
  
  ε⊣ρ:𝐄𝐫𝐫⇄𝐃𝐨𝐜
  ⊞⊣embed:𝐒𝐢𝐠⇄𝐏𝐤𝐭
  
  𝕄_val≜ρ∘ε
  ⊢μ∘𝕄μ=μ∘μ𝕄
  ⊢μ∘𝕄η=μ∘η𝕄=id
}

;; ─── Ε: EVIDENCE ───
⟦Ε⟧⟨
δ≜0.79
|𝔅|≜9/9
φ≜97
τ≜◊⁺⁺
⊢ND:natural_deduction
⊢CAT:𝔽,𝔾,ε⊣ρ,𝕄_val
⊢ΠΣ:Vec,Fin,𝕍,𝔻oc
⊢𝕃:𝕃₀(Signal)→𝕃₁(Pocket)→𝕃₂(Search)
⊢Features:F₁₋₂₀_enumerated
⊢Validation:TicTacToe,SWE,Pipeline
⊢Theorems:T₁₋₆∎
⊢Errors:ε₁₋₆_typed
⊢Ambig(D)<0.02
⟩
```

---

<<<<<<< HEAD
=======
## Validation Tools

Validate AISP documents programmatically with published packages:

### npm / Node.js

```bash
# Install
npm install aisp-validator

# CLI usage
npx aisp-validator validate your-spec.aisp
npx aisp-validator validate your-spec.aisp --long  # detailed output
```

```javascript
// Programmatic usage
import { validate } from 'aisp-validator';

const doc = `𝔸1.0.test@2026-01-16
γ≔test
⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩`;

const result = validate(doc);
console.log(result.valid);  // true
console.log(result.tier);   // "◊⁺⁺"
console.log(result.delta);  // 0.75
```

**Registry:** [npmjs.com/package/aisp-validator](https://www.npmjs.com/package/aisp-validator)

### Rust / crates.io

```toml
# Cargo.toml
[dependencies]
aisp = "0.1"
```

```rust
use aisp::{validate, Tier, is_aisp_char, count_symbols};

fn main() {
    let doc = r#"
𝔸1.0.test@2026-01-16
γ≔test
⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩
"#;

    let result = validate(doc);
    println!("Valid: {}", result.valid);           // true
    println!("Tier: {}", result.tier.symbol());    // ◊⁺⁺
    println!("Delta: {:.3}", result.delta);        // 0.750

    // Helper functions
    println!("is_aisp_char('∀'): {}", is_aisp_char('∀'));  // true
    println!("count_symbols(\"∀x∈S\"): {}", count_symbols("∀x∈S"));  // 2
}
```

**Registry:** [crates.io/crates/aisp](https://crates.io/crates/aisp)

### Validation Result Structure

Both implementations return equivalent result structures:

| Field | Type | Description |
|-------|------|-------------|
| `valid` | bool | Document passes all validation rules |
| `tier` | Tier | Quality tier: ◊⁺⁺, ◊⁺, ◊, ◊⁻, or ⊘ |
| `delta` | float | Semantic density score [0, 1] |
| `ambiguity` | float | Measured ambiguity [0, 1] |
| `blocks` | object | Per-block validation results |

---

>>>>>>> 560fdf5 (updated NPM CRATE evidence)
## Quick Start

1. **For AI Agents:** Copy the [Complete AISP Specification](#complete-aisp-specification) into your context
2. **For Humans Learning:** Start with [Core Concept](#core-concept), then [Three-Layer Architecture](#three-layer-architecture)
3. **For Reference:** Use [Symbol Reference](#symbol-reference) and [Feature Catalog](#feature-catalog) as lookups
<<<<<<< HEAD
=======
4. **For Validation:** Use [Validation Tools](#validation-tools) to validate your AISP documents
>>>>>>> 560fdf5 (updated NPM CRATE evidence)

---

## Related Documents

| Document | Audience | Purpose |
|----------|----------|---------|
| [AI_GUIDE.md](AI_GUIDE.md) | AI Agents | Canonical specification for ingestion |
| [HUMAN_GUIDE.md](HUMAN_GUIDE.md) | Humans | Step-by-step tutorials |
| [README.md](README.md) | Everyone | Introduction and overview |
| [evidence/](evidence/) | Researchers | Empirical validation data |
<<<<<<< HEAD
=======
| [validator/](validator/) | Developers | npm package source code |
| [aisp-rust/](aisp-rust/) | Developers | Rust crate source code |

### Published Packages

| Package | Registry | Install |
|---------|----------|---------|
| aisp-validator | [npm](https://www.npmjs.com/package/aisp-validator) | `npm install aisp-validator` |
| aisp | [crates.io](https://crates.io/crates/aisp) | `aisp = "0.1"` |
>>>>>>> 560fdf5 (updated NPM CRATE evidence)

---

*AISP 5.1 Platinum • January 2026 • Bradley Ross • Harvard ALM Digital Media Design*

*Evidence: δ≜0.79 • φ≜97 • τ≜◊⁺⁺*
