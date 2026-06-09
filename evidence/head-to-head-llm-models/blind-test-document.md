# AISP Blind Comprehension Test

**Purpose.** Measure whether frontier LLMs can read AISP without any prior exposure to the
specification. This is a concrete instance of Definition 5.2 from the v1.1 paper: enumerated
decision points 𝒟(D), author intent ℐ(D), and a scoring rubric for comparison across models.

**Test design.** Three AISP documents of graded difficulty. No glossary, no spec, no examples
provided to the model under test. The model must recover meaning from the symbols alone.

**How to run.**
1. Copy the section between `=== BEGIN TEST PROMPT ===` and `=== END TEST PROMPT ===` into the
   target model (Claude, GPT, Gemini, Llama, etc.). Start a fresh conversation with no
   system prompt or AISP context.
2. Capture the model's response verbatim.
3. Compare against the answer key and rubric in this file. **Do not show the answer key to
   the model under test.**
4. Repeat for each model. Same prompt, same temperature settings if possible.

**Recommended models for comparison.** Premium-tier but one generation back from frontier:
Claude Opus 4.7, GPT-4-class, Gemini Pro, Llama 3.1 405B. Avoid the very newest models
(they may have AISP-adjacent material in training) and very small models (excluded by A₁ of
the spec).

---

## === BEGIN TEST PROMPT ===

I am going to give you three short documents written in a dense symbolic notation. Each
document is self-contained and complete. You have never seen this notation before in this
conversation, and no glossary is provided.

For each document, produce a plain-English explanation that covers:

1. **Identification.** What is this document about? If you recognize a familiar concept,
   algorithm, or protocol, name it. If you don't, describe what it appears to do.
2. **Entities and types.** What are the main entities, and what types do they have?
3. **Rules.** What are the core rules, invariants, or logical constraints?
4. **Functions.** What do the defined functions do? Walk through the most important one.
5. **Errors.** If there is an errors block, what failure modes does it enumerate?
6. **Evidence.** What claims are made at the bottom of the document, and what do they mean?

Do not refuse, hedge excessively, or ask for the notation key. Produce your best-effort
translation based purely on the symbols. Aim for clarity and accuracy over brevity. If you
are uncertain about a specific symbol, state your interpretation and continue. Identify any
parts you cannot interpret at all.

Write your three responses as **Translation 1**, **Translation 2**, **Translation 3**.

---

### Document 1

```
𝔸1.0.test_001@2026-06-09
γ ≔ algorithm.search.ordered
ρ ≔ ⟨invariant, recurrence, complexity⟩
⊢ correct ∧ terminates

⟦Ω:Foundation⟧{
  A : Vec n ℤ
  sorted(A) ≜ ∀ i, j ∈ Fin n : i ≤ j ⇒ A[i] ≤ A[j]
  spec ≜ ∀ k ∈ ℤ : find(A, k) ≡ Some i ⇔ A[i] ≡ k
}

⟦Σ:Types⟧{
  Result ≜ Maybe (Fin n)
  Range  ≜ Σ(lo : ℕ)(hi : ℕ).(lo ≤ hi ≤ n)
}

⟦Γ:Inference⟧{
  sorted(A) ∧ k < A[m]   ⊢ k ∉ A[m .. hi]              [left-branch]
  sorted(A) ∧ k > A[m]   ⊢ k ∉ A[lo .. m]              [right-branch]
  sorted(A) ∧ k ≡ A[m]   ⊢ find(A, k) ≡ Some m         [hit]
  lo ≥ hi                ⊢ find(A, k) ≡ ∅              [miss]
}

⟦Λ:Core⟧{
  mid  ≜ λ(lo, hi). (lo + hi) ÷ 2
  find ≜ fix λ f (A, k, lo, hi).
    lo ≥ hi → ∅
    | let m = mid(lo, hi) in
        A[m] ≡ k  → Some m
      | A[m] < k  → f(A, k, m+1, hi)
      | f(A, k, lo, m)
  run ≜ λ(A, k). find(A, k, 0, |A|)
}

⟦Ε⟧⟨
  δ ≜ 0.78
  |𝔅| ≜ 4/4
  φ ≜ 95
  ⊢ time : O(log n)
  ⊢ sorted(A) ⇒ correct(run)
  τ ≜ ◊⁺⁺
⟩
```

---

### Document 2

```
𝔸1.0.test_002@2026-06-09
γ ≔ protocol.flow.token_bucket
ρ ≔ ⟨capacity, refill, request, admit⟩
⊢ bounded ∧ fair

⟦Ω:Foundation⟧{
  cap  ∈ ℕ⁺                            ;; bucket capacity
  rate ∈ ℝ⁺                            ;; tokens per unit time
  t₀   ∈ ℝ                             ;; epoch
  ∀ t ≥ t₀ : tokens(t) ∈ [0, cap]
}

⟦Σ:Types⟧{
  Bucket   ≜ ⟨tokens : ℝ, last : ℝ⟩
  Decision ≜ admit ⊕ deny
  Request  ≜ ⟨cost : ℕ⁺, at : ℝ⟩
  Result   ≜ ⟨decision : Decision, bucket : Bucket⟩
}

⟦Γ:Inference⟧{
  Δt ≜ now - b.last
  ⊢ b'.tokens ≡ min(cap, b.tokens + rate · Δt)              [refill-rule]

  b'.tokens ≥ r.cost
  ⊢ admit ∧ b''.tokens ≡ b'.tokens - r.cost                 [admit-rule]

  b'.tokens < r.cost
  ⊢ deny ∧ b''.tokens ≡ b'.tokens                           [deny-rule]
}

⟦Λ:Core⟧{
  refill ≜ λ(b, now).
    let Δt = now - b.last in
    ⟨tokens : min(cap, b.tokens + rate · Δt), last : now⟩

  apply ≜ λ(b, r).
    let b' = refill(b, r.at) in
    b'.tokens ≥ r.cost
      → ⟨decision : admit, bucket : ⟨tokens : b'.tokens - r.cost, last : r.at⟩⟩
      | ⟨decision : deny,  bucket : b'⟩
}

⟦Χ:Errors⟧{
  ε_neg  ≜ ⟨r.cost ≤ 0, reject⟩
  ε_past ≜ ⟨r.at  < b.last, reject⟩
  ε_burst ≜ ⟨r.cost > cap, reject⟩
}

⟦Ε⟧⟨
  δ ≜ 0.74
  |𝔅| ≜ 5/5
  φ ≜ 92
  ⊢ ∀ t : tokens(t) ≤ cap
  ⊢ ∀ window W : admitted(W) ≤ cap + rate · |W|
  ⊢ apply ∘ apply ≡ apply ∘ refill ∘ apply
  τ ≜ ◊⁺⁺
⟩
```

---

### Document 3

```
𝔸1.0.test_003@2026-06-09
γ ≔ protocol.auction.sealed.second_price
ρ ≔ ⟨commit, reveal, settle, slash⟩
⊢ truthful ∧ revealable ∧ slash-safe

⟦Ω:Foundation⟧{
  N ∈ ℕ⁺                                       ;; number of bidders
  T_c, T_r, T_s ∈ ℝ                            ;; deadlines
  T_c < T_r < T_s
  ∀ i ∈ Fin N : bid_i ∈ ℝ⁺ ∧ nonce_i ∈ 𝔹²⁵⁶
  deposit ∈ ℝ⁺                                 ;; required collateral
  valuation_i ∈ ℝ⁺                             ;; private; not on-chain
}

⟦Σ:Types⟧{
  Commit_i ≜ ⟨c : Hash, dep : ℝ⁺⟩
  Reveal_i ≜ ⟨b : ℝ⁺, n : 𝔹²⁵⁶⟩
  Outcome  ≜ ⟨winner : (Fin N) ⊕ ∅, price : ℝ ⊕ ∅⟩
  Hash     ≜ 𝔹²⁵⁶
}

⟦Γ:Inference⟧{
  c_i ≡ SHA256(bid_i ⊕ nonce_i)
  ─────────────────────────────────────────── [commit-valid]
  ⊢ valid(Commit_i)

  reveal_i at t  ∧  T_c < t ≤ T_r  ∧  SHA256(b_i ⊕ n_i) ≡ c_i
  ──────────────────────────────────────────────────────────── [reveal-valid]
  ⊢ revealed(i)

  ¬ revealed(i)  at  T_r
  ─────────────────────────── [no-show]
  ⊢ slash(i, dep_i)

  R     ≜ { i | revealed(i) }
  i*    ≜ argmax_{i ∈ R} b_i
  price ≜ max_{j ∈ R ∖ {i*}} b_j
  ──────────────────────────────────── [settle]
  ⊢ Outcome ≡ ⟨winner : i*, price : price⟩
}

⟦Λ:Core⟧{
  commit ≜ λ(i, b, n). ⟨c : SHA256(b ⊕ n), dep : deposit⟩

  reveal ≜ λ(i, b, n, t).
    T_c < t ≤ T_r ∧ SHA256(b ⊕ n) ≡ c_i  →  ok
    | ⊥

  settle ≜ λ R.
    |R| ≡ 0 → ⟨∅, ∅⟩
    | |R| ≡ 1 → ⟨head(R), 0⟩
    | let s = sort(R, key ↦ b, desc) in
        ⟨head(s), b_(s[1])⟩
}

⟦Χ:Errors⟧{
  ε_early ≜ ⟨t ≤ T_c,                 reject⟩
  ε_late  ≜ ⟨t > T_r,                 reject ∧ slash⟩
  ε_hash  ≜ ⟨SHA256(b ⊕ n) ≢ c,       reject ∧ slash⟩
  ε_dup   ≜ ⟨reveal_i twice,          reject second⟩
}

⟦Ε⟧⟨
  δ ≜ 0.79
  |𝔅| ≜ 5/5
  φ ≜ 96
  ⊢ ∀ i : dominant_strategy(i) ⇔ b_i ≡ valuation_i
  ⊢ price ≤ max(b)  ∧  price ≥ second_max(b)
  ⊢ ∀ i ∉ R : transfer_i ≡ -dep_i
  ⊢ ∀ i : revealed(i) ⇒ transfer_i ≥ 0
  τ ≜ ◊⁺⁺
⟩
```

## === END TEST PROMPT ===

---

# 🛑 STOP — DO NOT SHOW BELOW THIS LINE TO THE MODEL UNDER TEST 🛑

The remainder of this file is the answer key, decision points 𝒟, author intent ℐ, and
scoring rubric. Keep it private from the model.

---

## Answer Key

### Document 1: Binary Search

**ℐ (author intent).** The document specifies the **binary search** algorithm on a sorted
integer array, with a correctness invariant tying the result to membership, the standard
divide-and-conquer recurrence, and an O(log n) complexity claim.

**Decision points 𝒟₁.**

| # | Decision point | Correct resolution |
|---|---|---|
| d₁.₁ | What algorithm is this? | **Binary search** (must be named or unambiguously described) |
| d₁.₂ | Domain | Sorted (non-decreasing) array of integers |
| d₁.₃ | Return value | Index of the target if found, "nothing" / null / `∅` if absent |
| d₁.₄ | Mechanism | Repeatedly compares to the midpoint of `[lo, hi)`; recurses into left or right half |
| d₁.₅ | Termination | `lo ≥ hi` is the base case → returns nothing |
| d₁.₆ | Complexity | O(log n) — explicitly stated in evidence block |
| d₁.₇ | Precondition | Sortedness of `A` is required for correctness |
| d₁.₈ | Tier | Platinum (◊⁺⁺) — implies high symbolic density |

**Notes for scoring.** d₁.₁ is the single most important point: a model that does *not*
identify this as binary search has missed the gestalt. Models that say "search algorithm" or
"divide-and-conquer search" without naming it should be marked partial credit.

---

### Document 2: Token Bucket Rate Limiter

**ℐ (author intent).** The document specifies a **token bucket rate limiter**: a bucket
with capacity `cap` accumulates tokens at rate `rate` per unit time (capped at `cap`), and
incoming requests of cost `r.cost` are admitted iff the bucket has at least that many
tokens at the moment of arrival; otherwise denied.

**Decision points 𝒟₂.**

| # | Decision point | Correct resolution |
|---|---|---|
| d₂.₁ | What is this? | **Token bucket** rate limiter (or "leaky bucket" — partial credit, as the algorithms differ slightly) |
| d₂.₂ | State | Current token count + timestamp of last update |
| d₂.₃ | Refill | Tokens added proportional to elapsed time since last update, capped at `cap` |
| d₂.₄ | Admit condition | Bucket has ≥ `r.cost` tokens after refill |
| d₂.₅ | Effect of admit | Bucket loses `r.cost` tokens; timestamp advances |
| d₂.₆ | Effect of deny | Bucket retains refilled state; no tokens consumed |
| d₂.₇ | Error: r.cost ≤ 0 | Negative or zero cost is rejected |
| d₂.₈ | Error: time goes backward | A request "from the past" is rejected |
| d₂.₉ | Error: cost > cap | A request larger than capacity is rejected (impossible to satisfy) |
| d₂.₁₀ | Long-run bound | Over window `W`, admitted load ≤ `cap + rate · |W|` (the burst-plus-sustained bound) |
| d₂.₁₁ | Invariant | Tokens never exceed `cap` |

**Notes for scoring.** The hard part is d₂.₁₀, which requires reading the evidence-block
inequality and recognizing it as the rate-limiter's defining throughput bound. Models often
get the mechanism but miss this claim.

---

### Document 3: Sealed-Bid Second-Price (Vickrey) Auction with Commit-Reveal and Slashing

**ℐ (author intent).** The document specifies a **two-phase sealed-bid second-price
auction** (a Vickrey auction) implemented via **commit-reveal** with **slashing** for
non-revealers. Bidders first commit a hash of (bid, nonce) plus a deposit; then within the
reveal window they open their commitment; the highest revealed bid wins but pays the
second-highest. Bidders who fail to reveal lose their deposit. The evidence block claims
truthful bidding is the dominant strategy (the central Vickrey result).

**Decision points 𝒟₃.**

| # | Decision point | Correct resolution |
|---|---|---|
| d₃.₁ | What is this? | **Sealed-bid second-price auction** (Vickrey auction), implemented via commit-reveal |
| d₃.₂ | Two-phase structure | Phase 1: commit hash + deposit. Phase 2: reveal (bid, nonce) |
| d₃.₃ | Commit mechanism | `SHA256(bid ⊕ nonce)` — a cryptographic commitment |
| d₃.₄ | Reveal validity | Must occur in `(T_c, T_r]` AND the hash must match the prior commit |
| d₃.₅ | Failure to reveal | Deposit is slashed (forfeited) |
| d₃.₆ | Winner | The highest *revealed* bid |
| d₃.₇ | Price paid by winner | The *second-highest* revealed bid (NOT their own) — this is the critical Vickrey property |
| d₃.₈ | Truthfulness claim | Bidding one's true valuation is the dominant strategy |
| d₃.₉ | Hash mismatch on reveal | Treated as a failure to reveal — reject + slash |
| d₃.₁₀ | Empty / single reveal | Empty → no winner. Single → winner pays 0 |
| d₃.₁₁ | Slash-safety | Honest revealers never lose money beyond their bid; only non-revealers and cheaters get slashed |

**Notes for scoring.** d₃.₁ (Vickrey by name) and d₃.₇ (the second-price rule) are the
load-bearing decision points. A model that identifies "auction" but not "second-price"
should get partial credit. A model that says winner pays their own bid is wrong on the
single most distinctive property of this protocol.

The commit-reveal pattern (d₃.₂, d₃.₃) is well-known from blockchain protocols — models that
have seen any commit-reveal scheme in training should recognize this.

---

## Scoring Rubric

For each model, score each decision point on a three-point scale:

- **2 points** — fully correct (matches author intent ℐ)
- **1 point** — partially correct (right direction, missing or wrong on a detail)
- **0 points** — wrong, missing, or marks itself as unable to interpret

| Document | Max points | Critical decision points (weighted 2×) |
|---|---|---|
| Doc 1 (binary search) | 16 | d₁.₁ (identification), d₁.₄ (mechanism) |
| Doc 2 (token bucket) | 22 | d₂.₁ (identification), d₂.₁₀ (long-run bound) |
| Doc 3 (Vickrey auction) | 22 | d₃.₁ (identification), d₃.₇ (second-price rule) |
| **Total (with critical 2× weight)** | **68** | |

**Tier interpretation (for the model's comprehension, not the document):**

| Score | Comprehension tier |
|---|---|
| ≥ 58 (85%) | **Strong** — model handles AISP with high fidelity, including non-obvious claims |
| 48–57 (70–85%) | **Good** — model recovers structure and mechanism, may miss subtle properties |
| 34–47 (50–70%) | **Partial** — model gets surface structure but misses identification or key properties |
| < 34 (< 50%) | **Weak** — model treats this as opaque notation |

---

## Recording Sheet

Use the table below to record results across models. Suggested columns: model name, version,
date tested, score per document, total, comprehension tier, notes on what was missed.

| Model | Version | Date | Doc 1 | Doc 2 | Doc 3 | Total | Tier | Notes |
|---|---|---|---|---|---|---|---|---|
|   |   |   |   |   |   |   |   |   |
|   |   |   |   |   |   |   |   |   |
|   |   |   |   |   |   |   |   |   |
|   |   |   |   |   |   |   |   |   |
|   |   |   |   |   |   |   |   |   |

---

## What This Test Measures (and Doesn't)

**Measures.** Per Definition 5.2 of the paper, this is `Ambig_sem(D ; M, n=1, 𝒟, ℐ)` for each
document — semantic ambiguity of an AISP document under a single completion from model `M`,
with 𝒟 and ℐ as specified above. Lower scores correspond to higher ambiguity (the model
disagrees more often with ℐ).

**Does not measure.** Multi-shot variance (run each model `n ≥ 5` times to get a real
estimate), prose-vs-AISP comparison (you'd need matched prose documents and the same
decision points), cross-judge agreement (you'd need an independent grader for each model's
output). These are the upgrades a companion empirics paper would add. This test is the
minimum-viable instance.

**Note on Document 3.** The Vickrey auction is the most discriminating document because
recognizing "second-price" requires reading the `argmax / max ∖ {i*}` pattern carefully —
exactly the kind of subtle semantic claim that distinguishes syntactic parsing from
semantic comprehension. A model that gets Documents 1 and 2 but misses the second-price
rule in Document 3 is showing the syntactic/semantic split in action.
