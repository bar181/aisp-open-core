## What this is, and one honest caveat up front

This file is a blind comprehension test for an AISP specification: a dense symbolic notation that encodes algorithms and protocols as typed foundations, inference rules, and core functions. The test measures *semantic ambiguity*: whether a model recovers the author's intended meaning from the symbols alone, scored against a fixed set of decision points.

The analogy I'd use with students: it's like handing someone an unfamiliar sheet-music notation and checking whether they can hum the tune, not just name the key signature. There are two distinct skills hiding here. Parsing the notation is one. Recognizing the song is another. The test wants to separate them. As I'll argue below, the header partly gives the song title away, which is the central design flaw.

The caveat: I've read the whole file, including the answer key. So my run is contaminated and is **not** a valid blind data point for your benchmark. I'm completing it as a worked example and as the basis for review. If you want a clean Claude Opus 4.8 number, paste only the `BEGIN`/`END` section into a fresh conversation with no key present. That said, my identifications below come from the symbols themselves, not the key: these three concepts are unmistakable from their structure regardless of the answer sheet.

---

## The three translations (model-under-test output)

**Translation 1.** This is **binary search** on a sorted integer array. The foundation declares `A : Vec n ℤ`, defines `sorted` as non-decreasing order, and pins correctness with `spec`: `find(A,k)` returns `Some i` exactly when `A[i] = k`. Types: `Result = Maybe (Fin n)`, an optional in-bounds index; `Range` is a dependent pair carrying a proof that `lo ≤ hi ≤ n`. The inference block states the four standard facts: if `k < A[m]` the target can't be in the right region, if `k > A[m]` it can't be in the left region, equality is a hit at `m`, and `lo ≥ hi` is a miss. The core function `find` is a fixpoint over `(A, k, lo, hi)`: base case returns nothing when `lo ≥ hi`; otherwise take `m = ⌊(lo+hi)/2⌋`, return `Some m` on a match, recurse into `(m+1, hi)` when `A[m] < k`, else recurse into `(lo, m)`. `run` seeds the half-open interval `[0, |A|)`. No errors block. Evidence claims `O(log n)` time and correctness conditional on sortedness, at the highest tier `◊⁺⁺`. The metadata symbols `δ = 0.78`, `|𝔅| = 4/4`, `φ = 95` are unexplained; I read them as density, block-completeness, and a confidence score, but that's inference, not knowledge.

**Translation 2.** This is a **token bucket rate limiter**. Foundation: integer capacity `cap`, positive real `rate` (tokens per unit time), an epoch `t₀`, and the invariant that token count stays in `[0, cap]`. Types: `Bucket = ⟨tokens, last⟩` (count plus last-update timestamp), a binary `Decision = admit ⊕ deny`, a `Request = ⟨cost, at⟩`, and a `Result` pairing decision with the new bucket. Rules: the refill rule sets `tokens' = min(cap, tokens + rate·Δt)` over elapsed `Δt`; admit fires when refilled tokens `≥ cost` and subtracts the cost; deny leaves the refilled bucket untouched. The core `apply` refills first, then branches on whether the bucket covers the request cost. The errors block rejects three cases: non-positive cost, a timestamp earlier than the last update (time going backward), and a cost exceeding total capacity (永远 unsatisfiable). Evidence: tokens never exceed `cap`; over any window `W`, admitted load is bounded by `cap + rate·|W|` (the burst-plus-sustained throughput bound); and a composition identity, `apply ∘ apply ≡ apply ∘ refill ∘ apply`. Tier `◊⁺⁺`.

**Translation 3.** This is a **sealed-bid second-price (Vickrey) auction** built on a **commit-reveal** scheme with **slashing** for non-revealers. Foundation: `N` bidders, three ordered deadlines `T_c < T_r < T_s` (commit, reveal, settle), per-bidder bid and 256-bit nonce, a required `deposit`, and a private off-chain `valuation`. Types: `Commit = ⟨hash, deposit⟩`, `Reveal = ⟨bid, nonce⟩`, an `Outcome` carrying an optional winner index and optional price, and `Hash = 𝔹²⁵⁶`. Rules: a commit is valid when its hash equals `SHA256(bid ⊕ nonce)`; a reveal counts only inside `(T_c, T_r]` and only if the hash matches the commit; failure to reveal by `T_r` slashes the deposit. Settlement takes `R`, the revealed set, picks the winner `i*` as the argmax bid, and sets the **price to the second-highest revealed bid** (`max` over `R` excluding the winner). This second-price rule is the defining property. The core `settle` handles the corner cases: empty `R` yields no winner, a single revealer pays zero, otherwise sort descending and charge the runner-up's bid. Errors: reveal before `T_c` is rejected; reveal after `T_r` is rejected and slashed; a hash mismatch is rejected and slashed; a duplicate reveal rejects the second. Evidence makes the central Vickrey claim: truthful bidding (`bid = valuation`) is the dominant strategy. It also bounds the price between the second-max and the max, sets non-revealers' transfer to `-deposit`, and guarantees revealers a non-negative transfer (slash-safety). Tier `◊⁺⁺`.

---

## Self-score against the rubric (contaminated run)

A correct read hits every decision point, including the two hard ones the key flags: `d₂.₁₀` (the `cap + rate·|W|` bound) and `d₃.₇` (second-price, not own-bid).

| Model | Version | Date | Doc 1 | Doc 2 | Doc 3 | Total | Tier |
|---|---|---|---|---|---|---|---|
| Claude | Opus 4.8 | 2026-06-09 | 16/16 | 22/22 | 22/22 | 60/60 | Strong (contaminated) |

Treat this as an upper bound, not a measurement. I saw the key, and these are textbook concepts I'd recognize blind anyway, so the number tells you almost nothing about AISP-reading difficulty. The interesting data would come from models that *don't* already know commit-reveal.

One scoring bug worth fixing: your per-document maxima (16, 22, 22) sum to 60, but the grand total is listed as 68 "with critical 2× weight." That figure doesn't reconcile. The per-doc maxima are already just `2 × (number of decision points)` with no weighting baked in, and applying a 2× weight to the six critical points yields 72, not 68. Either drop the weighting and call it 60, or apply it consistently and relabel the maxima.

---

## Brutally honest review of the test design

Scored as a measurement instrument, 10 items, 1 to 10, lower is worse.

| # | Criterion | Score | Why |
|---|---|---|---|
| 1 | Construct validity | 4 | Conflates two abilities: reading the notation vs. recognizing a known concept. A model can score high by pattern-matching the algorithm and ignoring the symbols. |
| 2 | Confound control | 2 | The `γ` genre tags leak the answer in plain ASCII. `protocol.auction.sealed.second_price` literally names the property that `d₃.₁` and `d₃.₇` are supposed to test. So does `algorithm.search.ordered` and `protocol.flow.token_bucket`. |
| 3 | Difficulty grading | 3 | Billed as "graded difficulty," but all three are standard, heavily-documented concepts. The real gradient is length, not difficulty. None is novel enough to defeat recall. |
| 4 | Answer-key security | 5 | Key lives in the same file behind a manual `STOP` line. Prominent, but one careless paste or one naive harness leaks it. Split into two files. |
| 5 | Scoring objectivity | 4 | Free-text grading by a human with a 2/1/0 scale, no grader rubric, no inter-rater check. The math also doesn't reconcile (see above). |
| 6 | Statistical rigor | 3 | n=1, acknowledged. No temperature normalization across providers, who have different defaults. No variance estimate. |
| 7 | Baseline / control | 2 | No matched-prose version, so an absolute score is uninterpretable. 60/60 means nothing without knowing how the same model scores on prose-equivalent specs and on genuinely unnamed algorithms. This is the load-bearing gap. |
| 8 | Reproducibility | 7 | Run instructions are clear, the prompt is self-contained, the recording sheet is ready. Genuinely usable. |
| 9 | Decision-point coverage | 7 | The `𝒟` sets are thoughtful, the critical points are well chosen, and the partial-credit notes are sensible. |
| 10 | Honesty about limits | 9 | The "What this measures and doesn't" section is candid and correct about multi-shot variance, baselines, and cross-judge agreement. Best part of the document. |

**Overall: 4.5/10 as a benchmark, roughly 7/10 as a teaching artifact.** As a classroom demonstration of the syntactic-versus-semantic split, it works and it's vivid. As an instrument that produces a defensible number, it has two holes big enough to invalidate the result: the genre tags give away the answers, and there's no prose baseline to anchor the score.

### Fixes, in phases (minimal dependencies, run in order)

Phase 1, kill the leaks. Strip or hash the `γ` and `ρ` lines from the documents under test, or replace them with opaque IDs. Right now `d₃.₁` is answered by reading line 2. This is the single highest-value change and it costs nothing.

Phase 2, add a control. Write a matched-prose version of each spec with the same `𝒟` points, and add one genuinely synthetic algorithm with no textbook name (compose two known ideas into something unnamed). Now the score has meaning: AISP-vs-prose delta on known concepts, plus absolute comprehension on something the model can't have memorized.

Phase 3, make scoring mechanical. Convert each decision point into a closed checkable form (expected string, regex, or a short grader-model prompt with its own rubric). Fix the total arithmetic. Run n≥5 per model and report mean and spread, not a single number.

Phase 4, then scale to multiple models. Only worth doing once Phases 1 to 3 hold, otherwise you're averaging noise.

I can produce a completed copy of this file with the recording sheet filled, the genre tags stripped from the test documents, and the scoring totals corrected, if you want it as a deliverable to hand to students or co-authors. Want the Word or markdown version?
