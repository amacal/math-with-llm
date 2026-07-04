# Extended Euclidean GCD

## Overview

The basic Euclidean algorithm finds the greatest common divisor of two integers, but throws away everything except the final answer. The extended version keeps track of something extra at each step: how to express the current remainder as a combination of the two original inputs. By the time the algorithm terminates, the GCD itself is expressed that way, giving two integers x and y such that:

$$a \cdot x + b \cdot y = \gcd(a, b)$$

This equation is known as Bézout's identity, and the extended algorithm is the standard way to compute the coefficients x and y.

## Correctness

The argument is that at every step of the Euclidean algorithm, the current remainder can be written as a linear combination of the original a and b. This is maintained as an invariant by tracking a coefficient pair alongside each of the two working values. At the very start, before any step is taken, the two working values are a and b themselves, each trivially a combination of the originals:

$$a = a \cdot 1 + b \cdot 0 \qquad b = a \cdot 0 + b \cdot 1$$

So the initial coefficient pairs are (xa, ya) = (1, 0) for the first value and (xb, yb) = (0, 1) for the second. Now suppose at some point in the algorithm the two working values are each expressed as a combination of the originals:

$$\text{current}_a = a \cdot x_a + b \cdot y_a \qquad \text{current}_b = a \cdot x_b + b \cdot y_b$$

The next step computes the remainder of dividing the first value by the second. Using the same rearrangement as in the basic algorithm, the new remainder is:

$$r = \text{current}_a - q \cdot \text{current}_b$$

where q is the quotient. Substituting the linear combinations above into this expression shows that the new remainder is itself a combination of a and b:

$$r = (a \cdot x_a + b \cdot y_a) - q \cdot (a \cdot x_b + b \cdot y_b) = a \cdot (x_a - q \cdot x_b) + b \cdot (y_a - q \cdot y_b)$$

so the updated coefficients are read straight off this expression:

$$x_r = x_a - q \cdot x_b \qquad y_r = y_a - q \cdot y_b$$

Since this holds at every step and the seeds satisfy it trivially, the invariant is maintained throughout the loop. Stated precisely, at every point during the loop the current working values satisfy:

$$\text{current}_a = \text{initial}_a \cdot x_a + \text{initial}_b \cdot y_a \qquad \text{current}_b = \text{initial}_a \cdot x_b + \text{initial}_b \cdot y_b$$

Termination of the underlying reduction is identical to the basic Euclidean algorithm (see `gcd-euclidean-basic.md`): b strictly decreases each step and reaches zero, at which point the value in the a position is the greatest common divisor. When the loop terminates with b = 0, the invariant above collapses to exactly the Bézout identity:

$$\gcd(a, b) = a \cdot x_a + b \cdot y_a$$

The coefficient bookkeeping adds only constant work per iteration and does not change either the termination or the correctness argument.

## Complexity

The number of steps is O(log(min(a, b))), identical to the basic algorithm. Tracking the coefficients adds only a constant number of arithmetic operations per step, so the overall complexity is unchanged.

## Edge cases

The inputs are unsigned 64-bit integers, but the Bézout coefficients can be negative, so they are tracked as signed 64-bit integers. At each step, the quotient q is computed from unsigned values and then converted to i64. The only case where q could exceed the maximum i64 value is when the divisor b equals 1, but when b = 1 the remainder a mod b is 0, so b becomes 0 after this step and the loop exits — the coefficient update for xb and yb would be computed and immediately discarded, since the algorithm returns xa and ya rather than xb and yb, so the implementation safely skips that update entirely.

When one or both inputs are zero, the loop never executes and the seed values are returned directly:

$$\gcd(0, 0) = \text{undefined} \qquad \gcd(0, n) = n \text{ with } (x, y) = (0, 1) \qquad \gcd(n, 0) = n \text{ with } (x, y) = (1, 0)$$

The first case returns None since every positive integer divides zero. The other two follow directly from b = a·0 + n·1 and a = n·1 + 0·0. When a equals b, one iteration produces remainder 0 and the result is (n, (0, 1)). Multiple valid coefficient pairs exist in general — for example gcd(6, 3) could produce (1, 0) or (0, 2) — and the algorithm returns whichever it naturally computes.

## Worked example

Trace gcd(35, 15), tracking the coefficient pairs at each step. The invariant maintained throughout is:

$$a = 35 \cdot x_a + 15 \cdot y_a$$

updated at each step by:

$$x_a' = x_b \qquad y_a' = y_b \qquad x_b' = x_a - q \cdot x_b \qquad y_b' = y_a - q \cdot y_b$$

The pair starts at:

$$x_a=1,\ y_a=0 \qquad x_b=0,\ y_b=1$$

which matches a = 35·1 + 15·0 = 35. Since 35 = 2·15 + 5, the quotient is 2, giving the updated pair:

$$x_b' = 1 - 2 \cdot 0 = 1,\ y_b' = 0 - 2 \cdot 1 = -2$$

The second iteration shifts everything over, so the working pair becomes:

$$x_a=0,\ y_a=1 \qquad x_b=1,\ y_b=-2$$

which matches a = 35·0 + 15·1 = 15. Since 15 = 3·5 + 0, the quotient is 3, giving:

$$x_b' = 0 - 3 \cdot 1 = -3,\ y_b' = 1 - 3 \cdot (-2) = 7$$

The final step shifts once more, so a is now checked against the pair (xa, ya) = (1, -2):

$$a = 35 \cdot 1 + 15 \cdot (-2) = 5$$

Since the second value has reached 0, the loop exits with gcd(35, 15) = 5 and Bézout coefficients (1, -2). Checking directly: 35·1 + 15·(-2) = 35 − 30 = 5, confirming the identity.
