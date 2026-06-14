# Extended Euclidean GCD

## Key insight

The basic Euclidean algorithm finds the greatest common divisor of two integers, but throws away everything except the final answer. The extended version keeps track of something extra at each step: how to express the current remainder as a combination of the two original inputs. By the time the algorithm terminates, the GCD itself is expressed that way, giving us two integers x and y such that:

$$a \cdot x + b \cdot y = \gcd(a, b)$$

This equation is known as Bézout's identity, and the extended algorithm is the standard way to compute the coefficients x and y.

## Why the coefficients always exist

The argument is that at every step of the Euclidean algorithm, the current remainder can be written as a linear combination of the original a and b. We maintain this as an invariant by tracking two coefficient pairs alongside the two working values.

At the very start, before any step is taken, the two working values are a and b themselves. Each can trivially be written as a combination of the originals:

$$a = a \cdot 1 + b \cdot 0 \qquad b = a \cdot 0 + b \cdot 1$$

So the initial coefficient pairs are (xa, ya) = (1, 0) for the first value and (xb, yb) = (0, 1) for the second. Now suppose at some point in the algorithm we have two values, each expressed as a combination of the originals:

$$\text{current}_a = a \cdot x_a + b \cdot y_a \qquad \text{current}_b = a \cdot x_b + b \cdot y_b$$

The next step computes the remainder of dividing the first by the second. Using the same rearrangement as in the basic algorithm, the new remainder is:

$$r = \text{current}_a - q \cdot \text{current}_b$$

where q is the quotient. Substituting the linear combinations into this:

$$r = (a \cdot x_a + b \cdot y_a) - q \cdot (a \cdot x_b + b \cdot y_b) = a \cdot (x_a - q \cdot x_b) + b \cdot (y_a - q \cdot y_b)$$

So the new remainder is also a linear combination of a and b, with coefficients:

$$x_r = x_a - q \cdot x_b \qquad y_r = y_a - q \cdot y_b$$

Since this holds at every step and the seeds satisfy it trivially, the invariant is maintained throughout. When the algorithm terminates with b = 0, the value in the a position is the GCD, and its coefficient pair (xa, ya) satisfies:

$$\gcd(a, b) = a \cdot x_a + b \cdot y_a$$

## The loop invariant

To be precise about what is maintained, the invariant is that at every point during the loop, the current working values satisfy:

$$\text{current}_a = \text{initial}_a \cdot x_a + \text{initial}_b \cdot y_a$$
$$\text{current}_b = \text{initial}_a \cdot x_b + \text{initial}_b \cdot y_b$$

This holds at the seeds by construction, and the update rule for the coefficients preserves it at each step. At termination, b = 0 and a is the GCD, so the first equation gives the Bézout identity directly.

## Termination and correctness

Termination and correctness of the GCD computation are identical to the basic Euclidean algorithm — b strictly decreases each step and reaches zero, at which point the value in the a position is the greatest common divisor. The coefficient tracking adds only constant work per iteration and does not affect these arguments.

## Overflow handling

The inputs are unsigned 64-bit integers, but the Bézout coefficients can be negative, so they are tracked as signed 64-bit integers. At each step, the quotient q is computed from unsigned values and then converted to i64. The only case where q can exceed the maximum i64 value is when the divisor b equals 1. But when b = 1, the remainder a mod b is 0, so b becomes 0 after this step and the loop exits. The coefficient update for xb and yb would be computed and immediately discarded since the algorithm returns xa and ya, not xb and yb. The implementation detects this case and skips the update entirely, which is safe because the values being skipped are never used.

## Edge cases

When one or both inputs are zero, the loop never executes and the seed values are returned directly. gcd(0, 0) is undefined and returns None. gcd(0, n) returns n with coefficients (0, 1), since b = a·0 + n·1. gcd(n, 0) returns n with coefficients (1, 0), since a = n·1 + 0·0. When a equals b, one iteration produces remainder 0, and the result is (n, (0, 1)). Multiple valid coefficient pairs exist in general — for example gcd(6, 3) could produce (1, 0) or (0, 2) — and the algorithm returns whichever it naturally computes.

## Worked example

Trace gcd(35, 15), tracking the coefficient pairs at each step. The invariants throughout are:

$$a = 35 \cdot x_a + 15 \cdot y_a$$
$$x_a' = x_b \qquad y_a' = y_b \qquad x_b' = x_a - q \cdot x_b \qquad y_b' = y_a - q \cdot y_b$$

We start with:

$$x_a=1,\ y_a=0 \qquad x_b=0,\ y_b=1$$
$$a = 35 \cdot 1 + 15 \cdot 0 = 35$$
$$x_b' = 1 - 2 \cdot 0 = 1,\ y_b' = 0 - 2 \cdot 1 = -2$$

The second iteration does:
$$x_a=0,\ y_a=1 \qquad x_b=1,\ y_b=-2$$
$$a = 35 \cdot 0 + 15 \cdot 1 = 15$$
$$x_b' = 0 - 3 \cdot 1 = -3,\ y_b' = 1 - 3 \cdot (-2) = 7$$

The final step is:
$$x_a=1,\ y_a=-2$$
$$a = 35 \cdot 1 + 15 \cdot (-2) = 5$$

## Complexity

The number of steps is O(log(min(a, b))), identical to the basic algorithm. Tracking the coefficients adds only a constant number of arithmetic operations per step, so the overall complexity is unchanged.
