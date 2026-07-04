# Chinese Remainder Theorem (CRT)

## Overview

The Chinese Remainder Theorem answers a natural question: if you know two remainders of an unknown number — say, it leaves remainder 2 when divided by 3, and remainder 3 when divided by 5 — can you recover the number? The answer is yes, provided the two moduli share no common factor. The solution is unique within the range from 0 to the product of the two moduli, and the construction reduces to a single modular inverse computation.

## Problem statement

Given two congruences with coprime moduli:

$$x \equiv a_1 \pmod{m_1} \qquad x \equiv a_2 \pmod{m_2} \qquad \gcd(m_1, m_2) = 1$$

find the unique solution x in the range from 0 to the product:

$$x \in [0,\ m_1 \cdot m_2)$$

## Construction

Since x must satisfy the first congruence, it can be written in the form:

$$x = a_1 + k \cdot m_1$$

for some integer k. This satisfies the first congruence for any k. Substituting into the second congruence and isolating k:

$$a_1 + k \cdot m_1 \equiv a_2 \pmod{m_2}$$

$$k \cdot m_1 \equiv a_2 - a_1 \pmod{m_2}$$

$$k \equiv (a_2 - a_1) \cdot m_1^{-1} \pmod{m_2}$$

The modular inverse of m1 exists because the moduli are coprime (see `mod-inverse.md`). Once k is determined, the solution is x = a1 + k·m1, reduced to the range from 0 to the product of the moduli.

## Correctness

Two things need proving: that a solution exists at all, and that it is the only one in range. For existence, the range from 0 to the product m1·m2 contains exactly m1·m2 integers, and the set of all possible pairs of remainders also contains exactly m1·m2 combinations — m1 choices for the first remainder and m2 for the second. The uniqueness argument below shows that no two different values of x in that range can produce the same pair of remainders, so m1·m2 distinct inputs produce m1·m2 distinct pairs; since there are exactly m1·m2 possible pairs, every combination must be hit, including the target pair (a1, a2).

For uniqueness, suppose x and y both satisfy both congruences. Then m1 divides x − y and m2 divides x − y. Since the two moduli are coprime, both dividing the same value means their product divides it too:

$$m_1 \cdot m_2 \mid (x - y)$$

But both x and y lie in the range [0, m1·m2), so their difference lies strictly between the negation and the value of the product. The only multiple of the product in that open interval is zero, so x equals y — there is exactly one solution in range.

## Complexity

The dominant cost is the modular inverse computation, which calls extended GCD. Everything else is constant work:

$$O(\log(\min(m_1, m_2)))$$

## Edge cases

Computing the difference in the construction requires care with unsigned 64-bit integers: when the second remainder is smaller than the first, naive subtraction would underflow. The safe form reduces the difference before subtracting from the modulus:

$$m_2 - ((a_1 - a_2) \bmod m_2)$$

An additional mod m2 is applied to the whole expression to handle the case where the difference is exactly divisible by m2 — without it, the inner expression would equal m2 itself rather than 0. When the moduli are not coprime, the modular inverse call returns None and the algorithm propagates it upward. Inputs larger than their modulus are reduced at entry, and intermediate products are checked for overflow with `checked_mul`.

## Worked example

Solve the system:

$$x \equiv 2 \pmod{3} \qquad x \equiv 3 \pmod{5}$$

Write the general form satisfying the first congruence, then substitute into the second:

$$x = 2 + 3k \qquad 2 + 3k \equiv 3 \pmod{5} \qquad 3k \equiv 1 \pmod{5}$$

Find the inverse of 3 modulo 5. By inspection, 3 · 2 = 6 ≡ 1 (mod 5), so the inverse is 2. Then:

$$k \equiv 1 \cdot 2 = 2 \pmod{5} \qquad x = 2 + 3 \cdot 2 = 8$$

Verification: 8 mod 3 = 2 and 8 mod 5 = 3, both matching the original congruences, and 8 lies in the expected range [0, 15).
