# Segmented Sieve

## Overview

The basic Sieve of Eratosthenes finds all primes up to n using O(n) space for a scratch array. When n is large — say 10^12 — that array requires roughly 125 gigabytes even with a bitmap, which is not workable. The segmented sieve solves this by observing that you do not need to materialise the entire range at once. Given a target range [L, R], it is enough to sieve one small window at a time, reusing the same scratch array for each window.

The key insight is that every composite c ≤ R has a prime factor p ≤ sqrt(c) ≤ sqrt(R). This means the only small primes needed to cross out all composites in [L, R] are those up to sqrt(R), and there are at most O(sqrt(R)) of them. Those small primes are precomputed with one call to the basic sieve on the range [2, sqrt(R)], which costs O(sqrt(R)) space — a dramatic reduction from O(R).

## Algorithm

The procedure for sieving [L, R] has three steps. First, run the basic sieve up to ceil(sqrt(R)) to obtain all small primes. Second, allocate a boolean scratch array of size R - L + 1, indexed so that position i represents the number L + i. Third, for each small prime p, find its first multiple that falls within [L, R] and cross out all subsequent multiples by stepping forward in increments of p.

The first multiple of p that is ≥ L is ceil(L / p) · p, which in integer arithmetic is written `((L + p - 1) / p) * p`. One edge case requires attention: if this formula returns p itself (possible when L ≤ p), the number p is prime and must not be marked composite. The fix is a single check: if the computed start equals p, advance by one more step to 2p.

## Why ceil rather than floor for the sqrt limit

When computing limit = sqrt(R) to pass to the basic sieve, using floor can silently drop a prime. If R = 49, the exact square root is 7, but floating point might return 6.999..., which truncates to 6. Then 7 is never used as a small prime, and 49 = 7 × 7 survives in the window marked as prime. Using ceil(sqrt(R)) prevents this: even if the float is slightly below the true value, rounding up restores the correct prime.

## Correctness

No composite in [L, R] can survive. Let c be any composite with L ≤ c ≤ R. Since c is composite it has a prime factor p satisfying p ≤ sqrt(c). Because c ≤ R, we have sqrt(c) ≤ sqrt(R), so p ≤ sqrt(R). Therefore p appears in the precomputed list of small primes. When p is processed, every multiple of p in [L, R] is crossed out, and c is one such multiple. No composite escapes.

No prime in [L, R] is falsely crossed out. A number n is crossed out only when it appears as a multiple of some small prime p, meaning p divides n. If p divides n and p < n, then n is composite by definition. The edge-case guard from the Algorithm section above ensures p is never used to cross out itself.

Together these two directions show the surviving numbers are exactly the primes in [L, R].

## Complexity

The precomputation runs the basic sieve on an input of size sqrt(R), costing

$$O\!\left(\sqrt{R}\,\log\log\sqrt{R}\right)$$

time and the same in space for the small-prime list.

For the window, each small prime p contributes approximately (R - L) / p crossings. Summing over all primes p ≤ sqrt(R) gives a total proportional to (R - L) times the sum of prime reciprocals up to sqrt(R). By Mertens' theorem that sum grows as log(log(sqrt(R))), giving window sieving a cost of

$$O\!\left((R - L)\,\log\log\sqrt{R}\right)$$

Since log log sqrt(R) = log((log R)/2) ≈ log log R, both terms share the same logarithmic factor. The space is O(sqrt(R)) for the small primes and O(R - L) for the window; choosing the window size around sqrt(R) makes both terms O(sqrt(R)), which is the headline improvement over the basic sieve's O(R) space.

The alternative — testing each number in [L, R] independently by trial division — costs O((R - L) · sqrt(R)) time, far worse than the segmented sieve's O((R - L) · log log R).

## Worked example

Find all primes in the window [10, 30]. Before touching the window itself, compute the small primes needed to sieve it: since ceil(sqrt(30)) = 6, running the basic sieve up to 6 gives the small-prime list {2, 3, 5} — these are the only primes that could possibly divide a composite number as large as 30.

Next, allocate a scratch array of size 30 - 10 + 1 = 21, all initialized to true, using the convention that index i represents the number 10 + i, so index 0 is the number 10 and index 20 is the number 30.

Process the first small prime, p = 2. Its first multiple at or above 10 is ceil(10/2) · 2 = 10, so crossing out begins there and steps forward by 2, marking indices 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, and 20 as composite — these correspond to the numbers 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, and 30, every even number in the window.

Process the second small prime, p = 3. Its first multiple at or above 10 is ceil(10/3) · 3 = 12, so crossing out begins there and steps forward by 3, marking indices 2, 5, 8, 11, 14, 17, and 20 — the numbers 12, 15, 18, 21, 24, 27, and 30. Several of these (12, 18, 24, 30) were already marked composite by p = 2, and marking them again is harmless.

Process the third and final small prime, p = 5. Its first multiple at or above 10 is ceil(10/5) · 5 = 10, so crossing out begins there and steps forward by 5, marking indices 0, 5, 10, 15, and 20 — the numbers 10, 15, 20, 25, and 30. Note that 25 is crossed out only here, by p = 5, since neither 2 nor 3 divides it.

With all three small primes processed, collect every index whose entry is still true: indices 1, 3, 7, 9, 13, and 19, corresponding to the numbers 11, 13, 17, 19, 23, and 29. These six survivors are exactly the primes in [10, 30].
