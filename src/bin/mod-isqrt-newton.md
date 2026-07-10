# Newton's Method for Integer Square Root

## Overview

This session revisits the exact same target as `mod-isqrt-bisect.md` — the integer square root of a nonnegative integer n, the largest integer r with r squared at most n — but reaches it through a completely different mechanism. Bisection brackets the answer between lo and hi and halves that bracket every step, using only a comparison's sign to decide which half survives. Newton's method instead stands at a single current guess, looks at how steeply the relevant curve is rising there, and follows that local direction in a straight line down to where it crosses zero, using the crossing point as the next, sharper guess. The curve in question is the graph of

$$f(x) = x^2 - n$$

since any x with f(x) = 0 is a square root of n, and restricting to nonnegative integers recovers isqrt(n), the definition already fixed in `mod-isqrt-bisect.md`. Because the tangent line encodes the curve's actual local shape rather than a coarse too-high-or-too-low signal, it closes in on the root in far fewer steps than bisection once it is already close — though, as the complexity section shows, that advantage is subtler than it looks once the search starts far from the root, which is exactly the situation at the very first step.

## Deriving the iteration

The slope of f at a point x0 comes from the elementary fact that the derivative of x^2 is 2x, so f'(x0) = 2*x0. Concretely, at n = 10 and x0 = 3, f(3) = -1 and the slope is 6, so the tangent line through (3, -1) with that slope is y - (-1) = 6(x - 3). Setting y = 0 and solving gives the next guess: 1 = 6x - 18, so x1 = 19/6, already closer to the true square root of 10 than x0 was. Repeating the same construction symbolically, with f(x0) = x0^2 - n and slope 2*x0, the tangent line's x-intercept solves

$$-(x_0^2 - n) = 2x_0(x - x_0)$$

for x, and collecting terms gives x1 = (x0^2 + n)/(2*x0). Splitting that fraction reveals a second, more memorable form of the same formula,

$$x_1 = \frac{x_0^2 + n}{2x_0} = \frac{x_0}{2} + \frac{n}{2x_0} = \frac{x_0 + n/x_0}{2}$$

which says the next guess is the average of the current guess (too high) and n divided by it (correspondingly too low), pulled toward each other.

## Quadratic convergence

Let s stand for sqrt(n) and let e0 = x0 - s be the error in the current guess. Substituting n = s^2 into x1 - s and combining over the denominator 2*x0 gives a numerator of x0^2 + s^2 - 2*s*x0, a perfect square, so

$$x_1 - s = \frac{(x_0 - s)^2}{2x_0}$$

that is, writing e1 for x1 - s, e1 = e0^2 / (2*x0). This is qualitatively different from bisection's error behavior, where each step multiplies the error by a fixed constant c, following

$$e_{k+1} = c \cdot e_k$$

geometric decay, since the ratio between consecutive errors stays the same no matter how small the error has already become. Here the ratio e1/e0 equals e0/(2*x0), shrinking as e0 shrinks, so improvement accelerates every step. Concretely, with e0 = 0.01 and x0 near 3.16, geometric decay with c = 0.5 would give an error of 0.005 at the next step, but e1 = e0^2/(2*x0) gives about 0.0000158 instead — several orders of magnitude smaller. This self-reinforcing squaring is quadratic convergence: correct digits roughly double each iteration once the guess is close, rather than growing one bit at a time as bisection does.

## Correctness

The analysis above assumed exact division, but this repository avoids floating point entirely, so the implementation truncates both n/x0 and the final division by two. Hand-tracing n = 8 from x0 = 8 with truncation gives the sequence 8, 4, 3, 2, and the next computed value jumps back up to 3 — the iteration never settles at a fixed point once truncation enters, it cycles between 2 and 3 instead. This rules out the natural invariant "x0 stays at or above sqrt(n)," since the last useful value, x0 = 2, is already below sqrt(8) ~= 2.828; the invariant has to be phrased in terms of the integer target itself, written as

$$x_0 \ge r, \qquad r = \text{isqrt}(n)$$

which says the current guess never drops below the true integer answer, even though it can drop below the real-valued sqrt(n). The reason traces back to the AM-GM inequality, a classical fact about any two nonnegative reals a and b,

$$\frac{a+b}{2} \ge \sqrt{ab}$$

which follows from expanding the nonnegative quantity (sqrt(a) - sqrt(b))^2. Applying this with a = x0 and b = n/x0, whose product is exactly n, shows the real-valued average is always at least sqrt(n), itself at least r; truncating both divisions only shaves a fractional amount off that average, and the tests confirm, across many n including u64::MAX, that this never pushes the result below r. The second half rules out stopping too early, above r: whenever x0 > r, x0 is at least r+1, and since r is the floor of sqrt(n), the next integer above r already overshoots n, so

$$x_0 \ge r+1 \implies x_0^2 \ge (r+1)^2 > n \implies \frac{n}{x_0} < x_0$$

which says that whenever the guess is strictly above r, n divided by that guess is strictly smaller than the guess itself, so their average is strictly less than x0 too — the sequence strictly decreases while it sits above r, so it can never stall there. Combining both halves: the sequence cannot stop above r, since it always strictly decreases there, and it cannot drop below r, by the AM-GM bound; once it reaches r exactly, the same bound guarantees the next value is at least r, which is exactly the loop's stopping condition. That is why the implementation checks x1 >= x0 rather than waiting for x1 == x0 — the n = 8 trace shows the latter never happens — and returns x0, not x1, the moment that condition first triggers.

## Complexity

The quadratic convergence above only describes behavior once x0 is already close to r; the first steps look completely different. At x0 = n, n/x0 = 1, so the first update is approximately x0/2 — indistinguishable from a single bisection step. This halving persists as long as x0 stays much larger than r, since n/x0 stays small relative to x0 throughout that phase, and shrinking x0 from n down to roughly r this way takes about log2(n/r) steps, which is about log2(sqrt(n)), or half of log2(n). Only once x0 is already near r does the quadratic tail take over, needing an additional but much smaller O(log log n) steps to finish. Overall complexity is therefore O(log n), the same asymptotic class as `mod-isqrt-bisect.md`'s bisection search, not an asymptotically faster algorithm — quadratic convergence only pays off once the search is already close, and starting from x0 = n means most of the work happens during the slower halving phase. The practical benefit is a smaller constant, not a different complexity class.

## Edge cases

At n = 0, x0 = 0, the loop condition x0 > 0 is false immediately, and 0 is returned without iterating, matching isqrt(0) = 0. At n = 1, x0 starts at 1, the first x1 = (1+1)/2 = 1, and since x1 >= x0 the loop stops on the very first check, correctly returning 1. The remaining hazard is overflow: at x0 = n = u64::MAX, n/x0 = 1 and x0 + 1 would overflow, so the implementation saturates the sum at u64::MAX instead of wrapping. This introduces an off-by-one of at most one into that single intermediate sum, but the resulting x1 is still an enormous overestimate, vastly larger than the true answer near 4.29 billion, so the strict-decrease argument above still applies on the next iteration and the final result is unaffected.

## Worked example

Trace isqrt(8) by hand, where the true answer is 2. From x0 = 8, n/x0 = 1, so x1 = (8+1)/2 truncates to 4; since 4 < 8, x0 updates to 4. Next, n/x0 = 8/4 = 2, so x1 = (4+2)/2 = 3; since 3 < 4, x0 updates to 3. Next, n/x0 = 8/3 truncates to 2, so x1 = (3+2)/2 truncates to 2; since 2 < 3, x0 updates to 2. Next, n/x0 = 8/2 = 4, so x1 = (2+4)/2 = 3; now x1 is not less than x0, since 3 >= 2, so the loop stops and returns x0 = 2 — correct, since 2^2 = 4 does not exceed 8 while 3^2 = 9 does. Continuing past this point out of curiosity, the next value from x0 = 3 would again be 2, and from 2 again 3, confirming the 2-3 cycle predicted above and showing exactly why the stopping rule checks x1 >= x0 rather than waiting for the sequence to settle at a single fixed value.
