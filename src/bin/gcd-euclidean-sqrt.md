# Continued Fractions

## Overview

A continued fraction rewrites a number as a nested tower of integer parts and reciprocals: pull out the integer part of a fraction, flip whatever fractional piece is left over, and repeat the same extraction on that flipped value. For 17/7 this produces 17/7 = 2 + 1/(2 + 1/3), a three-term expression built entirely from the integers 2, 2, 3. The headline discovery of this session is that, for a ratio of two integers a and b, this sequence of integers is not merely similar to the sequence of quotients produced by running the Euclidean algorithm on (a, b) — it is the exact same sequence, because both processes repeatedly apply the identical division equation. A second idea, the convergents, gives a fast way to reconstruct the fraction at every truncation depth using a two-term linear recurrence, avoiding the need to re-simplify a deeply nested fraction from scratch each time a term is added. The third idea applies both of the first two to sqrt(2), whose continued fraction repeats forever, and whose convergents satisfy an exact integer identity tied to Pell's equation. Because this file weaves together three results rather than one, it runs toward the upper end of this repo's usual length for a single session.

## Continued fractions are Euclidean quotients

The extraction step at the heart of a continued fraction expansion is, written as an equation, exactly a division with remainder: pulling out the integer part q of a ratio of two integers and calling the fractional remainder r over the original denominator gives

$$\frac{a}{b} = q + \frac{r}{b} \qquad \text{equivalently} \qquad a = b \cdot q + r$$

This is the same division equation used by the Euclidean algorithm on the pair (a, b), where q is the quotient and r is the remainder, so the very first term of the continued fraction and the very first quotient of the Euclidean algorithm are the same number, produced by the same equation — a fact that carries the whole argument, since everything that follows is just this observation applied one level deeper.

The argument continues by induction on the remaining terms. Suppose that at some stage the continued fraction process is currently expanding the fraction built from two consecutive Euclidean remainders,

$$\frac{r_{k-1}}{r_k}$$

Extracting a term from that fraction applies the identical division equation to that pair, since r_(k-1) and r_k play the same role that a and b played at the start:

$$r_{k-1} = r_k \cdot a_k + r_{k+1}$$

Because both processes divide the exact same pair of numbers at every stage, the term a_k they produce is forced to be the same integer in both, at every step, not just the first. The two processes also stop together: once the Euclidean algorithm reaches a remainder of zero, the equation

$$r_{n-1} = r_n \cdot a_n + 0$$

shows that r_(n-1) over r_n is a whole number with no fractional piece left to flip, so the continued fraction expansion terminates at exactly the index where the Euclidean algorithm does. This is also why every rational number has a finite continued fraction expansion: the Euclidean algorithm on any pair of integers always terminates, by Lame's theorem (already established in `gcd-euclidean-basic.md`), and so does the continued fraction built from the same steps.

## Convergents and the recurrence

Truncating a continued fraction after k+1 terms gives a fraction called the k-th convergent:

$$\text{the } k\text{-th convergent} = \frac{p_k}{q_k}$$

Recomputing this fraction from scratch for every truncation depth means re-simplifying an increasingly nested expression, which is wasteful; instead, both p_k and q_k satisfy a two-term linear recurrence driven only by the newest term a_k, seeded by two base pairs:

$$p_k = a_k \cdot p_{k-1} + p_{k-2} \qquad q_k = a_k \cdot q_{k-1} + q_{k-2}$$

$$p_{-1} = 1 \qquad q_{-1} = 0 \qquad p_0 = a_0 \qquad q_0 = 1$$

This design mirrors the way `gcd-euclidean-extended.rs` threads Bezout coefficients through the same division loop the basic algorithm already runs, instead of making a separate pass over a recorded list — here the running pairs (p_(k-1), p_k) and (q_(k-1), q_k) are updated once per term instead.

The recurrence follows from the fact that extending a truncated continued fraction by one more term is the same as replacing its current last term, a_(k-1), with a_(k-1) + 1/a_k, since that reciprocal is exactly what the next level of nesting inserts. Writing the value of a truncation ending in a placeholder last term x as the recurrence one step back, treating the final term as a variable,

$$N(x) = x \cdot p_{k-2} + p_{k-3}$$

and substituting x = a_(k-1) + 1/a_k gives:

$$N\!\left(a_{k-1} + \frac{1}{a_k}\right) = a_{k-1} \cdot p_{k-2} + p_{k-3} + \frac{p_{k-2}}{a_k} = p_{k-1} + \frac{p_{k-2}}{a_k}$$

The first two terms collapse to p_(k-1) because that combination is exactly the recurrence definition of p_(k-1) itself, one level back. Multiplying the whole expression by a_k clears the remaining fraction, giving

$$a_k \cdot p_{k-1} + p_{k-2}$$

which is exactly p_k by definition. The same substitution applies unchanged to the q sequence, so both recurrences are justified by the same piece of algebra.

## Approximating sqrt(2) and Pell's equation

Applying the extraction process to an irrational number produces an infinite continued fraction, since a finite one would always evaluate to a ratio of two integers (built up from the innermost term outward using only addition and reciprocals, both of which preserve "ratio of two integers"), contradicting irrationality. For sqrt(2), the first term is 1, since sqrt(2) lies between 1 and 2, leaving sqrt(2) - 1 to flip. Rationalizing 1/(sqrt(2)-1) by multiplying by (sqrt(2)+1)/(sqrt(2)+1) collapses the denominator by a difference of squares,

$$(\sqrt{2}-1)(\sqrt{2}+1) = (\sqrt{2})^2 - 1^2 = 1$$

so the flipped value simplifies cleanly:

$$\frac{1}{\sqrt{2}-1} = \sqrt{2} + 1$$

Extracting a term from sqrt(2) + 1 gives 2, since sqrt(2) + 1 is about 2.414, leaving a leftover of

$$(\sqrt{2}+1) - 2 = \sqrt{2} - 1$$

which is the identical expression the process started flipping after the very first step. Because the state has returned to something already seen, every subsequent term repeats forever, giving the full expansion:

$$\sqrt{2} = [1;\ 2,\ 2,\ 2,\ 2,\ \ldots]$$

Feeding that truncated term sequence through the same convergent recurrence produces rational approximations to sqrt(2) whose quality can be verified exactly, with no floating point at all. Computing the first few convergents by hand shows p_k^2 - 2 q_k^2 alternating between -1 and 1:

$$1^2 - 2 \cdot 1^2 = -1 \qquad 3^2 - 2 \cdot 2^2 = 1 \qquad 7^2 - 2 \cdot 5^2 = -1$$

Dividing p_k^2 - 2 q_k^2 = ±1 through by q_k^2 gives an exact statement of approximation quality,

$$\left(\frac{p_k}{q_k}\right)^2 - 2 = \pm\frac{1}{q_k^2}$$

so as q_k grows without bound the squared convergent closes in on 2 at a rate governed by q_k^2 rather than q_k, a tighter guarantee than a typical rational approximation of the same denominator size would offer. This exact identity is a case of Pell's equation,

$$x^2 - 2y^2 = \pm 1$$

and it gives a test oracle with zero rounding error: every convergent of sqrt(2) must satisfy it exactly.

## Correctness

The correctness of `convergents` rests on the recurrence derivation above: each successive (p_k, q_k) pair is produced from only the two previous pairs and the current term, and this was shown to reproduce the value of the correspondingly-truncated nested fraction exactly, not approximately. The correctness of feeding sqrt(2)'s hand-derived term sequence into that same function rests on the Pell identity: a computed convergent is correct exactly when it satisfies

$$p_k^2 - 2 q_k^2 = \pm 1$$

alternating in sign as k increases from -1, an exact integer check rather than a numerical approximation.

This file implements the convergent recurrence and the sqrt(2) approximation, but not a standalone function deriving a rational number's continued fraction terms from the Euclidean algorithm; the rational-number tests instead supply hand-computed term lists directly to `convergents`. The quotient/term equivalence was established as theory above but is a natural next piece of code to add.

## Complexity

`convergents` performs one pass over a term slice of length n, doing a constant amount of arithmetic per entry, so its cost is O(n) in the number of terms, independent of where those terms came from. `sqrt2(n)` builds a length-n term vector directly (no division loop, since the term pattern was derived in closed form) and then calls `convergents`, so it is also O(n). Deriving the term sequence of a rational a/b via the Euclidean algorithm, as reasoned about above but not yet implemented here, would cost O(log(min(a,b))) by the same Lame's-theorem argument used in `gcd-euclidean-basic.md`. Three distinct results therefore cover the three pieces of this session:

$$\text{convergents}: O(n) \qquad \text{sqrt2}(n): O(n) \qquad \text{continued fraction terms of } a/b: O(\log(\min(a,b)))$$

## Edge cases

`convergents` returns `None` on an empty terms slice, since there is no leading term to seed p_0, q_0, and `sqrt2` returns `None` for n = 0 for the same reason. The numerators and denominators for sqrt(2)'s convergents grow roughly like 2.414 raised to the term index, since a_k = 2 for every k beyond the first; at n = 50 this stays under u64::MAX, but the growth rate means a moderately larger n would silently wrap in a release build or panic in a debug build, since `convergents` multiplies directly in u64 with no overflow check. The Pell-identity test sidesteps this only at the comparison itself, by casting p and q to i128 before squaring — the u64 pairs `convergents` returns would already have wrapped if n were pushed far enough beyond 50.

## Worked example

Trace `convergents` on the term list for 17/7, namely [2, 2, 3], obtained by hand-running the Euclidean algorithm as 17 = 2·7+3, 7 = 2·3+1, 3 = 3·1+0. The seed pairs are p_(-1)=1, q_(-1)=0 and p_0=2, q_0=1, matching a_0 = 2 over 1. The first loop iteration uses a_1 = 2: p_1 = 2·2 + 1 = 5 and q_1 = 2·1 + 0 = 2, so the second convergent is 5/2. The second iteration uses a_2 = 3: p_2 = 3·5 + 2 = 17 and q_2 = 3·2 + 1 = 7, so the third and final convergent is 17/7 — exactly the original fraction, confirming the round-trip. Along the way the three convergents 2/1, 5/2, and 17/7 are respectively about 2, 2.5, and 2.4286, oscillating around the true value 17/7 and getting closer at every step, which is the same alternating-and-shrinking behavior later verified exactly, via the Pell identity, for the convergents of sqrt(2).
