# BigInteger Extended GCD

## Overview

The Extended Euclidean GCD session computed, for fixed-width 64-bit integers, not just the greatest common divisor of two numbers but also two coefficients x and y satisfying an identity known as Bézout's identity:

$$a \cdot x + b \cdot y = \gcd(a, b)$$

The BigInteger GCD session separately computed the gcd itself for arbitrary-precision numbers, using the `BigNumber` type built earlier. This session merges the two: the same Bézout coefficients, but for `BigNumber` operands. The obstacle is that `BigNumber` is unsigned only, its subtraction returning `Option<BigNumber>` and yielding `None` whenever the true result would be negative, while Bézout coefficients are frequently negative. The u64 version sidestepped this for free, since a signed 64-bit integer already sat next to the unsigned one in the standard library; at arbitrary precision no such escape hatch exists, so the real content of this session is designing a signed representation on top of the unsigned `BigNumber` and re-deriving the coefficient arithmetic in terms of it.

## The signed coefficient type

The natural representation for a signed value built on an unsigned magnitude is sign-magnitude: a boolean flag recording whether the value is non-negative, paired with a `BigNumber` holding its absolute value. Call this type `Signed`. Negating a `Signed` value is just flipping the sign flag while leaving the magnitude untouched — negation changes only which side of zero a number falls on, not how large it is.

Two operations on `Signed` are needed to evaluate the coefficient recurrence for both x and y:

$$x_r = x_a - q \cdot x_b \qquad y_r = y_a - q \cdot y_b$$

where q is the quotient from dividing the current a by the current b. The first operation, multiplying a `Signed` value by a plain unsigned `BigNumber`, keeps the sign exactly as it was and scales the magnitude by q, since q is always non-negative. The second, subtracting two `Signed` values, is where the actual sign bookkeeping happens.

Subtraction reduces to negation followed by addition: a minus b is the same as a plus the negation of b. Addition itself splits into two cases: same-sign operands add the magnitudes and keep the shared sign, while opposite-sign operands subtract the smaller magnitude from the larger and take the sign of whichever operand supplied the larger magnitude. Applying this to self minus other: same original signs put self and the negated other on opposite sides of zero, landing in the subtract-magnitudes case; different original signs put them on the same side, landing in the add-magnitudes case — the reverse of first-glance intuition. It is easy to build a version that instead branches on the sign comparison made *after* the swap-for-ordering step rather than on the original signs, which silently picks the wrong branch whenever the magnitudes needed swapping.

One edge case deserves a moment: when the two magnitudes in the subtract-magnitudes branch are exactly equal, the result is magnitude zero, and the sign attached to it is a free choice, canonicalized here as positive. This is purely cosmetic, since every `Signed` value produced feeds only into further multiplications and additions in the coefficient recurrence, and a zero magnitude times or plus anything produces the same result regardless of which sign bit it carried.

## Correctness

The invariant driving the algorithm is exactly the one from the u64 Extended Euclidean GCD session, unchanged by the switch to arbitrary precision since the underlying algebra never referenced the width of the integer type:

$$\text{current}_a = a \cdot x_a + b \cdot y_a \qquad \text{current}_b = a \cdot x_b + b \cdot y_b$$

Because this invariant and its proof are representation-independent — the argument only ever adds, subtracts, and multiplies symbolic quantities, never appealing to how they are stored — it transfers to `BigNumber` and `Signed` operands unmodified, and is cited here rather than re-derived.

The base case holds because seeding (xa, ya) = (1, 0) and (xb, yb) = (0, 1) gives:

$$\text{current}_a = a \cdot 1 + b \cdot 0 = a \qquad \text{current}_b = a \cdot 0 + b \cdot 1 = b$$

matching the actual starting values. The maintenance step holds because, given the invariant above, the remainder r from the division step satisfies r = current_a − q·current_b, and substituting and expanding gives:

$$a \cdot (x_a - q \cdot x_b) + b \cdot (y_a - q \cdot y_b) = a \cdot x_a + b \cdot y_a - q \cdot (a \cdot x_b + b \cdot y_b) = \text{current}_a - q \cdot \text{current}_b = r$$

So next_x = xa − q·xb and next_y = ya − q·yb correctly express the new remainder as a combination of a and b, and the loop's shift step preserves the invariant into the next iteration.

Termination follows the basic Euclidean GCD argument: current_b strictly decreases and is bounded below by zero, so it must eventually reach zero. At that point, since gcd(current_a, current_b) equals gcd(a, b) at every step (established in the basic Euclidean GCD session) and gcd(x, 0) equals x, the value left in current_a is exactly gcd(a, b), and the invariant collapses into Bézout's identity for the original inputs:

$$a \cdot x_a + b \cdot y_a = \gcd(a, b)$$

## Complexity

The BigInteger GCD session established, via a telescoping-sum argument, that the plain gcd computation over `BigNumber` operands costs, where n is the limb-count of the larger input:

$$O(n^2)$$

total, not the naive O(n^3) that multiplying per-step cost by iteration count would suggest, since operand sizes shrink as the algorithm proceeds. The question here is whether `Signed` coefficient bookkeeping changes this bound.

In the u64 version, coefficients were fixed-width i64 values, so the extra bookkeeping per step was O(1), folded into the existing per-step cost for free. That does not carry over here: a `Signed` coefficient's magnitude can grow to roughly n limbs and, unlike current_a and current_b, does not shrink as the loop proceeds. Each iteration's coefficient multiplication, xb times q, costs roughly n times the limb-count of q, and naively bounding q's size by n too across O(n) iterations again suggests the same overcounting trap as the plain GCD case:

$$O(n^3)$$

The resolution is a second, distinct telescoping-type argument: the quotients q1, q2, and so on across the whole run have a product bounded by roughly a divided by gcd(a, b), since each quotient is exactly the factor by which the algorithm reduces the current pair at that step:

$$q_1 \cdot q_2 \cdots q_k \approx \frac{a}{\gcd(a,b)}$$

Since limb-count is proportional to the logarithm of a value, and the log of a product is the sum of the logs of its factors, this product bound translates into a bound on the *sum* of the quotients' limb-counts across the whole algorithm — not per iteration — and this sum is O(n) even in the worst case, gcd(a, b) equals 1:

$$\sum_i \text{limbs}(q_i) = O(n)$$

Treating the coefficient's size — roughly n, effectively constant across iterations — as a fixed multiplier applied to this O(n) budget of quotient limb-count, the total cost of every coefficient multiplication over the whole algorithm matches, rather than exceeds, the existing bound:

$$n \cdot O(n) = O(n^2)$$

So overall complexity stays O(n²), but reaching that required this second telescoping argument over the quotient sizes, not the trivial constant-per-step reasoning that sufficed in the fixed-width case.

## Edge cases

The gcd(0, 0) case is undefined for the same reason established in BigInteger GCD: every positive integer divides zero, so no greatest common divisor exists, and the function returns `None` before the loop runs. The u64 version's overflow concern — the quotient could exceed i64's maximum, though only when b equals 1, where the resulting update is immediately discarded anyway — simply does not arise here, since `BigNumber` has no fixed width and never overflows.

## Worked example

Trace gcd(48, 18). The seeds are (xa, ya) = (1, 0) and (xb, yb) = (0, 1), matching:

$$a = 48 \cdot 1 + 18 \cdot 0 \qquad b = 48 \cdot 0 + 18 \cdot 1$$

Dividing 48 by 18 gives quotient 2 and remainder 12, so:

$$x_b' = 1 - 2 \cdot 0 = 1 \qquad y_b' = 0 - 2 \cdot 1 = -2$$

with the sign following the larger magnitude, 2, from the right-hand operand. The loop shifts: current_a becomes 18, current_b becomes 12, and the coefficient pairs become (xa, ya) = (0, 1) and (xb, yb) = (1, −2).

Dividing 18 by 12 gives quotient 1 and remainder 6, so next_x = 0 − 1·1 = −1, and next_y is the interesting computation:

$$y_b' = 1 - 1 \cdot (-2) = 1 + 2 = 3$$

Here self = (positive, 1) and other = (negative, 2) have different original signs, so this falls into the add-magnitudes case, giving magnitude 1 + 2 = 3 with self's sign, positive — not the −1 that a version branching on the post-swap sign rather than the original signs would compute, since swapping to put the larger magnitude on the left flips the sign flag first. The loop shifts again: current_a becomes 12, current_b becomes 6, and the coefficient pairs become (xa, ya) = (1, −2) and (xb, yb) = (−1, 3).

Dividing 12 by 6 gives quotient 2 and remainder 0, so:

$$x_b' = 1 - 2 \cdot (-1) = 3 \qquad y_b' = -2 - 2 \cdot 3 = -8$$

The loop shifts one final time: current_a becomes 6, current_b becomes 0, and the coefficient pairs become (xa, ya) = (−1, 3) and (xb, yb) = (3, −8). Since current_b is now zero, the loop exits, returning gcd 6 with Bézout coefficients (xa, ya) = (−1, 3). Checking directly:

$$48 \cdot (-1) + 18 \cdot 3 = -48 + 54 = 6$$

confirming the identity.
