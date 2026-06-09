### Translation 1

**Identification**
This document specifies a **Binary Search algorithm**. It is designed to find a specific target integer within a sorted array, repeatedly halving the search space.

**Entities and types**

* **$A$**: An array (vector) of $n$ integers.
* **$k$**: The target integer being searched for.
* **`Result`**: An optional index (represented as `Maybe`), returning the position if found, or empty ($\emptyset$) if not.
* **`Range`**: A bounded segment defined by a lower bound (`lo`) and upper bound (`hi`) such that $lo \le hi \le n$.

**Rules**
The core logical constraint is that the array $A$ must be **sorted** in non-decreasing order. The inference rules dictate the search logic:

* **Left-branch**: If $k$ is less than the midpoint value $A[m]$, $k$ cannot exist in the right half of the range.
* **Right-branch**: If $k$ is greater than the midpoint value, $k$ cannot exist in the left half.
* **Hit**: If $k$ exactly matches the midpoint value, the search succeeds.
* **Miss**: If the lower bound meets or exceeds the upper bound ($lo \ge hi$), the search space is exhausted, meaning the item is absent.

**Functions**

* **`mid`**: Calculates the arithmetic center of `lo` and `hi`.
* **`find`**: The core recursive function. It first checks the base case (if $lo \ge hi$, return empty). Otherwise, it calculates the midpoint $m$. If the target $k$ matches $A[m]$, it returns the index $m$. If $A[m] < k$, it recurses into the right half (`m+1` to `hi`). Otherwise, it recurses into the left half (`lo` to `m`).
* **`run`**: The entry-point function that executes `find` over the entire array bounds, from index $0$ to the length of $A$.

**Errors**
There is no explicit errors block defined in this document.

**Evidence**
The document claims two major properties:

1. **Time Complexity:** The algorithm operates in logarithmic time, $O(\log n)$.
2. **Correctness:** Assuming the input array $A$ is strictly sorted, the `run` function perfectly satisfies the specification (it will return an index if and only if the element exists at that index).

---

### Translation 2

**Identification**
This document specifies a **Token Bucket Rate Limiter**. It is a network or software protocol used to control the flow of requests, ensuring usage stays within defined limits by requiring "tokens" to process actions.

**Entities and types**

* **`Bucket`**: The core state, tracking the current number of `tokens` (a real number) and the `last` time it was updated.
* **`Request`**: An incoming action, carrying a `cost` (a positive integer) and an arrival timestamp (`at`).
* **`Decision`**: A binary outcome to either `admit` or `deny` the request.
* **`Result`**: A paired output containing the `Decision` and the newly updated `Bucket` state.

**Rules**
The system is bounded by a maximum capacity (`cap`) and refills at a steady `rate` over time.

* **Refill rule**: As time passes ($\Delta t$), tokens are added proportionally (`rate` $\times$ $\Delta t$), but the total can never exceed `cap`.
* **Admit rule**: If the bucket has enough tokens to cover the request's `cost`, the request is admitted, and the tokens are deducted from the bucket.
* **Deny rule**: If the tokens fall short of the `cost`, the request is denied, and the token count remains unchanged.

**Functions**

* **`refill`**: Calculates the time difference between "now" and the bucket's last update. It adds the appropriately generated tokens (capped at maximum capacity) and advances the bucket's timestamp.
* **`apply`**: Evaluates a request against a bucket. It first triggers `refill` to bring the bucket up to date. It then checks if the refilled tokens can cover the request cost, returning the corresponding admit/deny `Decision` and the resulting bucket state.

**Errors**
The protocol explicitly rejects requests under three failure modes:

* **`ε_neg`**: The request cost is zero or negative.
* **`ε_past`**: The request arrives "from the past" (timestamp is older than the bucket's last update).
* **`ε_burst`**: The request cost is strictly greater than the bucket's total capacity (impossible to ever satisfy).

**Evidence**
The document claims:

1. The token count will mathematically never exceed `cap` at any time $t$.
2. **Throughput Bound:** Over any given time window $W$, the total admitted cost is strictly bounded by the bucket's capacity plus the tokens generated during that window: $cap + rate \times |W|$.
3. The `apply` function maintains consistent state composition even when interrupted by arbitrary refills.

---

### Translation 3

**Identification**
This document outlines a **Sealed-Bid Second-Price Auction**, also known as a Vickrey Auction, implemented on-chain using a cryptographic **Commit-Reveal scheme**.

**Entities and types**

* **`Commit_i`**: A bidder's initial hidden submission, containing a cryptographic hash (`c`) and a monetary `deposit`.
* **`Reveal_i`**: The subsequent unmasking of the bid, containing the raw bid value (`b`) and a cryptographic nonce (`n`).
* **`Outcome`**: The final result, indicating the winning bidder and the clearing price.

**Rules**
The protocol relies on strict chronological deadlines: commit phase ($T_c$), reveal phase ($T_r$), and settlement ($T_s$).

* **Commit Valid**: A valid commit must securely hash the bid and the nonce using SHA-256.
* **Reveal Valid**: A reveal is only accepted if it happens during the reveal window (after $T_c$, before $T_r$) and the provided bid/nonce perfectly recreate the original submitted hash.
* **No-show**: If a bidder commits but fails to reveal their bid before the deadline, they are penalized via "slashing" (losing their deposit).
* **Settle**: The winner is the participant with the highest revealed bid. The price they actually pay is the **second-highest** bid among the valid reveals.

**Functions**

* **`commit`**: Hashes a participant's bid and nonce, binding them to a required deposit.
* **`reveal`**: Verifies that the current time falls within the legal reveal window and validates the hash against the prior commitment.
* **`settle`**: Processes all valid reveals ($R$). If no one reveals, the auction yields nothing. If exactly one person reveals, they win and pay a price of $0$. If multiple reveal, it sorts the bids in descending order, declares the highest bidder the winner, and charges them the value of the second bid in the sorted list.

**Errors**

* **`ε_early`**: Attempting to reveal before the commit deadline has passed (rejected).
* **`ε_late`**: Attempting to reveal after the reveal deadline (rejected and deposit slashed).
* **`ε_hash`**: The revealed bid/nonce does not match the committed hash (rejected and deposit slashed).
* **`ε_dup`**: Attempting to reveal the same bid twice (the second attempt is rejected).

**Evidence**
The document formalizes several critical game-theoretic properties:

1. **Truthfulness:** The dominant strategy for any participant is to bid their exact, true valuation (the defining property of a Vickrey auction).
2. The final price will always sit between the highest and second-highest submitted bids.
3. Participants who fail to reveal strictly lose their deposit value (transfer is $-dep_i$).
4. **Slash-safety:** Any participant who honestly follows the protocol and reveals their bid will never end up with a negative net transfer; they are protected from arbitrary financial penalty.
