# Baby-Step Giant-Step (Discrete Logarithm)

## Overview

Given a prime p, a generator g of the multiplicative group (Z/pZ)*, and a target h in {1, ..., p-1}, Baby-Step Giant-Step (BSGS) finds the integer x in 0..p-2 such that g^x ≡ h (mod p). This x is called the discrete logarithm of h base g modulo p. The algorithm runs in O(sqrt(p) * log p) time and O(sqrt(p)) space, compared to O(p * log p) for naive brute force.

## The group structure

The integers {1, 2, ..., p-1} form a multiplicative group under mod p when p is prime. Every non-zero element has a multiplicative inverse (guaranteed by p being prime), and the group has order p-1. A generator g is an element whose powers g^0, g^1, ..., g^(p-2) cycle through every element of the group exactly once before returning to 1 at g^(p-1). Every h in the group has a unique discrete logarithm in 0..p-2 with respect to g.

## The key decomposition

Let m = ceil(sqrt(p)). Any exponent x in 0..p-2 can be written as x = k*m + y where k and y each range over 0..m-1. The maximum value of this representation is (m-1)*m + (m-1) = m^2 - 1. Since m = ceil(sqrt(p)) implies m^2 >= p, we have m^2 - 1 >= p-1 > p-2, so every valid exponent is reachable. Using floor instead of ceil would leave m^2 < p in some cases, creating gaps.

Substituting x = k*m + y into the equation g^x ≡ h (mod p) gives:

$$g^{km} \cdot g^y \equiv h \pmod{p}$$

Multiplying both sides by g^(-y) isolates the k-dependent side:

$$g^{km} \equiv h \cdot g^{-y} \pmod{p}$$

The left side varies with k; the right side varies with y. Both range over m values. Finding a pair (k, y) where the two sides are equal gives x = k*m + y.

## The two phases

The baby-step phase computes all m values of the right-hand side — h * g^(-y) mod p for y = 0..m-1 — and stores them in a hash map as value → y. This costs O(m) time and O(m) space. The giant-step phase computes g^(km) mod p for k = 0..m-1 and checks each result against the hash map. The first hit gives k and y, and the answer is x = k*m + y.

The name comes from the step sizes: y increments by 1 in the exponent each baby step, while k increments the exponent by m (a giant step relative to the base). Geometrically, the exponent space is a grid of m rows and m columns; baby steps index one column, giant steps walk row by row, and the hash map finds the intersection.

## Why the hash map holds baby steps, not giant steps

Baby steps depend on h (they compute h * g^(-y)), so the table must be rebuilt for each new query. Giant steps compute g^(km), which depends only on g and p and not on h. If multiple discrete logs are needed for the same group (same g and p), the giant-step table can be precomputed once and reused across all queries, with only the baby-step phase re-run for each new h.

## Finding a generator

For a safe prime p — one where p-1 = 2*q and q is also prime — a candidate g is a generator if and only if both g^2 ≢ 1 (mod p) and g^q ≢ 1 (mod p). This follows because the only proper divisors of p-1 = 2q are 2 and q, so checking that g raised to each of these is not 1 confirms that no proper subgroup contains g, forcing its order to be the full p-1. This safe-prime shortcut is a special case of a more general test: for an arbitrary prime p, the same idea works by checking g^((p-1)/q) for every distinct prime factor q of p-1, not just the two divisors available when p-1 happens to have the special form 2*q. That general test, together with a proof that a generator always exists for any prime p, is the subject of the Primitive Roots mod p session.

## Correctness

The algorithm is correct because the decomposition x = k*m + y covers all exponents in 0..p-2 without gaps (as argued above), and the hash map lookup finds a matching pair in O(1) per giant step. The returned x always satisfies g^x ≡ h (mod p) by construction, since it is only returned after an explicit match between a baby-step value and a giant-step value confirms the underlying congruence.

## Complexity

Time: O(sqrt(p) * log p). Both phases run O(sqrt(p)) iterations; each iteration calls mod_exp at cost O(log p). Space: O(sqrt(p)) for the hash map storing m baby-step values.

## Edge cases

When the function returns None, it means no matching pair (k, y) was found among the O(sqrt(p)) values checked. The only structural case where this must happen is h = 0: since g^x mod p is a nonzero residue for every x when p is prime (the group {1, ..., p-1} excludes 0 entirely), no exponent can ever produce h = 0, so a query with h = 0 always exhausts the search and correctly returns None. This implementation also assumes p is prime and g is a genuine generator of the full group without independently verifying either — a composite p breaks the entire group-theoretic argument silently, since (Z/pZ)* is only guaranteed to be a group of order p-1 when p is prime. The Primitive Roots mod p session addresses exactly this gap by checking primality before doing any group-theoretic work.

## Worked example

Take p = 11, g = 2, h = 8. We want x such that 2^x ≡ 8 (mod 11). The answer is x = 3.

Set m = ceil(sqrt(11)) = 4. The inverse of g mod p is mod_inverse(2, 11) = 6, since 2 * 6 = 12 ≡ 1 (mod 11).

Baby steps — compute h * inv^y mod p for y = 0..3:

$$y=0: \quad 8 \cdot 6^0 \equiv 8 \pmod{11}$$

$$y=1: \quad 8 \cdot 6^1 \equiv 48 \equiv 4 \pmod{11}$$

$$y=2: \quad 8 \cdot 6^2 \equiv 8 \cdot 36 \equiv 8 \cdot 3 \equiv 24 \equiv 2 \pmod{11}$$

$$y=3: \quad 8 \cdot 6^3 \equiv 8 \cdot 216 \equiv 8 \cdot 7 \equiv 56 \equiv 1 \pmod{11}$$

Hash map: {8 → 0, 4 → 1, 2 → 2, 1 → 3}.

Giant steps — compute 2^(k*4) mod 11 for k = 0..3:

$$k=0: \quad 2^0 = 1 \quad \text{— found in map at } y=3$$

Match at k = 0, y = 3. Answer: x = 0 * 4 + 3 = 3. Verify: 2^3 = 8 ≡ 8 (mod 11). Correct.
