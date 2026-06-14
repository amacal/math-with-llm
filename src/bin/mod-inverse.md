# Modular Inverse

## Key insight

In regular arithmetic, every non-zero number has a multiplicative inverse: the inverse of 3 is 1/3, because 3 × 1/3 = 1. Modular arithmetic works with integers only, so fractions are not available. The modular inverse of a modulo m is an integer x such that:

$$a \cdot x \equiv 1 \pmod{m}$$

This is the closest integer analogue to 1/a. It does not always exist — and knowing exactly when it does is the key to understanding the algorithm.

## When does the inverse exist?

The inverse of a modulo m exists if and only if gcd(a, m) = 1. To see why, consider what happens when gcd(a, m) = d > 1. The values of a·x mod m, as x ranges over all integers, cycle through multiples of d — they are always divisible by d. Since 1 is not divisible by d, the value 1 is never reached, and no inverse exists.

When gcd(a, m) = 1, the extended Euclidean algorithm gives us integers x and y satisfying Bézout's identity:

$$a \cdot x + m \cdot y = 1$$

Reducing both sides modulo m, the term m·y vanishes since it is a multiple of m, leaving:

$$a \cdot x \equiv 1 \pmod{m}$$

So x is exactly the modular inverse we are looking for.

## Normalisation to [0, m)

The Bézout coefficient x produced by the extended GCD may be negative. Both x and x + m are valid inverses, since adding m to x does not change the congruence:

$$a \cdot (x + m) = a \cdot x + a \cdot m \equiv a \cdot x \equiv 1 \pmod{m}$$

The convention is to return the unique representative in the range [0, m). Since the Bézout coefficient satisfies |x| < m, a single addition of m is always sufficient to bring a negative x into range.

## Worked example

Find the inverse of 3 modulo 11 — that is, find x such that:

$$3 \cdot x \equiv 1 \pmod{11}$$

Run the extended Euclidean algorithm on (3, 11), tracking the Bézout coefficients for a throughout. The invariant maintained at every step is:

$$a = 3 \cdot x_a + 11 \cdot y_a$$

The first step has quotient q = 0 since 3 < 11, so the coefficients do not change — the pair simply swaps and the algorithm effectively starts from (11, 3) in the next step.

$$x_a=1,\ y_a=0 \qquad x_b=0,\ y_b=1$$
$$a = 3 \cdot 1 + 11 \cdot 0 = 3$$
$$x_b' = 1 - 0 \cdot 0 = 1,\ y_b' = 0 - 0 \cdot 1 = 0$$

&nbsp;

$$x_a=0,\ y_a=1 \qquad x_b=1,\ y_b=0$$
$$a = 3 \cdot 0 + 11 \cdot 1 = 11$$
$$x_b' = 0 - 3 \cdot 1 = -3,\ y_b' = 1 - 3 \cdot 0 = 1$$

&nbsp;

$$x_a=1,\ y_a=0 \qquad x_b=-3,\ y_b=1$$
$$a = 3 \cdot 1 + 11 \cdot 0 = 3$$
$$x_b' = 1 - 1 \cdot (-3) = 4,\ y_b' = 0 - 1 \cdot 1 = -1$$

&nbsp;

$$x_a=-3,\ y_a=1 \qquad x_b=4,\ y_b=-1$$
$$a = 3 \cdot (-3) + 11 \cdot 1 = 2$$
$$x_b' = -3 - 2 \cdot 4 = -11,\ y_b' = 1 - 2 \cdot (-1) = 3 \quad \text{(discarded, b = 0)}$$

&nbsp;

$$x_a=4,\ y_a=-1$$
$$a = 3 \cdot 4 + 11 \cdot (-1) = 1$$

The GCD is 1, confirming the inverse exists. The Bézout coefficient is xa = 4, which is already in [0, 11), so the modular inverse of 3 modulo 11 is 4.

Sanity check:

$$3 \times 4 = 12 = 11 + 1 \equiv 1 \pmod{11} \checkmark$$

## Edge cases

When gcd(a, m) > 1 no inverse exists and the algorithm returns None. When a = 0, gcd(0, m) = m, so the inverse exists only if m = 1. When m = 1, every integer is congruent to 0 modulo 1, so the only element of [0, 1) is 0, and the inverse is Some(0) for any a. When m = 0, modular arithmetic is undefined and the algorithm returns None.

## Complexity

The algorithm is a thin wrapper over extended GCD, adding only a constant amount of work for the existence check and normalisation. The complexity is therefore the same:

$$O(\log(\min(a, m)))$$
