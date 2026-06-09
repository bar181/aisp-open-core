# Appendix C — Blind Cross-Vendor Comprehension Test

*Pilot exhibit · 2026-06-09*

This exhibit documents a pilot test of the contribution C1 from §1.4 of the main paper:
the claim that frontier large language models parse AISP natively, without fine-tuning,
glossary, or in-context examples. It is *not* a measurement of `Ambig_sem` per
Definition 5.2 — see §C.7 for what it does and does not establish.

---

## C.1 Purpose

The test asks a single question: **can frontier LLMs, given an AISP document and no other
context, recover its semantic content into prose?** If the answer is yes across vendors,
the precondition for every other AISP claim is met. If the answer is no, the paper's
central thesis fails empirically and nothing downstream needs to be measured.

The test does not address the AISP-vs-prose semantic-ambiguity hypothesis (Hypothesis 5.3
in §5.2). That requires matched prose baselines and adversarial documents, which are
deferred to a companion empirics paper.

---

## C.2 Methodology

**Design.** Three AISP documents specifying textbook computational concepts (binary search,
token bucket rate limiter, sealed-bid second-price auction with commit-reveal). Each
document was authored to be valid AISP 5.1 (Platinum tier, δ ≥ 0.74, all required blocks
present), self-contained, and recoverable from symbols alone without prior AISP exposure.

**Prompt.** A single prompt instructing the model to produce a plain-English translation
covering six aspects of each document: identification, entities and types, rules,
functions, errors, and evidence. No glossary, no AISP examples, no system prompt
modification. The full prompt is reproduced in the test file (`aisp_blind_test.md`,
between `BEGIN TEST PROMPT` and `END TEST PROMPT`).

**Decision points.** Each document has an enumerated set 𝒟(D) of decision points the
author considered necessary for a correct reading. Critical decision points (the
algorithm's identification and its defining property) are weighted 2×. Author intent
ℐ(D) is supplied alongside each decision point. Total weighted maximum across all three
documents: 72.

**Models tested.**

| Model | Vendor | Condition |
|---|---|---|
| Gemini | Google | Blind (no answer key, no system prompt) |
| GPT-5.5 | OpenAI | Blind (no answer key, no system prompt) |
| Claude Opus 4.8 | Anthropic | Contaminated (model had access to answer key in context) |

The Opus 4.8 run is a contaminated upper bound used as a methodological control, not as a
blind data point. Its primary value is qualitative — Opus 4.8 surfaced design flaws in
the test (the γ-tag leak and ceiling effects) that informed the analysis below.

**Scoring.** Each decision point scored 0 (wrong/absent), 1 (partially correct), or 2
(fully correct), against author intent ℐ. Critical decision points weighted 2×. Scoring
performed by the author with the rubric in `aisp_blind_test.md` §"Scoring Rubric."

---

## C.3 Test Documents (Summary)

Full AISP source for each document is in the test file. Compact characterization here.

| Document | Concept | Density δ | Tier | Decision points |
|---|---|---:|---|---:|
| D₁ | Binary search on a sorted integer array | 0.78 | ◊⁺⁺ | 8 (2 critical) |
| D₂ | Token bucket rate limiter | 0.74 | ◊⁺⁺ | 11 (2 critical) |
| D₃ | Sealed-bid second-price (Vickrey) auction with commit-reveal and slashing | 0.79 | ◊⁺⁺ | 11 (2 critical) |

Critical decision points:

- **D₁:** identification (`d₁.₁`) and mechanism (`d₁.₄`).
- **D₂:** identification (`d₂.₁`) and long-run throughput bound `admitted(W) ≤ cap + rate · |W|` (`d₂.₁₀`).
- **D₃:** identification as Vickrey/second-price (`d₃.₁`) and the second-price pricing rule (`d₃.₇`).

---

## C.4 Results

### C.4.1 Per-document scores (weighted, max 72)

| Model | D₁ (max 20) | D₂ (max 26) | D₃ (max 26) | Total | Percent | Tier |
|---|---:|---:|---:|---:|---:|---|
| Gemini | 20 | 26 | 26 | **72** | 100% | Strong |
| GPT-5.5 | 20 | 26 | 26 | **72** | 100% | Strong |
| Opus 4.8 (contaminated) | 20 | 26 | 26 | **72** | 100% | Strong (upper bound) |

### C.4.2 Critical decision points (the six weighted 2× points)

| Critical point | Gemini | GPT-5.5 | Opus 4.8 |
|---|:-:|:-:|:-:|
| d₁.₁ — identify as binary search | ✓ | ✓ | ✓ |
| d₁.₄ — midpoint comparison + halve | ✓ | ✓ | ✓ |
| d₂.₁ — identify as token bucket | ✓ | ✓ | ✓ |
| d₂.₁₀ — long-run bound `cap + rate · \|W\|` | ✓ | ✓ | ✓ |
| d₃.₁ — identify as Vickrey/second-price | ✓ | ✓ | ✓ |
| d₃.₇ — winner pays second-highest bid | ✓ | ✓ | ✓ |

Six of six critical points recovered by every model in every condition.

### C.4.3 Subtle non-critical points worth highlighting

These were not flagged critical in the rubric but were the points most likely to be missed
by a model engaged in surface pattern-matching rather than comprehension:

| Point | Description | Gemini | GPT-5.5 |
|---|---|:-:|:-:|
| d₂.₁₀ (semantic) | Recovered the inequality *and* interpreted "burst + sustained throughput" decomposition | Named "Throughput Bound" | "burst up to capacity plus sustained throughput at refill rate" |
| d₃.₈ | Stated truthful bidding is the dominant strategy, named it the defining Vickrey property | ✓ | ✓ |
| d₃.₁₁ | Recovered the slash-safety claim (honest revealers protected from negative transfers) | "protected from arbitrary financial penalty" | "honest revealers are slash-safe" |

The d₂.₁₀ result is the strongest single signal in the run. The symbolic statement is
`admitted(W) ≤ cap + rate · |W|`. The textbook interpretation is "burst capacity plus
sustained refill throughput." Both blind models produced the textbook interpretation,
unprompted, rather than reciting the inequality. This is interpretive translation, not
symbol-to-word substitution.

---

## C.5 Qualitative Findings

### Finding 1: Native readability is supported across vendors

Two blind frontier models from two different vendors (Google, OpenAI) recovered every
enumerated decision point on three AISP documents without prior AISP exposure. This is the
foundational result the C1 thesis requires. Replication across vendors, while at n=1 per
vendor, eliminates the alternative explanation that AISP comprehension is an artifact of
one model family's training data.

### Finding 2: Models translate by passing through semantics, not by transliteration

The single most interesting observation in the test data is that both blind models produced
translations that *interpreted* symbolic claims rather than restating them. The d₂.₁₀ case
is the clearest:

- AISP source: `admitted(W) ≤ cap + rate · |W|`
- Gemini: *"the total admitted cost is strictly bounded by the bucket's capacity plus the tokens generated during that window... Throughput Bound"*
- GPT-5.5: *"admitted traffic is bounded by `cap + rate × |W|`... this is a safe rate limiter that allows controlled bursts but prevents unlimited traffic"*

Neither model said only what the inequality says. Both said what the inequality *means*,
in the conventional engineering register. This is the property §6 of the main paper
("The Translation Gradient") claims: that rendering AISP into prose is a directional
operation that preserves the upstream meaning while adapting the surface for the audience.
The test data is the first empirical evidence the paper has that this happens.

### Finding 3: The test cannot discriminate at the ceiling

All three models scored 72/72. The test does not produce a useful ranking of frontier
models on AISP comprehension; it produces evidence that they reach parity at the ceiling
on textbook concepts. A discriminating test would require either (a) novel non-textbook
documents that defeat pattern-matching on known structures, or (b) adversarial documents
designed to invert specific decision points (e.g., a Dutch auction encoded similarly to
a Vickrey auction to test whether models read the symbols or recognize the genre).

### Finding 4: A design flaw was identified during scoring

The contaminated Opus 4.8 run identified that the `γ ≔ ...` context-tag field at the top
of each document discloses the concept name in plain text (`protocol.auction.sealed.second_price`
in D₃, for example). This is a leak that inflates the identification decision points
(`d₁.₁`, `d₂.₁`, `d₃.₁`). The non-identification points — including all six critical
points on the harder documents and all three "subtle" points in §C.4.3 — are not
recoverable from the γ tag and are unaffected. The leak is acknowledged in §C.7 and the
test design will strip γ in any future run.

---

## C.6 Discussion

The pilot supports the C1 thesis: frontier-class large language models from at least three
vendors (Anthropic, Google, OpenAI) parse AISP at the level the paper requires, without
fine-tuning, glossary access, or in-context examples. This is the empirical precondition
for every other AISP claim, and it now has cross-vendor support at n=1 per vendor.

The pilot does *not* support the stronger Hypothesis 5.3 from §5.2 of the main paper —
that `Ambig_sem(D_AISP) ≪ Ambig_sem(D_prose)` on matched documents. That measurement
requires (i) matched prose versions of the same documents under the same 𝒟 and ℐ, and
(ii) at least one novel document that defeats recall. Both are part of the v1.1 empirical
companion suite and are deferred to that work.

The qualitative finding in §C.5 Finding 2 — that models translate by passing through
semantics rather than by transliteration — strengthens the §6 "Translation Gradient"
framing of the main paper from an aesthetic claim to one with at least pilot empirical
support. This is the finding most worth highlighting in the paper itself; the score data
is secondary.

---

## C.7 Limitations

1. **n = 1 per vendor.** No variance estimate. Production-grade measurement requires n ≥ 5 per model.
2. **Ceiling effect.** All three documents scored at maximum. The test does not discriminate among frontier models.
3. **γ-tag leak.** The genre tag in each document discloses the concept name in plain text. Identification decision points are inflated; non-identification points are not affected.
4. **No prose baseline.** Without matched prose versions of the documents under the same 𝒟 and ℐ, the absolute scores cannot be compared to a control condition. The AISP-vs-prose delta — the quantity Hypothesis 5.3 makes claims about — is not measured here.
5. **No novel-document control.** All three documents are textbook concepts. Models could in principle score high by recognizing the structure from training data and projecting decision points from memory rather than from the symbolic content. The d₂.₁₀ and d₃.₁₁ results in §C.5 argue against pure pattern-matching but do not rule it out.
6. **Single grader.** Scoring was performed by the author. Inter-grader agreement is not established. The 2/1/0 scoring scale and partial-credit rules in `aisp_blind_test.md` are designed to minimize judgment variance but do not eliminate it.
7. **Single shot per model.** No control for sampling variance within a model. Each model produced one translation per document.
8. **Temperature not normalized.** Default vendor temperature settings were used for each model. Direct comparability is not guaranteed.

Each of these is addressable in the v1.1 companion suite (see §C.8).

---

## C.8 What this exhibit licenses, and what it does not

**Licenses.** The paper to claim, in the body of the contribution C1 discussion:

> A pilot blind comprehension test on three textbook AISP documents was performed against
> two frontier-class models from independent vendors (Google Gemini, OpenAI GPT-5.5),
> with a third model used as a contaminated upper-bound control. Both blind models
> recovered every enumerated decision point on every document, including subtle
> non-critical claims (the throughput bound `cap + rate · |W|`, the slash-safety property
> for honest revealers, the dominant-strategy result for second-price bidding). Both
> blind models produced translations that interpreted symbolic claims into their
> conventional engineering register rather than restating them. We interpret this as
> pilot-level support for the C1 thesis of native cross-vendor readability and for the
> §6 Translation Gradient framing, while noting the limitations enumerated in
> Appendix C.7.

**Does not license.** Any quantitative claim about ambiguity reduction relative to prose
(Hypothesis 5.3), any ranking of model AISP comprehension, any claim about long-pipeline
behavior, any benchmark performance claim. These require the companion empirics paper.

---

## C.9 Reproducibility

The test file, including the prompt, the three documents, the decision-point sets 𝒟,
the author intent ℐ for each, and the scoring rubric, is published at
[`github.com/bar181/aisp-open-core`](https://github.com/bar181/aisp-open-core) under the
MIT licence as `evidence/blind-comprehension-pilot/`. Any reader can re-run the test
against any frontier model and score the response using the same rubric.

The translations produced by Gemini, GPT-5.5, and Opus 4.8 in this pilot are archived
verbatim in the same directory.

---

## C.10 Next Test in the Pipeline

The v1.1 empirical suite, in design, will add four conditions to the present test:

| Condition | Purpose | Status |
|---|---|---|
| Blind AISP (current) | Establish native readability | Pilot complete (this exhibit) |
| AISP with stripped γ/ρ | Eliminate context-tag leak | Designed |
| Matched prose | Establish baseline for Hypothesis 5.3 | Designed |
| Pseudocode | Compare AISP to a familiar symbolic format | Designed |
| Novel composed document | Defeat textbook recall | Designed |
| Adversarial document | Test mechanism-inversion failures | Designed |

The 2 × 2 design (AISP/prose × textbook/novel) with the failure-mode taxonomy in
`aisp_blind_test.md` §"Failure Mode Taxonomy" is the experimental shape that will produce
a publishable measurement of Hypothesis 5.3. The pilot in this exhibit is the
zero-th run that confirmed the experimental design is worth scaling.

---

*End of Appendix C.*
