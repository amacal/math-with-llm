# Monte Carlo Integration

## Overview

The goal of this session is to estimate the numerical value of a definite integral of some function f(x) over an interval [a, b] using nothing but a source of pseudorandom numbers, without doing any calculus and without needing a closed-form antiderivative of f. This directly generalizes the hit-or-miss method built in `rand-monte-carlo-pi.rs`, which estimated pi by scattering random points in a square and testing whether each landed inside a quarter-circle. Here the region of interest is no longer a fixed circle but the area under an arbitrary curve y = f(x), and the method that emerges — averaging the function's own values at random points, rather than testing hits and misses — is called Monte Carlo integration, sometimes qualified as "crude" or "sample-mean" to distinguish it from variance-reduction refinements such as importance sampling, out of scope here.

## Why hit-or-miss does not simply generalize

The first idea one reaches for is to generalize hit-or-miss directly: enclose the curve in a bounding box spanning [a, b] on one axis and [min f(x), max f(x)] on the other, scatter random 2D points in that box, and count the fraction landing below the curve. This works in principle, but it depends on knowing a valid bound for f's range in advance, and finding that bound exactly is, in general, no easier than the original problem — an arbitrary function's true maximum and minimum over [a, b] may themselves require search or optimization. Estimating the bound instead, by sampling a handful of x-values and taking the largest observed f(x), does not fix this: a sampled maximum is not guaranteed to reach the true peak, and if the assumed box height is even slightly too low somewhere between samples, every point whose true height exceeds that bound is silently mishandled. Running more trials with that same, too-low box does not repair the problem — the error settles toward a fixed nonzero constant rather than shrinking to zero, since the flaw is a structural bias in what the box can represent, not statistical noise that averaging washes out. This is a genuine correctness defect, not merely an inefficiency, and it is why this session abandons the bounding-box idea for a method that never needs to know f's range at all.

## The mean-value identity

The alternative starts from a small, concrete case: take f(x) = x^2 on [0, 1], whose true integral is 1/3. Evaluating f at three points, 0, 0.5, and 1, gives values 0, 0.25, and 1, whose average is 5/12 — close to, though not exactly, 1/3, even with only three points. This suggests that "area under the curve" can be recovered from "average height of the curve," reframed as area equals width times average height, exactly the formula for a plain rectangle but with the constant height replaced by an average value. Formally, this average value is written using the same weighted-average idea already used for E[Z] in the Pi session, where E[Z] summed each of the two possible outcomes of Z weighted by its probability. Here f(x) ranges over a continuum of values rather than two discrete outcomes, so the discrete sum is replaced with an integral, and the discrete probability weight is replaced by a constant weight per unit length, call it w. For that weight to sum to 1 across an interval of length (b - a), it must equal 1/(b - a), which is precisely the density of a variable X distributed uniformly on [a, b]. Writing the expectation of f(X) as this continuous weighted average gives

$$E[f(X)] = \int_a^b f(x) \, w \, dx = \frac{1}{b-a}\int_a^b f(x) \, dx,$$

which says that E[f(X)] for X uniform on [a, b] is exactly the average value of f over the interval. Rearranging this identity the other way around gives the fact this whole method rests on:

$$\int_a^b f(x) \, dx = (b - a) \cdot E[f(X)].$$

This says that a definite integral is nothing more than the width of the interval multiplied by the mean value of the function being integrated, and it converts the original calculus problem into a probability problem: estimate a mean, and multiply by a known constant.

## Estimating the mean and its variance

Estimating E[f(X)] uses exactly the same tool as the Pi session: draw n independent points X_1, ..., X_n uniformly at random from [a, b] (via the same LCG-based normalization technique from `rand-lcg.rs`, rescaled to [a, b) instead of [0, 1)), and average the resulting f-values. The full estimator for the integral is therefore (b - a) times the sample average of f(X_1), ..., f(X_n). Analyzing how good this estimator is requires generalizing the variance formula for Z derived in the Pi session, since f(X) is no longer restricted to the two outcomes 0 and 1. The same "weighted average of squared deviations from the mean" idea that produced that earlier formula generalizes, using the same density w = 1/(b - a), to

$$\mathrm{Var}(f(X)) = \int_a^b \big(f(x) - E[f(X)]\big)^2 \, w \, dx,$$

the continuous analogue of a weighted average of squared deviations. Unlike the variance of Z from the Pi session, which was bounded above by one quarter no matter what p was, the variance of f(X) has no such universal ceiling: a constant function has zero variance since it never deviates from its own mean, while a function that swings wildly across a wide range over [a, b] has correspondingly large variance. The convergence rate of Monte Carlo integration therefore genuinely depends on the specific integrand being estimated, in a way that pi-estimation via a fixed geometric ratio never had to confront.

## Correctness

The estimator (b - a) times the sample average of f(X_1), ..., f(X_n) is correct because these are independent copies of the random variable f(X), and by the Law of Large Numbers their sample average converges, as n grows, to the true expected value E[f(X)]. Multiplying that converged average by (b - a) gives (b - a) times E[f(X)], which the mean-value identity above shows is exactly the true integral of f over [a, b]. This argument depends on X_1, ..., X_n being genuinely uniform and independent draws from [a, b], the same requirement on the generator already established and tested in the Pi session.

## Complexity

Estimating the integral from n samples costs O(n) time, since each sample requires one constant-time draw from the generator, one rescaling into [a, b], and one evaluation of f. Space usage is O(1), since only a running sum of f-values is kept, never the individual samples. The rate of convergence follows the same derivation as the Pi session — averaging n independent copies divides variance by n, and taking a square root gives the standard deviation of the full estimator as

$$\frac{(b-a)\sqrt{\mathrm{Var}(f(X))}}{\sqrt{n}},$$

so error still shrinks proportionally to 1 over the square root of n, and halving it still requires quadrupling n, exactly as before. What is new is the square root of Var(f(X)) as a scaling factor: since this quantity has no universal bound, unlike the corresponding factor in the Pi session which never exceeded one half, two integrands sampled with the same n can converge at very different practical rates, even though the exponent on n is identical.

## Edge cases

Requesting zero samples would divide by zero in the final average, so the implementation returns `None` for n equal to 0 rather than attempting any computation. An interval where a is greater than b is treated as invalid and also returns `None`, since the method assumes a is the lower bound. The remaining boundary case, a exactly equal to b, needs no special handling: every sampled x collapses to the single point a regardless of the random draw, so the sum of f-values stays finite, but the (b - a) factor out front is exactly zero, making the whole estimate exactly `Some(0.0)`, correctly matching the true value of an integral over a zero-width interval for any f.

## Worked example

Trace the estimator by hand on f(x) = 4/(1 + x^2) over [0, 1], whose true integral is exactly pi — the same constant recovered geometrically in `rand-monte-carlo-pi.rs`, now recovered through this session's sample-mean method on a completely different function. Using the three points 0, 0.5, and 1 as a stand-in for a random sample, f evaluates to 4, 3.2, and 2, summing to 9.2. Dividing by the count of 3 gives a sample average of about 3.067, unchanged by multiplying with the interval width, since (b - a) here is exactly 1. Comparing 3.067 to pi, about 3.14159, shows an undershoot of roughly 0.075 with only three points — consistent with the theory above, since three samples are far too few for the standard deviation term to have shrunk appreciably. Increasing the sample count toward the tens of thousands, as the actual test suite does for this integrand, drives this same estimator toward pi to within the tolerance used in `rand-monte-carlo-integration.rs`.
