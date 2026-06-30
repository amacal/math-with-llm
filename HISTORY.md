# Session History

Ordered by date descending. Each entry records the date, the concept covered, the file, and the key ideas built in that session. Dependencies on prior entries are noted explicitly — these are the concepts a review should check before introducing the next problem.

---

## 2026-06-30 — Karatsuba Multiplication
**File:** `src/bin/big-uint-karatsuba.rs`

Karatsuba reduces big integer multiplication from O(n^2) to O(n^log_2(3)) ≈ O(n^1.585) by saving one recursive sub-multiplication per level. The key algebraic identity: A_hi*B_lo + A_lo*B_hi = (A_lo + A_hi)(B_lo + B_hi) - A_lo*B_lo - A_hi*B_hi, so only three multiplications (z0, z1, z2) are needed instead of four. Multiplication by (2^64)^k is a free shift (prepend k zero chunks). The subtraction order for z1 is safe because z0 + z1 + z2 = (A_lo+A_hi)(B_lo+B_hi) guarantees neither step underflows. The recurrence T(n) = 3T(n/2) + O(n) solves to O(n^log_2(3)) by the Master Theorem (log_2(3) ≈ 1.585 > 1 = c, so recursive work dominates). Had four sub-multiplications been kept, log_2(4) = 2 recovers exactly schoolbook O(n^2). Base case: both operands have one chunk, computed via mul128. Pivot strategy: pad the shorter operand to match the longer before splitting, ensuring pivot >= 1 always.

**Depends on:** Big Integer Arithmetic / Schoolbook (`big-uint-naive.rs`) — representation, addition, subtraction, mul128, canonical invariant

---

## 2026-06-28 — Big Integer Arithmetic (Schoolbook)
**File:** `src/bin/big-uint-naive.rs`

Arbitrary-precision unsigned integers represented as `Vec<u64>` in base 2^64, least-significant chunk first. The core invariant — no trailing zeros except `vec![0]` for zero — gives every number a unique representation, making equality a plain element comparison. Comparison is lexicographic from the most significant chunk downward. Addition and subtraction use `overflowing_add`/`overflowing_sub` with a single carry or borrow that is always 0 or 1 (the two-overflows-simultaneously argument). Subtraction returns `Option<BigNumber>` to signal the unsigned underflow case; trailing zeros are stripped to restore the invariant. `mul128` computes the full 128-bit product of two `u64` values by splitting into 32-bit halves and combining four products; correctness rests on the fact that `high` from `mul128` is at most `0xfffffffffffffffe`, leaving room for a carry without overflow. Schoolbook multiplication follows the distributive law: each `(i,j)` pair contributes `a_i * b_j` at position `i+j`. Deferred carry handling: instead of chasing cascading carries immediately, overflow counts are accumulated in a separate `carries` array (safe because each cell receives at most `min(n,m)` increments) and merged into the result in a final linear pass. Complexity O(n*m).

**Depends on:** — (self-contained foundation; introduces the big-integer track)

---

## 2026-06-18 — Baby-Step Giant-Step (Discrete Logarithm)
**File:** `src/bin/mod-bsgs.rs`

BSGS solves g^x ≡ h (mod p) in O(sqrt(p) * log p) time and O(sqrt(p)) space. The key idea is the decomposition x = k*m + y with m = ceil(sqrt(p)), which rewrites the equation as g^(km) ≡ h * g^(-y) (mod p). Baby steps precompute all m right-hand values into a hash map; giant steps scan left-hand values and look for a collision. Correctness rests on the coverage argument: m = ceil(sqrt(p)) ensures m^2 - 1 >= p-2, so every valid exponent is reachable — floor would leave gaps. The baby-step table depends on h and must be rebuilt per query; the giant-step table depends only on g and p and can be reused across queries for the same group. Generator finding for safe primes requires checking only two conditions: g^2 ≢ 1 and g^((p-1)/2) ≢ 1, since p-1 = 2q has only two proper divisors.

**Depends on:** Modular Exponentiation (both phases), Modular Inverse (baby-step computation of g^(-y)), Miller-Rabin (primality check for p)

---

## 2026-06-18 — Pollard's Rho Factoring Algorithm
**File:** `src/bin/mod-pollard-rho.rs`

Pollard's Rho finds a non-trivial factor of a composite n in expected O(n^(1/4)) time and O(1) space. The core insight is that a collision mod p (a prime factor of n) suffices: if x ≡ y (mod p) but x ≢ y (mod n), then p divides |x - y| and p divides n, forcing 1 < gcd(|x - y|, n) < n. The birthday paradox guarantees such a collision after O(sqrt(p)) draws from the residues mod p, giving O(n^(1/4)) in the worst case (p ≈ sqrt(n)). Floyd's cycle detection finds the collision in O(1) space: tortoise advances one step, hare two, and they must meet once both are inside the inevitable cycle of the finite sequence. The function f(x) = x^2 + c mod n provides pseudo-random mixing mod p. Failure modes — full collision mod n, or degenerate cycles — are handled by retrying with a different c. Even n is handled as a special case (return 2) since the quadratic sequence cannot generate a collision mod 2 through normal dynamics.

**Depends on:** Miller-Rabin (primality check before factoring), Modular Exponentiation (used in Miller-Rabin), Euclidean GCD (gcd call at every step of the inner loop)

---

## 2026-06-17 — RSA Cryptosystem
**File:** `src/bin/mod-rsa.rs`

RSA is built on three observations: n = p·q is easy to compute but hard to factor; φ(n) = (p-1)(q-1) is easy given p and q but hard from n alone; and Euler's theorem guarantees m^(de) ≡ m (mod n) whenever de ≡ 1 (mod φ(n)) and gcd(m, n) = 1. The public exponent e is chosen with gcd(e, φ(n)) = 1; the private exponent d is its modular inverse mod φ(n), computed via extended GCD. Encryption is c = m^e mod n, decryption is m = c^d mod n; the roles of e and d are symmetric, which makes signing (encrypt with d, verify with e) mathematically equivalent. The message constraint m < min(p, q) ensures gcd(m, n) = 1 so Euler's theorem applies. Textbook RSA is deterministic and vulnerable to dictionary attacks on small message spaces — real deployments require random padding (OAEP). Complexity: keygen O(log n), encryption O(log e), decryption O(log n); the asymmetry is intentional since encryption is the high-frequency operation.

**Depends on:** Miller-Rabin (primality check on p and q), Euler's Totient Function (computing φ(n)), Euler's Theorem / Fermat's Little Theorem (correctness proof), Modular Exponentiation (encrypt/decrypt), Modular Inverse / Extended Euclidean GCD (computing d from e)

---

## 2026-06-16 — Miller-Rabin Primality Test
**File:** `src/bin/mod-miller-rabin.rs`

The Fermat test fails against Carmichael numbers — composites where a^(n-1) ≡ 1 (mod n) for all coprime bases. The smallest is 561 = 3 × 11 × 17; it fools Fermat because (p-1) | (n-1) for each prime factor p (Korselt's criterion), which follows from CRT plus Fermat applied to each prime separately.

Miller-Rabin strengthens Fermat by exploiting the fact that x^2 ≡ 1 (mod p) has exactly two solutions (±1) when p is prime — a consequence of p being prime forcing divisibility onto one of (x-1) or (x+1). Write n-1 = 2^s × d (d odd) and inspect the sequence a^d, a^(2d), ..., a^(n-1). For a prime the sequence either starts at 1 or passes through -1; a third square root of 1 proves compositeness. A base that fails to detect a composite is a strong liar; Rabin proved at most 1/4 of all bases are strong liars for any composite, so k independent bases reduce the false positive rate to (1/4)^k. Complexity: O(log n) per base.

**Depends on:** Modular Exponentiation, Euler's Theorem / Fermat's Little Theorem, Chinese Remainder Theorem (for the Carmichael number analysis)

---

## 2026-06-16 — Euler's Theorem & Modular Inverse via Exponentiation
**File:** `src/bin/mod-euler.rs`

Euler's Theorem: for any a with gcd(a, n) = 1, a^φ(n) ≡ 1 (mod n). The proof goes through the group structure of (Z/nZ)* — the integers coprime to n form a multiplicative group of order φ(n), and by Lagrange's theorem the order of any element divides the group order, forcing a^φ(n) = 1. Fermat's Little Theorem is the special case n = p prime, giving a^(p-1) ≡ 1 (mod p).

The theorem yields an alternative formula for modular inverse: multiplying a^φ(n) ≡ 1 by a^(-1) gives a^(-1) ≡ a^(φ(n)-1) (mod n). The implementation computes φ(n) by trial division then applies modular exponentiation. Overall complexity is O(sqrt(n)) — worse than extended GCD's O(log n) — but reduces to O(log n) when n is a known prime, since φ(p) = p-1 requires no factorization.

**Depends on:** Euler's Totient Function, Modular Exponentiation, Euclidean GCD

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
