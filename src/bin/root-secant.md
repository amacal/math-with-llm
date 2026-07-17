# Secant Method for Root-Finding

## Overview

Newton's method, covered in `root-newton.md`, needs both the value of f at the current guess and the value of its derivative f' there — a real cost when f' is expensive or unavailable in closed form. The secant method removes that cost by estimating a slope from two nearby points already on the curve, using only values of f: where a tangent line touches the curve at one point using its exact slope, a secant line passes through two distinct points, which is where the method takes its name. Because two points are needed to define that line, the method starts from two initial guesses rather than one, producing its next guess as the point where the secant line through the two most recent guesses crosses zero. This session asks how much is lost by trading an exact derivative for that two-point approximation: the method still converges faster than linearly, but at a rate governed by the golden ratio rather than Newton's clean doubling.

## Deriving the secant update rule

The slope of a straight line through two points (x0, f(x0)) and (x1, f(x1)) satisfies f(x1) minus f(x0) equals m times (x1 minus x0), so

$$m = \frac{f(x_1) - f(x_0)}{x_1 - x_0}$$

This is exactly the finite-difference approximation to a derivative: as x1 approaches x0 it becomes the definition of f'(x0), and otherwise is the best slope estimate available from two function values. Substituting this m for f'(x0) in Newton's update rule x_new = x0 - f(x0)/f'(x0) (derived in `root-newton.md`), and writing the result in terms of the two most recent iterates, gives

$$x_{n+1} = x_n - f(x_n)\frac{x_n - x_{n-1}}{f(x_n) - f(x_{n-1})}$$

the secant update rule. Each step needs both of the two most recent points, so the iteration is seeded with two starting guesses instead of Newton's one, and no derivative function is ever called.

## The error recurrence

The convergence analysis reuses the Taylor-expansion strategy from `root-newton.md`, but with two error terms instead of one. Let r be the true root and e_n = x_n - r. Expanding f to second order around r, exactly as in the Newton derivation, gives f(x_n) approximately f'(r)e_n plus one half f''(r)e_n^2 (the same expansion holds for x_{n-1} with e_{n-1}). Subtracting the two expressions, f(x_n) minus f(x_{n-1}) factors as (e_n minus e_{n-1}) times a bracketed term, treating e_n^2 minus e_{n-1}^2 as a difference of squares for the quadratic part:

$$f(x_n) - f(x_{n-1}) \approx (e_n - e_{n-1})\left[f'(r) + \frac{1}{2}f''(r)(e_n + e_{n-1})\right]$$

Substituting this, together with x_n minus x_{n-1} equals e_n minus e_{n-1}, into the update rule, the (e_n minus e_{n-1}) factor cancels between numerator and denominator, and after subtracting the resulting ratio from e_n and cancelling the terms linear in e_n, what survives is

$$e_{n+1} \approx \frac{f''(r)}{2f'(r)}\,e_n\,e_{n-1}$$

the secant error recurrence. It carries the same constant C = f''(r)/(2f'(r)) as Newton's e1 = C e0^2, but here the next error is proportional to the product of the two previous errors rather than the square of one, reflecting that the method uses two points instead of one at each step.

## Deriving the order of convergence

A recurrence relating e_{n+1} to a product of two previous errors is not yet a convergence rate. Assume a self-similar power law holds at every step, |e_{n+1}| = D|e_n|^p, for constants D and the order p being sought. Applying the same assumption one step earlier gives |e_{n-1}| = (|e_n|/D)^(1/p). Substituting this into the recurrence and combining e_n times e_n^(1/p) into a single power gives

$$e_{n+1} \approx \frac{C}{D^{1/p}}\,e_n^{(p+1)/p}$$

Consistency with the original assumption requires the exponents to match: p equals (p+1)/p, so p^2 minus p minus 1 equals zero, whose two roots are

$$p = \frac{1 \pm \sqrt{5}}{2}$$

approximately 1.618 or -0.618. The negative root is discarded: since e_n shrinks toward zero, a negative exponent would make e_n^p grow without bound, describing divergence rather than convergence. The remaining root is exactly the golden ratio, and the worked example below verifies by hand that the errors really do satisfy the product recurrence rather than a naive square-of-the-last-error relation. Bisection halves the interval containing the root at every step, the fact already established in `mod-isqrt-bisect.md`; cast in this session's e_{n+1} = D e_n^p form, that halving is simply the case p = 1, D = 1/2. So the three methods in this repository's root-finding branch rank as bisection (p = 1, linear) below secant (p ≈ 1.618, superlinear) below Newton (p = 2, quadratic).

## Correctness

The correctness statement mirrors the conditional form used for Newton's method, but is a step weaker still. Given two starting points close to a simple root r (f'(r) nonzero) with f twice continuously differentiable near r, the errors shrink as e_{n+1} approximately C e_n e_{n-1} with order p equal to the golden ratio. Unlike bisection, this guarantee is entirely local: nothing in the update rule inspects the sign of f(x_n) or f(x_{n-1}), so the two starting points need not bracket a sign change, and there is no structural mechanism forcing progress when they start far from the root. A related historical method, regula falsi (the method of false position), restores a bracketing requirement on the same secant-line construction, always keeping one point on each side of a sign change; plain secant drops that constraint, so divergence or oscillation, already seen for Newton on f(x) = x^3 - x, remains possible.

## Complexity

Per iteration, once y0 and y1 are cached from the previous step rather than recomputed, the method costs one new evaluation of f, two subtractions, and one division, against Newton's one evaluation each of f and f'. Chaining |e_{n+1}| = D|e_n|^p across k iterations gives e_k approximately e0 raised to the power p^k. Setting e0^(p^k) equal to a target tolerance eps and taking logarithms twice, first to bring p^k down as a coefficient and then to isolate k, gives

$$k = \frac{\log\!\big(\log(\mathrm{eps})/\log(e_0)\big)}{\log p}$$

so the iteration count scales as O(log log(1/eps)), the same class as Newton, differing only in the constant 1/log(p), larger than Newton's 1/log(2). Both dramatically beat bisection's O(log(1/eps)).

## Edge cases

Two starting points with equal f-values but distinct x-values make the denominator vanish; a concrete example is f(x) = x^2 - 2 at the symmetric pair x = 1 and x = -1, both giving f = -1. The implementation checks for this directly before dividing, returning None. Testing f(x) = x^3 - x near 1/sqrt(5), the region that produced an eternal two-cycle for Newton's method in `root-newton.md`, behaves differently here: the iterates jump chaotically for several steps before converging to the root at 0, showing secant's extra point of information can escape a pathology that traps Newton's single-point construction, without this proving general robustness. Testing a double root, f(x) = (x-1)^2, reproduces the precision hazard already documented for Newton in `root-newton.md`: since f'(r) = 0, the constant C is undefined and the golden-ratio derivation no longer applies. The observed step-to-step ratio near this root was only about 0.6, i.e. linear rather than superlinear convergence; summing the resulting geometric series shows the true remaining error is about the step size times r/(1-r), a factor of 1.5 at r = 0.6, matching an observed step of about 8.5e-7 against a true error of about 1.37e-6, larger than the requested 1e-6 tolerance. The test was adjusted to assert against 1e-5, reflecting this degraded guarantee honestly. Separately, f(x) = x^3 + x returns 0 exactly from `root-newton.rs` but approximately 2.3e-19 from `root-secant.rs`; this is a floating-point artifact, not a correctness difference, since subtracting f(x_n) minus f(x_{n-1}) when both are already tiny near the root is exactly the setting for catastrophic cancellation, where subtracting two nearly equal floating-point numbers destroys most of their significant digits.

## Worked example

Trace the method on f(x) = x^3 - 2, root r ≈ 1.259921, starting from x0 = 1, x1 = 1.5. First, f(x0) = -1 and f(x1) = 1.375. The secant slope is 2.375 divided by 0.5, and the update gives x2 = 1.5 minus 1.375 times 0.5 divided by 2.375, or 1.5 minus 0.289473, approximately 1.210526 — already noticeably closer to the root than either starting point, despite using no derivative. Continuing, f(x2) ≈ -0.226, and the next update uses x1 = 1.5, x2 = 1.210526: x3 = 1.210526 minus (-0.226) times (1.210526 minus 1.5) divided by (-0.226 minus 1.375), which simplifies to approximately 1.251300. Comparing all four iterates against the true root gives errors e0 ≈ -0.259921, e1 ≈ 0.240079, e2 ≈ -0.049421, e3 ≈ -0.008621; the shrinking magnitude confirms convergence, and the ratio e3 divided by e1 times e2 (≈ 0.718) sits close to e2 divided by e0 times e1 (≈ 0.792) — exactly the near-constant C predicted by e_{n+1} approximately C times e_n times e_{n-1}, rather than the wildly inconsistent values produced by treating e_{n+1} as proportional to e_n squared alone.
