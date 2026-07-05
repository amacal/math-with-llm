# Session History — 2026-07

Part of the session history series; see `CLAUDE.md`'s "Session history" section for the entry format and cross-file conventions. Entries are ordered by date descending within this file (most recent first).

**Previous:** `HISTORY-2026.06.md`

---

## 2026-07-05 — Multi-Modulus NTT with CRT Reconstruction
**File:** `src/bin/poly-mul-crt.rs`

Multi-modulus NTT multiplies polynomials whose output coefficients exceed any single NTT prime by running the NTT independently modulo three primes (998244353, 985661441, 469762049) and recovering each true coefficient via two successive CRT calls. The key correctness condition is that c[k] < p1*p2*p3 for every output coefficient: uniqueness of the CRT solution then forces the recovered value to equal c[k] exactly, since c[k] itself satisfies all three congruences by construction. When a single prime p suffices depends on whether n*B^2 < p, where n is the transform length and B bounds the input coefficients; the three-prime product (~4.6e26) pushes this threshold far above u64::MAX, so any output coefficient that fits in u64 is guaranteed exact. Two bugs were found: the inverse NTT divides by the transform size n=16, not by p-1, and these coincide only for p=17 (the small test prime), masking the error until a production prime was used; and the CRT chaining loop initially accumulated against the raw p1 residues at every step instead of the running combined residue, causing the second CRT call to silently ignore the p2 information. The CRT function was widened to u128 throughout to handle intermediate moduli up to p1*p2*p3. Complexity is O(n log n), dominated by the three NTT passes; reconstruction is O(n) with a constant log(p) factor absorbed into the constant.

**Depends on:** Number Theoretic Transform (NTT), Chinese Remainder Theorem
**Unlocks:** —

---

## 2026-07-05 — Number Theory Step by Step, Section 1.3: Euclidean Algorithm
**Source:** *Number Theory Step by Step* (Kuldeep Singh), Chapter 1, Section 1.3 (Exercises 1-19)

Section 1.3 proves Bezout's Identity via well-ordering (the least positive value of a set S = {ax+by : ax+by>0} is shown to be gcd(a,b)) and builds the Euclidean Algorithm and its reverse on top of it. Worked gcd(156,18)=6 via 156=18*8+12, 18=12*1+6, 12=6*2+0, then back-substituted to find 156(-1)+18(9)=6, reusing the same Bezout machinery already coded in gcd-euclidean-extended.rs but running it by hand in the book's back-substitution style instead of the forward-threaded style used in that file. Euclid's Lemma (a|bc, gcd(a,b)=1 => a|c) got an intuitive prime-factorization argument via 198|5x => 198|x: since gcd(198,5)=1, none of 198's prime factors (2, 3^2, 11) can hide inside the 5, so they must all be in x. The harder proof exercises (12, 15, 17, 19) all resolved via two reused techniques: proving set-equality of common divisors in both directions (the same technique behind gcd(a,b)=gcd(b,r) in gcd-euclidean-basic.md), and mutual divisibility (X|Y and Y|X, both positive, so X=Y) -- the latter closed exercise 17's proof that gcd(ma,mb)=mg. Exercise 12(ii)'s induction needed an extra multiplicativity lemma (gcd(a,m)=1 and gcd(b,m)=1 implies gcd(ab,m)=1), proved by multiplying the two Bezout equations a*x1+m*y1=1 and b*x2+m*y2=1 together and regrouping. Recurring precision slips: mixing up which quantity a divisibility statement applies to (claiming "d|x, d|y" instead of "d|a, d|b"), and asserting "gcd(ma,mb) = mg" directly from a linear combination instead of the weaker "gcd(ma,mb) | mg" that the combination actually gives.

**Depends on:** Extended Euclidean GCD, Modular Inverse (both reused the Bezout machinery), gcd-euclidean-basic's Proposition 1.14 argument
**Unlocks:** Section 1.4 (Linear Diophantine Equations)

---

## 2026-07-04 — Number Theoretic Transform (NTT)
**File:** `src/bin/poly-mul-ntt.rs`

NTT multiplies two polynomials in O(n log n) by evaluating both at n points and multiplying pointwise, instead of convolving coefficients directly — using the facts that a degree-<=d polynomial is determined by d+1 points, and that evaluating a product at a point equals the product of the two values there. The n points are the powers of h = g^((p-1)/n), an element of order exactly n built from a primitive root g. Fast evaluation splits a polynomial into even/odd halves via P(x) = E(x^2) + x*O(x^2); because h^(n/2) = -1, the pair (h^k, h^(k+n/2)) squares to the same value, letting this halving recurse all the way down whenever n is a power of 2. Recursive correctness follows by induction on the combine formula, and the inverse transform reuses the same routine with h^(-1) in place of h plus a final division by n, justified by an orthogonality identity proved via telescoping. The in-place (offset, stride) implementation surfaced five real bugs: a wrong base case, a read-position formula that assumed contiguous instead of interleaved halves, a read/write clobbering hazard needing a scratch buffer, a twiddle factor that must advance with the loop index rather than staying fixed, and a zero-padding direction mistake at the convolution level. Complexity is T(n) = 2T(n/2) + O(n) = O(n log n), the Master theorem's boundary case, versus Karatsuba's O(n^log2 3) where the branching factor exceeds the halving factor.

**Depends on:** Primitive Roots mod p (root construction, order-of-a-power lemma), Naive Polynomial Multiplication (convolution baseline, cyclic/linear distinction), Miller-Rabin Primality Test (two-square-roots-of-1 fact), Euler's Theorem / Lagrange's theorem (order divides group order), Modular Inverse (computing h^-1, n^-1), Karatsuba Multiplication (Master theorem contrast)
**Unlocks:** Multi-modulus NTT with CRT Reconstruction (`poly-mul-crt.rs`)

---

## 2026-07-01 — Primitive Roots mod p
**File:** `src/bin/mod-primitive-root.rs`

A primitive root mod p is an element whose order — the smallest k with g^k = 1 mod p — equals p-1, the full size of the group (Z/pZ)*. Lagrange's theorem (order divides group order) turns the naive O(p) order check into an O(log^2 p) one: g is a primitive root exactly when g^((p-1)/q) != 1 mod p for every distinct prime factor q of p-1. Existence is guaranteed by a counting argument: if no element had order p-1, every element would satisfy x^((p-1)/q) = 1 for some prime factor q, but each such equation has at most (p-1)/q solutions in a field, and for p=7 the totals (3+2=5) fall short of the group size 6, a contradiction. Primitive roots make up a phi(p-1)/(p-1) fraction of all residues — a global property, so searching upward from g=2 has no advantage beyond avoiding the two guaranteed failures g=1 and g=p-1. This generalizes the safe-prime-specific shortcut found in the Baby-Step Giant-Step session (checking only g^2 and g^((p-1)/2)) into a test that works for any p. Two real bugs surfaced during implementation: `factorize` initially treated any nontrivial Pollard's Rho factor as prime without recursively checking it, and `primitive_root` never verified p itself was prime. Complexity is O(p^(1/4) log p), dominated by factoring p-1 via Pollard's Rho.

**Depends on:** Miller-Rabin Primality Test, Pollard's Rho Factoring Algorithm, Modular Exponentiation, Euler's Theorem (Lagrange's theorem), Sieve of Eratosthenes (Mertens' theorem), Baby-Step Giant-Step (safe-prime generator shortcut, generalized here)
**Unlocks:** Number Theoretic Transform (NTT) — needs a primitive root as the modular root of unity
