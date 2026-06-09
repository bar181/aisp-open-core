## Translation 1

Document 1 describes **binary search**.

It defines a search algorithm for a sorted integer array. The goal is to find whether a target value `k` appears in array `A`, and if so, return its index. If target is not present, return an empty result.

`A` is a vector of `n` integers. The array must be sorted, meaning earlier positions cannot contain larger values than later positions. `Result` is a maybe-type: either `Some i`, where `i` is a valid index, or empty. `Range` represents the active search interval using `lo` and `hi`, with bounds inside array length.

Core rules are standard binary search rules. If target is less than middle value, target cannot be in right half. If target is greater than middle value, target cannot be in left half. If target equals middle value, search succeeds. If lower bound meets or passes upper bound, search interval is empty, so search fails.

`mid` computes midpoint as `(lo + hi) ÷ 2`. `find` is recursive. It checks whether search range is empty. If not, it compares target against midpoint value. Equal returns midpoint. Greater searches right half. Smaller searches left half. `run` starts search over full array from `0` to array length.

There is no explicit errors block. Missing target is handled by returning empty.

The evidence claims that runtime is `O(log n)`, sortedness implies correctness, and symbolic confidence is high. In plain English: for a sorted array, this algorithm terminates, returns correct result, and runs fast by cutting search space in half each step. 

---

## Translation 2

Document 2 describes a **token bucket rate limiter**.

It defines a protocol for controlling request flow. A bucket holds tokens. Tokens refill over time up to capacity. Each request has a cost. A request is admitted only when enough tokens are available. Otherwise, request is denied.

Main entities are `cap`, `rate`, `Bucket`, `Request`, `Decision`, and `Result`. `cap` is maximum token capacity. `rate` is token refill speed per unit time. A `Bucket` stores current token count and last update time. A `Request` has positive cost and arrival time. A `Decision` is either admit or deny. A `Result` contains both decision and updated bucket.

Core rules are refill, admit, and deny. Refill computes elapsed time since last bucket update, adds `rate × elapsed time`, and caps tokens at `cap`. Admit happens when refilled token count is at least request cost. Admit subtracts request cost. Deny happens when token count is too low. Deny keeps refilled token count unchanged.

`refill` updates bucket state to current request time. `apply` is key function. It first refills bucket at request arrival time. Then it checks whether bucket has enough tokens. If yes, return admit and subtract cost. If no, return deny and keep refilled bucket.

Errors are invalid requests. `ε_neg` rejects zero or negative request cost. `ε_past` rejects requests timestamped before bucket’s last update, because time cannot move backward. `ε_burst` rejects request cost greater than bucket capacity, because such request can never be satisfied.

Evidence claims tokens never exceed capacity. It also claims that over any time window `W`, admitted traffic is bounded by `cap + rate × |W|`. That means system allows one burst up to capacity plus sustained throughput at refill rate. Composition claim says applying requests and refill operations remains consistent. In plain English: this is a safe rate limiter that allows controlled bursts but prevents unlimited traffic. 

---

## Translation 3

Document 3 describes a **sealed-bid second-price auction**, also called a **Vickrey auction**, implemented with **commit-reveal** and **slashing**.

It defines an auction where bidders first commit hidden bids, then later reveal them. Highest revealed bid wins, but winner pays second-highest revealed bid. Deposits punish bidders who fail to reveal or reveal dishonestly.

Main entities are bidders, bids, nonces, deadlines, deposits, commitments, reveals, and outcomes. There are `N` bidders. Each bidder has positive bid and 256-bit nonce. There are three deadlines: commit deadline `T_c`, reveal deadline `T_r`, and settlement time `T_s`, with `T_c < T_r < T_s`. A commitment contains hash and deposit. A reveal contains bid and nonce. Outcome contains winner and price, or empty winner and empty price.

Core rules define commit validity, reveal validity, slashing, and settlement. Commit is valid when commitment hash equals `SHA256(bid_i ⊕ nonce_i)`. Reveal is valid when bidder reveals after commit period, before reveal deadline, and hash matches original commitment. If bidder does not reveal by reveal deadline, deposit is slashed. Settlement considers only revealed bidders.

`commit` creates hidden commitment from bid and nonce. `reveal` checks timing and hash match. `settle` determines winner and price. If no one reveals, no winner and no price. If one bidder reveals, that bidder wins and pays zero. If multiple bidders reveal, revealed bids are sorted descending. Highest revealed bidder wins, but pays second-highest bid.

Errors are timing and honesty failures. `ε_early` rejects reveal too early. `ε_late` rejects reveal too late and slashes deposit. `ε_hash` rejects mismatched hash and slashes deposit. `ε_dup` rejects second reveal by same bidder.

Evidence claims truthful bidding is dominant strategy: each bidder does best by bidding real private valuation. It claims price obeys second-price auction logic. It claims non-revealers lose deposit. It also claims honest revealers are slash-safe. In plain English: this protocol preserves bid secrecy during commit phase, reveals bids later, awards item to highest revealed bidder at second-highest price, and uses deposits to discourage manipulation. 
