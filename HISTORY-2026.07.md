# Session History — 2026-07

Part of the session history series; see `CLAUDE.md`'s "Session history" section for the entry format and cross-file conventions. Entries are ordered by date descending within this file (most recent first).

**Previous:** `HISTORY-2026.06.md`

---

## 2026-07-10 — Newton's Method for Integer Square Root
**File:** `src/bin/mod-isqrt-newton.rs`

Newton's method computes isqrt(n) by iterating x1 = (x0 + n/x0)/2, derived from scratch as the x-intercept of the tangent line to f(x) = x^2 - n at the current guess x0, and shown to converge quadratically (e1 = e0^2/(2*x0), versus bisection's geometric e_(k+1) = c*e_k) via the substitution s = sqrt(n), e0 = x0 - s. Hand-tracing the integer-truncated version on n = 8 (8, 4, 3, 2, then back to 3) revealed the sequence never settles at a fixed point but enters a 2-3 cycle, which falsified the naive invariant "x0 >= sqrt(n)" (the final value 2 is below sqrt(8) ~= 2.828) and motivated the real invariant x0 >= r = floor(sqrt(n)) instead, together with a stopping rule of x1 >= x0 (not x1 == x0), returning x0. The invariant's lower bound follows from AM-GM ((a+b)/2 >= sqrt(a*b)) applied to x0 and n/x0, whose product is n; the upper-bound half (the sequence strictly decreases while x0 > r) follows because x0 > r forces x0 >= r+1, hence x0^2 > n by the definition of r, hence n/x0 < x0. Complexity has two phases: an initial halving phase indistinguishable from bisection (since n/x0 is tiny while x0 >> r, taking about (1/2)*log2(n) steps), followed by a much smaller O(log log n) quadratic tail once x0 nears r, so overall complexity is O(log n) — the same class as mod-isqrt-bisect.rs's bisection, not asymptotically faster, since the search starts far from the root. Overflow at x0 = n = u64::MAX is handled with a saturating add, whose off-by-one is harmless since the resulting guess remains a massive overestimate that the strict-decrease argument still corrects on the next step. All tests pass across low, mid, and high (u64::MAX-adjacent) ranges.

**Depends on:** Integer Square Root (`mod-isqrt-bisect.rs`) — same problem, contrasting algorithm and stopping-condition reasoning
**Unlocks:** —

---

## 2026-07-10 — Integer Square Root
**File:** `src/bin/mod-isqrt-bisect.rs`

isqrt(n) computes the largest r with r^2 <= n using only integer bisection, motivated directly by a floating-point sqrt precision bug flagged in the segmented sieve's notes. An early n/2 upper bound was shown to fail for n = 1, 2, 3, but the crudest possible bracket lo=0, hi=n turned out to cost the same O(log n) asymptotic complexity as any tighter bound, since log(sqrt(n)) = log(n)/2 is only a constant-factor difference. Hand-tracing bisection on n=2 surfaced two real stalls: a floor-rounded mid = (lo+hi)/2 combined with lo=mid on the non-overshoot branch never advances once hi = lo+1, and setting hi=mid on the overshoot branch is a no-op whenever mid already equals hi. Both were fixed together by discarding mid entirely on overshoot (hi = mid-1, since an overshooting mid can never be the answer) and rounding mid up rather than down, which as a side effect absorbs the final two-candidate check into the loop itself instead of needing a separate check after it stops. Overflow is handled by treating any checked-multiplication overflow of mid*mid as an automatic overshoot, sound because n itself is bounded by the same 64-bit range; a narrower edge case, n = u64::MAX causing hi-lo+1 to overflow on the very first iteration, is handled with a saturating add whose resulting off-by-one is harmless since the inflated mid is discarded by the overshoot branch regardless. Complexity is O(log n), an exponential improvement over a linear scan from 0.

**Depends on:** — (self-contained bisection algorithm; motivated by, but not technically dependent on, the floating-point sqrt bug flagged in Segmented Sieve's notes)
**Unlocks:** Newton's Method for Integer Square Root (`mod-isqrt-newton.rs`) — contrasting algorithm for the same problem, quadratic convergence versus geometric

---

## 2026-07-08 — Number Theory Step by Step, Section 1.4: Linear Diophantine Equations
**Source:** *Number Theory Step by Step* (Kuldeep Singh), Chapter 1, Section 1.4 (Exercises 1-16, Supplementary Problems 1.1-1.24)

Section 1.4 builds the full theory of linear Diophantine equations ax+by=c on top of Bezout's Identity from Section 1.3, working through all 16 Exercises 1.4 and all 24 Supplementary Problems 1 in book order; this is one of the largest sessions in the repo, spanning 40 problems, so the entry below trims to the load-bearing facts rather than recapping every exercise. Proposition 1.16 shows that from one solution x0,y0, x=x0+bt and y=y0-at is also a solution for any a,b, by direct substitution. Proposition 1.17 pins down solvability itself: ax+by=c has integer solutions if and only if gcd(a,b) divides c, proved in both directions using the fact that gcd(a,b) divides any linear combination of a and b, plus Bezout's Identity for the converse. A concrete counterexample (6x+10y=8, where x=8,y=-4 solves it but is never produced by x=3+10m) showed Proposition 1.16's formula is incomplete whenever gcd(a,b) exceeds 1, motivating Proposition 1.18: the complete general solution is x=x0+t(b/g) and y=y0-t(a/g) with g=gcd(a,b), and this formula was proved to capture every solution, not just infinitely many of them. The harder necessity direction needed a new lemma, gcd(a/g,b/g)=1, derived by dividing Bezout's equation by g and applying the "gcd divides any combination" fact to force the divisor down to 1; that coprimality is what lets Euclid's Lemma pin down a/g dividing (y0-y) rather than just the whole product. Corollary 1.19 falls out of Proposition 1.18 as the g=1 special case. The theory was applied to constrained word problems (coins, stamps, hotdogs and buns, ATM notes, a fish-and-chips receipt) requiring inequality constraints on the free parameter t to enforce non-negativity. Exercise 14's printed general solution to akx+bky=c (gcd(a,b)=1, k dividing c) was flagged and corrected: the book states step sizes b/k and a/k, which are demonstrably non-integer in general (verified against Exercise 1(b)'s own numbers), whereas the correct result, x=x0+bt and y=y0-at, follows once gcd(ak,bk)=k is shown via mutual divisibility. The capstone result, Supplementary Problem 1.23 (a^n divides b^n implies a divides b), was proved without prime factorization — which this repo has not yet covered as a proven theorem — instead using the gcd-decomposition trick a=d*a', b=d*b' with gcd(a',b')=1, combined with Section 1.3's Exercise 15(iii) fact that gcd(a,b)=1 implies gcd(a^n,b^n)=1: this reduces the problem to a'^n dividing b'^n while being coprime to it, forcing a'^n=1 and hence a=gcd(a,b), which divides b by definition. Problem 1.24 closed the section by combining Section 1.3's Exercise 12(ii) multiplicativity lemma with Euclid's Lemma to prove pairwise-coprime divisibility propagation across n factors.

**Depends on:** Extended Euclidean GCD, Modular Inverse (Bezout machinery), Number Theory Step by Step Section 1.3 (Euclid's Lemma, mutual divisibility, the Exercise 12(ii) multiplicativity lemma, and the Exercise 15(iii) gcd(a^n,b^n)=1 fact — all reused directly)
**Unlocks:** Fundamental Theorem of Arithmetic / unique prime factorization — flagged as a genuine gap during Supplementary Problem 1.23, worked around here but a natural next target

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
