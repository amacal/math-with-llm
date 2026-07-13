# Wiener's Attack on RSA

## Overview

Wiener's attack breaks RSA outright — recovering the private exponent d and factoring n — whenever d is chosen too small, using only the public key (n, e). The intuition is that RSA key generation hides an equation relating e, d, and the secret totient phi(n), and that equation forces e/n to be an extremely good rational approximation of some small fraction m/d whenever d is small. "Extremely good" turns out to be good enough to invoke a classical fact about continued fractions: a sufficiently good rational approximation to a number is not merely close to it, it is forced to appear as one of that number's continued-fraction convergents. Since the convergents of e/n are a short, explicitly computable list, this hands an attacker a small set of candidates to test, one of which — if d was small enough — is guaranteed to unlock the factorization of n. This session is a synthesis of several previously built pieces (RSA itself, the continued-fraction machinery, and integer square roots) plus one genuinely new piece of theory, so it runs longer than a typical single-concept session.

## The hidden approximation

RSA key generation fixes d as e's inverse modulo phi(n), a congruence already proved and used in `mod-rsa.md`: d times e is congruent to 1 modulo phi(n). Clearing that congruence into a plain equation, with m standing for the unknown integer multiple of phi(n) (chosen distinct from n, e, and d to avoid collision with the RSA modulus), gives

$$e \cdot d = 1 + m \cdot \varphi(n)$$

This equation is exact, but it involves phi(n), which the attacker does not know. The next fact fixes that: phi(n) is very close to n. Recall that phi(n) factors as the product (p-1)(q-1); expanding that product gives

$$\varphi(n) = (p-1)(q-1) = n - p - q + 1$$

and subtracting phi(n) from n leaves

$$n - \varphi(n) = p + q - 1$$

so the gap between phi(n) and n is exactly p+q-1. For balanced RSA primes, p and q are both roughly the square root of n, so p+q is roughly the square root of n as well — a quantity with about half as many digits as n itself. As n grows, this gap becomes vanishingly small relative to n, which is what lets an approximation involving n stand in for one involving phi(n).

Dividing the key equation by d*phi(n) term by term gives an exact identity relating e/phi(n) to m/d:

$$\frac{e}{\varphi(n)} - \frac{m}{d} = \frac{1}{d \cdot \varphi(n)}$$

Both d and phi(n) are large, so the right-hand side is tiny: m/d is an excellent approximation to e/phi(n). But the attacker needs a statement about e/n, not e/phi(n). Inserting and subtracting e/phi(n) turns e/n − m/d into a sum of two pieces, one of which is the identity just derived:

$$\frac{e}{n} - \frac{m}{d} = \left(\frac{e}{n} - \frac{e}{\varphi(n)}\right) + \left(\frac{e}{\varphi(n)} - \frac{m}{d}\right)$$

The first piece factors as e times the difference of reciprocals of n and phi(n), and substituting phi(n) − n = 1 − p − q gives

$$\frac{e}{n} - \frac{e}{\varphi(n)} = e \cdot \frac{\varphi(n) - n}{n \cdot \varphi(n)} = \frac{e \cdot (1 - p - q)}{n \cdot \varphi(n)}$$

for that piece. Since e is always less than phi(n), this piece is bounded in size by roughly (p+q)/n, which — since p+q is on the order of the square root of n — behaves like 1 over the square root of n:

$$\frac{e \cdot (1-p-q)}{n \cdot \varphi(n)} = O\!\left(\frac{p+q}{n}\right) = O\!\left(\frac{1}{\sqrt{n}}\right)$$

The second piece, 1/(d*phi(n)), is smaller still for any reasonably sized d. So overall,

$$\left|\frac{e}{n} - \frac{m}{d}\right| = O\!\left(\frac{1}{\sqrt{n}}\right)$$

with the (p+q)/n term dominating.

## Legendre's theorem and the size bound on d

The piece of theory that makes this approximation useful, rather than just a curiosity, is Legendre's theorem on best rational approximations: if a fraction a/b in lowest terms satisfies

$$\left|x - \frac{a}{b}\right| < \frac{1}{2b^2}$$

for some real number x, then a/b is guaranteed to be one of the convergents of x's continued fraction expansion — not merely close to x, but forced to appear on that specific, short list. This was checked concretely in this session against the convergents of sqrt(2) already computed in `gcd-euclidean-sqrt.md`. For the genuine convergent 17/12, the approximation error and the Legendre threshold are

$$\left|\sqrt{2} - \frac{17}{12}\right| \approx 0.002453 \quad < \quad \frac{1}{2 \cdot 12^2} \approx 0.003472$$

so the bound holds, as it must for an actual convergent. For 10/7, which looks close to sqrt(2) but is not a convergent, the same comparison gives

$$\left|\sqrt{2} - \frac{10}{7}\right| \approx 0.014358 \quad > \quad \frac{1}{2 \cdot 7^2} \approx 0.010204$$

so the bound fails — consistent with 10/7 never appearing on the convergent list.

Applying Legendre's theorem with b = d, the attack succeeds once |e/n − m/d| < 1/(2d²). Combining this with the O(1/sqrt(n)) bound derived above and solving for d gives the session's bound:

$$d < \frac{n^{1/4}}{\sqrt{2}}$$

The published literature states a more conservative bound, obtained by tracking the constants in p+q more carefully rather than dropping them:

$$d < \frac{n^{1/4}}{3}$$

both bounds agree on the essential shape, that d must be smaller than roughly the fourth root of n. Either way, the conclusion is structural, not a matter of search effort: if d exceeds this bound, there is no guarantee the true m/d is among the convergents of e/n at all, so no amount of searching that particular list would find it.

## Recovering phi(n), p, and q

Given a candidate convergent m/d from the list, the key equation rearranges directly to a candidate value for phi(n):

$$\varphi(n)_{\text{candidate}} = \frac{e \cdot d - 1}{m}$$

valid only when m divides e*d − 1 exactly (phi(n) must be a positive integer). From there, p + q and p*q are both known — p*q = n directly, and p + q = n + 1 − phi(n) from the earlier expansion — so p and q are the two roots of the quadratic whose sum and product are these values, by Vieta's formulas:

$$t^2 - (n + 1 - \varphi(n)_{\text{candidate}}) \cdot t + n = 0$$

Solving via the quadratic formula requires the discriminant (n + 1 − phi(n)_candidate)² − 4n to be a non-negative perfect square, checked here using the bisection-based `isqrt` from `mod-isqrt-bisect.md`. If it is, the two roots t1 and t2 are computed and checked against t1*t2 = n as a final confirmation.

## Correctness

The correctness argument has two halves. The first half is existence: when d is below the derived bound, Legendre's theorem guarantees the true (m, d) pair is one of the O(log n) convergents of e/n, so testing every convergent in that list is guaranteed to reach it. The second half is that reaching it is unambiguous, not a coincidence: since n = p*q with p and q both prime, n has essentially one nontrivial factorization. Any candidate that survives all three checks — the divisibility of e*d − 1 by m, the discriminant being a non-negative perfect square, and t1*t2 equalling n exactly — has produced two positive integers multiplying to exactly n, which forces them to be the true p and q. There is no room for a spurious candidate to pass all three checks by accident. Conversely, when d exceeds the bound, Legendre's theorem simply gives no guarantee that the true pair is on the list; this is a structural absence, not a matter of insufficient search, which is exactly what a large-d test case (d = 200000 against this session's n, far above its bound) demonstrates by returning no candidate at all.

## Complexity

The number of convergents to test is O(log n), the same bound established for the Euclidean algorithm in `gcd-euclidean-fraction.md`, since the continued-fraction terms of e/n come from that identical process. Each candidate costs a constant number of multiplications and divisions plus one call to `isqrt`, whose bisection search costs O(log n) as established in `mod-isqrt-bisect.md`. Multiplying candidate count by per-candidate cost gives an overall O(log² n) operation count for the search. This treats each multiplication and division as a single O(1) machine operation, which holds only because every value here fits inside fixed-width u64/u128 words; for n large enough to require arbitrary-precision arithmetic, each of those operations would itself cost more, proportional to the bit-length of the operands, making the true bit-complexity somewhat higher than the O(log² n) operation count alone suggests.

## Edge cases

Two real overflow bugs surfaced during implementation. First, the existing extended-GCD routine tracks its Bezout coefficients as i64, which is sufficient only up to inputs around 2^63; once p and q were chosen near u32::MAX, phi(n) approached u64::MAX, and a Bezout coefficient needed magnitude close to phi(n) itself, exceeding what i64 can represent and triggering a multiplication overflow panic. This was resolved not by widening the coefficient type to i128, but by choosing smaller primes (near 2^30 rather than 2^32), keeping phi(n), and therefore the coefficients, safely within i64's range — a deliberate scope decision rather than a general fix. Second, computing the discriminant's p_plus_q squared in u64 could itself overflow before the subtraction of 4n ever happened, since squaring roughly doubles the bit-length; this was fixed by widening that specific multiplication to u128, mirroring the pattern already used in `mod_exp`. A companion guard rejects any candidate whose phi(n) candidate is at least n + 1 before ever subtracting, justified by the fact that phi(n) < n holds for every genuine n, so such a candidate could never be genuine and can be discarded outright rather than risking an underflow. A third, non-arithmetic bug also surfaced: an early version called the continued-fraction generator on the private d instead of the public n, which "solved" the problem by cheating — using exactly the secret information a real attacker would not have — and was caught by noticing this could never correspond to a genuine attack.

## Worked example

Take p = 97 and q = 101, both prime, giving

$$n = 97 \cdot 101 = 9797, \qquad \varphi(n) = 96 \cdot 100 = 9600$$

Choose the private exponent d = 7 directly; since 9600 factors as 2^7·3·5², it shares no factor of 7, so gcd(7, 9600) = 1. Computing e as 7's inverse mod 9600 via the extended Euclidean algorithm gives

$$e = 2743, \qquad \text{check: } 2743 \cdot 7 = 19201 = 2 \cdot 9600 + 1$$

This n is close to the edge of the bound derived above:

$$n^{1/4} \approx 9.95, \qquad \frac{n^{1/4}}{\sqrt{2}} \approx 7.04$$

so d = 7 only barely qualifies under the looser bound, making this a genuinely tight, non-trivial instance rather than a comfortable one.

Expanding e/n = 2743/9797 via the Euclidean algorithm gives the continued-fraction terms

$$[0,\ 3,\ 1,\ 1,\ 2,\ 1,\ 97,\ 4]$$

(the leading 0 appears because e is smaller than n, exactly the asymmetry noted in `gcd-euclidean-fraction.md`). Building convergents from these terms via the p_k = a_k·p_(k-1) + p_(k-2) recurrence gives, in order,

$$\frac{0}{1},\ \frac{1}{3},\ \frac{1}{4},\ \frac{2}{7},\ \frac{5}{18},\ \frac{7}{25},\ \frac{684}{2443},\ \frac{2743}{9797}$$

The fourth convergent, 2/7, is exactly the hidden (m, d) pair: m = 2, d = 7. Checking the divisibility condition,

$$e \cdot d - 1 = 2743 \cdot 7 - 1 = 19200 = 2 \cdot 9600$$

confirms 19200 divides evenly by m = 2, giving the candidate value phi(n)_candidate = 9600.

From that candidate, the sum of the roots is

$$p + q = n + 1 - \varphi(n)_{\text{candidate}} = 9798 - 9600 = 198$$

and the discriminant is

$$198^2 - 4 \cdot 9797 = 39204 - 39188 = 16$$

a perfect square with isqrt(16) = 4. The two roots are then

$$t_1 = \frac{198+4}{2} = 101, \qquad t_2 = \frac{198-4}{2} = 97$$

and 101·97 = 9797 = n exactly, confirming the recovered factors are the true p and q — recovered entirely from the public pair (9797, 2743), with the private exponent 7 never used as an input to the attack itself.
