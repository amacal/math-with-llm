# Pell's Equation via Continued Fractions

## Overview

Pell's equation asks for integer solutions to

$$x^2 - D \cdot y^2 = 1$$

for a fixed positive integer D that is not itself a perfect square, and the goal of this session is to find the fundamental solution — the smallest positive pair (x, y) satisfying it — by expanding sqrt(D) as a continued fraction and reading the answer off one of its convergents. This generalizes the D = 2 case explored in `gcd-euclidean-fraction.md`, where the convergents of sqrt(2) were shown, almost as a side observation, to satisfy

$$p_k^2 - 2 q_k^2 = \pm 1;$$

this session turns that side observation into a full method for arbitrary non-square D. The convergent recurrence from that file is reused directly here, without re-deriving it, since it was already established and validated there:

$$p_k = a_k \cdot p_{k-1} + p_{k-2} \qquad q_k = a_k \cdot q_{k-1} + q_{k-2} \qquad p_{-1} = 1,\ q_{-1} = 0,\ p_0 = a_0,\ q_0 = 1,$$

with the first pair giving the shared recurrence shape for both sequences and the seeds fixing where it starts. Because this session synthesizes several genuinely new pieces of theory — a general recurrence for expanding sqrt(D), a periodicity proof, and a parity-dependent rule for locating the solution — it runs toward the longer end of this repo's usual length for a single session.

## The general recurrence for expanding sqrt(D)

Unlike sqrt(2), whose continued fraction repeats after a single hand-rationalization step, a general sqrt(D) needs a systematic recurrence rather than redoing algebra by hand at every term. Writing each "tail" of the expansion in the canonical form

$$x_n = \frac{m_n + \sqrt{D}}{d_n}$$

the value x_n is called the complete quotient at step n, while a_n, the integer floor of x_n, is the partial quotient — the same vocabulary already used for the rational case in `gcd-euclidean-fraction.md`, just applied here to an irrational tail instead of a rational one. Starting from the seeds m_0 = 0, d_0 = 1, a_0 = floor(sqrt(D)), the next triple is produced by

$$m_{n+1} = d_n \cdot a_n - m_n \qquad d_{n+1} = \frac{D - m_{n+1}^2}{d_n} \qquad a_{n+1} = \left\lfloor \frac{m_{n+1} + \sqrt{D}}{d_{n+1}} \right\rfloor$$

derived by writing x_n minus a_n as a single fraction over d_n, and rationalizing the reciprocal of that leftover piece the same way sqrt(2) minus 1 was rationalized in the earlier session, but carried out symbolically instead of numerically so it works at every step. Computing a_n itself must avoid ever calling a floating-point square root, so this session reuses the bisection search from `mod-isqrt-bisect.md` but replaces its predicate with a more general one: a candidate integer a is accepted exactly when (a times d minus m) is at most 0, or its square is at most D. The first case follows because sqrt(D) is never negative, so a nonpositive left side is automatically at most sqrt(D); the second case requires squaring both sides of the inequality, which is only valid once both sides are confirmed nonnegative, so the two cases have to be checked in that order rather than squared blindly. Because a_0 itself is just this same predicate evaluated at m = 0, d = 1, the routine doubles as an isqrt implementation without any separate code path.

## Why the expansion is eventually periodic

The pair (m_n, d_n) cannot grow without bound: since d_n and d_(n+1) are both positive, their product, which equals D minus m_(n+1) squared, must be nonnegative, forcing m_(n+1) squared to be at most D and so |m_(n+1)| is at most sqrt(D). With both m and d confined to a finite range determined by D, the state space of possible (m, d) pairs is finite, while the sequence of states runs on forever; by the pigeonhole principle some pair must eventually repeat. Because the recurrence above depends only on the current (m, d) pair and nothing else — a genuinely one-step, deterministic function — a repeated pair forces every subsequent state to repeat too, giving periodicity forever rather than just a single coincidental recurrence. A further classical fact, cited here rather than proved from scratch, pins down exactly where the period starts for sqrt(D) specifically: the first complete quotient x_1 is always a "reduced" quadratic irrational (greater than 1, with its conjugate strictly between -1 and 0), and a theorem usually attributed to Galois says a quadratic irrational's continued fraction is purely periodic exactly when it is reduced. This guarantees the periodic block always begins at index 1 for every non-square D, which is why the cycle-detection code only ever needs to compare against the very first computed triple rather than searching a growing history.

## From convergents to a solution

Feeding the a_n sequence through the convergent recurrence produces p_k, q_k pairs whose Pell values p_k^2 minus D times q_k^2 were compared, for D = 7, against the d-sequence from the same expansion; the pattern found was

$$p_k^2 - D \cdot q_k^2 = (-1)^{k+1} \cdot d_{k+1}$$

and this pattern was then confirmed numerically against several other values of D, including 13 and 61, but it was never proved from first principles in this session, so it should be treated as an empirically strong conjecture rather than an established fact. A full proof would go through the exact-tail identity sqrt(D) equals (x_(k+1) times p_k plus p_(k-1)) over (x_(k+1) times q_k plus q_(k-1)), substituting x_(k+1) in its (m, d) form and clearing denominators, and is flagged here as genuinely open work for a future session. Taking the identity as given, d_(k+1) equals 1 exactly when k+1 is a multiple of the period length r, so at k = r - 1 the sign is (-1) raised to r: an even r gives plus 1 immediately, while an odd r gives minus 1, meaning the wrong equation was solved and a second lap through the period is required, landing on k = 2r - 1 instead, where the exponent 2r is always even regardless of the parity of r itself. A sharp observation made during the session is that k = 2r - 1 therefore works unconditionally for both parities, since 2r is always even; the parity branch is only there to save the cost of a second lap when r happens to be even already.

## Correctness

Correctness rests on three layers stacked together: the (m, d, a) recurrence genuinely reconstructs the continued fraction expansion of sqrt(D), because it was derived by the same rationalization technique already validated for sqrt(2), just carried out symbolically; the expansion is genuinely periodic, proved by the pigeonhole-plus-determinism argument above rather than merely observed; and the fundamental solution genuinely sits at k = r - 1 or k = 2r - 1 depending on the parity of r, granting the empirically-discovered-but-unproved Pell identity stated above. That last grant, together with the unproved classical bound on d_n needed for a tight complexity bound below, are the two explicitly acknowledged gaps in this session's rigor, left open rather than papered over.

## Complexity

In terms of the period length r, `pqa` costs O(r log D): the (m, d, a) recurrence runs for r iterations, each performing one O(log D) bisection search for the partial quotient, and the convergent recurrence adds another O(r) steps of constant-looking arithmetic — constant only if the exponential growth of p_k and q_k themselves is ignored, since these numerators and denominators grow Fibonacci-style because every a_k is at least 1, and were only tested this session up to values still comfortably inside u64 (the D = 61 case reaches into the billions but no further). The period r itself is guaranteed finite by the pigeonhole argument above, but this session did not derive a tight closed-form bound on r as a function of D, since that requires bounding d_n the way m_n was bounded, and d_n's bound (classically 0 < d_n <= 2 times sqrt(D)) was only observed empirically across the test cases rather than proved. The literature result, citable but not derived here, is r = O(sqrt(D) times log D).

## Edge cases

When D is itself a perfect square, x^2 - D y^2 factors over the integers as (x minus sqrt(D) times y) times (x plus sqrt(D) times y) equals 1, and since two integers multiplying to 1 must both equal 1 or both equal -1, the only solution is the trivial (1, 0), never a nontrivial one. In the implementation this shows up as d_next hitting exactly 0 the moment a_0 squared equals D, and left unguarded this produced a real panic one step later: an unsigned-subtraction underflow while computing m_next from a degenerate triple with d equal to 0, rather than the division-by-zero one might expect. The fix checks isqrt(D) squared against D upfront and returns None immediately, before the degenerate triple can ever be produced.

## Worked example

Trace D = 7 by hand. The seed is a_0 = floor(sqrt(7)) = 2, since 2 squared is 4 and 3 squared is 9. The recurrence then produces (m, d, a) equal to (2, 3, 1), then (1, 2, 1), then (1, 3, 1), then (2, 1, 4), at which point the next (m, d) pair repeats (2, 3), the very first one computed after the seed, so the period is r = 4 and the expansion is sqrt(7) = [2; overline(1, 1, 1, 4)]. Feeding a_0 through a_3 into the convergent recurrence gives p_0/q_0 = 2/1, p_1/q_1 = 3/1, p_2/q_2 = 5/2, and p_3/q_3 = 8/3; checking the last of these, 8 squared minus 7 times 3 squared equals 64 minus 63, which is exactly 1. Since r = 4 is even, k = r - 1 = 3 already lands on the fundamental solution directly, with no need for a second lap: (x, y) = (8, 3). For contrast, D = 13 has an odd period of length 5, so its fundamental solution only appears after a second lap, at k = 2 times 5 minus 1 = 9, landing on the considerably larger pair (649, 180), which checks out since 649 squared minus 13 times 180 squared equals 421201 minus 421200, again exactly 1.
