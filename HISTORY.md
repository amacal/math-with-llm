# Session History

Ordered by date descending. Each entry records the date, the concept covered, the file, and the key ideas built in that session. Dependencies on prior entries are noted explicitly — these are the concepts a review should check before introducing the next problem.

---

## 2026-06-11 — Chinese Remainder Theorem
**File:** `src/bin/gcd-crt.rs`

Given two congruences with coprime moduli, find the unique solution mod `m₁·m₂`. Construction: write `x = a₁ + k·m₁`, substitute into the second congruence, solve for `k` using modular inverse of `m₁ mod m₂`.

Key result: uniqueness follows from both moduli dividing `x - y` and their product exceeding the difference. Unsigned subtraction `(a₂ - a₁) mod m₂` requires care — missing the inner `% m₂` before subtracting from `m₂` was a real bug found during the session.

**Depends on:** Modular Inverse (and transitively Extended Euclidean GCD)

---

## 2026-06-11 — Modular Inverse
**File:** `src/bin/mod-inverse.rs`

A thin wrapper over extended GCD. The modular inverse of `a mod m` exists iff `gcd(a, m) = 1`; when it does, the Bézout coefficient `x` is the answer (after normalisation to `[0, m)`). Existence explored by hand with `a=2, m=4`.

Key result: normalisation uses `m - |x|` rather than `(x + m as i64) as u64 % m` to avoid overflow when `m > i64::MAX`.

**Depends on:** Extended Euclidean GCD

---

## 2026-06-11 — Extended Euclidean GCD
**File:** `src/bin/gcd-euclidean-extended.rs`

Augments the Euclidean algorithm to also produce Bézout coefficients `x, y` such that `ax + by = gcd(a, b)`. The coefficients are threaded through the same reduction steps: each new remainder inherits updated coefficients by substitution. Seeds are `(xa, ya) = (1, 0)` and `(xb, yb) = (0, 1)`.

Key result: the loop invariant (`a = initial_a·xa + initial_b·ya`) holds at every step. The overflow edge case — when `q > i64::MAX` — can only occur when `b = 1`, meaning the loop is about to exit and the coefficient update is discarded safely.

**Depends on:** Euclidean GCD

---

## 2026-06-11 — Binary GCD (Stein's Algorithm)
**File:** `src/bin/gcd-binary.rs`

Same asymptotic complexity as Euclidean GCD but built on different primitives: shifts and subtracts instead of division. Two properties drive everything: GCD is preserved under subtraction, and common factors of 2 can be stripped and restored. Subtracting two odd numbers always yields an even — this guarantees bit reduction every two steps.

Key result: the base case `(a + b) << k` works because exactly one of `{a, b}` is zero at termination, so `a + b` is the survivor and `k` restores the stripped factors.

**Depends on:** Euclidean GCD (for contrast and the shared complexity argument)

---

## 2026-06-10 — Euclidean GCD
**File:** `src/bin/gcd-euclidean-basic.rs`

The first algorithm. Establishes the core invariant that underpins almost everything else in this track: `gcd(a, b) == gcd(b, a mod b)`, because `a mod b` is a linear combination of `a` and `b`, so any common divisor is preserved.

Key results: termination argument (b strictly decreases), O(log(min(a,b))) via Lamé's theorem, correct handling of zeros and the swap-for-free property when `a < b`.

**Depends on:** —
