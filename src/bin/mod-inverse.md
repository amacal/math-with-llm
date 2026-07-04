# Modular Inverse

## Overview

In regular arithmetic, every non-zero number has a multiplicative inverse: the inverse of 3 is 1/3, because 3 × 1/3 = 1. Modular arithmetic works with integers only, so fractions are not available. The modular inverse of a modulo m is an integer x such that:

$$a \cdot x \equiv 1 \pmod{m}$$

This is the closest integer analogue to 1/a. It does not always exist — and knowing exactly when it does is the key to understanding the algorithm.

## Correctness

The inverse of a modulo m exists if and only if gcd(a, m) = 1. To see why it fails otherwise, consider a = 2 and m = 4, where gcd(2, 4) = 2. As x ranges over the integers, 2·x mod 4 only ever lands on {0, 2}, because every value of 2·x is itself a multiple of 2. Since 1 is not a multiple of 2, it is never reached, and no inverse exists. The same argument works for any gcd(a, m) = d > 1: every value of a·x mod m is a multiple of d, and since 1 is not divisible by d whenever d > 1, the equation a·x ≡ 1 (mod m) has no solution.

When gcd(a, m) = 1, the extended Euclidean algorithm gives integers x and y satisfying Bézout's identity:

$$a \cdot x + m \cdot y = 1$$

Reducing both sides modulo m, the term m·y vanishes since it is a multiple of m, leaving a·x ≡ 1 (mod m) — so x is exactly the modular inverse being looked for. Producing this x is exactly the coefficient-tracking computation already proved correct in `gcd-euclidean-extended.md`; this session reuses that algorithm as-is and adds only the existence check above and the range adjustment below.

That range adjustment is needed because the Bézout coefficient x produced by extended GCD may be negative. Both x and x + m are valid inverses, since adding m to x does not change the congruence:

$$a \cdot (x + m) = a \cdot x + a \cdot m \equiv a \cdot x \equiv 1 \pmod{m}$$

The convention is to return the unique representative in the range [0, m). Since the Bézout coefficient satisfies |x| < m, a single addition of m is always sufficient to bring a negative x into range.

## Complexity

The algorithm is a thin wrapper over extended GCD, adding only a constant amount of work for the existence check and normalisation. The complexity is therefore the same:

$$O(\log(\min(a, m)))$$

## Edge cases

When gcd(a, m) > 1 no inverse exists and the algorithm returns None. When a = 0, gcd(0, m) = m, so the inverse exists only if m = 1. When m = 1, every integer is congruent to 0 modulo 1, so the only element of [0, 1) is 0, and the inverse is Some(0) for any a. When m = 0, modular arithmetic is undefined and the algorithm returns None.

## Worked example

Find the inverse of 5 modulo 8 — a case chosen so the raw Bézout coefficient comes out negative, exercising the normalisation step rather than skipping it. Running the Euclidean division steps on (8, 5) gives 8 = 1·5 + 3, then 5 = 1·3 + 2, then 3 = 1·2 + 1, then 2 = 2·1 + 0, confirming gcd(5, 8) = 1 so an inverse exists. Back-substituting from the second-to-last step upward recovers the Bézout coefficients: starting from 1 = 3 − 1·2, replace 2 using 2 = 5 − 1·3 to get 1 = 3 − (5 − 3) = 2·3 − 5, then replace 3 using 3 = 8 − 1·5 to get:

$$1 = 2 \cdot (8 - 5) - 5 = 2 \cdot 8 - 3 \cdot 5$$

Reading off the coefficient attached to a = 5 gives x = −3. Checking directly: 5·(−3) + 8·2 = −15 + 16 = 1, confirming the Bézout identity holds before any adjustment. Since −3 falls outside the target range [0, 8), the normalisation step adds m once: −3 + 8 = 5, so the modular inverse of 5 modulo 8 is reported as 5. As a sanity check, 5 × 5 = 25 = 3·8 + 1, so 25 ≡ 1 (mod 8), confirming 5 is indeed its own inverse modulo 8.

For contrast, attempt the same procedure on a pair that should fail: find the inverse of 4 modulo 6. Since gcd(4, 6) = 2, the Correctness section above predicts no inverse exists, and a direct check confirms it — as x ranges over 0, 1, 2, 3, 4, 5, the value 4x mod 6 only ever takes the values 0, 4, 2, 0, 4, 2, repeating without ever landing on 1. Running extended GCD on (4, 6) would report a GCD of 2 rather than 1, and the implementation returns None at that point instead of attempting a normalisation that could never produce a valid inverse.
