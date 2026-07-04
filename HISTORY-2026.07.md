# Session History — 2026-07

Part of the session history series; see `CLAUDE.md`'s "Session history" section for the entry format and cross-file conventions. Entries are ordered by date descending within this file (most recent first).

**Previous:** `HISTORY-2026.06.md`

---

## 2026-07-04 — Number Theoretic Transform (NTT)
**File:** `src/bin/poly-mul-ntt.rs`

NTT multiplies two polynomials in O(n log n) by evaluating both at n points and multiplying pointwise, instead of convolving coefficients directly — using the facts that a degree-<=d polynomial is determined by d+1 points, and that evaluating a product at a point equals the product of the two values there. The n points are the powers of h = g^((p-1)/n), an element of order exactly n built from a primitive root g. Fast evaluation splits a polynomial into even/odd halves via P(x) = E(x^2) + x*O(x^2); because h^(n/2) = -1, the pair (h^k, h^(k+n/2)) squares to the same value, letting this halving recurse all the way down whenever n is a power of 2. Recursive correctness follows by induction on the combine formula, and the inverse transform reuses the same routine with h^(-1) in place of h plus a final division by n, justified by an orthogonality identity proved via telescoping. The in-place (offset, stride) implementation surfaced five real bugs: a wrong base case, a read-position formula that assumed contiguous instead of interleaved halves, a read/write clobbering hazard needing a scratch buffer, a twiddle factor that must advance with the loop index rather than staying fixed, and a zero-padding direction mistake at the convolution level. Complexity is T(n) = 2T(n/2) + O(n) = O(n log n), the Master theorem's boundary case, versus Karatsuba's O(n^log2 3) where the branching factor exceeds the halving factor.

**Depends on:** Primitive Roots mod p (root construction, order-of-a-power lemma), Naive Polynomial Multiplication (convolution baseline, cyclic/linear distinction), Miller-Rabin Primality Test (two-square-roots-of-1 fact), Euler's Theorem / Lagrange's theorem (order divides group order), Modular Inverse (computing h^-1, n^-1), Karatsuba Multiplication (Master theorem contrast)
**Unlocks:** Multi-modulus NTT with CRT reconstruction (combining results mod several primes via `gcd-crt.rs` to exceed any single prime's representable range) — proposed as the next session; fast big-integer multiplication via NTT as an alternative to Karatsuba

---

## 2026-07-01 — Primitive Roots mod p
**File:** `src/bin/mod-primitive-root.rs`

A primitive root mod p is an element whose order — the smallest k with g^k = 1 mod p — equals p-1, the full size of the group (Z/pZ)*. Lagrange's theorem (order divides group order) turns the naive O(p) order check into an O(log^2 p) one: g is a primitive root exactly when g^((p-1)/q) != 1 mod p for every distinct prime factor q of p-1. Existence is guaranteed by a counting argument: if no element had order p-1, every element would satisfy x^((p-1)/q) = 1 for some prime factor q, but each such equation has at most (p-1)/q solutions in a field, and for p=7 the totals (3+2=5) fall short of the group size 6, a contradiction. Primitive roots make up a phi(p-1)/(p-1) fraction of all residues — a global property, so searching upward from g=2 has no advantage beyond avoiding the two guaranteed failures g=1 and g=p-1. This generalizes the safe-prime-specific shortcut found in the Baby-Step Giant-Step session (checking only g^2 and g^((p-1)/2)) into a test that works for any p. Two real bugs surfaced during implementation: `factorize` initially treated any nontrivial Pollard's Rho factor as prime without recursively checking it, and `primitive_root` never verified p itself was prime. Complexity is O(p^(1/4) log p), dominated by factoring p-1 via Pollard's Rho.

**Depends on:** Miller-Rabin Primality Test, Pollard's Rho Factoring Algorithm, Modular Exponentiation, Euler's Theorem (Lagrange's theorem), Sieve of Eratosthenes (Mertens' theorem), Baby-Step Giant-Step (safe-prime generator shortcut, generalized here)
**Unlocks:** Number Theoretic Transform (NTT) — needs a primitive root as the modular root of unity
