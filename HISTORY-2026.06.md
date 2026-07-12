# Session History — 2026-06

Part of the session history series; see `CLAUDE.md`'s "Session history" section for the entry format and cross-file conventions. Entries are ordered by date descending within this file (most recent first).

**Next:** `HISTORY-2026.07.md`

---

## 2026-06-30 — Naive Polynomial Multiplication (Linear Convolution)
**File:** `src/bin/poly-mul-naive.rs`

Polynomial multiplication is equivalent to linear convolution of the coefficient vectors. The key formula is c[k] = sum of a[i] * b[k-i] over all valid i, derived directly from the distributive law: every pair (i, j) with i+j=k contributes to the k-th output coefficient. The output has n+m-1 coefficients because degrees add (the leading terms multiply as x^(n-1) * x^(m-1) = x^(n+m-2)) and coefficient count is degree plus one. Complexity is O(n*m) because each of the n*m pairs (i,j) is visited exactly once. The distinction between linear convolution (output length n+m-1, no wrapping) and cyclic convolution (output length n, indices mod n) was introduced — NTT computes cyclic convolution natively, and zero-padding recovers linear convolution from it.

**Depends on:** — (self-contained; introduces the polynomial multiplication track as a stepping stone toward NTT)
**Unlocks:** Primitive Roots mod p, Number Theoretic Transform (NTT)

---

## 2026-06-30 — Segmented Sieve
**File:** `src/bin/sieve-segmented.rs`

The segmented sieve finds primes in [L, R] using O(sqrt(R)) space instead of O(R). Small primes up to sqrt(R) are precomputed with the basic sieve; each is then used to cross out its multiples inside the window [L, R]. The first multiple of p that falls in the window is ceil(L/p) * p, computed as `((L + p - 1) / p) * p`. A guard ensures p itself is never marked composite. Correctness: every composite c <= R has a prime factor p <= sqrt(c) <= sqrt(R), so p is always in the precomputed list. Floating point sqrt must use ceil, not floor, to avoid silently dropping a prime when R is a perfect square. Time is O(sqrt(R) log log R) for precomputation plus O((R - L) log log R) for the window.

**Depends on:** Sieve of Eratosthenes (`sieve-eratosthenes.rs`) — the precomputation step is exactly the basic sieve
**Unlocks:** —

---

## 2026-06-30 — Sieve of Eratosthenes
**File:** `src/bin/sieve-eratosthenes.rs`

The sieve finds all primes up to n in O(n log log n) time and O(n) space by elimination: mark every number as a candidate, then for each prime p cross out all multiples from p*p upward. Stopping at sqrt(n) is correct because any composite c <= n has a prime factor q <= sqrt(c) <= sqrt(n), so all composites are guaranteed to be crossed out once every prime up to sqrt(n) has been processed. Starting the inner pass at p*p (not 2*p) is correct because every smaller multiple p*k with k < p was already crossed out when k (or a prime factor of k) was processed earlier. Complexity is O(n log log n) because the total number of crossings is proportional to n times the sum of 1/p over primes p <= n, and that sum grows as log(log(n)) by Mertens' theorem — much slower than the harmonic series log(n).

**Depends on:** — (self-contained; introduces Mertens' theorem and the harmonic series as background)
**Unlocks:** Segmented Sieve (the precomputation step reuses the basic sieve directly); Primitive Roots mod p (Mertens'-theorem bound on generator search density)

---

## 2026-06-30 — Karatsuba Multiplication
**File:** `src/bin/big-uint-karatsuba.rs`

Karatsuba reduces big integer multiplication from O(n^2) to O(n^log_2(3)) ~= O(n^1.585) by saving one recursive sub-multiplication per level. The key algebraic identity is A_hi*B_lo + A_lo*B_hi = (A_lo + A_hi)(B_lo + B_hi) - A_lo*B_lo - A_hi*B_hi, so only three multiplications (z0, z1, z2) are needed instead of four. Multiplication by (2^64)^k is a free shift (prepend k zero chunks), and the subtraction order for z1 is safe because z0 + z1 + z2 equals the cross term exactly, guaranteeing neither subtraction underflows. The recurrence T(n) = 3T(n/2) + O(n) solves to O(n^log_2(3)) by the Master theorem, since log_2(3) ~= 1.585 exceeds the linear combine cost; keeping all four sub-multiplications instead would recover exactly schoolbook O(n^2). The base case (both operands one chunk) uses `mul128`, and the shorter operand is padded to match the longer before splitting so the pivot is always at least 1.

**Depends on:** Big Integer Arithmetic (Schoolbook) (`big-uint-naive.rs`) — representation, addition, subtraction, mul128, canonical invariant
**Unlocks:** Number Theoretic Transform (NTT) — contrasted against as the O(n log n) alternative to Karatsuba's O(n^log_2(3))

---

## 2026-06-28 — Big Integer Arithmetic (Schoolbook)
**File:** `src/bin/big-uint-naive.rs`

Arbitrary-precision unsigned integers are represented as `Vec<u64>` in base 2^64, least-significant chunk first. The core invariant — no trailing zeros except `vec![0]` for zero — gives every number a unique representation, making equality a plain element comparison, and comparison is lexicographic from the most significant chunk downward. Addition and subtraction use `overflowing_add`/`overflowing_sub` with a single carry or borrow that is always 0 or 1, since two chunks can never both overflow simultaneously in a way that needs a carry greater than 1; subtraction returns `Option<BigNumber>` to signal unsigned underflow, and trailing zeros are stripped afterward to restore the invariant. `mul128` computes the full 128-bit product of two `u64` values by splitting into 32-bit halves and combining four partial products, which works because the `high` result is bounded by `0xfffffffffffffffe`, always leaving room for a carry without overflow. Schoolbook multiplication then follows the distributive law directly — each `(i,j)` pair contributes `a_i * b_j` at position `i+j` — with carries deferred into a separate `carries` array (safe since each cell receives at most `min(n,m)` increments) and merged in a final linear pass. Complexity is O(n*m).

**Depends on:** — (self-contained foundation; introduces the big-integer track)
**Unlocks:** Karatsuba Multiplication — reuses the representation, addition/subtraction, and `mul128` primitives directly

---

## 2026-06-18 — Baby-Step Giant-Step (Discrete Logarithm)
**File:** `src/bin/mod-bsgs.rs`

BSGS solves g^x = h mod p in O(sqrt(p) * log p) time and O(sqrt(p)) space. The key idea is the decomposition x = k*m + y with m = ceil(sqrt(p)), which rewrites the equation as g^(km) = h * g^(-y) mod p; baby steps precompute all m right-hand values into a hash map, and giant steps scan left-hand values looking for a collision. Correctness rests on a coverage argument: m = ceil(sqrt(p)) ensures m^2 - 1 >= p-2, so every valid exponent is reachable, whereas using floor would leave gaps. The baby-step table depends on h and must be rebuilt per query, while the giant-step table depends only on g and p and can be reused across queries in the same group. As a special case, generator finding for safe primes (p-1 = 2q) requires checking only two conditions, g^2 != 1 and g^((p-1)/2) != 1, since p-1 has only two proper divisors.

**Depends on:** Modular Exponentiation (both phases), Modular Inverse (baby-step computation of g^(-y)), Miller-Rabin Primality Test (primality check for p)
**Unlocks:** Primitive Roots mod p — generalizes the safe-prime-specific generator test into the general Lagrange-based test for any p

---

## 2026-06-18 — Pollard's Rho Factoring Algorithm
**File:** `src/bin/mod-pollard-rho.rs`

Pollard's Rho finds a non-trivial factor of a composite n in expected O(n^(1/4)) time and O(1) space. The core insight is that a collision mod p (a prime factor of n) suffices: if x = y mod p but x != y mod n, then p divides |x - y| and p divides n, forcing 1 < gcd(|x - y|, n) < n. The birthday paradox guarantees such a collision after O(sqrt(p)) draws from the residues mod p, giving O(n^(1/4)) overall since p can be as large as sqrt(n). Floyd's cycle detection finds the collision in O(1) space: the tortoise advances one step and the hare two, and they must meet once both are inside the sequence's inevitable cycle, using the pseudo-random mixing function f(x) = x^2 + c mod n. Failure modes — a full collision mod n, or a degenerate cycle — are handled by retrying with a different c, and even n is handled as a special case (return 2) since the quadratic sequence cannot generate a collision mod 2 through normal dynamics.

**Depends on:** Miller-Rabin Primality Test (primality check before factoring), Modular Exponentiation (used in Miller-Rabin Primality Test), Euclidean GCD (gcd call at every step of the inner loop)
**Unlocks:** Primitive Roots mod p — factors p-1 via Pollard's Rho to find the prime divisors needed for the Lagrange-based test; Trial Division Factorization (`mod-factorize-trial.rs`) — contrasted against as the O(sqrt(n)) trial-division alternative to this algorithm's O(n^(1/4)) cost

---

## 2026-06-17 — RSA Cryptosystem
**File:** `src/bin/mod-rsa.rs`

RSA is built on three observations: n = p*q is easy to compute but hard to factor; phi(n) = (p-1)(q-1) is easy given p and q but hard from n alone; and Euler's theorem guarantees m^(d*e) = m mod n whenever d*e = 1 mod phi(n) and gcd(m, n) = 1 — a condition that m < min(p, q) conveniently guarantees but does not require. The public exponent e is chosen with gcd(e, phi(n)) = 1, and the private exponent d is its modular inverse mod phi(n), computed via extended GCD. Encryption is c = m^e mod n and decryption is m = c^d mod n; the roles of e and d are symmetric, which is why signing (encrypt with d, verify with e) is mathematically equivalent to the encryption scheme. Textbook RSA is deterministic and vulnerable to dictionary attacks on small message spaces — real deployments require random padding (OAEP) on top of this mathematical core. Complexity is keygen O(log n), encryption O(log e), decryption O(log n); the intentional asymmetry matches usage, since encryption is the high-frequency operation performed by everyone who holds the public key.

**Depends on:** Miller-Rabin Primality Test (primality check on p and q), Euler's Totient Function (computing phi(n)), Euler's Theorem & Modular Inverse via Exponentiation (correctness proof), Modular Exponentiation (encrypt/decrypt), Modular Inverse (computing d from e), Extended Euclidean GCD (the algorithm underlying Modular Inverse)
**Unlocks:** —

---

## 2026-06-16 — Miller-Rabin Primality Test
**File:** `src/bin/mod-miller-rabin.rs`

The Fermat test fails against Carmichael numbers, composites where a^(n-1) = 1 mod n for every coprime base a. The smallest is 561 = 3 x 11 x 17; it fools Fermat because (p-1) divides (n-1) for each prime factor p (Korselt's criterion), which follows from CRT plus Fermat's little theorem applied to each prime factor separately. Miller-Rabin strengthens Fermat by exploiting the fact that x^2 = 1 mod p has exactly two solutions (+-1) when p is prime — a consequence of primality forcing divisibility onto one of (x-1) or (x+1). Writing n-1 = 2^s * d with d odd, the sequence a^d, a^(2d), ..., a^(n-1) either starts at 1 or passes through -1 for a genuine prime, and a third square root of 1 anywhere in the sequence proves compositeness. A base that fails to detect a composite is called a strong liar, and Rabin proved at most 1/4 of all bases are strong liars for any composite, so k independent bases reduce the false-positive rate to (1/4)^k. Complexity is O(log n) per base.

**Depends on:** Modular Exponentiation, Euler's Theorem & Modular Inverse via Exponentiation, Chinese Remainder Theorem (for the Carmichael number analysis)
**Unlocks:** RSA Cryptosystem, Pollard's Rho Factoring Algorithm, Baby-Step Giant-Step (Discrete Logarithm), Primitive Roots mod p (primality checks throughout), Number Theoretic Transform (NTT) (two-square-roots-of-1 fact)

---

## 2026-06-16 — Euler's Theorem & Modular Inverse via Exponentiation
**File:** `src/bin/mod-euler.rs`

Euler's theorem states that for any a with gcd(a, n) = 1, a^phi(n) = 1 mod n. The proof goes through the group structure of (Z/nZ)*: the integers coprime to n form a multiplicative group of order phi(n), and Lagrange's theorem says the order of any element divides the group order, which forces a^phi(n) = 1. Fermat's little theorem is the special case n = p prime, giving a^(p-1) = 1 mod p. The theorem also yields an alternative formula for the modular inverse: multiplying both sides of a^phi(n) = 1 by a^(-1) gives a^(-1) = a^(phi(n)-1) mod n, computed in the implementation by finding phi(n) via trial division and then applying modular exponentiation. Overall complexity is O(sqrt(n)) — worse than extended GCD's O(log n) — but drops to O(log n) when n is a known prime, since phi(p) = p-1 requires no factorization at all.

**Depends on:** Euler's Totient Function, Modular Exponentiation, Euclidean GCD
**Unlocks:** RSA Cryptosystem (correctness proof), Primitive Roots mod p (Lagrange's theorem), Number Theoretic Transform (NTT) (order-divides-group-order fact)

---

## 2026-06-14 — Modular Exponentiation
**File:** `src/bin/mod-exponent.rs`

Computing a^b mod m in O(log b) uses the square-and-multiply method. The binary representation of b determines which powers of a contribute to the product, and consecutive powers are related by squaring, so a single least-significant-bit-first pass suffices. Two running values are maintained: `power` (the current power of the base) and `result` (the accumulated product of powers at set bit positions), both reduced mod m after every multiplication to keep intermediates bounded, with u128 used for the intermediate product to avoid overflow before reduction. The loop invariant is that after n bits are processed, `result` equals base^(exp & (2^n - 1)) mod m and `power` equals base^(2^n) mod m, so once all bits are consumed the mask covers the full exponent and the invariant gives the answer. Edge cases: exp = 0 returns 1, modulus = 1 returns 0, and modulus = 0 returns None.

**Depends on:** Euclidean GCD (modular arithmetic foundation — the reduction identity (a*b) mod m = ((a mod m)*(b mod m)) mod m underpins every multiplication step)
**Unlocks:** Euler's Theorem & Modular Inverse via Exponentiation, Miller-Rabin Primality Test, Baby-Step Giant-Step (Discrete Logarithm), RSA Cryptosystem, Primitive Roots mod p — the core exponentiation primitive reused throughout the modular-arithmetic track

---

## 2026-06-14 — Euler's Totient Function
**File:** `src/bin/mod-totient.rs`

Euler's totient function phi(n) counts the integers in [1, n] coprime to n. The key structural property is multiplicativity: when gcd(m, n) = 1, phi(m*n) = phi(m) * phi(n), and combined with the prime-power formula phi(p^k) = p^k - p^(k-1), this gives a product formula over the prime factorization of n. The O(sqrt(n)) complexity relies on the fact that any n > 1 remaining after trial division must itself be prime — if it were composite (n = a*b), at least one factor would be at most sqrt(n) and would already have been divided out by an earlier iteration. The implementation uses p*p <= current n (not the original n) as the loop guard, since n shrinks as factors are removed during the run.

**Depends on:** Euclidean GCD (coprimality, the factorization argument)
**Unlocks:** Euler's Theorem & Modular Inverse via Exponentiation, RSA Cryptosystem (computing phi(n))

---

## 2026-06-11 — Chinese Remainder Theorem
**File:** `src/bin/gcd-crt.rs`

The Chinese Remainder Theorem finds the unique solution x mod (m1*m2) to two congruences with coprime moduli m1 and m2. The construction writes x = a1 + k*m1 and substitutes into the second congruence to solve for k, giving k = (a2 - a1) * m1^(-1) mod m2 via the modular inverse of m1 mod m2. Uniqueness follows because any two solutions differ by a multiple of both moduli, and since the moduli are coprime their product must divide that difference, which is impossible for two distinct solutions inside the same range of size m1*m2. A real bug surfaced during the session: (a2 - a1) must be reduced mod m2 before being treated as a value to multiply, since a naive unsigned subtraction can underflow when a2 < a1. Complexity is O(log(min(m1, m2))), dominated by the extended Euclidean algorithm inside the modular inverse call.

**Depends on:** Modular Inverse (and transitively Extended Euclidean GCD)
**Unlocks:** Miller-Rabin Primality Test (Carmichael number analysis via CRT), Multi-Modulus NTT with CRT Reconstruction (`poly-mul-crt.rs`)

---

## 2026-06-11 — Modular Inverse
**File:** `src/bin/mod-inverse.rs`

The modular inverse of a mod m is a thin wrapper over extended GCD: it exists exactly when gcd(a, m) = 1, and when it does, the Bezout coefficient x from a*x + m*y = gcd(a,m) = 1 is the answer after normalizing into the range [0, m). Existence was explored by hand with a = 2, m = 4, where gcd(2,4) = 2 != 1 confirms no inverse exists in that case. Normalization uses m - |x| rather than (x + m) % m, which avoids overflow when m is close to i64::MAX and x is negative. The result satisfies a * a^(-1) = 1 mod m by construction, which is exactly the defining property being computed. Complexity is O(log(min(a, m))), inherited directly from the extended Euclidean algorithm underneath.

**Depends on:** Extended Euclidean GCD
**Unlocks:** Chinese Remainder Theorem, Baby-Step Giant-Step (Discrete Logarithm), RSA Cryptosystem, Number Theoretic Transform (NTT) — computing modular inverses throughout; Number Theory Step by Step, Section 1.3: Euclidean Algorithm (Bezout machinery reused for back-substitution), Number Theory Step by Step, Section 1.4: Linear Diophantine Equations

---

## 2026-06-11 — Extended Euclidean GCD
**File:** `src/bin/gcd-euclidean-extended.rs`

Extended Euclidean GCD augments the ordinary algorithm to also produce Bezout coefficients x and y such that a*x + b*y = gcd(a, b). The coefficients are threaded through the same reduction steps used by the basic algorithm: each new remainder inherits updated coefficients by direct substitution, seeded with (x_a, y_a) = (1, 0) and (x_b, y_b) = (0, 1). The loop invariant holds at every step — a_current = a_initial * x_a + b_initial * y_a — which is what guarantees the final coefficients are correct once a_current reaches the gcd. An overflow edge case, when the quotient q exceeds i64::MAX, can only occur when b = 1, meaning the loop is about to exit anyway, so the discarded coefficient update never affects the result. Termination and complexity are identical to the basic Euclidean algorithm: O(log(min(a,b))) via Lame's theorem.

**Depends on:** Euclidean GCD
**Unlocks:** Modular Inverse — a thin wrapper that reads the Bezout coefficient directly off this algorithm; Number Theory Step by Step, Section 1.3: Euclidean Algorithm (Bezout machinery reused for back-substitution), Number Theory Step by Step, Section 1.4: Linear Diophantine Equations; Continued Fractions (the incremental-state-threading design reused for computing convergents alongside a loop); Trial Division Factorization (`mod-factorize-trial.rs`) — transitivity of divisibility justifies never backtracking the divisor search

---

## 2026-06-11 — Binary GCD (Stein's Algorithm)
**File:** `src/bin/gcd-binary.rs`

Binary GCD (Stein's algorithm) reaches the same asymptotic complexity as Euclidean GCD but replaces division with shifts and subtraction. Two properties drive the algorithm: GCD is preserved under subtraction of one value from the other, and any common factors of 2 can be stripped out at the start and restored at the end by shifting. Subtracting two odd numbers always yields an even result, which guarantees the working values lose at least one factor of 2 every two steps, bounding the number of iterations logarithmically. The base case works because exactly one of {a, b} reaches zero at termination, so the surviving value combines with the stripped shift factor 2^k to give gcd(a, b) = (a + b) << k. Complexity is O(log(min(a,b))), matching the Euclidean algorithm, since the same bounded-halving argument applies to both.

**Depends on:** Euclidean GCD (for contrast and the shared complexity argument)
**Unlocks:** —

---

## 2026-06-10 — Euclidean GCD
**File:** `src/bin/gcd-euclidean-basic.rs`

Euclidean GCD is the first algorithm in this track and establishes the core invariant that gcd(a, b) = gcd(b, a mod b), which holds because a mod b is a linear combination of a and b, so any common divisor of the pair is preserved across the reduction. Termination follows because b strictly decreases at each step while staying non-negative, and the algorithm handles zero directly since gcd(a, 0) = a. Complexity is O(log(min(a,b))) by Lame's theorem, which bounds the number of steps using the Fibonacci sequence as the pathological worst case. The implementation also handles negative inputs correctly and swaps a and b for free when a < b, since the first iteration would otherwise just perform that swap anyway.

**Depends on:** —
**Unlocks:** Extended Euclidean GCD, Binary GCD (Stein's Algorithm) — contrast, Modular Exponentiation (reduction identity), Euler's Totient Function (coprimality), Euler's Theorem & Modular Inverse via Exponentiation (gcd foundation), Pollard's Rho Factoring Algorithm (gcd call in the inner loop) — the foundational algorithm reused throughout this repo; Number Theory Step by Step, Section 1.3: Euclidean Algorithm (the gcd(a,b)=gcd(b,r) invariant matches Proposition 1.14); Continued Fractions (division-with-remainder mechanics, Lame's theorem)
