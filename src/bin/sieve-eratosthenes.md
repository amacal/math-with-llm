# Sieve of Eratosthenes

## Overview

The sieve finds all primes up to n by elimination rather than individual testing. Every integer from 2 to n starts as a candidate. The algorithm repeatedly takes the smallest surviving candidate — which must be prime — and crosses out all of its multiples. When no candidate remains whose square is still within range, all surviving numbers are prime.

This is qualitatively different from Miller-Rabin, which tests one number at a time. The sieve amortises work across all composites at once, making it efficient when the entire set of primes up to n is needed, rather than a single primality answer for one large number.

## Why we stop at sqrt(n)

Any composite number c with c ≤ n must have a prime factor q satisfying q ≤ sqrt(c) ≤ sqrt(n). This means every composite below n has at least one factor that lies within the range we sieve. Once we have processed all primes up to sqrt(n), every composite has been crossed out by one of those primes. Stopping earlier would miss composites; stopping later wastes work but does not change the answer.

## Why we start each pass at p * p

When processing prime p, the multiples p * 2, p * 3, ..., p * (p-1) have all already been crossed out. Each k < p was either prime — in which case it was processed before p, and p * k was crossed out then — or composite, in which case it has a prime factor q < k < p, and p * k was crossed out when q was processed. The first multiple that could still be unmarked at this point is p * p.

## Correctness

No false positives: a number survives (remains marked) only if no prime q smaller than it ever crossed it out, meaning it has no prime divisor below itself. A number with no prime divisor below itself is prime.

No false negatives: every composite c ≤ n has a prime factor q ≤ sqrt(n). When q was processed, every multiple of q up to n was crossed out, including c. So no composite can survive.

Together these two directions show that the set of surviving numbers is exactly the set of primes up to n.

## Complexity

The dominant cost comes from the crossing-out phase. For each prime p, the inner loop performs approximately n/p crossings, so the total number of crossings across all primes is proportional to n times the sum of the reciprocals of the primes up to n. That total is captured by

$$\text{total crossings} \;\propto\; n \sum_{p \le n,\ p \text{ prime}} \frac{1}{p}$$

which says that the running time is governed entirely by how fast the sum of prime reciprocals grows. Mertens' theorem — a classical result from analytic number theory — states that this sum grows like log(log(n)), so substituting it in gives the sieve's total running time:

$$T(n) = O(n \log \log n)$$

This is dramatically better than the O(n · sqrt(n)) cost of trial-dividing each number individually, and also better than O(n log n), since log(log(n)) grows extraordinarily slowly — for any n that fits in memory, log(log(n)) is a small single-digit number.

For contrast, the ordinary harmonic series grows much faster:

$$1 + \frac{1}{2} + \frac{1}{3} + \cdots + \frac{1}{n} = O(\log n)$$

Restricting the sum to prime denominators only, as the sieve's analysis does, drops the growth rate from log(n) all the way to log(log(n)), because primes thin out — by the prime number theorem there are only about n / log(n) primes up to n, so the reciprocal terms being summed become increasingly sparse.

Space is O(n), for the boolean scratch array — the unavoidable cost of remembering, for every number up to n, whether it has been crossed out.

## Edge cases

When n is smaller than 4, limit = floor(sqrt(n)) is at most 1, so the range 2..=limit in the crossing-out loop is empty and no composite is ever marked. This is not a bug: for n = 2 and n = 3, every number in the range [2, n] genuinely is prime, so there is nothing to cross out, and the second loop still correctly collects [2] or [2, 3]. For n = 0 or n = 1, both loops run over an empty range and the function correctly returns an empty vector, since there are no primes at or below 1.

## Worked example

Sieve up to n = 20, where the scratch array starts all true at every index from 2 through 20. Since sqrt(20) is approximately 4.47, the crossing-out phase only considers candidates p up to limit = 4.

Processing p = 2 first: it starts at p·p = 4 and steps forward by 2, crossing out 4, 6, 8, 10, 12, 14, 16, 18, and 20 — every even number from 4 upward.

Processing p = 3 next: it starts at p·p = 9 and steps forward by 3, crossing out 9 and 15 (12 and 18 were already crossed out by p = 2, so marking them again has no effect).

The loop then reaches p = 4, but scratch[4] is already false from the p = 2 pass, so nothing happens there — 4 was never a genuine candidate, it just had not been explicitly skipped yet. After p = 4, the next candidate would be p = 5, but 5 exceeds the limit of 4, so the crossing-out phase stops.

Collecting every index from 2 to 20 that is still marked true gives the survivors: 2, 3, 5, 7, 11, 13, 17, 19.

Verify by hand: every even number above 2 is gone because it is a multiple of 2; 9 and 15 are gone because they are multiples of 3; nothing needed to happen at 5 or 7, because their squares (25 and 49) already exceed 20, so any composite multiple of 5 or 7 within range was already crossed out by a smaller prime factor. The eight survivors are exactly the primes below 20.
