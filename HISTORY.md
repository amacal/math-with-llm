# Session History

Ordered by date descending. Each entry records the date, the concept covered, the file, and the key ideas built in that session. Dependencies on prior entries are noted explicitly — these are the concepts a review should check before introducing the next problem.

---

## 2026-06-14 — Modular Exponentiation
**File:** `src/bin/mod-exponent.rs`

Computing a^b mod m in O(log b) using the square-and-multiply method. The binary representation of b determines which powers of a contribute to the product; consecutive powers are related by squaring, so a single LSB-first pass suffices. Two running values are maintained: `power` (current power of the base) and `result` (accumulated product of powers at set bit positions). Both are reduced mod m after every multiplication to keep intermediates bounded; u128 is used for the intermediate product to avoid overflow before reduction.

Loop invariant: after n bits processed, result = base^(exp & (2^n - 1)) mod m and power = base^(2^n) mod m. When all bits are consumed the mask covers the full exponent, giving the correct answer. Edge cases: exp = 0 returns 1; modulus = 1 returns 0; modulus = 0 returns None.

**Depends on:** Euclidean GCD (modular arithmetic foundation — the reduction identity (a·b) mod m = ((a mod m)·(b mod m)) mod m underpins every multiplication step)

---

## 2026-06-14 — Euler's Totient Function
**File:** `src/bin/mod-totient.rs`

φ(n) counts integers in [1, n] coprime to n. The key structural property is multiplicativity: when gcd(m, n) = 1,

$$\varphi(mn) = \varphi(m) \cdot \varphi(n)$$

Combined with the prime power formula,

$$\varphi(p^k) = p^k - p^{k-1}$$

this gives a product formula over the prime factorization of n.

Key result: the O(sqrt(n)) complexity relies on the argument that any remaining n > 1 after trial division must be prime — if it were composite (n = a·b), at least one factor would be ≤ sqrt(n) and would have been divided out already. The implementation uses p·p ≤ current n as the loop guard (not original n), since n shrinks as factors are removed.

**Depends on:** Euclidean GCD (coprimality, the factorization argument)

---

## 2026-06-11 — Chinese Remainder Theorem
**File:** `src/bin/gcd-crt.rs`

Given two congruences with coprime moduli, find the unique solution mod m₁·m₂. Construction: write x = a₁ + k·m₁, substitute into the second congruence, solve for k using modular inverse of m₁ mod m₂.

Key result: uniqueness follows from both moduli dividing x − y and their product exceeding the difference. Unsigned subtraction requires care — missing the inner reduction before subtracting from m₂ was a real bug found during the session:

$$k \equiv (a_2 - a_1) \cdot m_1^{-1} \pmod{m_2}$$

**Depends on:** Modular Inverse (and transitively Extended Euclidean GCD)

---

## 2026-06-11 — Modular Inverse
**File:** `src/bin/mod-inverse.rs`

A thin wrapper over extended GCD. The modular inverse of a mod m exists iff gcd(a, m) = 1; when it does, the Bézout coefficient x is the answer after normalisation to [0, m). Existence explored by hand with a=2, m=4.

Key result: normalisation uses m − |x| rather than (x + m) % m to avoid overflow when m > i64::MAX.

$$a^{-1} \bmod m = x \quad \text{where } ax + my = 1$$

**Depends on:** Extended Euclidean GCD

---

## 2026-06-11 — Extended Euclidean GCD
**File:** `src/bin/gcd-euclidean-extended.rs`

Augments the Euclidean algorithm to produce Bézout coefficients x, y such that

$$ax + by = \gcd(a, b)$$

The coefficients are threaded through the same reduction steps: each new remainder inherits updated coefficients by substitution. Seeds are (xₐ, yₐ) = (1, 0) and (x_b, y_b) = (0, 1).

Key result: the loop invariant holds at every step:

$$a_{\text{current}} = a_{\text{initial}} \cdot x_a + b_{\text{initial}} \cdot y_a$$

The overflow edge case — when q > i64::MAX — can only occur when b = 1, meaning the loop is about to exit and the coefficient update is discarded safely.

**Depends on:** Euclidean GCD

---

## 2026-06-11 — Binary GCD (Stein's Algorithm)
**File:** `src/bin/gcd-binary.rs`

Same asymptotic complexity as Euclidean GCD but built on different primitives: shifts and subtracts instead of division. Two properties drive everything: GCD is preserved under subtraction, and common factors of 2 can be stripped and restored. Subtracting two odd numbers always yields an even — this guarantees bit reduction every two steps.

Key result: the base case works because exactly one of {a, b} is zero at termination, so a + b is the survivor and the shift restores the stripped factors:

$$\gcd(a, b) = (a + b) \ll k$$

**Depends on:** Euclidean GCD (for contrast and the shared complexity argument)

---

## 2026-06-10 — Euclidean GCD
**File:** `src/bin/gcd-euclidean-basic.rs`

The first algorithm. Establishes the core invariant that underpins almost everything else in this track:

$$\gcd(a, b) = \gcd(b,\ a \bmod b)$$

This holds because a mod b is a linear combination of a and b, so any common divisor is preserved.

Key results: termination argument (b strictly decreases), O(log(min(a, b))) via Lamé's theorem, correct handling of zeros and the swap-for-free property when a < b.

**Depends on:** —
