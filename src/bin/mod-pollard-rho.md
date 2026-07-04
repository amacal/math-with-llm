# Pollard's Rho Factoring Algorithm

## Overview

Given a composite integer n, Pollard's Rho finds a non-trivial factor d with 1 < d < n. It does not prove primality — a Miller-Rabin check comes first to confirm that n is composite before the factoring machinery runs. The algorithm is probabilistic: it restarts with a different constant when an attempt fails.

## The core insight

Let p be any prime factor of n (unknown to us). If we can find two values x and y in [0, n) with x ≡ y (mod p) but x ≢ y (mod n), then p divides |x - y| (by definition of congruence mod p), and p divides n (since p is a factor). A standard property of GCD says that if p divides both arguments, then p divides their GCD as well, which pins down the quantity of interest:

$$1 < p \leq \gcd(|x - y|,\, n) < n$$

The left inequality says the GCD is a genuine common factor greater than 1, and the right inequality holds because x ≢ y (mod n) means n cannot divide |x - y| (both values lie in [0, n), so |x - y| < n). Together these say gcd(|x - y|, n) is a non-trivial factor of n — exactly what we set out to find.

The key asymmetry is this: we only need a collision mod p, not mod n. The residue space mod p has size p, which is much smaller than n, so collisions appear far sooner than a brute-force search mod n would require.

## Why O(sqrt(p)) steps suffice: the birthday paradox

The birthday paradox says that if you draw values uniformly at random from a set of size m, you expect a repeated value after approximately sqrt(m) draws. This follows from an approximation for the probability that all k draws are distinct,

$$P(\text{all distinct}) \approx e^{-k^2 / 2m}$$

which decays toward zero once k grows past roughly sqrt(m), meaning a repeated value becomes overwhelmingly likely by that point.

The iteration function used to generate the pseudo-random sequence is

$$f(x) = (x^2 + c) \bmod n$$

chosen because squaring is cheap to compute while still mixing the input enough that, modulo any prime factor p of n, the resulting sequence behaves like a stream of independent random draws from {0, ..., p-1}. Applying the birthday paradox with m = p: after about sqrt(p) iterations, two sequence values x_i and x_j will satisfy x_i ≡ x_j (mod p). Since p <= sqrt(n) when n has two roughly equal prime factors, we have sqrt(p) <= n^(1/4), giving the overall expected complexity of O(n^(1/4)).

## Floyd's cycle detection

Storing all sqrt(p) generated values to check every pair would cost O(sqrt(p)) space. Floyd's algorithm eliminates this: run two pointers (tortoise and hare) through the same sequence, the tortoise advancing one step at a time and the hare two. Since the range of f is finite, the sequence must eventually cycle. Once both pointers are inside the cycle, the hare gains exactly one position on the tortoise per round, so they must meet after at most cycle-length additional steps. When they meet, their values satisfy x_i ≡ x_{2i} (mod p) — a collision mod p — without any stored history.

The sequence has a "rho" shape (a tail of non-repeating values followed by a cycle, like the Greek letter ρ) because f is deterministic and the range is finite: once a value repeats, the sequence enters a cycle and never leaves it. The tail consists of values visited only before the cycle begins. The name of the algorithm comes from this shape.

## Failure modes and restarts

Two failure modes exist. First, if gcd(|x - y|, n) = n, the pointers have collided mod n rather than just mod p — the collision is too strong and yields a trivial result. Second, some values of c produce degenerate sequences that cycle without the pointers ever achieving a useful collision. In both cases, the attempt returns None and the outer loop retries with a different constant c in f(x) = x^2 + c. The values c = 0 and c = 2 are conventionally avoided: c = 0 reduces f to pure squaring (a highly structured sequence) and c = 2 is empirically poor.

## Correctness

The algorithm is correct whenever it returns a factor d: by construction, d = gcd(|x - y|, n) and 1 < d < n, so d is a non-trivial divisor of n. When it returns None, it signals that no factor was found with the attempted c values and iteration limits — not that n is prime. Primality is the responsibility of Miller-Rabin, called before the factoring loop.

## Complexity

Expected time: O(n^(1/4)) for a semiprime (n = p * q with p and q roughly equal primes of size sqrt(n)). More precisely O(p^(1/2)) where p is the smallest prime factor of n — smaller factors are found faster. Space: O(1) (two pointers, no stored history).

## Edge cases

Even numbers are handled as a special case before the main algorithm runs: if n is even, the factor 2 is returned immediately without ever constructing the sequence. This is necessary because f(x) = x^2 + c mod n cannot generate a genuine collision mod 2 through the normal sequence dynamics — the residues mod 2 only take two values, and squaring mod 2 is the identity, so the birthday-paradox argument that guarantees fast collisions elsewhere does not carry over to this smallest possible prime factor. Small n also needs care: for n = 1 there is no non-trivial factor to find, and for n itself prime the algorithm is not meant to be called at all, since primality is checked by Miller-Rabin beforehand rather than by Pollard's Rho discovering that no factor exists.

## Worked example

Factor n = 77 = 7 × 11 using f(x) = (x^2 + 1) mod 77, starting with hare = 0, tortoise = 1.

The sequence mod 7: f(x) ≡ x^2 + 1 (mod 7). Values: f(0) = 1, f(1) = 2, f(2) = 5, f(5) ≡ 26 ≡ 5 (mod 7), so 5 is a fixed point mod 7.

Tracing the pointers:

After step 1: hare = f(0) = 1, tortoise = f(f(1)) = f(2) = 5. Both mod 7: 1 and 5. Differ. gcd(|1 - 5|, 77) = gcd(4, 77) = 1.

After step 2: hare = f(1) = 2, tortoise = f(f(5)) = f(26). Now f(26) = (676 + 1) mod 77 = 677 mod 77 = 677 - 8 * 77 = 677 - 616 = 61. Both mod 7: hare = 2 mod 7 = 2, tortoise = 61 mod 7 = 61 - 8*7 = 5. Still differ. gcd(|2 - 61|, 77) = gcd(59, 77) = 1.

After step 3: hare = f(2) = 5, tortoise = f(f(61)). f(61) = (3721 + 1) mod 77 = 3722 mod 77 = 3722 - 48*77 = 3722 - 3696 = 26. f(26) = 61 (computed above). Both mod 7: hare = 5 mod 7 = 5, tortoise = 61 mod 7 = 5. Equal mod 7! gcd(|5 - 61|, 77) = gcd(56, 77). Since 56 = 7 × 8 and 77 = 7 × 11, gcd = 7.

A non-trivial factor of 77 is found at step 3: d = 7.
