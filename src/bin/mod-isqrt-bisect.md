# Integer Square Root

## Overview

The goal is to compute the integer square root of a nonnegative integer n using only integer arithmetic, with no call to a floating-point square root function anywhere in the algorithm. The quantity being computed is the largest integer whose square does not exceed n, which is written

$$\text{isqrt}(n) = \max\{\, r \in \mathbb{Z}_{\ge 0} : r^2 \le n \,\}$$

and this is the same thing as floor(sqrt(n)), just phrased without reference to the real-valued square root at all. The motivation comes from a bug already on record in this repository: the segmented sieve, documented in `sieve-segmented.md`, computes a square-root limit for its precomputation step, and a naive floor of a floating-point square root could silently drop a prime whenever rounding error pushed the true value just under an integer boundary. An exact integer isqrt removes that failure mode entirely, since every comparison stays between integers and never touches a float.

## Choosing a search bracket

Before any search can begin, two integers lo and hi are needed that bracket the answer, with lo no larger than isqrt(n) and hi no smaller. An early instinct was to use n/2 as an upper bound, reasoning that halving n should overshoot the square root comfortably. That bound fails outright for small n: at n = 1, isqrt(1) = 1, but 1/2 truncates to 0, which sits below the true answer, and the same failure recurs at n = 2 and n = 3. Chasing a tighter, always-correct bound turns out to be unnecessary. The crudest possible bracket, lo = 0 and hi = n, is trivially correct for every n, since 0 squared is never more than n and n itself is never less than its own square root once n is at least 1. The only apparent cost of this loose bound is a wider search range, and that cost is only a constant factor, not a change in complexity class: if a tighter bound near sqrt(n) were used instead of n, the bisection step count would shrink from being proportional to log(n) to being proportional to log(sqrt(n)), and

$$\log\!\big(\sqrt{n}\big) = \tfrac{1}{2}\log(n)$$

which shows the tighter bound only halves the constant in front of the log, leaving the bisection search at O(log n) either way. That settles the question in favor of the simplest possible bracket.

## The bisection loop and its two stalls

With lo = 0 and hi = n fixed as the starting bracket, the natural next step is ordinary bisection: repeatedly pick a midpoint, test it, and shrink whichever side of the bracket the midpoint disqualifies. Working this out by hand on a small case, n = 2, exposed two genuine off-by-one stalls that a careless implementation would fall into. The first arises from using a floor-rounded midpoint, mid = (lo + hi) / 2 with integer division, together with the update rule that sets lo to mid whenever mid squared does not exceed n. Once the bracket narrows to hi = lo + 1, floor division sends mid back to lo every time, so the "did not overshoot" branch reassigns lo to the value it already had, and the loop never terminates. The second stall is the mirror image on the other side: if the overshooting branch is implemented as hi = mid, and mid happens to already equal hi, that branch also leaves the bracket completely unchanged.

Both stalls are fixed by two decisions working together. The overshoot branch must discard mid entirely rather than merely capping hi at it, since a value whose square exceeds n can never be isqrt(n) itself; the assignment is hi = mid - 1, and because mid is always at most hi, this always drops hi by at least one from its previous value. The other branch is fixed by rounding the midpoint up rather than down,

$$\text{mid} = \text{lo} + \left\lceil \frac{\text{hi} - \text{lo}}{2} \right\rceil$$

which for hi = lo + 1 sends mid to hi itself rather than back to lo, so the loop is guaranteed to make progress on that side too. With both fixes in place, the two update rules together become

$$\text{lo} = \text{mid} \quad (\text{mid}^2 \le n) \qquad\qquad \text{hi} = \text{mid} - 1 \quad (\text{mid}^2 > n)$$

which is the complete branch logic executed on every iteration of the loop, and one pleasant consequence of the ceiling-rounded midpoint is that no special final check is needed after the loop ends: an earlier draft of the design looped only while hi minus lo was at least two and then inspected hi by hand afterward, but the ceiling rounding absorbs that last comparison directly into the loop's own machinery, since it forces mid to land exactly on hi whenever the bracket has narrowed to two adjacent integers.

## Correctness

The bracket invariant maintained throughout the loop is lo <= isqrt(n) <= hi, and one half of that invariant is strictly stronger than it looks: lo is only ever reassigned inside the branch that has already verified mid squared <= n, so lo squared <= n holds at literally every point in the algorithm's execution, from the initial lo = 0 onward, not merely at the end. The value hi carries no such guarantee, since it is only ever known to be an upper bound on the answer, not a validated candidate itself. The loop condition is simply lo < hi, so it terminates precisely when lo equals hi, and at that moment the invariant lo <= isqrt(n) <= hi collapses to a single value, forcing isqrt(n) to equal that common value exactly. There is no ambiguity left to resolve, since the sandwich has closed to a point.

## Complexity

Every iteration strictly shrinks the bracket: on the non-overshoot branch lo rises to mid, and the ceiling-rounded midpoint guarantees mid is strictly greater than the old lo whenever hi exceeds lo; on the overshoot branch hi drops to mid minus one, strictly below the old hi since mid never exceeds hi. Because the bracket width is roughly halved by each choice of mid, the number of iterations needed to shrink an initial width of n down to zero is O(log n), with each iteration doing constant work, giving O(log n) overall — a sharp complexity-class improvement over linearly scanning every candidate from 0 upward, which costs O(n).

## Edge cases

At n = 0, the initial bracket is already lo = hi = 0, so the loop never runs and 0 is returned immediately; n = 1 is handled just as directly, since lo = 0 and hi = 1 collapse to lo = hi = 1 after one iteration. A subtler hazard is that mid can be as large as n itself during early iterations, so mid squared can overflow a 64-bit unsigned integer long before mid approaches the true answer; guarding the multiplication with a checked multiplication and treating overflow as an automatic overshoot is sound, since n itself is bounded by the same 64-bit range, so any product that overflows that range is certainly larger than n regardless of its exact unrepresentable value. A narrower hazard appears only when n equals the maximum representable value itself, since then hi minus lo equals that same maximum on the first iteration, and adding one before halving would itself overflow; clamping that sum instead of overflowing introduces an off-by-one of exactly one in that single midpoint, but the resulting midpoint is still astronomically larger than any possible isqrt(n), so it is discarded by the overshoot branch regardless, leaving the final answer unaffected.

## Worked example

Trace isqrt(17) by hand. The bracket starts at lo = 0 and hi = 17. The first midpoint is mid = 0 + ceil(17/2) = 9, and 9 squared is 81, which exceeds 17, so this is the overshoot branch and hi drops to 8, leaving the bracket at lo = 0, hi = 8. The next midpoint is mid = 0 + ceil(8/2) = 4, and 4 squared is 16, which does not exceed 17, so this is the non-overshoot branch and lo rises to 4, leaving the bracket at lo = 4, hi = 8. The next midpoint is mid = 4 + ceil(4/2) = 6, and 6 squared is 36, which exceeds 17, so hi drops to 5, leaving the bracket at lo = 4, hi = 5 — exactly the narrow case that the ceiling-rounded midpoint was designed to handle cleanly. The next midpoint is mid = 4 + ceil(1/2) = 5, landing exactly on hi as expected, and 5 squared is 25, which exceeds 17, so hi drops to mid minus one, which is 4, leaving the bracket at lo = 4, hi = 4. The loop condition lo < hi is now false, so the loop exits and returns lo, which is 4. This matches the true answer directly, since 4 squared is 16 and does not exceed 17, while 5 squared is 25 and does.
