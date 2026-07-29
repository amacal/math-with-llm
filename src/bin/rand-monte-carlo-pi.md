# Monte Carlo Estimation of Pi via Hit-or-Miss Sampling

## Overview

The goal of this session is to estimate the numerical value of pi using nothing but a source of pseudorandom numbers and a simple geometric test, rather than any formula from trigonometry or calculus. The idea is to scatter many random points inside a square, check for each one whether it also lands inside a circle inscribed in that square, and use the fraction of points that land inside the circle to recover an estimate of pi. This general strategy — approximating a hard-to-compute quantity by repeatedly sampling randomly and averaging — is called the Monte Carlo method, and the specific version used here, where a sample either counts or is discarded based on a simple geometric test, is called hit-or-miss (or rejection) sampling. Unlike the exact, deterministic algorithms built earlier in this repository, this one produces only an estimate whose accuracy improves as more samples are taken, trading exactness for a completely different kind of computational tool.

## From a random integer to a random point in the square

The Linear Congruential Generator built in `rand-lcg.rs` produces a sequence of integers x_n confined to the range from 0 up to m minus 1 via the recurrence x_(n+1) = (a times x_n plus c) mod m, proved to reach full period under the Hull-Dobell conditions in that earlier session. Such an integer is not yet a coordinate inside a unit square, whose coordinates must range over the reals between 0 and 1, not the integers between 0 and m minus 1. Dividing every output by m closes this gap: since x_n never reaches m and is never negative, the quotient x_n / m always lands in the half-open interval from 0 up to but not including 1. Taking two consecutive outputs, dividing each by m, and pairing them as (x, y) produces a point somewhere inside the unit square with corners at (0,0), (1,0), (1,1), and (0,1).

## The hit-or-miss test and the area ratio pi/4

Inside that unit square sits a quarter of a circle of radius 1, centered at the origin — the boundary curve x^2 + y^2 = 1, restricted to the quadrant where both coordinates are non-negative. A point counts as a hit if it satisfies

$$x^2 + y^2 \le 1$$

and as a miss otherwise. If the points are scattered uniformly at random across the square, then for any random point, the probability that it happens to be a hit equals the ratio of the quarter-disk's area to the square's area, since geometric probability for a uniform distribution is always area of the target region divided by area of the whole space. The unit square has area 1, and the quarter-disk, being one quarter of a full circle of radius 1 whose area is pi times 1 squared, has area

$$\frac{\pi}{4}$$

so the probability of a hit is exactly pi divided by 4. Rearranging that relationship gives a way to recover pi itself from an observed hit probability:

$$\pi = 4 \times \frac{\text{hits}}{\text{total trials}}$$

which is the entire algorithm: generate many (x, y) pairs, count how many are hits, divide by the total, and multiply by 4.

## Convergence: the Law of Large Numbers and the 1/sqrt(n) rate

Because each trial is random, the observed fraction hits divided by total will never sit exactly on pi divided by 4 for any finite number of trials — it is itself a random quantity that only tends toward the true value as the number of trials grows, a phenomenon called the Law of Large Numbers. The rate at which it tends there can be made precise by modeling each trial as a variable Z that equals 1 on a hit and 0 on a miss, with hit probability p equal to pi divided by 4. Such a variable has variance

$$\mathrm{Var}(Z) = p(1-p)$$

obtained from the probability-weighted average of its squared deviations from the mean p. Averaging n independent copies of Z divides the variance by n, since variance of a sum of independent variables adds while dividing by n scales variance by the square of that factor, giving Var(Z) divided by n for the average; taking a square root then gives the standard deviation of the average as

$$\frac{\sqrt{\mathrm{Var}(Z)}}{\sqrt{n}}$$

so the typical spread of the pi estimate around its true value shrinks proportionally to 1 over the square root of n, not to 1 over n. Halving that error therefore requires quadrupling the number of trials, a cost that is inherent to any estimator built this way rather than a symptom of a poor implementation.

## Independence of coordinates and the spectral pitfall

The area-ratio argument above assumes the (x, y) points are genuinely spread uniformly and independently across the whole square, and this can silently fail even when the underlying generator has full period. Because the recurrence x_(n+1) = a times x_n plus c mod m is linear, plotting consecutive pairs (x_n, x_(n+1)) without the wraparound would trace a single straight line of slope a; the mod m wraparound cuts that line into a small number of parallel, shifted segments rather than removing the linear structure. When a is small relative to m, only a handful of such stripes exist, and the generated points cluster onto them instead of filling the square — the same defect, known as failing the spectral test, that made the historical generator RANDU notorious. A first implementation here used a full-period multiplier of only 5, which produced a pi estimate converging tightly but to approximately 3.087 rather than 3.14159, since the striped points sampled the quarter-disk's area unevenly; switching to a much larger multiplier (0x5d588b65) restored correct convergence, confirming full period alone is necessary but not sufficient for a generator to be trusted as a multi-dimensional entropy source.

## Correctness

The algorithm is correct because the observed hit fraction is an unbiased estimator of the true hit probability p equal to pi divided by 4 — its expected value equals p exactly, by the definition of probability as long-run relative frequency — and because its variance shrinks to 0 as the number of trials grows, per the derivation above. An estimator with the correct mean and vanishing variance must converge to that mean, which is precisely the Law of Large Numbers; multiplying the converged hit fraction by 4 then converges to pi itself. This depends critically on the (x, y) points behaving as genuinely uniform, independent samples of the square, exactly the assumption the spectral pitfall above threatens and a well-chosen multiplier restores.

## Complexity

Estimating pi from n trials costs O(n) time, since each trial does a fixed, constant amount of work: two calls to the generator, two squarings, one addition, and one comparison. Space usage is O(1), since the algorithm only ever needs a running count of hits and of total trials, never storing the individual sample points. The O(n) time cost is not an artifact of this particular implementation but an unavoidable consequence of the 1 over square root of n convergence rate: reaching one additional decimal digit of precision, a tenfold reduction in error, requires a hundredfold increase in n.

## Edge cases

Requesting zero trials would divide by zero in the final ratio, since both the hit count and the total count would be zero; the implementation special-cases n equal to 0 by treating it as a single trial instead, a fix chosen deliberately over a naive `n + 1` adjustment applied to every input, since that alternative silently ran one extra trial for every value of n except n equal to u32::MAX (where saturating addition has no effect), an inconsistency uncovered by comparing those two specific inputs directly. Squaring each u32-derived coordinate as a u64 never overflows, since the largest possible square, from a value just below 2^32, still falls short of 2^64; only the sum of two such squares can exceed a u64, and the implementation uses `checked_add` to detect exactly that overflow, treating it as the geometric condition x^2 + y^2 exceeding 1, since the two thresholds coincide up to a single, negligible boundary point.

## Worked example

Reusing the full-period trace from `rand-lcg.md` with m = 8, a = 5, c = 1, seed 0, the generator produces 0, 1, 6, 7, 4, 5, 2, 3 before repeating. Dividing each by 8 gives the normalized values 0, 0.125, 0.75, 0.875, 0.5, 0.625, 0.25, 0.375, and pairing them consecutively gives four points: (0, 0.125), (0.75, 0.875), (0.5, 0.625), and (0.25, 0.375). Testing each against x^2 + y^2 less than or equal to 1: the first gives 0 plus 0.015625, a hit; the second gives 0.5625 plus 0.765625 equal to 1.328125, a miss, since it exceeds 1; the third gives 0.25 plus 0.390625 equal to 0.640625, a hit; and the fourth gives 0.0625 plus 0.140625 equal to 0.203125, a hit. Three of the four points are hits, giving an estimate of 4 times 3 divided by 4, which equals 3 — noticeably below the true value of 3.14159, illustrating exactly the kind of noise expected from only four trials, and the reason the convergence argument above requires letting n grow rather than trusting any single small batch.
