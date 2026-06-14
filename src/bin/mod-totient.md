# Euler's Totient Function

## What the function computes

Euler's totient function φ(n) counts the integers in [1, n] that are coprime to n — that is, integers k where gcd(k, n) = 1. For example, φ(12) = 4 because only {1, 5, 7, 11} share no common factor with 12. The remaining eight numbers are divisible by 2 or 3, which are the prime factors of 12.

## Multiplicativity

The central property that makes φ computable efficiently is multiplicativity: when gcd(m, n) = 1,

$$\varphi(mn) = \varphi(m) \cdot \varphi(n)$$

This fails when the two factors share a prime — for instance, φ(4) = 2 but φ(2)·φ(2) = 1, because 4 = 2·2 and gcd(2, 2) ≠ 1. The correct factorization must use coprime parts. Since distinct prime powers are always coprime, the prime factorization n = p₁^k₁ · p₂^k₂ · … · pₘ^kₘ always gives coprime parts, and φ decomposes cleanly across them.

## φ on prime powers

For a prime p and exponent k, the integers in [1, p^k] that fail to be coprime to p^k are exactly the multiples of p. The multiples of p in [1, p^k] form the sequence p, 2p, 3p, …, p^k. Dividing each term by p gives 1, 2, 3, …, p^(k−1), so there are exactly p^(k−1) of them. Therefore:

$$\varphi(p^k) = p^k - p^{k-1}$$

This says: start with all p^k integers, then subtract the p^(k−1) multiples of p that are not coprime to p^k.

## The general formula

Combining multiplicativity with the prime power formula, for n = p₁^k₁ · … · pₘ^kₘ:

$$\varphi(n) = \prod_{i=1}^{m} \left( p_i^{k_i} - p_i^{k_i - 1} \right)$$

Each factor p^k − p^(k−1) can be rewritten as p^k(1 − 1/p), which gives the equivalent product formula φ(n) = n · ∏(1 − 1/p) over all distinct prime factors p of n.

## Algorithm

The implementation factors n by trial division. A candidate p starts at 2 and increments by 1. For each p, n is divided by p as many times as possible, recording the exponent k. When all copies of p are removed, t = p^k is computed and the factor (t − t/p) is multiplied into the result. The loop runs while p·p ≤ n (current n, shrinking as factors are removed).

After the loop, if n > 1, then the remaining n is a prime factor with exponent 1, contributing the factor (n − 1) to the result.

The key correctness invariant is that composite candidates (p = 4, 6, 9, …) never divide the current n, because all their prime factors were already divided out in earlier iterations. So only actual primes do any work.

## Why the remaining n is always prime

When the loop exits, p·p > n for the current (reduced) n. Suppose n > 1 and n were composite: then n = a·b for some a, b > 1, and at least one of them — say a — satisfies a ≤ sqrt(n). But then a ≤ p−1 was already tested by the loop, and since a divides n it would have been divided out. Contradiction. Therefore n must be prime.

## Complexity

The outer loop runs at most O(sqrt(n)) iterations in the worst case. That worst case is when n is a large prime: no factor is found until the loop exits, having tested every p from 2 up to sqrt(n). For highly composite n the loop exits much earlier as factors are removed and the current n shrinks. Each inner division loop runs O(log n) times in total across all primes (since each prime p contributes at most log_p(n) divisions). The dominant cost is the O(sqrt(n)) outer iterations.

## Worked example

Compute φ(45). First, factor 45 by trial division.

Start with n = 45, p = 2. Since 2 does not divide 45, move to p = 3. Since 3 divides 45: 45 → 15 → 5, so k = 2 and t = 9. Multiply result by 9 − 9/3 = 9 − 3 = 6. Now n = 5, p = 4. Since 4·4 = 16 > 5, the loop exits. Remaining n = 5 > 1, so multiply result by 5 − 1 = 4.

Final answer: φ(45) = 6 · 4 = 24.

Verify directly: 45 = 3² · 5. Numbers in [1, 45] not coprime to 45 are those divisible by 3 or 5. By inclusion-exclusion: 15 multiples of 3, 9 multiples of 5, 3 multiples of 15, giving 15 + 9 − 3 = 21 excluded. So 45 − 21 = 24. Matches.
