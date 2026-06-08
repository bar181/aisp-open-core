# AISP: A Proof-Carrying Symbolic Protocol for AI Cognition

**Bradley Ross**  
*Harvard University, ALM in Digital Media Design 2026*  
*Toronto, Ontario, Canada | Cambridge, Mass, USA*  
`bar181@github` · `linkedin.com/in/bradaross`  

**Version 3.0 (arXiv submission draft) — 2026-05-17**

**Companion artifacts.** Reference implementation, full glossary, replication evidence, and toolchain (`aisp-converter`, `aisp-validator`) at [`github.com/bar181/aisp-open-core`](https://github.com/bar181/aisp-open-core). Authoritative protocol source: [`AI_GUIDE.md`](https://github.com/bar181/aisp-open-core/blob/main/AI_GUIDE.md) (AISP 5.1 Platinum). Canonical AISP-native version of this paper at `formal-papers/aisp-protocol.md`; the present document is a derived human-readable view per the directional translation principle of §6.7.

---

## Abstract

When large language models (LLMs) communicate with each other through natural language, each hop pays a *translation tax*: a frontier model is asked to encode a thought into prose, a second model is asked to decode that prose back into a representation it can act on, and the residue between the two is *semantic ambiguity*. On the open-core AISP evidence set, the per-hop ambiguity rate for expert-authored natural-language messages between LLMs sits between 40% and 65% — a figure that compounds catastrophically over multi-step pipelines and is the root cause of brittleness in autonomous agent stacks. We present **AISP** (AI Symbolic Protocol), a proof-carrying symbolic protocol designed not as a specification language but as **the native communication medium of AI cognition** — a fixed-alphabet, deterministic-grammar lingua franca whose 512 symbols are *already in the training distribution of every frontier LLM* because they are the symbols of formal logic, type theory, and category theory drawn from arXiv, Coq, Lean, Agda, TLA⁺, and the broader formal-methods literature. AISP does not introduce new symbols; it codifies a grammar on top of the symbol substrate that frontier models already parse natively. The protocol fixes 512 atomic symbols (Σ₅₁₂) across eight categories, defines a deterministic block-structured grammar in which every well-formed document parses to a unique abstract syntax tree, and equips each document with a verifiable evidence block `⟦Ε⟧` carrying its semantic-density score δ ∈ [0, 1] and quality tier ◊. The protocol admits a categorical semantics with an error-recovery adjunction ε ⊣ ρ inducing a validation monad **𝕄**_val, and a natural-deduction calculus over twelve typed rules yielding fifteen mechanically checkable theorems. Empirically, AISP reduces per-hop ambiguity from 40–65% to below 2%, yielding a finite-N compounding improvement on the interval `[10², 6 × 10³]` over a ten-step pipeline. We report a prospective external-adoption case study in which an independent AI assistant, initially skeptical and without AISP system-prompt context, converged on the methodology after three iterations and produced σ-collapse on a target rubric dimension, validating the protocol on a corpus unfamiliar to the author. We characterize the dominant first-time-author failure mode as *English-wrapped AISP* (pattern descriptions inside set definitions rather than literal token enumeration), make the underlying assumptions explicit, and close with a failure-mode taxonomy and research agenda spanning learned conversion, compositional verification, and integration with neuro-symbolic memory.

**Keywords.** AI-to-AI communication · neural-symbolic AI · large language models · multi-agent systems · proof-carrying code · category theory · prompt engineering · cognitive architecture · formal specification · agent coordination.

---

## 1. Introduction

### 1.1 The translation tax

When an autonomous-agent system passes a thought between two LLMs — between a planner and an executor, between a critic and a refiner, between an upstream model and a downstream model in a tool-use chain — it does so almost exclusively through natural language. This is an architectural accident, not a design decision. LLMs were trained predominantly on human prose, so prose became the substrate of inter-model communication by default. But every prose hop incurs a *translation tax*: the upstream model encodes an internal representation into a prose string, the downstream model decodes the prose string into its own internal representation, and the two representations are not, in general, equivalent. The fraction of the upstream meaning that survives the round trip is what we measure as semantic alignment; its complement is *ambiguity*.

On the open-core AISP evidence set, the per-hop ambiguity rate for expert-authored natural-language messages between frontier LLMs sits between 40% and 65%, depending on task family and prompting style. This number is itself unremarkable — it is the central finding of a large prompt-engineering literature [Wei et al. 2022; Khattab et al. 2024]. What is remarkable is what happens when this loss compounds. For a ten-step pipeline at 59% per-step success, end-to-end success is `0.59¹⁰ ≈ 0.5%`. The same pipeline at 95% per-step yields `0.95¹⁰ ≈ 60%`. The factor between these regimes — roughly two orders of magnitude — is precisely the gap between an automation that requires constant human supervision and one that does not.

### 1.2 The inversion

This paper proposes an inversion of the standard architecture. Instead of treating natural language as the canonical communication medium and structured outputs as a special case, we treat **a fixed symbolic protocol as the canonical medium** and natural language as a *translation*, generated on demand for human consumption.

The inversion is enabled by an underrecognized property of contemporary frontier LLMs: the symbols of formal mathematics — ∀, ∃, ⊢, λ, ⊕, ⊗, Σ, Π, ↦, ≜, ε ⊣ ρ, ⟨⟩, ⟦⟧, and several hundred others — are *already in the model's training distribution* through arXiv preprints, Wikipedia mathematics articles, formal-methods source files (Coq, Lean, Agda, TLA⁺, Z, Alloy), LaTeX-rendered textbooks, and the broader corpus of mathematical exposition online. AISP does not invent new symbols. It selects 512 of these pre-existing symbols, fixes their semantics, defines a grammar over their composition, and adds a proof-carrying evidence block. The protocol layer is light; the substrate is pre-installed.

This inversion has three properties that natural language cannot match:

> **(i)** *Deterministic parse.* Every well-formed AISP document parses to a unique AST. There is no probabilistic ambiguity at the grammar level.
> **(ii)** *Self-certifying.* Every AISP document carries an in-band evidence block whose validity is mechanically checkable in linear time.
> **(iii)** *Native to the consumer.* Frontier LLMs already know the mathematical symbols AISP uses because those symbols are in the training corpus. The protocol requires no fine-tuning, no system-prompt scaffolding, no special tokenizer.

What makes this an *inversion* rather than just another DSL is the direction of derivation. Specification languages like Z [Spivey 1992] or TLA⁺ [Lamport 2002] are written by humans for humans to read, with optional mechanical checking. AISP is *written by AI for AI to read*, with optional human translation. The human-readable view is downstream of the AISP source, not upstream of it.

### 1.3 What AISP is, and is not

AISP is **a communication protocol**, in the same sense that TCP is a communication protocol: it defines what an exchange looks like, what guarantees it carries, and what counts as well-formed. Its uses include but are not limited to:

> *Agent instructions* — telling an autonomous agent what to do
> *Multi-agent coordination* — handoffs, contracts, declared post-conditions
> *Memory and knowledge encoding* — content-addressed storage of agent state
> *Reasoning traces* — explicit step-by-step proofs that are themselves AISP documents
> *State machines* — precise transition rules
> *API contracts* — formal pre- and post-conditions
> *Safety constraints* — provable safety properties
> *Judge rubric specification* — deterministic predicates for evaluation across heterogeneous judge models
> *Self-modification* — AISP documents that specify edits to other AISP documents, with the `⟦Ε⟧` block carrying the validity of the modification

It is *not* a programming language: it does not execute, and it is not lowered to machine code. It is *not* a knowledge graph: relations are first-class but the topology is open. It is *not* a replacement for natural language in human-to-human communication: humans should keep using prose for prose's purposes. It is the missing piece between the two — the part where AI talks to AI without going through a lossy human-language detour.

### 1.4 Contributions

This paper makes seven contributions:

* **C1 (Protocol).** We define AISP, a proof-carrying symbolic protocol over a fixed 512-symbol alphabet with a deterministic block-structured grammar, in which every well-formed document carries a machine-checkable evidence block certifying its semantic-density score δ ∈ [0, 1] and quality tier ◊ ∈ {◊⁺⁺, ◊⁺, ◊, ◊⁻, ⊘}.
* **C2 (Semantics).** We give a categorical semantics with four categories (**Blk**, **Val**, **Pkt**, **Sig**), two functors, two natural transformations, and an error-recovery adjunction ε ⊣ ρ whose induced monad **𝕄**_val is the validation pipeline.
* **C3 (Calculus).** We give a natural-deduction calculus over twelve typed inference rules with fifteen mechanically checkable theorems, including beam-search termination, tamper-evidence of content-addressed memory, density monotonicity, adjunction-law preservation, and self-validation of conforming documents.
* **C4 (Modalities).** We describe six communication modalities AISP serves — instruction, coordination, memory, reasoning, contract, and self-modification — and show why they share a single formalism rather than requiring six separate ones.
* **C5 (Empirical).** We report on the open-core evidence set: ambiguity reduction from 40–65% to <2% per hop, a finite-N compounding improvement on the interval `[10², 6 × 10³]` at N = 10, decomposition of baseline ambiguity into format / scope / semantic components, and a prospective external-adoption case study in which an independent AI assistant validated the methodology on an unfamiliar corpus.
* **C6 (Assumptions and failure modes).** We make the underlying assumptions of the protocol explicit (§3) and provide a failure-mode taxonomy including the dominant first-time-author failure mode, *English-wrapped AISP* (F_wrap), in which authors translate from English into AISP-shaped notation rather than writing in AISP natively.
* **C7 (Self-demonstration).** Appendix D presents this paper's central claims in AISP itself: the protocol describes its own properties in its own notation, with a machine-checkable evidence block.

### 1.5 Roadmap

§2 surveys related work. §3 makes the protocol's underlying assumptions explicit and bounds the scope of claims. §4 defines the protocol. §5 gives the formal semantics, calculus, and theorems. §6 describes the six communication modalities. §7 describes an optional cognitive substrate. §8 reports the empirical evaluation including the external-adoption case study. §9 discusses limitations, threats, failure modes, and comparison to alternatives. §10 concludes.

---

## 2. Background and Related Work

### 2.1 Symbolic and neural-symbolic AI

The symbolic tradition runs from McCarthy's situation calculus and Newell & Simon's physical-symbol-system hypothesis [Newell & Simon 1976] through knowledge-representation systems [Brachman & Levesque 2004] and into modern neural-symbolic methods [Garcez & Lamb 2020; Marra et al. 2024]. AISP is symbolic *at the interface*; the neural component sits inside the agent that *consumes* AISP, not in the protocol itself. This is the same separation of concerns that distinguishes a typed source language from the optimizing compiler that lowers it.

### 2.2 Multi-agent communication languages

The closest prior art to AISP — and the most overlooked in current LLM literature — is the agent-communication-language tradition: **KQML** [Finin et al. 1994] and **FIPA-ACL** [FIPA 2002]. Both defined performatives, content languages, and conversation protocols for software agents to exchange beliefs, desires, and intentions. They were ahead of their time but predated transformer LLMs and assumed a symbolic-AI agent on each end. AISP is the closest thing to a *post-LLM* successor: it inherits the principle that inter-agent communication should be *structured, typed, and self-describing*, but replaces the rigid performative ontology with a deterministic surface syntax that frontier LLMs parse natively.

### 2.3 Formal specification languages

Z [Spivey 1992], VDM [Jones 1990], B [Abrial 1996], TLA⁺ [Lamport 2002], and Alloy [Jackson 2012] all treat specifications as mathematical objects. AISP inherits this stance but optimizes for two constraints those languages do not face: (i) the *consumer is an LLM*, so the alphabet must be tokeniser-friendly and the grammar must be parseable without external tooling; and (ii) the *author may itself be an LLM*, so the protocol must be self-validating in-band rather than requiring an external prover. The `⟦Ε⟧` block discharges both constraints.

### 2.4 Prior symbolic protocols for LLMs

Two predecessor experiments by the author — Omega-AGI [Ross 2024a] and Ω-Synth/Ω-Synth+ [Ross 2024b] — explored compact symbolic vocabularies for AI agent coordination. AISP differs in three respects: (a) the alphabet is *fixed and bounded* at 512 symbols rather than open-ended; (b) the protocol carries its own proof in `⟦Ε⟧`; and (c) the semantics is given categorically rather than only operationally. SynthLang [SynthLang 2024] explored logographs and microparticles for token-efficient prompting but did not formalize a denotational semantics or verification calculus.

### 2.5 Structured generation and programmatic prompting

JSON-Schema-constrained decoding [Willard & Louf 2023; OpenAI 2023] guarantees syntactic conformance but does not constrain *meaning*: a well-formed JSON object can still encode a misunderstood intent. DSPy [Khattab et al. 2024] and LMQL [Beurer-Kellner et al. 2023] raise the abstraction level at which prompts are composed but treat the prompt body itself as opaque natural language. AISP is orthogonal: a DSPy module can emit AISP just as it emits prose; an LMQL query can ingest AISP just as it ingests JSON. The contribution of AISP is the *content layer*, not the orchestration layer.

### 2.6 Proof-carrying code

The `⟦Ε⟧` block is descended from Necula's proof-carrying code [Necula 1997]: each document ships with a machine-checkable certificate of the properties it claims. Where Necula's PCC certifies memory safety of compiled binaries, AISP certifies semantic well-formedness, density, and tier of communication. The check is cheap — linear in document size — and runs at parse time.

### 2.7 Type theory and category theory

The categorical and dependent-typed core of AISP draws on Pierce [2002], Mac Lane [1998], and the constructive type theory of Martin-Löf [1984]. We use this machinery only where it earns its keep: most consequentially, in §5.2, where error recovery is shown to be a categorical adjoint to error detection.

---

## 3. Assumptions

The claims of this paper rest on a set of assumptions about the consumer, the cryptographic substrate, the measurement methodology, and the author's cognitive context. We enumerate them explicitly so the scope of the empirical and theoretical claims is bounded, and so independent replicators can identify which assumptions hold in their setting.

### 3.1 A1: Symbolic native parseability

We assume that frontier LLMs trained on standard contemporary corpora (arXiv, Wikipedia, GitHub including formal-methods source repositories, LaTeX-rendered textbooks) have sufficient distributional exposure to the Σ₅₁₂ alphabet that they parse it without specialized training or fine-tuning. This is the *enabling condition* of the inversion thesis (§1.2): if frontier LLMs did not already parse mathematical notation natively, AISP would be just another DSL requiring adoption costs, and the architecture would lose its central advantage over structured-output schemas.

**Operational test for A1.** Present a frontier LLM with an AISP document containing no system-prompt context. Ask it to identify the document's structure, the meaning of specific symbols, and the predicates being asserted. If the model produces correct parses for typed expressions like `Δ⊗λ ⊥ ↦ crash`, `Σ_AI_tells ∩ tokens(e) ≡ ∅`, and `∀ p : ℋ.id(p) ≡ SHA256(𝒩(p))` without scaffolding, A1 holds.

**Scope of A1.** Assumed for: Claude (Anthropic), GPT-4 and successors (OpenAI), Gemini (Google), and frontier open-weight models with comparable training-data composition (Llama-3+ class). Not assumed for: smaller models without exposure to formal-methods literature, fine-tuned domain-specific models, or models trained on filtered corpora that exclude mathematical content.

### 3.2 A2: Grammar specification availability when precision matters

We assume that where block-structure conventions, tier-function semantics, or evidence-block grammar matter for parsing precision, the consumer LLM has access to `AI_GUIDE.md` (or an equivalent grammar reference) in its context window. The alphabet is native; the *grammar layer on top of the alphabet* is light but not zero. A model can identify `⟦Ω:Foundation⟧{ ... }` as a typed block from context, but the specific semantics of Ω versus Σ versus Γ are defined in the grammar reference.

In practice, A2 is a one-time context-window cost (`AI_GUIDE.md` is ~19 KB) paid once per session, after which the model parses arbitrary AISP documents without further scaffolding.

### 3.3 A3: Cryptographic primitive integrity

We assume SHA-256 is collision-resistant within the security horizon of the deployment. Theorem T₇ (tamper-evidence of content-addressed pockets; §5.4) depends on this; if SHA-256 collision resistance falls, the pocket-immutability proof falls with it, and a replacement hash function with equivalent properties must be substituted.

### 3.4 A4: Judge competence for empirical ambiguity measurement

For empirical measurements of `Ambig(D)` in §8, we assume the judge model is competent enough to recognize the pre-registered decision-point set `𝒟(D)` and evaluate alignment with author intent. A judge that itself misunderstands the task contaminates the measurement. We mitigate this by using majority-over-three judges drawn from a different vendor family than the document author, but we cannot eliminate the risk of shared-prior contamination.

### 3.5 A5: Pipeline-step independence

The finite-N compounding analysis (§8.4) treats per-step pipeline failures as independent: if step *i* succeeds with probability `s_i`, then a pipeline of *N* steps succeeds with probability `∏ s_i ≈ s^N` for uniform per-step success rates. In real-world pipelines, failures may be correlated — an upstream error often produces downstream confusion that increases the next step's failure rate. Under correlation, `s^N` is an *upper bound* on success probability, and the compounding gap between regimes is conservative rather than optimistic.

### 3.6 A6: Empirical alphabet sufficiency

The 512-symbol bound is empirically sufficient for the workloads we have tested across the open-core evidence set (specification, coordination, memory encoding, reasoning traces, judge rubrics). We do not prove expressive completeness for all possible future agent workloads. Reserved slots `∅:[448, 511]` exist to absorb future operators without breaking the bound; whether the 64 reserved slots are sufficient over a multi-year horizon is an open question.

### 3.7 A7: Corpus representativeness for set mining

Populated sets used in AISP rubrics (e.g., `Σ_AI_tells`, `Σ_owner_evidence`, `Σ_approved_voice`) are mined from a labeled corpus of accepted and rejected examples in the deployment domain. We assume the labeled corpus is representative of the deployment distribution. Set transferability across domains has not been characterized; a `Σ_AI_tells` mined from one organization's email corpus may not transfer wholesale to another organization without re-mining.

### 3.8 A8: Author cognitive capacity to recognize when AISP is required

We assume authors can recognize when their prose has lost precision and AISP is the appropriate medium. The English-wrapped AISP failure mode (F_wrap; §9.3) shows this recognition is not automatic — first-time authors typically attempt to translate from English into AISP rather than writing directly in AISP, producing pattern-description strings inside set definitions that look like AISP but inherit the interpretive latitude of English. This assumption is the weakest of the eight: in practice it requires either author training, pair-authoring with an AISP-aware collaborator, or a validator that flags pattern-description leakage.

### 3.9 Where the assumptions fail: when AISP is the wrong tool

When the assumptions above do not hold, AISP is not the right tool. Specifically:

> When the consumer is a human reader (A1 inapplicable) — use prose with formal definitions inline.
> When the consumer is a model with limited formal-methods exposure (A1 fails) — fine-tune on AISP first, or use structured-output schemas.
> When the deployment requires post-quantum security (A3 horizon-bounded) — substitute a quantum-resistant hash function.
> When the empirical-measurement budget cannot afford multi-judge protocols (A4 unaffordable) — use single-judge measurement with explicit confidence bounds and treat the numbers as estimates rather than reported values.
> When pipeline steps are tightly coupled (A5 fails) — model failures as a Markov chain rather than independent Bernoullis, and use the chain's stationary distribution for compounding analysis.

Stating where the protocol does not apply is part of stating what it claims.

---

## 4. The AISP Protocol

This section defines AISP at the level of an external user implementing a parser or generator. Internal cognitive machinery is deferred to §7.

### 4.1 Design principles

AISP is governed by five principles, in priority order:

> **P1 Determinism.** For every well-formed document *D*, parsing yields a unique abstract syntax tree: `∀D. ∃! AST. parse(D) → AST`.
> **P2 Proof-carrying.** Every document is a Σ-type: `𝔻oc ≜ Σ(content)(π : Γ ⊢ wf(content))`. The proof π is the `⟦Ε⟧` block.
> **P3 Boundedness.** The alphabet is finite and fixed: `|Σ₅₁₂| = 512`, partitioned into 8 categories of 64 symbols each.
> **P4 Density.** Quality is measured by semantic density δ ∈ [0, 1] and discretized into a five-element tier lattice ◊.
> **P5 Compositionality.** Block composition is associative and the well-formedness predicate is preserved by standard combinators.

*Human readability* is explicitly **not** a top-priority principle. AISP targets AI as primary consumer; human readability is a downstream property delivered by tooling — cheatsheets, converters, IDE plugins, prose renderings — rather than a constraint on the core syntax. This is the inversion described in §1.2 made operational: human-readable views are *derived* from AISP source, not the other way around.

### 4.2 The Σ₅₁₂ alphabet

Σ₅₁₂ is partitioned into eight ranges of 64 symbols each (Table 1).

**Table 1.** *Σ₅₁₂ categories and ID ranges.*

| Category | Symbol | ID range | Role |
|---|---|---|---|
| Transmuters | **Ω** | [0, 63] | logical connectives, definition, fixed points, evaluation control |
| Topologics | **Γ** | [64, 127] | sets, relations, shape, blackboard letters |
| Quantifiers | **∀** | [128, 191] | quantification, indexed operators, type constructors, tier glyphs |
| Contractors | **Δ** | [192, 255] | binding, state, pre/post-conditions |
| Domains | **𝔻** | [256, 319] | type domains (ℝ, ℕ, signal spaces, hashes, signatures) |
| Intents | **Ψ** | [320, 383] | intent vectors, fitness/risk, viability |
| Delimiters | **⟦⟧** | [384, 447] | block markers, context, references |
| Reserved | **∅** | [448, 511] | operator extensions (scan, prune, project, gradient, …) |

Each symbol *s* ∈ Σ₅₁₂ has fixed semantics independent of context (anti-drift property; §4.7), declared via the gloss predicate `ℙ(symbol, prose-meaning)` enumerated exhaustively in the open-core `reference.md` and summarized in Appendix A.

The alphabet boundary at 512 is not arbitrary. Three constraints determined it: (i) it is large enough to express the working vocabulary of formal logic, type theory, and category theory used by mathematics-literate models without resorting to user-defined symbols; (ii) it is small enough that the entire glossary fits in a single LLM context window with room to spare; (iii) it is a power of two, which permits efficient bit-packed symbol identifiers and category dispatch.

### 4.3 Document structure

An AISP document is a sequence of blocks:

```
𝔻oc ≜ 𝔸 ≫ CTX? ≫ REF? ≫ ⟦Ω⟧ ≫ ⟦Σ⟧ ≫ ⟦Γ⟧ ≫ ⟦Λ⟧ ≫ ⟦Χ⟧? ≫ ⟦Ε⟧
```

where `≫` is sequential composition and `?` marks optional blocks. The five required blocks are:

| Block | Role |
|---|---|
| `⟦Ω⟧` | Metalogic — invariants, document-level claims, ambiguity bound |
| `⟦Σ⟧` | Types — glossary, type universe, schemata |
| `⟦Γ⟧` | Rules — relations, inference rules, axioms |
| `⟦Λ⟧` | Functions — definitions, recursors, fixed points |
| `⟦Ε⟧` | Evidence — δ, φ (completeness), τ (tier), proof tags |

Optional blocks include `⟦Χ⟧` (typed errors), `⟦ℭ⟧` (categories), `⟦ℜ⟧` (rosetta mappings), and `⟦Θ⟧` (theorems).

### 4.4 Semantic density

For a document *D* with token stream τ⃗ = ∂(*D*), semantic density is

> δ(D) ≜ |{ t ∈ τ⃗ | t.k ∈ 𝔄 }| ÷ |{ t ∈ τ⃗ | t.k ≢ ws }|

i.e. the ratio of *alphabet tokens* (members of the foundational symbol set 𝔄 ⊂ Σ₅₁₂) to *non-whitespace tokens*. δ is computable in linear time, invariant under reflowing whitespace, and serves as a cheap proxy for `1 − Ambig(D)`: high δ correlates with low ambiguity because more of the document is in the fixed-meaning alphabet.

### 4.5 Quality tier

The tier function `⌈·⌉ : ℝ → ◊` discretizes density into a five-element totally ordered lattice:

| Tier glyph | Name | Density range | Use case |
|---|---|---|---|
| ◊⁺⁺ | Platinum | δ ≥ 0.75 | production specs, AI-to-AI contracts |
| ◊⁺ | Gold | 0.60 ≤ δ < 0.75 | high-quality documentation |
| ◊ | Silver | 0.40 ≤ δ < 0.60 | working drafts, prototypes |
| ◊⁻ | Bronze | 0.20 ≤ δ < 0.40 | initial conversions, learning |
| ⊘ | Reject | δ < 0.20 | invalid; revision required |

Tier is a coarse but practically useful safety lever: CI/CD pipelines can gate communication with `validate --min-tier=gold` to reject prose drift before it enters an agent's context.

### 4.6 Ambiguity bound

The protocol-level invariant is

> ∀ *D* ∈ AISP. Ambig(D) < 0.02,
> where Ambig(D) ≜ 1 − |Parse_u(D)| / |Parse_t(D)|

with Parse_u counting *unambiguous* parses (those that resolve every decision point in alignment with author intent) and Parse_t counting *total* sampled parses across a fixed evaluation protocol (§8.1). This invariant is parser-checkable in linear time and is asserted by every well-formed `⟦Ε⟧` block.

Two distinctions matter. *First*, the parser-checkable invariant is a static property of the document; the empirical Ambig(D) on completion samples (§8) is a downstream measurement. AISP guarantees the first directly and the second statistically. *Second*, the bound applies *per document* — and therefore *per communication hop* — not to compounded pipelines, which are analyzed in §8.4.

### 4.7 Anti-drift

For an LLM consumer, the danger is that symbol meanings drift across contexts. AISP forbids this:

> ∀ *s* ∈ Σ₅₁₂. Mean(s) ≡ Mean₀(s)

i.e. every symbol's meaning is fixed to its declared gloss `ℙ(s, ·)` regardless of context. Detected drift triggers `reparse(original)` (§5.6, error ε_token). The agent enforcement clause is reproduced in §7.5.

---

## 5. Formal Semantics

### 5.1 Type universe

AISP types live in a stratified Tarski-style universe hierarchy `𝕌₀ ⊂ 𝕌₁ ⊂ 𝕌_ω`. Primitives (𝔹, ℕ, ℤ, ℝ, 𝕊) live in 𝕌₀. Tensor spaces `𝑉_H = ℝ⁷⁶⁸`, `𝑉_L = ℝ⁵¹²`, `𝑉_S = ℝ²⁵⁶` also live in 𝕌₀; their direct sum `Signal ≜ 𝑉_H ⊕ 𝑉_L ⊕ 𝑉_S` is used in §7. Dependent types `Vec ≜ Πn:ℕ. 𝕌₀ → 𝕌₀` and `Fin ≜ Πn:ℕ. {k:ℕ | k < n}` live in 𝕌₁. A document is itself a Σ-type:

> 𝔻oc ≜ Σ(b⃗ : Vec n 𝔅)(π : Γ ⊢ wf(b⃗))

This is the Curry–Howard reading of P2: *every* document is a pair of a block sequence and a proof of its well-formedness.

### 5.2 Categorical semantics

We define four categories.

> **Definition 5.1** *(Block category).* **Blk** has objects 𝔅 (block kinds: Ω, Σ, Γ, Λ, Χ, Ε, …) and morphisms `hom(A, B) ≜ A → B` (block transformers), with the usual composition and identities.

> **Definition 5.2** *(Validation category).* **Val** has objects `𝕍 = Σ(ν:𝔹)(τ:◊)(δ:ℝ[0,1])(φ:Fin 101)` — validation tuples — and a refinement order `V ⊑ W` as morphism.

> **Definition 5.3** *(Pocket category).* **Pkt** has objects 𝒫 (pockets; §7.2) and morphisms `bind(P, Q)`.

> **Definition 5.4** *(Signal category).* **Sig** has objects in the signal space `ℝᵈ ⊕ ℝᵈ ⊕ ℝᵈ` and morphisms as linear maps.

#### 5.2.1 Functors

> 𝔽 : **Blk** ⇒ **Val**, defined by `𝔽.ob(b) ≜ validate(b)` and `𝔽.mor(f) ≜ 𝔽(cod f) ⊒ 𝔽(dom f)`. 𝔽 transports a block to its validation tuple and a block-morphism to a refinement.
> 𝔾 : **Pkt** ⇒ **Sig**, defined by `𝔾.ob(p) ≜ p.ℋ.𝑉` (the pocket's header signal) and `𝔾.mor(f) ≜ 𝔾(cod f) ∼ 𝔾(dom f)`.

#### 5.2.2 Natural transformations

> η : ∂ ⟹ 𝔽 — the *tokenization transformation*. For each block kind `b`, `η_b : ∂(b) → 𝔽(b)` sends the token stream of a block to its validation tuple.
> ζ : `Id_Pkt` ⟹ 𝔾 ∘ 𝔾⁻¹ — the *signal round-trip*. Establishes coherence of pocket-signal embedding/projection.

#### 5.2.3 Adjunctions and the validation monad

The most consequential categorical fact is that *error recovery is adjoint to error detection*:

> ε ⊣ ρ : **Err** ⇄ **Doc**

with unit `η(d) : d → ρ(ε(d))` (recovery weakly improves the document) and counit `ε(ρ(e)) ⊑ e` (re-detection of a recovered error refines the original). The composite

> 𝕄_val ≜ ρ ∘ ε

is a monad with multiplication `μ : 𝕄² → 𝕄` and unit `η : Id → 𝕄`, satisfying the standard laws (Lemma 5.10). This is the validation pipeline — applied repeatedly, it converges to a stable document.

> **Lemma 5.10** *(Monad laws for 𝕄_val).* The composite `ρ ∘ ε` satisfies `μ ∘ 𝕄μ = μ ∘ μ𝕄` and `μ ∘ 𝕄η = μ ∘ η𝕄 = id`.

*Proof sketch.* Standard from the ε ⊣ ρ adjunction via the triangle identities; full diagram in Appendix B. □

### 5.3 Inference rules

We give twelve typed natural-deduction rules. The five most consequential are reproduced here; the full set is in Appendix B.

```
                                                ⊢ wf(d)    δ(d) ≥ ¾
    ───────────── [ax-header]                ─────────────────────── [◊⁺⁺-I]
    d↓₁ ≡ 𝔸  ⊢ wf₁(d)                              ⊢ d : ◊⁺⁺


    wf₁(d)    wf₂(d)                            Post(A) ⊆ Pre(B)
    ────────────────── [∧I-wf]                ──────────────────── [bind-zero]
        ⊢ wf(d)                                ⊢ Δ⊗λ(A, B) = 3


              SHA256(𝒩(p)) ≡ ℋ.id(p)
              ───────────────────────── [pkt-valid]
                     ⊢ intact(p)
```

Rule **[ax-header]** asserts that a document begins with the literal symbol 𝔸 (the protocol marker). Rule **[∧I-wf]** introduces overall well-formedness from header and block-count premises. Rules **[◊·-I]** introduce tier judgments from density bounds. Rule **[bind-zero]** declares a binding zero-cost when the post-condition of A discharges the pre-condition of B (foundational for inter-agent contracts; §6.2). Rule **[pkt-valid]** discharges pocket integrity from a SHA-256 content-hash match (depends on A3).

### 5.4 Theorems

We list fifteen theorems, with selected proofs in Appendix B. T₁–T₁₄ are protocol-level theorems; T₁₅ is the document-level self-validation theorem.

| # | Statement |
|---|---|
| T₁ | ∀ L. Signal(L) ≡ L *(direct-sum decomposition is lossless)* |
| T₂ | ∀ A, B. \|{ Δ⊗λ(A, B) }\| ≡ 1 *(binding is total and deterministic)* |
| T₃ | 𝔽(id_A) = id_{𝔽A} *(functor preserves identity)* |
| T₄ | 𝔽(g ∘ f) = 𝔽(g) ∘ 𝔽(f) *(functor preserves composition)* |
| T₅ | ∀ d, s. s ∈ 𝔄 ⇒ δ(d ⊕ s) ≥ δ(d) *(density monotone under alphabet enrichment)* |
| T₆ | ∀ d. ρ(ε(d)) ⊒ d *(adjunction unit: recovery refines)* |
| T₇ | ∀ p. tamper(𝒩) ⇒ SHA256(𝒩) ≠ ℋ.id ⇒ ¬reach(p) *(tamper-evidence; depends on A3)* |
| T₈ | ∀ ψ*. ∃ t : ℕ. search terminates at t *(beam search terminates)* |
| T₉ | ∀ p ∈ result. μ_r(p) ≤ τ *(safety gate is sound)* |
| T₁₀ | 𝔼[μ_f(search(K))] ≥ 𝔼[μ_f(greedy)] *(beam ≥ greedy in expectation)* |
| T₁₁ | ∃ t. θ_t ≈ θ_{t+1} *(parameter convergence, Robbins–Monro)* |
| T₁₂ | ∀ d. ∃ n : ℕ. opt_δ(d, n) = opt_δ(d, n+1) *(density optimizer converges)* |
| T₁₃ | ∀ pats, n. \|gen(pats, n)\| ≤ \|pats\| *(generalization contracts)* |
| T₁₄ | ∀ τ₁, τ₂ ∈ ◊. τ₁ ≤ τ₂ ∨ τ₂ ≤ τ₁ *(tier lattice is totally ordered)* |
| T₁₅ | validate(D) ≡ ◊⁺⁺ ⇐ wf(D) ∧ δ(D) ≥ ¾ ∧ all_required_blocks(D) ∧ verified(⟦Ε⟧) *(self-validation closure)* |

T₁₅ is what permits a document to assert its own conformance: a well-formed document with sufficient density, all required blocks present, and a verified evidence block is Platinum-tier *by construction*, with the validator confirming this in linear time.

### 5.5 Compositional safety chain

The three layers of §7 are linked by a compositional proof chain:

```
   𝕃₀.⊢ stable      𝕃₀.⊢ deterministic
   ─────────────────────────────────── [P₁]
                 𝕃₁.⊢ integrity

   𝕃₁.⊢ integrity   𝕃₁.⊢ zero_copy
   ─────────────────────────────────── [P₂]
                 𝕃₂.⊢ bounded

   𝕃₂.⊢ terminates 𝕃₂.⊢ bounded
   ─────────────────────────────────── [P₃]
       system.⊢ safe ∧ system.⊢ optimal
```

This is the AISP analogue of CompCert's compositional refinement chain [Leroy 2009].

### 5.6 Typed error algebra

Errors are modelled as Σ-types `ε ≜ Σ(ψ : 𝔻oc → 𝔹)(ρ : Πd:𝔻oc. ψ(d) = ⊤ → 𝔻oc)`, i.e. a detection predicate ψ paired with a recovery function ρ defined on documents the predicate accepts. AISP defines eleven canonical errors with their recoveries (`ε_parse`, `ε_ambig`, `ε_token`, `ε_H`, `ε_C`, `ε_E`, `ε_dist`, `ε_veto`, `ε_sig`, `ε_dead`, `ε_risk`; see Appendix B). The pipeline `ρ* ≜ foldl(>=>) pure { ρᵢ | ψᵢ = ⊤ }` is well-typed because Kleisli composition over the Maybe monad is associative.

---

## 6. Communication Modalities

This section establishes C4: that AISP serves six distinct communication modalities through a single formalism rather than requiring six separate languages.

### 6.1 Agent instructions

The most common use of AISP is the directed instruction: one agent (planner, user, supervisor) emits an AISP document, and a second agent (executor) ingests it. The instruction modality is the simplest because there is no return channel inside the document itself — the executor's response is a separate AISP document, possibly emitted at a much later time.

An instruction document populates `⟦Ω⟧` with invariants the executor must preserve, `⟦Σ⟧` with the types of inputs and outputs, `⟦Γ⟧` with rules of execution, and `⟦Λ⟧` with the function the executor is to compute or the procedure to follow. The `⟦Ε⟧` block carries the author's claim that the instruction is well-formed and tiered.

Compare with a natural-language instruction. The prose version requires the executor to *infer* which fragments are invariants, which are types, which are rules, and which are intended outputs. The AISP version *declares* each. The reduction in inference burden is the source of the empirical ambiguity reduction reported in §8.

### 6.2 Multi-agent coordination

When two agents must hand off work mid-task, the central object is the *contract*: a pre-condition the upstream agent guarantees and a post-condition the downstream agent requires. AISP encodes this with the binding-cost calculus `Δ⊗λ` from §5.3:

```
Post(A) ⊆ Pre(B)    ⇒    Δ⊗λ(A, B) = 3        [zero-cost handoff]
Type(A) ≠ Type(B)   ⇒    Δ⊗λ(A, B) = 2        [adapter required]
Sock(A) ∩ Sock(B) ≡ ∅ ⇒  Δ⊗λ(A, B) = 1        [null binding]
Logic(A) ∩ Logic(B) ⇒ ⊥  ⇒ Δ⊗λ(A, B) = 0      [logical incompatibility / crash]
```

By T₂, the binding is total and deterministic: every pair (A, B) has exactly one cost. The orchestrator selects the appropriate path: zero-cost handoffs route immediately; adapter-required handoffs invoke a synthesis step; null bindings reject silently; incompatible bindings abort the workflow with a logged proof of incompatibility.

This is the AISP analogue of session types [Honda et al. 1998]: a typed discipline for safe inter-process communication. The novelty here is not the discipline but its expression in a notation the LLM endpoints already parse.

### 6.3 Memory and knowledge representation

Agents that must remember across sessions need a memory format. AISP serves this role through the *pocket*:

> 𝒫 ≜ ⟨ℋ : Header, ℳ : Membrane, 𝒩 : Nucleus⟩

with `ℋ.id ≡ SHA256(𝒩)` (content-addressed immutability). The nucleus 𝒩 is itself an AISP document: a self-validating, proof-carrying record of what the agent learned, with `⟦Ε⟧` certifying its quality. The membrane ℳ accumulates learned affinities and confidence; the header ℋ carries the addressing signal. By T₇, tampering with 𝒩 changes the hash and renders the pocket unreachable — the memory cannot be silently rewritten.

This is the substrate of the open-core reference implementation's `brain.db`/`memory.db` architecture and, more broadly, of the HAI-OS research line [Ross 2026b]. The point for the present paper is that the same protocol that expresses instructions (§6.1) also expresses what agents remember about *executing* instructions, and the two are interoperable.

### 6.4 Reasoning traces

When an agent's reasoning must be inspected — for audit, for debugging, for downstream consumption by a critic — the trace itself is best expressed in AISP. A reasoning trace populates `⟦Γ⟧` with the rules invoked, `⟦Λ⟧` with the function applications, and a `⟦Θ⟧` block (optional) with the theorems claimed and discharged. Each step is a typed inference; the entire trace is a `𝔻oc` whose `⟦Ε⟧` block certifies that every rule application is well-formed.

The contrast with natural-language chain-of-thought [Wei et al. 2022] is sharp. A prose chain-of-thought is *suggestive*: it persuades a reader (human or model) that a conclusion follows, without mechanically checking that it does. An AISP reasoning trace is *deductive*: each step matches an inference rule, and the validator can confirm or refute the chain in linear time. This is the cognitive analogue of the difference between informal mathematical exposition and a Coq proof script.

### 6.5 Contracts, safety constraints, and judge rubrics

API contracts and safety constraints are special cases of §6.1 and §6.2, but they merit separate treatment because they are the most operationally consequential. A contract document declares pre-conditions in `⟦Σ⟧`, post-conditions and invariants in `⟦Ω⟧`, and an explicit safety predicate in `⟦Χ⟧`. By the [bind-zero] rule (§5.3), two agents whose contracts compose `Post(A) ⊆ Pre(B)` form a zero-cost handoff.

A particularly important sub-modality is *judge rubric specification*. When a multi-judge ensemble is asked to score an artifact across heterogeneous dimensions, judge variance is itself a contamination source: judges with different interpretations of the same rubric item produce score spreads that conflate true signal with rubric ambiguity. AISP-formatted rubrics with *populated sets* (literal-token enumerations, not pattern descriptions) eliminate this variance source by making the predicate the judges execute identical across model families. The external-adoption case study in §8.5.4 reports σ-collapse from 3.42 to near-zero on a target rubric dimension after AISP-formatting, validating this sub-modality on an external corpus.

The strongest practical consequence is *machine-checkable safety property declaration*. A safety constraint expressed in AISP is not a recommendation; it is a typed predicate the validator can evaluate. This is the AISP analogue of dependent refinement types [Vazou et al. 2014] in the agent-coordination setting.

### 6.6 Self-modification

The most powerful — and most subtle — modality is self-modification: an AISP document specifying changes to another AISP document. This works because every AISP document is a Σ-type carrying a well-formedness proof; an *edit* is then a function `𝔻oc → 𝔻oc` that the modifying document specifies, plus a proof that the edited result remains well-formed. The `⟦Ε⟧` block of the modifying document carries the proof.

This is how the open-core toolchain bootstraps: `aisp-converter` is a function from prose to AISP, expressible (and expressed) as an AISP document. `aisp-validator` is a function `𝔻oc → 𝕍`, also expressible in AISP. The entire toolchain is, in principle, a fixed point of AISP describing AISP.

### 6.7 The translation gradient: human-readable views

A human reading an AISP document is reading a translation. The translation is generated by a renderer — `aisp-converter` in reverse, or an LLM prompted with the gloss table `ℙ`. Multiple translations of the same source are possible, ordered by audience: a Platinum tier for AI-to-AI exchange, a Gold tier for technical documentation, a prose render for casual human reading, a slide rendering for executive presentation.

The key property is *directional fidelity*: rendering AISP → prose is lossy *and acceptable*, because the canonical form is preserved upstream. The reverse — prose → AISP — is the operation whose ambiguity AISP eliminates. This is the inversion described in §1.2 made operational.

The present document is an example of this directional fidelity: it is a prose rendering of an AISP source (Appendix D contains the source). Edits to the substantive content belong on the AISP source, not on this rendering, with the prose view regenerated as needed.

---

## 7. Cognitive Architecture *(Optional Substrate)*

The architecture in this section is *not* required to use AISP. A consumer that ingests AISP documents and acts on them is a valid AISP client. The architecture below is offered for systems that wish to *remember*, *learn*, and *coordinate* across long horizons; it is the substrate used in the open-core reference implementation and in the HAI-OS research stack [Ross 2026b].

### 7.1 Three-layer model (𝕃₀ / 𝕃₁ / 𝕃₂)

> *𝕃₀ Signal.* Every linguistic surface decomposes into a hierarchical signal: `L ≡ 𝑉_H(L) ⊕ 𝑉_L(L) ⊕ 𝑉_S(L)`, where 𝑉_H captures *semantic* embedding (768-d), 𝑉_L captures *topological* structure (512-d), 𝑉_S captures *safety* signature (256-d). `𝑉_H ∩ 𝑉_S ≡ ∅` and `𝑉_L ∩ 𝑉_S ≡ ∅` enforce that safety classification is independent of semantic and topological projections.
> *𝕃₁ Pocket.* Knowledge is stored in content-addressed *pockets* `𝒫 ≜ ⟨ℋ : Header, ℳ : Membrane, 𝒩 : Nucleus⟩` where `ℋ.id ≡ SHA256(𝒩)`. The membrane ℳ holds learned affinities; the nucleus 𝒩 holds canonical AISP, LLVM IR, WASM, and a signature.
> *𝕃₂ Search.* Retrieval and reasoning run a determinantal-point-process-initialized beam search with safety gating.

### 7.2 Pocket physics

Three invariants govern pockets: (i) *immutability* — any change to 𝒩 changes ℋ.id, so tampering is undetectable only at the cost of being unreachable (T₇, depends on A3); (ii) *access separation* — `Read(ℋ) ∩ Decomp(𝒩) ≡ ∅` ensures the header can be read freely without exposing the nucleus; (iii) *alignment* — `Align(ℋ.𝑉) ≡ 64` for cache-line efficiency.

### 7.3 Hebbian learning dynamics

Affinities between bound atoms evolve under a clipped logistic update:

```
⊕(A, B) ⇒ ℳ.aff[A, B] += 1
⊖(A, B) ⇒ ℳ.aff[A, B] -= 10
ℳ.aff[A, B] < τ_v ⇒ skip(B)
⊕ ⇒ conf' ≡ σ(logit(conf) + α),   α = 0.1
⊖ ⇒ conf' ≡ σ(logit(conf) - β),   β = 0.05
```

Success boosts affinity by +1; failure decrements by −10, an asymmetry that makes the dynamics conservative. Aged unused pockets evict after `τ_s = 90 days`.

### 7.4 Beam search and ghost physics

The beam search runs with width `K = 5`, viability threshold `τ = 0.8`, risk regularizer `λ_r = 0.1`, learning rate `η = 0.01`, and horizon `T = 100`. The fitness score is

> μ_f(x) = σ(θ₁ · sim_H(x) + θ₂ · fit_L(x) + θ₃ · aff_M(x))

a logistic combination of semantic similarity, topological fit, and learned affinity. The risk

> μ_r(p) = Σ_{x ∈ p} r(x) + λ_r · |p|

includes a size penalty to prevent risk-free runaway growth. The safety gate `∀ b. μ_r(b) > τ ⇒ ✂(b)` yields T₉.

### 7.5 Agent enforcement clause

The cognitive substrate enforces a behavioural clause on agents that consume AISP:

```
∀ agent: task ∈ {spec, instruct, coordinate} ⇒ output(AISP)
∀ response: Ambig(response) < 0.02 ∧ δ ≥ 0.40
prose_only ∧ task(spec) ⇒ reject ∧ request(AISP)
```

i.e. an AISP-conformant agent emits AISP outputs for any of the six communication modalities of §6, holds the per-message ambiguity bound, and rejects prose-only inputs for tasks that require structured exchange.

---

## 8. Empirical Evaluation

### 8.1 Methodology and operational definitions

We measure the ambiguity rate `Ambig(D)` of a communication document *D* as follows. A *decision-point set* `𝒟(D)` is defined ex ante for each evaluation task: it enumerates the orthogonal interpretive choices a downstream agent must resolve to act on *D* (e.g., "which value of *k* in `top-k`?", "does 'list' mean ordered or unordered?", "does 'unique' mean per-session or per-user?"). For *n* sampled completions (`n = 32` in the open-core suite) from a fixed frontier LLM under each prompting regime, a held-out scoring agent evaluates whether each completion's resolution of `𝒟(D)` matches the author-stated intent. Let `k` be the count of completions that match on all of `𝒟(D)`. Then

> Ambig(D) ≜ 1 − k/n

The *judge* used in the open-core suite is a different frontier LLM than the author, prompted with the gloss table and the author-stated intent rubric, with disagreements broken by majority over `m = 3` independent judge runs. This is a workable but not gold-standard protocol; the threats to its validity are discussed in §8.6. The judge protocol depends on assumption A4.

Three prompting regimes are compared:

> *Prose* — natural-language messages authored by a domain expert.
> *Structured* — JSON-Schema-constrained outputs with field-level descriptions [OpenAI 2023].
> *AISP* — protocol-conformant documents at tier ≥ Gold.

### 8.2 Decomposition of natural-language ambiguity

The 40–65% baseline is itself heterogeneous. We decompose it into three components:

| Component | Definition | Approx. share of baseline |
|---|---|---|
| Format ambiguity | The downstream model is unsure of the *shape* of the response. | ~10–15% |
| Scope ambiguity | The downstream model is unsure of the *quantifier scope* (per-user vs global, per-step vs aggregate). | ~15–20% |
| Semantic ambiguity | The downstream model is unsure of the *intended meaning* of a load-bearing term. | ~15–30% |

Structured-output schemas largely eliminate the first row; programmatic frameworks like DSPy mitigate the second; *neither addresses the third*. AISP addresses all three, but its distinctive contribution is the third: the fixed-meaning property of Σ₅₁₂ (§4.7) makes semantic ambiguity expressible as a *type error* rather than as residual uncertainty.

### 8.3 Headline results

**Table 2.** *Headline metrics on the open-core evidence set.*

| Metric | Prose baseline | Structured output | AISP |
|---|---|---|---|
| Per-hop Ambig(D) | 40 – 65 % | 25 – 40 % | **< 2 %** |
| Per-step pipeline success | 35 – 60 % | 60 – 75 % | **≥ 95 %** |
| Clarification requests per task | 3 – 5 | 1 – 2 | **0 – 1** |
| Document token footprint (relative) | 1.00 × | 1.20 × | **0.50 – 0.70 ×** |

The token-footprint row reflects a consistent finding: AISP documents are typically 30–50% *shorter* than the equivalent prose specification, because mathematical notation compresses what natural language must spell out. Combined with the reliability improvement, this means total per-task cost is lower under AISP despite the added validation pass.

### 8.4 Finite-N compounding analysis

The practical importance of per-hop ambiguity is that it compounds across pipeline steps. For a pipeline of *N* independent communication hops, each with per-hop success probability `s = 1 − Ambig(D)`, end-to-end success is `s^N` (under assumption A5).

For the *N* = 10 case typical of agent workflows:

| Regime | s | s¹⁰ | End-to-end success |
|---|---|---|---|
| Prose, worst case | 0.35 | 0.000028 | 0.003 % |
| Prose, best case | 0.60 | 0.0060 | 0.60 % |
| Structured, midpoint | 0.68 | 0.021 | 2.1 % |
| AISP | 0.98 | 0.817 | 81.7 % |

The improvement ratio of AISP over prose at *N* = 10 falls in the interval `[135 ×, 27 000 ×]` depending on prose baseline; the geometric midpoint is `≈ 2 000 ×`. We previously summarized this as "≈97×" in the open-core README; that figure corresponds to a particular prose midpoint (`s = 0.59`) at *N* = 10. The interval above is the defensible range.

The compounding does *not* extend to an infinite product. For any fixed positive ambiguity `∇ > 0`, `lim_{N→∞} (1 − ∇)^N = 0`. AISP wins because the constant in front of *N* in the exponential decay is roughly two orders of magnitude smaller than prose's, *not* because the limit is qualitatively different. The practical horizon — agent pipelines of length 5–50 — is where the constant matters.

### 8.5 Case studies

#### 8.5.1 Tic-Tac-Toe rule baseline

A six-rule prose specification of Tic-Tac-Toe contains six distinct ambiguities (turn order under simultaneous bids, win-condition tie-breaking, draw declaration, board reset, legal-move definition, end-state determinism). The AISP version eliminates all six by construction. Ledger: `evidence/tic-tac-toe/` in the open-core repository.

#### 8.5.2 Rosetta validation

The full Σ₅₁₂ symbol set was round-tripped: prose → AISP → prose. Forward conversion was the rule-based `aisp-converter`; reverse was a frontier LLM prompted with `ℙ`. Symbol-level round-trip fidelity was 100%. Document-level semantic round-trip was not measured.

#### 8.5.3 SWE-Bench results

On a 60-instance representative subsample of the SWE-Bench verified-500 corpus, we compared two conditions: (i) the **AISP workflow** — a two-step process in which a frontier LLM first generates an AISP specification from the bug description and original repository state, then uses that specification to generate the patch — versus (ii) a **one-shot LLM-only baseline** using the same frontier model. Both conditions had access to the original repository state for each instance under a *time-machine protocol* (historically-accurate repo state per instance, restricted to commits prior to the canonical fix to prevent future-fix leakage).

**Results.** AISP workflow: 92% pass rate (55/60). One-shot LLM-only baseline: 44% pass rate (26/60). Absolute improvement: 48 percentage points; relative improvement: 2.1×.

The 2.1× per-task improvement is consistent with per-hop ambiguity reduction (§8.3) compounding across the AISP pipeline's two structured hops — specification generation and patch generation from specification — versus the single noisy hop of direct patch generation in the baseline.

A preliminary trial run *without* the time-machine protocol (i.e., without restricting repo state to the pre-fix historical version) failed under both conditions, with 0% pass over 10 'easy' instances. We identified the failure as a testing-harness artifact — the harness could not reliably evaluate against the canonical patch when post-fix repo state was accessible — rather than a methodology issue under test, and re-ran the trial under the corrected time-machine protocol. The 92% / 44% figures reported above use the corrected protocol exclusively.

**Limitations of this case study.** *n* = 60 is a subsample of the 500-instance verified subset; the subsampling procedure was selected for representativeness across difficulty bands but was not formally pre-registered. Future replication should pre-register the sampling protocol and ideally cover the full verified-500 set. The frontier-model choice was fixed across both conditions (same model, same temperature) to isolate the methodology variable; results may vary across model families.

#### 8.5.4 External-adoption case study

During an unrelated engagement, an independent AI assistant working on a content-evaluation system was asked to evaluate the AISP methodology for use on its judge-rubric layer. The assistant's initial response was to recommend wholesale rejection of AISP in favour of cherry-picking three specific ideas (semantic density, evidence blocks, tier conversion). After reading `AI_GUIDE.md` and applying the methodology to a single high-variance judge dimension (D7, "owner-clarity," with initial cross-judge variance σ_v1 = 2.84 across a four-judge council), the assistant produced three successive AISP rubric specifications:

> *v3.0* — used English pattern-description strings inside set definitions (`Σ_owner_evidence ≜ {"approves|ratifies|gates|enforces", …}`). Re-running the judge council produced σ_v2 = 3.42 — *increased* variance relative to the English baseline.
> *v3.1* — same failure mode with minor syntactic adjustments. σ remained elevated.
> *v3.2* — after explicit correction to "trust AISP fluency and mine literal n-grams from labeled corpus rather than translating from English," the assistant rewrote the rubric using pure literal-token enumeration drawn from a labeled corpus of accepted and rejected examples. Re-running the judge council produced σ-collapse on the target dimension, with cross-judge variance reduced substantially below the v1 English baseline.

The assistant explicitly named the v3.0/v3.1 failure mode as "English-wrapped AISP" and attributed the unlock to "trusting AISP fluency" rather than translating from English. We formalize this failure mode as F_wrap in §9.3.

This case study is notable in three respects:

> *(i) Prospective validation on an unfamiliar corpus.* The assistant was working with its own labeled corpus, not the open-core evidence set. The methodology generalized.
> *(ii) Skeptical-to-converted trajectory.* The assistant began as a skeptic recommending rejection and converted to recommending wholesale adoption ("the methodology is now part of your stack") through empirical σ-reduction on its own data.
> *(iii) The dominant failure mode was author-side, not protocol-side.* The three failed iterations were the assistant translating from English rather than writing in AISP, despite the assistant's frontier-LLM training-data exposure to mathematical notation. This is direct evidence for the cognitive-substrate inversion thesis: the model already parsed AISP natively; what it had to unlearn was the prose-translation habit.

We treat this case study as the first prospective external validation of the methodology, with the limitations that (i) it is *n* = 1 in external-adoption count, (ii) the labeled-corpus and σ measurements are proprietary to the external engagement and reported here only in pattern, and (iii) replication across additional rubric dimensions and additional external corpora is required to generalize.

### 8.6 Inter-annotator considerations and limitations

The judge protocol in §8.1 has known limitations: (i) the scoring LLM may share biases with the authoring LLM, inflating apparent agreement; (ii) the decision-point set `𝒟(D)` is itself a judgment call, and a different rubric could produce different ambiguity numbers; (iii) the majority-over-three protocol is robust to single-judge noise but not to systematic judge bias.

Three mitigations would strengthen the empirical case substantially: (a) reporting Cohen's κ between independent human annotators on a representative subsample, (b) including at least one judge model from a different vendor family than the authoring model to control for shared-prior effects, and (c) pre-registering the decision-point sets `𝒟(D)` for an external replication. We treat all three as priority work for v2 of the evidence suite.

---

## 9. Discussion

### 9.1 Limitations

Four limitations warrant explicit acknowledgement.

> *L1 Sample size.* The headline metrics derive from an internal evaluation suite of moderate size plus a single external-adoption case study. Independent replication on a public benchmark — particularly with adversarial prompts and out-of-distribution task families — is the obvious next step.

> *L2 Author cost is bimodal.* AISP is precise but not effortless to author — and the cost has two distinct regimes, separated by whether the author trusts their native AISP fluency. *First-time authors* typically waste 2–4 iterations attempting to translate from English into AISP, producing pattern-description strings inside set definitions and English-language hedge words inside predicates. This is the F_wrap failure mode formalized in §9.3, and the external-adoption case study in §8.5.4 documents it occurring across three iterations before correction. *Second-stage authors*, who have absorbed the correction "trust your fluency; write AISP, do not translate to AISP," produce conforming documents in single passes at near-prose authoring speed. The cost is in the *un-learning* of the English-translation habit, not in the protocol's intrinsic difficulty. The rule-based `aisp-converter` handles ~70% of typical prose by token volume; the remainder benefits from LLM-assisted conversion that falls back to a frontier model on low-confidence output. The tooling gap can be closed with a small fine-tuned model (roadmap item F₂), but the cognitive gap is closed by author training or by pair-authoring with an AISP-aware collaborator. This is the weakest of the four limitations because it is bounded and addressable.

> *L3 LLM tokenizer idiosyncrasies.* Σ₅₁₂ relies on Unicode mathematical symbols. Tokenizer behaviour varies; in pathological cases a symbol may split into multiple BPE tokens, marginally inflating cost. We have not observed this affect *semantic* fidelity in practice. Addressed by roadmap F₄.

> *L4 No proof of expressive completeness.* §4 fixes the alphabet at 512 symbols. This is empirically sufficient (assumption A6) for the workloads we have tested, but we do not prove that Σ₅₁₂ is expressively complete for every communication an autonomous agent will ever need. Reserved slots (`∅`) exist to absorb future operators without breaking the bound; whether a stable bound can be maintained over a multi-year horizon is an open question.

### 9.2 Threats to validity

*Internal validity.* The ambiguity metric `Ambig(D)` depends on the operational definition of "decision point," which is itself a judgment call. We mitigate this by predefining decision points before sampling and by releasing the suite in the open-core repository for inspection.

*External validity.* Results generalize to frontier LLMs accessed via standard APIs in the period 2025–2026 (assumption A1). Behaviour on smaller open-weight models or on fine-tuned domain models is plausibly different; we expect AISP to *help more* on weaker models (because it reduces the inference burden) but have not measured this. Generalization to non-Latin tokenizers and to non-English speaking authoring contexts is also unexplored.

*Construct validity.* "Pipeline success" is a proxy for utility; a pipeline can succeed end-to-end and still produce a low-quality artifact. We pair the success metric with a downstream human-rated quality check in the longer evidence write-ups but do not present those numbers here.

*External-adoption validity.* The case study in §8.5.4 is *n* = 1. The σ-collapse pattern is consistent with the methodology's claims, but generalization across additional rubric dimensions and additional external corpora is required before treating the pattern as established for arbitrary deployment contexts.

### 9.3 Failure modes of AISP

When AISP underperforms expectations, the failure modes we have observed are:

> *F_tok Tokenizer split.* A composite symbol (e.g., `Δ⊗λ`) is split into three BPE tokens by the consumer, which then treats them as a sequence rather than a compound. Mitigation: the validator emits a warning when the symbol bytes fall outside a known-good BPE family. Addressed by roadmap F₄.

> *F_drift User-defined symbol drift.* A document defines a custom symbol whose meaning the consumer infers from context rather than from the declaration, drifting across long contexts. Mitigation: §4.7 anti-drift enforcement; in practice, restricting user-defined symbols to `⟦Σ⟧` declarations and re-emitting the gloss on re-entry.

> *F_blk Block-order violation.* A document presents `⟦Λ⟧` before `⟦Γ⟧`; some lenient consumers parse it anyway, which masks a real specification error. Mitigation: strict validation in `aisp-validator`.

> *F_render Translation drift in human-facing views.* An AISP document is rendered to prose for human review, the human edits the prose, and the round-trip back to AISP loses the intent. Mitigation: render-once-edit-on-source workflows; do not round-trip through prose for source-of-truth changes.

> *F_amb Adversarial ambiguity.* A document is syntactically AISP but constructed to exploit consumer-side cultural assumptions (e.g., default sort order). Mitigation: pre-registered decision-point sets for adversarial test families; this is partially open work.

> *F_wrap English-wrapped AISP (the dominant first-time-author failure mode).* The author writes English-language pattern descriptions inside what appear to be AISP set definitions — e.g., `Σ ≜ {"approves|ratifies|gates|enforces"}` rather than `Σ ≜ {"approve", "ratify", "gate", "enforce", "approves", "approval", …}` populated from a labeled corpus. The notation looks like AISP but inherits the interpretive latitude of English. *Diagnostic:* judges disagree on token attribution despite formal-looking specs; σ may *increase* relative to the English baseline because the formal notation falsely signals precision that is not present in the predicate. *Mitigation:* enforce literal-n-gram discipline; reject specs whose Σ-set members contain regex metacharacters (`|`, `*`, `?`), ellipsis-style descriptors, or English-language pattern words; require Σ-set members to be drawn from a labeled corpus rather than authored speculatively. *Prevalence:* observed in the external-adoption case study (§8.5.4) across three of three first-time AISP authoring iterations before explicit correction. This is the failure mode every first-time external adopter should expect to encounter; flagging it pre-emptively converts a likely first-week failure into something the protocol anticipates and prevents.

### 9.4 Comparison to alternatives

**Table 3.** *AISP versus alternative communication media for AI-to-AI exchange.*

| Property | Plain prose | Structured JSON | DSPy/LMQL | Domain DSL | **AISP** |
|---|---|---|---|---|---|
| Deterministic parse | ✗ | ✓ | ✓ (within DSL) | ✓ | **✓** |
| Semantically deterministic | ✗ | ✗ | partial | ✓ (within domain) | **✓** |
| Proof-carrying | ✗ | ✗ | ✗ | ✗ | **✓** |
| Native to frontier LLMs | partial | ✓ | requires wrapper | requires fine-tune | **✓** |
| Author cost (after un-learning) | low | low | medium | high | **low–medium** |
| Domain-portable | ✓ | ✓ | ✓ | ✗ | **✓** |
| Self-validating | ✗ | ✗ | ✗ | ✗ | **✓** |

AISP's distinguishing property is the proof-carrying row: every document carries its own validation certificate, and the certificate is checkable in linear time at the receiver.

### 9.5 Relation to neuro-symbolic memory

AISP is the *interface*; a complete agent needs a *memory*. In the HAI-OS research line [Ross 2026b], AISP documents are stored as content-addressed pockets (§7.2) with affinities learned across runs (§7.3). The combination yields a *learning flywheel*: high-fidelity communication produces high-fidelity execution traces, which yield high-fidelity affinities, which lower the cost of producing the next high-fidelity communication. Quantifying this flywheel rigorously is open work.

---

## 10. Conclusion and Future Work

We have presented AISP, a proof-carrying symbolic protocol for AI-to-AI communication. The protocol fixes the alphabet, makes the grammar deterministic, equips every document with a machine-checkable evidence block, and supports a categorical semantics in which validation is a functor and error recovery is an adjunction. Empirically, AISP reduces per-hop ambiguity from 40–65% to below 2% on the open-core evidence set, yielding a finite-N compounding improvement in the interval `[135 ×, 27 000 ×]` over a ten-step pipeline. The protocol serves six distinct communication modalities — instruction, coordination, memory, reasoning, contract, self-modification — through a single formalism. A prospective external-adoption case study (§8.5.4) documents skeptical-to-converted trajectory by an independent AI assistant, with σ-collapse on a target rubric dimension validating the methodology on an unfamiliar corpus.

The deepest claim of the paper is *the inversion*: that AISP is not an alternative to natural language as a communication medium between AI systems, but the *canonical* medium, with natural language being a translation layer for human consumption. This claim is empirical, not a priori; what we have shown is that the inversion is technically feasible, operationally beneficial in the regimes we have measured, and adoptable by external systems through the specific corrective trajectory we documented (un-learning English-translation habit, embracing native AISP authoring). Whether it becomes the prevailing pattern is a question of adoption, not of physics.

Four research directions seem most pressing:

> *F1 Independent replication* on a public benchmark with pre-registered decision-point sets and cross-vendor judge models, with at least three external-adoption case studies to corroborate §8.5.4.
> *F2 Learned conversion* at scale: a small model fine-tuned specifically on prose → AISP would shift author cost from the bimodal (F_wrap-prone) regime described in L₂ to a uniformly low regime, closing the largest practical adoption gap.
> *F3 Compositional verification*: extending the §5 calculus to *compose* AISP exchanges across multi-agent boundaries with the proof-carrying property preserved end-to-end. The compositional safety chain (§5.5) is a starting point.
> *F4 Cross-tokenizer stability*: characterizing the conditions under which Σ₅₁₂ symbols survive the BPE tokenization of frontier LLM families, and standardizing a known-good encoding.

The reference implementation, full glossary, evidence, and tooling are available at [`github.com/bar181/aisp-open-core`](https://github.com/bar181/aisp-open-core) under the MIT licence.

---

## Acknowledgements

This work was carried out as part of the author's Master's research at Harvard Extension School (ALM, Digital Media Design). The author thanks his thesis committee for sharpening the formal exposition during the capstone defence of May 2026, the Agentics Foundation community for sustained critique of early drafts, the independent practitioner whose external-adoption trajectory provided §8.5.4, and his collaborators on the HAI-OS substrate for the engineering work that made the empirical claims testable.

---

## References

Abrial, J.-R. (1996). *The B-Book: Assigning Programs to Meanings*. Cambridge University Press.

Anthropic (2024). *Claude 3 Model Card*. Technical Report.

Bai, Y., et al. (2022). *Constitutional AI: Harmlessness from AI Feedback*. arXiv:2212.08073.

Beurer-Kellner, L., Fischer, M., & Vechev, M. (2023). *Prompting Is Programming: A Query Language for Large Language Models*. PLDI 2023.

Brachman, R. J., & Levesque, H. J. (2004). *Knowledge Representation and Reasoning*. Morgan Kaufmann.

Brown, T., et al. (2020). *Language Models are Few-Shot Learners*. NeurIPS 2020.

Finin, T., Fritzson, R., McKay, D., & McEntire, R. (1994). KQML as an agent communication language. CIKM 1994.

FIPA (2002). *FIPA ACL Message Structure Specification*. Foundation for Intelligent Physical Agents.

Garcez, A. d'A., & Lamb, L. C. (2020). *Neurosymbolic AI: The 3rd Wave*. arXiv:2012.05876.

Honda, K., Vasconcelos, V. T., & Kubo, M. (1998). Language primitives and type discipline for structured communication-based programming. ESOP 1998.

Jackson, D. (2012). *Software Abstractions: Logic, Language, and Analysis* (rev. ed.). MIT Press.

Jones, C. B. (1990). *Systematic Software Development Using VDM* (2nd ed.). Prentice Hall.

Khattab, O., et al. (2024). *DSPy: Compiling Declarative Language Model Calls into Self-Improving Pipelines*. ICLR 2024.

Lamport, L. (2002). *Specifying Systems: The TLA⁺ Language and Tools*. Addison-Wesley.

Leroy, X. (2009). Formal verification of a realistic compiler. *Communications of the ACM*, 52(7), 107–115.

Mac Lane, S. (1998). *Categories for the Working Mathematician* (2nd ed.). Springer.

Marra, G., Dumančić, S., Manhaeve, R., & De Raedt, L. (2024). From statistical relational to neuro-symbolic artificial intelligence: A survey. *Artificial Intelligence*, 328.

Martin-Löf, P. (1984). *Intuitionistic Type Theory*. Bibliopolis.

Necula, G. C. (1997). Proof-carrying code. POPL 1997.

Newell, A., & Simon, H. A. (1976). Computer science as empirical inquiry: Symbols and search. *Communications of the ACM*, 19(3), 113–126.

OpenAI (2023). *Function Calling and Tool Use*. Platform Documentation.

Pierce, B. C. (2002). *Types and Programming Languages*. MIT Press.

Ross, B. (2024a). *Omega-AGI Symbolic Language*. GitHub gist `bar181/782cfd01f3fd8a635ea718048c1d1c1e`.

Ross, B. (2024b). *Ω-Synth+: Neural-Symbolic Language for AGI Coordination*. GitHub gist `bar181/bef77255749000a079d61a3260b9463c`.

Ross, B. (2026a). *AISP Open Core (Reference Implementation)*. GitHub: `github.com/bar181/aisp-open-core`.

Ross, B. (2026b). *HAI-OS: A Hybrid Agent-Intelligent Operating System*. In preparation.

Shinn, N., et al. (2023). *Reflexion: Language Agents with Verbal Reinforcement Learning*. NeurIPS 2023.

Spivey, J. M. (1992). *The Z Notation: A Reference Manual* (2nd ed.). Prentice Hall.

SynthLang (2024). *SynthLang Project Documentation*.

Vazou, N., Seidel, E. L., Jhala, R., Vytiniotis, D., & Peyton-Jones, S. (2014). *Refinement Types for Haskell*. ICFP 2014.

Wei, J., et al. (2022). *Chain-of-Thought Prompting Elicits Reasoning in Large Language Models*. NeurIPS 2022.

Willard, B. T., & Louf, R. (2023). *Efficient Guided Generation for Large Language Models*. arXiv:2307.09702.

Yao, S., et al. (2023). *Tree of Thoughts: Deliberate Problem Solving with Large Language Models*. NeurIPS 2023.

---

## Citing this paper

```bibtex
@misc{ross2026aispformal,
  author        = {Bradley Ross},
  title         = {{AISP}: A Proof-Carrying Symbolic Protocol for {AI} Cognition},
  year          = {2026},
  eprint        = {arXiv:2026.XXXXX},
  archivePrefix = {arXiv},
  primaryClass  = {cs.AI},
  url           = {https://github.com/bar181/aisp-open-core/blob/main/paper/AISP-formal-paper-v3.md},
  note          = {Companion reference implementation:
                   \url{https://github.com/bar181/aisp-open-core}}
}
```

---

## Appendix A · Σ₅₁₂ category summary

The full 512-symbol glossary lives in [`reference.md`](https://github.com/bar181/aisp-open-core/blob/main/reference.md). The eight-category top-level structure is reproduced here for the reader's convenience.

**Ω Transmuters [0–63].** `⊤ ⊥ ∧ ∨ ¬ → ↔ ⇒ ⇐ ⇔ ⊢ ⊨ ⊬ ⊭ ≡ ≢ ≜ ≔ ↦ ← ≈ ∼ ≅ ≃ ∝ ≪ ≫ ∘ · × λ Λ μ ν fix rec let in case if then else match ∎ □ ◇ ⊣ ⊸ π`

**Γ Topologics [64–127].** `∈ ∉ ∋ ∌ ⊂ ⊃ ⊆ ⊇ ⊄ ⊅ ∩ ∪ ∖ △ ∅ 𝒫 ℘ ℵ ω Ω ε δ ι κ τ θ φ ψ χ 𝔾 𝕍 𝔼 ℰ 𝒩 ℋ ℳ ℛ 𝔹 𝕊 𝕋 𝕌 𝕎 𝔸 𝔻 𝔽 ⟨ ⟩ ⟦ ⟧ ⟪ ⟫ ⌈ ⌉ ⌊ ⌋ ‖ |`

**∀ Quantifiers [128–191].** `∀ ∃ ∃! ∄ ⋀ ⋁ ⋂ ⋃ Σ Π ∏ ∐ ⨁ ⨂ ⨀ → ← ↔ ↣ ↠ ⤳ ⊕ ⊗ ⊖ ⊘ ⊙ ⊛ Vec Fin List Maybe Either Pair Unit Bool Nat Int Real String Hash Sig ◊ ◊⁺⁺ ◊⁺ ◊⁻`

**Δ Contractors [192–255].** `Δ⊗λ, State, Pre, Post, Type, Sock, Logic, Strip, DCE, Compat`. `State ≜ {⊥:0, ∅:1, λ:2, ⊤:3}`; priority `⊥ ≻ ∅ ≻ λ ≻ ⊤`.

**𝔻 Domains [256–319].** `ℝ ℕ ℤ ℚ ℂ 𝔹 𝕊 Signal 𝑉_H 𝑉_L 𝑉_S Tensor Hash Sig`. Dimensions: `d_H = 768, d_L = 512, d_S = 256, d_Σ = 1536; Hash ≜ 𝔹²⁵⁶; Sig ≜ 𝔹⁵¹²`.

**Ψ Intents [320–383].** `ψ ψ_* ψ_g ψ_have μ_f μ_r sim_H fit_L aff_M viable done conv`.

**⟦⟧ Delimiters [384–447].** `⟦Ω⟧ ⟦Σ⟧ ⟦Γ⟧ ⟦Λ⟧ ⟦Χ⟧ ⟦Ε⟧ ⟦ℭ⟧ ⟦ℜ⟧ ⟦Θ⟧ ⟦ℑ⟧ 𝔸 CTX REF`.

**∅ Reserved [448–511].** `⊞ ✂ Φ ‖_init ⊕ ⊖ ⊗ ⧺ ∂ σ ∇ conf aff skip veto inject synth bridge refine`.

---

## Appendix B · Selected proofs

**Proof of T₅ (density monotonicity under enrichment).** Let *D* have *n* alphabet tokens out of *m* non-whitespace tokens, so `δ(D) = n / m`. Appending `s ∈ 𝔄` yields `δ(D ⊕ s) = (n + 1) / (m + 1)`. We have `(n + 1) / (m + 1) ≥ n / m` iff `m(n + 1) ≥ n(m + 1)` iff `m ≥ n`, which holds because `n ≤ m` by construction. □

**Proof of T₇ (tamper-evidence).** Suppose an adversary modifies the nucleus 𝒩 of pocket *p* to 𝒩′ ≠ 𝒩. By pocket physics, `ℋ.id(p) = SHA256(𝒩)`. Collision resistance of SHA-256 (assumption A3) implies `SHA256(𝒩′) ≠ ℋ.id(p)` except with negligible probability. Content-addressed lookup hashes a query *q* and matches against `ℋ.id`; the modified nucleus's hash no longer matches, so `reach(p)` under any lookup of 𝒩 fails. □

**Proof of T₈ (search termination).** Let `B_t` be the beam at step *t* and `Φ(B_t)` the union of ghost-intent measures over its members. Either (i) `Φ(B_{t+1}) < Φ(B_t)` strictly, since each non-pruned expansion shrinks the ghost (definition of `ψ_g`), or (ii) `t = T` (horizon hit). The descent (i) cannot continue indefinitely since Φ is bounded below by 0; by well-foundedness, an index `t* ≤ T` exists with `done(B_{t*})`. □

**Proof of T₁₀ (beam ≥ greedy in expectation).** Greedy is the special case of beam search with `K = 1`. For `K > 1`, the beam explores *K* paths; the *max* over a set of *K* i.i.d. expected returns dominates the singleton case in expectation. The argmax-over-the-beam selection at termination yields `𝔼[μ_f(search(K))] ≥ 𝔼[μ_f(greedy)]`. □

**Proof of T₁₅ (self-validation closure).** Let *D* be a document satisfying: (i) `wf(D)` by the [∧I-wf] rule from the [ax-header] and [ax-blocks] premises; (ii) `δ(D) ≥ ¾` by direct computation of the density function; (iii) all required blocks `{Ω, Σ, Γ, Λ, Ε}` present, verified by structural pattern-match against the document grammar; (iv) `verified(⟦Ε⟧)` by checking the evidence block's internal `⊢` assertions in linear time. By [◊⁺⁺-I], the conjunction `wf(D) ∧ δ(D) ≥ ¾` yields `D : ◊⁺⁺`. The remaining premises (iii)–(iv) are required to discharge the conformance claim that goes beyond mere tier classification. Therefore `validate(D) ≡ ◊⁺⁺`. □

Proofs of the remaining theorems follow the patterns of standard categorical and inductive arguments; the full ledger is in [`guides/advanced/03_MATH.md`](https://github.com/bar181/aisp-open-core/blob/main/guides/advanced/03_MATH.md).

---

## Appendix C · Worked example (compact)

A production-grade AISP document specifying the validation pipeline itself, abridged:

```
𝔸5.1.complete@2026-01-09
γ ≔ aisp.specification.complete
ρ ≔ ⟨glossary, types, rules, functions, errors, proofs, parser, agent⟩
⊢ ND ∧ CAT ∧ ΠΣ ∧ μ

⟦Ω:Foundation⟧{
  𝔄 ≜ {⊤⊥∧∨¬→↔∀∃∃!λΠΣ≜≡≢∈∉⊂⊃∪∩∘⊕⊖⊗⟨⟩⟦⟧⊢⊨↦⇒∎}
  ∀ D ∈ AISP : Ambig(D) < 0.02
  Doc ≜ 𝔸 ≫ CTX? ≫ ⟦Ω⟧ ≫ ⟦Σ⟧ ≫ ⟦Γ⟧ ≫ ⟦Λ⟧ ≫ ⟦Χ⟧? ≫ ⟦Ε⟧
}

⟦Σ:Types⟧{
  𝕌₀ ⊂ 𝕌₁ ⊂ 𝕌_ω
  𝔻oc ≜ Σ(b⃗ : Vec n 𝔅)(π : Γ ⊢ wf(b⃗))
  ◊ ≜ {◊⁺⁺ ≻ ◊⁺ ≻ ◊ ≻ ◊⁻ ≻ ⊘}
}

⟦Γ:Inference⟧{
  d↓₁ ≡ 𝔸             ⊢ wf₁(d)        [ax-header]
  |b⃗| ≥ 2             ⊢ wf₂(d)        [ax-blocks]
  wf₁(d) ∧ wf₂(d)     ⊢ wf(d)         [∧I-wf]
  ⊢ wf(d) ∧ δ ≥ ¾    ⊢ d : ◊⁺⁺      [◊⁺⁺-I]
}

⟦Λ:Core⟧{
  ∂        ≜ fix λf s. s ≡ ε → [] | [hd s] ⧺ f(tl s)
  δ        ≜ λτ⃗. |{t ∈ τ⃗ | t.k ∈ 𝔄}| ÷ |{t ∈ τ⃗ | t.k ≢ ws}|
  ⌈⌉       ≜ λd. [≥¾↦◊⁺⁺, ≥⅗↦◊⁺, ≥⅖↦◊, ≥⅕↦◊⁻, _↦⊘](d)
  validate ≜ ⌈⌉ ∘ δ ∘ Γ? ∘ ∂
}

⟦Ε⟧⟨
  δ ≜ 0.81
  |𝔅| ≜ 18 / 18
  φ ≜ 98
  τ ≜ ◊⁺⁺
  ⊢ ND ∧ CAT ∧ ΠΣ ∧ μ
  ⊢ Σ₅₁₂ : 8cat × 64sym
  ⊢ Θ : T₁₋₁₅ ∎
  ⊢ Ambig < 0.02
⟩
```

The full version is `AI_GUIDE.md` in the open-core repository — itself a valid AISP document, and the canonical demonstration that the protocol is self-describing.

---

## Appendix D · This paper's central claims in AISP

The following AISP document expresses the central claims of this paper in the protocol it describes. It is intended as both a self-demonstration (the paper carries its own claims in its own notation) and as a machine-checkable summary.

```
𝔸5.1.formal-paper@2026-05-17
γ ≔ paper.aisp.formal.v3
ρ ≔ ⟨inversion, ambiguity, compounding, modalities, semantics, assumptions, failures⟩
⊢ ND ∧ CAT ∧ validate(this) ≡ ◊⁺⁺

⟦Ω:Claims⟧{
  ;; The inversion
  Canonical(AI↔AI) ≜ AISP
  Translation(AISP → Prose) : derived ∧ lossy_acceptable
  Translation(Prose → AISP) : difficult ∧ lossy_problematic

  ;; The ambiguity bound
  ∀ D ∈ ℒ_prose      : 𝔼[Ambig(D)] ∈ [0.40, 0.65]
  ∀ D ∈ ℒ_structured : 𝔼[Ambig(D)] ∈ [0.25, 0.40]
  ∀ D ∈ AISP         : Ambig(D) < 0.02

  ;; Compounding (finite N)
  ∀ N ∈ ℕ. P_e2e(N, s) ≜ s^N
  Ratio(prose, AISP, N=10) ∈ [135, 27000]
}

⟦Σ:Assumptions⟧{
  A₁:NativeParse        ≜ frontier_LLM ⊢ parse(Σ₅₁₂) without_training
  A₂:GrammarRef         ≜ AI_GUIDE.md ∈ context when precision_matters
  A₃:SHA256             ≜ collision_resistant within security_horizon
  A₄:JudgeCompetence    ≜ judge ⊢ recognize(𝒟(D))
  A₅:StepIndependence   ≜ ∀ steps. failures uncorrelated
  A₆:AlphabetSufficient ≜ |Σ₅₁₂| ≥ |workloads_tested|
  A₇:CorpusRep          ≜ labeled_corpus ≈ deployment_distribution
  A₈:AuthorRecognize    ≜ author ⊢ detect(F_wrap)  (weakest; mitigated by training)
}

⟦Σ:Modalities⟧{
  Modality ≜ {Instruction, Coordination, Memory, Reasoning, Contract, SelfMod}
  ∀ m ∈ Modality. expressible(m, AISP) ≡ ⊤
  ∀ m ∈ Modality. single_formalism(m) ≡ ⊤
}

⟦Γ:Semantics⟧{
  𝐁𝐥𝐤      ≜ ⟨Ob:𝔅, Hom:transformers, ∘, id⟩
  𝐕𝐚𝐥      ≜ ⟨Ob:𝕍, Hom:refinement, ∘, id⟩
  𝔽:𝐁𝐥𝐤⇒𝐕𝐚𝐥 ⊢ functor_laws
  ε ⊣ ρ    ⊢ adjunction
  𝕄_val    ≜ ρ ∘ ε ⊢ monad_laws
  Θ        ≜ T₁..T₁₅ ⊢ mechanically_checkable
}

⟦Λ:Validation⟧{
  validate ≜ ⌈⌉ ∘ δ ∘ Γ? ∘ ∂
  ∀ D. validate(D) ∈ ◊
  ∀ D ∈ AISP. validate(D) ≥ ◊⁻ ⇔ wf(D)
}

⟦Χ:Failures⟧{
  F_tok    ⇒ tokenizer_aware_validation
  F_drift  ⇒ §4.7 anti_drift
  F_blk    ⇒ strict_block_order
  F_render ⇒ render_once_edit_on_source
  F_amb    ⇒ pre_registered_𝒟(D)
  F_wrap   ⇒ mine_literal_ngrams_from_labeled_corpus  ;; dominant first-time failure
}

⟦Ε⟧⟨
  δ ≜ 0.76
  |𝔅| ≜ 8/8 used
  φ ≜ 97
  τ ≜ ◊⁺⁺
  ⊢ Claims : inversion ∧ ambiguity ∧ compounding ∧ modalities ∧ semantics
  ⊢ Assumptions : A₁₋₈ explicit
  ⊢ Failures : F_tok ∧ F_drift ∧ F_blk ∧ F_render ∧ F_amb ∧ F_wrap
  ⊢ Self-Demonstration : paper ⊢ AISP ⊢ paper
  ⊢ External-Validation : §8.5.4 σ-collapse on unfamiliar corpus
  ⊢ SWE-Bench : 92%(AISP-workflow) / 44%(LLM-only-baseline) on n=60 verified-500 subsample
  ⊢ Ambig(this) < 0.02
  ⊢ validate(this) ≡ ◊⁺⁺
⟩
```

A consumer that ingests Appendix D, parses the `⟦Ε⟧` block, and confirms the asserted invariants has performed the simplest possible empirical check of this paper's central claim — using nothing but the protocol the paper describes.
