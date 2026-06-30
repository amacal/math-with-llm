# Sieve of Eratosthenes

## Core idea

The sieve finds all primes up to n by elimination rather than individual testing. Every integer from 2 to n starts as a candidate. The algorithm repeatedly takes the smallest surviving candidate — which must be prime — and crosses out all of its multiples. When no candidate remains whose square is still within range, all surviving numbers are prime.

This is qualitatively different from Miller-Rabin, which tests one number at a time. The sieve amortises work across all composites at once, making it efficient when the entire set of primes up to n is needed.

## Why we stop at sqrt(n)

Any composite number c with c ≤ n must have a prime factor q satisfying q ≤ sqrt(c) ≤ sqrt(n). This means every composite below n has at least one factor that lies within the range we sieve. Once we have processed all primes up to sqrt(n), every composite has been crossed out by one of those primes. Stopping earlier would miss composites; stopping later wastes work but does not change the answer.

## Why we start each pass at p * p

When processing prime p, the multiples p * 2, p * 3, ..., p * (p-1) have all already been crossed out. Each k < p was either prime — in which case it was processed before p, and p * k was crossed out then — or composite, in which case it has a prime factor q < k < p, and p * k was crossed out when q was processed. The first multiple that could still be unmarked at this point is p * p.

## Correctness argument

No false positives: a number survives (remains marked) only if no prime q smaller than it ever crossed it out, meaning it has no prime divisor below itself. A number with no prime divisor below itself is prime.

No false negatives: every composite c ≤ n has a prime factor q ≤ sqrt(n). When q was processed, every multiple of q up to n was crossed out, including c. So no composite can survive.

Together these two directions show that the set of surviving numbers is exactly the set of primes up to n.

## Complexity

**Time: O(n log log n).** For each prime p ≤ n, the inner loop does approximately n/p crossings. The total work is therefore proportional to n times the sum of 1/p over all primes p ≤ n. A classical result (Mertens' theorem) states that this sum of prime reciprocals grows as log(log(n)), giving total time O(n log log n). This is much better than O(n sqrt(n)) from trial-dividing each number individually, and also better than O(n log n) — log(log(n)) grows extraordinarily slowly.

For contrast, the harmonic series 1 + 1/2 + 1/3 + ... + 1/n grows as log(n). Restricting to primes only gives log(log(n)) because primes thin out: by the prime number theorem there are about n / log(n) primes up to n, so the reciprocal terms become sparse.

**Space: O(n)** for the boolean scratch array. This is the unavoidable cost of knowing, for each number up to n, whether it has been crossed out.

## Worked example

Sieve up to n = 20. The scratch array starts all true at indices 2..=20.

Process p = 2 (sqrt(20) ≈ 4.47, so limit = 4). Start at p*p = 4, step by 2:
cross out 4, 6, 8, 10, 12, 14, 16, 18, 20.

Process p = 3. Start at p*p = 9, step by 3:
cross out 9, 15. (12, 18 already crossed out.)

Process p = 4: scratch[4] is false, skip.

p = 5 > limit = 4, stop the crossing-out phase.

Collect survivors: 2, 3, 5, 7, 11, 13, 17, 19.

Verify by hand: every even number above 2 is gone (multiples of 2); 9 and 15 are gone (multiples of 3); nothing needed from 5 or 7 because their squares (25, 49) exceed 20. The eight survivors are exactly the primes below 20.
