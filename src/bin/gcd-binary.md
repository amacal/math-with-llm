# Binary GCD (Stein's Algorithm)

Published by Josef Stein in 1967, though the algorithm was already known in ancient China (Mathematical Classic of Sun Zi, 3rd–5th century AD).

## Core idea
Exploits two properties of GCD:
1. `gcd(a, b) = gcd(a, b - a)` — GCD is preserved under subtraction
2. `gcd(2a, 2b) = 2 * gcd(a, b)` — common factors of 2 can be pulled out

All three reduction cases follow from these two properties.

## The three cases
- Both even: `gcd(a, b) = 2 * gcd(a >> 1, b >> 1)` — factor out the 2, remember it in `k`
- One even: `gcd(a, b) = gcd(a >> 1, b)` — the factor of 2 in `a` is irrelevant since `b` is odd
- Both odd: subtract the smaller from the larger — the result is always even, so the next step is guaranteed to be a shift

## Base case
When one number reaches zero, the answer is `(a + b) << k`. Since exactly one is zero, `a + b` is the surviving number, and `k` restores the common factors of 2 that were stripped out.

## Complexity
O(log(min(a, b))) — same as Euclidean GCD. Subtraction and shift steps must interleave (subtracting two odds always produces an even), so every two steps reduces at least one bit. The advantage over Euclidean GCD is practical: no division required, only shifts and subtracts — relevant on hardware without fast integer division.

## Contrast with Euclidean GCD
Euclidean GCD exploits `gcd(a, b) = gcd(b, a mod b)` — one step can eliminate many bits at once via division. Binary GCD avoids division entirely but achieves the same asymptotic complexity by guaranteeing a bit reduction every two steps.
