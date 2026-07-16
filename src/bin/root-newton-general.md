# Newton's Method for General Root-Finding

## Overview

The problem here is the same construction explored in `mod-isqrt-newton.md`, but stripped of the one specific function that session used: instead of always chasing a root of f(x) = x^2 - n, this session builds a routine that accepts any differentiable f, together with its derivative, and searches for a value where f is zero. The algorithm's standard name in the literature is Newton's method, sometimes called the Newton-Raphson method after Joseph Raphson's independent contribution to its modern form. The core geometric idea carries over unchanged: stand at a current guess, look at the tangent line to the curve there, and follow that line down to where it crosses zero to get a sharper guess. What changes is everything downstream of that idea. The isqrt session worked entirely in integers with an exact stopping test; here the target is typically irrational, so the whole computation moves to f64 and the stopping rule becomes a tolerance rather than an exact comparison. The isqrt session also had, for free, an unconditional proof, via the AM-GM inequality, that the iteration always converges starting from x0 = n; no such guarantee exists for an arbitrary f, and this session produces an explicit counterexample where the method oscillates forever instead of converging to anything. Because this session synthesizes several genuinely new pieces at once, a from-scratch introduction of Taylor's theorem used to prove the convergence rate in general, a hand-derived non-convergence example with its own curvature argument, and an implementation-driven discovery about how the multiplicity of a root interacts with floating-point stopping criteria, it runs longer than a typical single-concept session in this repository, closer to a synthesis entry than a thin extension of the earlier one.

## Deriving the general update rule

The tangent-line construction that produces the update formula was already set up in general point-slope form in `mod-isqrt-newton.md`: at a current guess x0, the line through (x0, f(x0)) with slope f'(x0) satisfies y minus f(x0) equals f'(x0) times (x minus x0), and setting y to zero and solving for x gives the next guess. That earlier session only ever carried this out for f(x) = x^2 - n, so redoing the same algebra symbolically for a generic f gives

$$0 - f(x_0) = f'(x_0)(x - x_0)$$

which rearranges, after distributing and isolating x, to

$$x_1 = x_0 - \frac{f(x_0)}{f'(x_0)}$$

the general Newton update rule, valid at any x0 where f'(x0) is nonzero. As a check that this is really the same construction as before and not a different one that merely resembles it, substituting f(x) = x^2 - n and f'(x0) = 2x0 into this formula and splitting the resulting fraction into two terms reproduces exactly

$$x_1 = \frac{x_0 + n/x_0}{2}$$

the isqrt-specific formula already proven correct in the earlier session, confirming that the general derivation specializes correctly to the case already understood there. One immediate consequence of writing the rule this way is a clean failure mode: if f'(x0) is exactly zero, the formula divides by zero, and geometrically this corresponds to a horizontal tangent line, which either never meets the x-axis at all or coincides with it only if x0 already is a root — either way, the construction cannot propose a next guess.

## When convergence fails

The isqrt session's convergence proof leaned entirely on a fact specific to f(x) = x^2 - n: it is convex everywhere, with constant second derivative f''(x) = 2, and starting at x0 = n the AM-GM argument showed the sequence always sits above sqrt(n) and decreases monotonically toward it. No such argument is available for a general f, and a concrete counterexample shows that "Newton's method always converges" is simply false without further conditions. The counterexample used throughout the rest of this section is the cubic

$$f(x) = x^3 - x$$

which has three real roots, at -1, 0, and 1, but whose shape away from those roots turns out to matter just as much as the roots themselves. Starting the iteration at x0 = 1/sqrt(5), direct computation gives f(x0) = x0 times (x0^2 minus 1), which equals x0 times (1/5 minus 1), or -4x0/5, and f'(x0) = 3x0^2 - 1 = 3/5 - 1 = -2/5, so

$$\frac{f(x_0)}{f'(x_0)} = \frac{-4x_0/5}{-2/5} = 2x_0$$

which makes the first update x1 = x0 - 2x0 = -x0: the iteration jumps to the exact mirror point on the other side of zero. Because f is an odd function, meaning f(-x) equals -f(x), and f' is even, meaning f'(-x) equals f'(x), applying the same update starting from x1 = -x0 must produce x2 = -x1 = x0 by the identical computation with every sign flipped, so the sequence runs x0, -x0, x0, -x0, forever, an infinite two-cycle that never lands on any of the three actual roots, even though f'(x) is never exactly zero at any point visited, which rules out the division-by-zero failure as an explanation for this particular breakdown. This same counterexample also falsifies two more naive claims one might make about the method: that it converges to "the first" root, or to whichever root happens to be closest to the starting point. Here x0 = 1/sqrt(5) is closer to the root at 0 than to either root at plus or minus 1, and yet the iteration never approaches 0 at all. The mechanism behind the failure is visible in the curvature: f''(x) = 6x changes sign at x = 0, an inflection point, and f itself has a local maximum and a local minimum somewhere between its roots -1 and 1, where f'(x) = 3x^2 - 1 = 0, at x equal to plus or minus 1/sqrt(3) — a "wiggle" that x^2 - n, convex everywhere with no inflection point at all, simply does not have. That wiggle is exactly what lets the tangent line, followed down to the x-axis, leap across to the far side of the curve instead of sliding toward the nearest root. This behavior was later confirmed directly in the f64 implementation: a test starting at 1.0 divided by 5.0f64.sqrt() correctly returns None after exhausting the iteration cap, matching the exact-arithmetic prediction, since floating-point negation is exact and the symmetric two-cycle does not drift under rounding.

## Quadratic convergence via Taylor's theorem

The isqrt session proved quadratic convergence, e1 = e0^2/(2x0) where e_k denotes x_k minus the true root, through algebra built around a perfect-square numerator that only appears for f(x) = x^2 - n. Reaching the same kind of result for a general f needs a tool that has not appeared anywhere else in this repository: Taylor's theorem, which says a smooth function can be approximated near a point not just by its tangent line, matching value and slope, but, more accurately, by also matching its curvature. Concretely, near a point a, a function f satisfies

$$f(x) \approx f(a) + f'(a)(x - a) + \frac{1}{2}f''(a)(x - a)^2$$

with an error term that shrinks faster than (x - a)^2 as x approaches a. As a warm-up check, applying this to f(x) = x^2, whose derivatives are f'(x) = 2x and f''(x) = 2, gives a^2 + 2a(x - a) + (x - a)^2, which expands and cancels exactly to x^2 with no leftover error at all, unsurprising since x^2 is already a degree-two polynomial with no higher-order terms to drop.

Now let s denote the true root, so f(s) = 0, and apply the same expansion centered at the current guess x0, evaluated at x = s:

$$f(s) \approx f(x_0) + f'(x_0)(s - x_0) + \frac{1}{2}f''(x_0)(s - x_0)^2$$

Writing e0 = x0 - s for the current error, so that s - x0 equals -e0, and using f(s) = 0 on the left side gives

$$0 \approx f(x_0) - f'(x_0)e_0 + \frac{1}{2}f''(x_0)e_0^2$$

Isolating f(x0) and substituting it into the Newton update x1 = x0 - f(x0)/f'(x0), then splitting the resulting fraction into two terms and using x0 minus e0 equals s, produces

$$e_1 = \frac{f''(x_0)}{2f'(x_0)}e_0^2$$

the general quadratic-convergence relation. Substituting f''(x) = 2 and f'(x0) = 2x0, the constants from the isqrt case, reduces this immediately to e1 = e0^2/(2x0), exactly the earlier result, confirming that the general derivation specializes correctly there too. The practical meaning of e1 being proportional to e0 squared, rather than to e0 itself, is that the number of correct digits roughly doubles every iteration once the guess is close enough to the root, in sharp contrast to bisection, where each step gains a fixed amount of precision regardless of how close the guess already is.

## Correctness

The correctness argument here is necessarily weaker than the isqrt session's, and stating that difference precisely is itself part of what correctness means for this algorithm. The construction is always well-defined at points where f'(x) is nonzero: at each step, the tangent line at the current guess is the best available linear approximation of f there, and its zero crossing is a reasonable next guess exactly to the extent that the tangent line tracks the actual curve nearby. What the isqrt session could additionally prove, that this process reliably converges to the true root from any legitimate starting point, required a specific structural fact about x^2 - n, namely constant, sign-unchanging curvature, that does not hold for f in general. The x^3 - x example demonstrates concretely that when f'' changes sign in the region being searched, the iteration can fail to converge to any root at all, cycling indefinitely instead. So the honest correctness statement is conditional: given a starting point close enough to a simple root, in a region where f does not change concavity, the sequence of guesses converges to that root, and the reasoning in the Taylor's-theorem section bounds how fast. Outside those conditions, starting too far away, or searching across an inflection point, no convergence guarantee is available, and the implementation's iteration cap exists precisely to terminate gracefully rather than loop forever when those conditions fail to hold.

## Complexity

Once the iteration is in a regime where it does converge, the quadratic error relation e1 approximately equal to C times e0^2, for some roughly-constant C near the root, means the number of correct digits doubles each step: reaching D correct digits from an initial handful takes on the order of log2(D) further iterations, and since D itself scales like log(1/eps) for a target tolerance eps, the total iteration count needed is on the order of log(log(1/eps)). This is a strictly better complexity class than bisection's O(log(1/eps)), not merely a smaller constant factor, since the double logarithm grows so slowly that even extreme precision targets need only a handful of extra iterations beyond what modest precision already required. This complexity result, however, only describes the well-behaved convergent case; it says nothing about inputs like the x^3 - x oscillation, where the loop instead runs for the full iteration cap n before giving up and reporting failure.

## Edge cases

Three distinct hazards surfaced this session, beyond the ones already covered above as part of the correctness argument itself. The first is the division-by-zero case already noted: whenever f'(x) evaluates to exactly 0.0 at some iterate, the implementation returns None immediately rather than dividing, since the tangent line there is horizontal and cannot propose a next guess. The second is non-convergence itself: the x^3 - x example starting at 1/sqrt(5) is not a contrived edge case but a genuine input on which the loop must run for its full iteration budget and then report failure, which is exactly why a hard iteration cap is a required part of the design rather than an optional safety net; without it, that specific input, and inputs like it, would loop forever.

The third hazard is more subtle and was discovered only while writing tests, not anticipated going in: it concerns what a stopping tolerance eps actually guarantees about the final answer's precision when the target is a repeated root. Testing against f(x) = (x - 1)^2, whose root at x = 1 is a double root, meaning f'(1) = 0 as well as f(1) = 0, an early version of the stopping check evaluated the size of f(x) directly and returned that value's x, without accounting for what such a check actually implies. Near a root of multiplicity k, f(x) is governed by its first nonvanishing Taylor term, which is proportional to (x minus the root) raised to the k-th power rather than to the first power, so bounding the size of f(x) by eps only guarantees the distance from x to the root is bounded by eps raised to the power 1/k; for k = 2 that bound is the square root of eps, three orders of magnitude looser than eps itself when eps is 1e-6, and the same reasoning applied to a root of multiplicity 4 gives eps raised to the power 1/4, an even looser bound of roughly 0.03 at that same eps. The fix was not to compensate by shrinking eps by some multiplicity-dependent factor, since the multiplicity of a root is not something the caller of a general-purpose root finder can be expected to know in advance; instead, the resolution was to drop the function-value-based check entirely and rely solely on the step-size check, the distance between x1 and x0 compared against eps. For a root of multiplicity k, Newton's method converges only linearly rather than quadratically, with each step's error shrinking by the constant factor (1 minus 1/k) rather than squaring, and the distance between consecutive iterates stays proportional to the remaining error by a factor of roughly (k minus 1), a relationship that does not degrade as k grows, unlike the function-value bound. This was confirmed both analytically and by direct computation: for f(x) = (x-1)^2 starting at x0 = 1.0001 with eps = 1e-6, relying only on the step-size check converges to within about 7.8e-7 of the true root in 7 iterations, safely inside the requested tolerance, whereas the same eps applied to a function-value check would only have guaranteed precision on the order of the square root of eps, about 1e-3.

## Worked example

Trace one iteration of finding the cube root of 2 by applying the update rule to f(x) = x^3 - 2, whose only real root is 2 raised to the power 1/3, approximately 1.26. Starting at x0 = 1, first compute f(x0) = 1^3 - 2 = -1, and, using the power rule, f'(x) = 3x^2, so f'(x0) = 3. The tangent line through (1, -1) with slope 3 satisfies y minus (-1) equals 3 times (x minus 1), and setting y to zero gives 1 = 3x - 3, so 3x = 4 and

$$x_1 = \frac{4}{3} \approx 1.333$$

already closer to the true root near 1.26 than the starting guess of 1 was, consistent with quadratic convergence beginning to take hold even after a single step from a starting point that is not yet very close. Repeating the same construction at x0 = 4/3 would use f(4/3) = 64/27 - 2 = 10/27 and f'(4/3) = 3 times 16/9, or 16/3, giving a further-refined x2 close to 1.2599, though tracing that second step fully by hand is left as a natural follow-up rather than part of this worked example, since the point of tracing the first step is only to confirm the update rule and the direction of improvement, not to reach full convergence by hand.
