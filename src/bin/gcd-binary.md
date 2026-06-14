# Binary GCD (Stein's Algorithm)

## Key insight

The Euclidean algorithm reduces the pair (a, b) by computing a mod b, which requires integer division. The binary GCD algorithm achieves the same result using only subtraction, comparison, and bit shifts. It rests on two properties of GCD that together replace division entirely.

The first property says that GCD is preserved under subtraction. If d divides both a and b, it also divides their difference, and any divisor of b and of a − b must divide a as well. So the pairs (a, b) and (a − b, b) share exactly the same common divisors:

$$\gcd(a,\ b) = \gcd(a - b,\ b) \qquad a \geq b$$

The second property says that a shared factor of 2 can be stripped from both arguments and reinstated at the end. When both values are even, 2 is a common factor; when only one is even, 2 is not shared and can be discarded from that argument alone:

$$\gcd(2a,\ 2b) = 2 \cdot \gcd(a,\ b) \qquad \gcd(2a,\ b) = \gcd(a,\ b) \quad (b \text{ odd})$$

## Proofs of the two properties

Both properties are equalities of greatest common divisors. The proof strategy for each follows the pattern established for the Euclidean algorithm: show that the two pairs share the same set of common divisors, then conclude their greatest elements must be equal.

**Property 1: gcd(a, b) = gcd(a − b, b) for a ≥ b.**

We show both directions. Suppose k divides both a and b, so a = kp and b = kq for integers p and q. Then the difference is:

$$a - b = kp - kq = k(p - q)$$

Since p − q is an integer, k divides a − b. Together with k dividing b, k is a common divisor of (a − b, b). For the reverse direction, suppose k divides both a − b and b, writing a − b = ks and b = kq. Rearranging:

$$a = (a - b) + b = ks + kq = k(s + q)$$

So k divides a, and is therefore a common divisor of (a, b). Both directions hold, so the two pairs share exactly the same set of common divisors, which means their greatest elements are equal.

**Property 2: gcd(2a, b) = gcd(a, b) when b is odd.**

Every common divisor of (a, b) is also a common divisor of (2a, b): if k divides a it divides 2a, and if it already divides b the pair (2a, b) is covered. For the reverse direction, suppose d divides both 2a and b. Since d divides b and b is odd, d must itself be odd — if d were even we could write d = 2m, and then 2m dividing b would force b to be even, contradicting the assumption. So d is odd, which means gcd(d, 2) = 1. By Bézout's identity applied to d and 2, there exist integers x and y with:

$$dx + 2y = 1$$

Multiplying both sides by a:

$$dax + 2ay = a$$

Since d divides 2a, it divides 2ay. Since d trivially divides dax, it divides their sum a. So d divides both a and b, and the two pairs again share the same common divisors.

**Property 3: gcd(2a, 2b) = 2 · gcd(a, b).**

By Bézout's identity (produced by the extended Euclidean algorithm), there exist integers x and y such that ax + by = gcd(a, b). Multiplying both sides by 2:

$$(2a)x + (2b)y = 2 \cdot \gcd(a, b)$$

Any common divisor of (2a, 2b) divides every linear combination of 2a and 2b, so it divides 2 · gcd(a, b). In the other direction, since gcd(a, b) divides a it follows that 2 · gcd(a, b) divides 2a; similarly it divides 2b. So 2 · gcd(a, b) is a common divisor of (2a, 2b), and every other common divisor divides it. That makes it the greatest.

## The three reduction cases

Every iteration handles one of three cases based on the parity of the current pair. A counter k tracks how many times both arguments were halved together, recording the accumulated power of 2 that must be restored at the end.

When both arguments are even, 2 is a common factor. Both are halved simultaneously and k is incremented:

$$\gcd(a,\ b) = 2 \cdot \gcd(a/2,\ b/2) \qquad (a \text{ even},\ b \text{ even})$$

When exactly one argument is even, its factor of 2 is not shared. It is halved without changing k, because this 2 does not divide the GCD:

$$\gcd(a,\ b) = \gcd(a/2,\ b) \qquad (a \text{ even},\ b \text{ odd})$$

When both arguments are odd, neither can be halved directly. The larger is replaced by the difference. The difference of two odd numbers is always even, so the very next iteration is guaranteed to be a shift — which means the subtraction step never occurs twice in a row:

$$\gcd(a,\ b) = \gcd(a - b,\ b) \qquad (a > b,\ a \text{ odd},\ b \text{ odd})$$

## Base case

The loop runs while both arguments are positive, and exits as soon as one reaches zero. At that point, the other holds the GCD of the original pair after all common factors of 2 have been stripped. Since k counts how many times both arguments were halved together, the final answer is:

$$\text{result} = (a + b) \cdot 2^k$$

Writing a + b rather than, say, max(a, b) works because at termination exactly one of the two is zero. Adding zero leaves the survivor unchanged, so a + b picks out the non-zero value without an extra branch.

## Termination

The algorithm terminates because at least one bit is eliminated every two steps. The subtraction case produces an even result, guaranteeing the next step is a shift. The shift case strictly reduces the bit length of one argument. So no two consecutive steps can both be subtractions, and every subtraction is followed by at least one shift. This ensures that the total bit length of the pair decreases steadily, and since both arguments are non-negative integers, the process must reach zero in a finite number of steps.

## Complexity

The interleaving argument above bounds the number of steps. Starting from inputs of at most n bits each, every two steps shorten at least one argument by at least one bit. The number of steps is therefore at most proportional to the total bit length of the pair:

$$O(\log(\min(a, b))) \text{ steps}$$

This matches the Euclidean algorithm. The practical advantage of binary GCD is that it uses only shifts and subtracts — single-cycle operations on virtually all hardware — whereas Euclidean GCD requires integer division, which is slower on many processors.

## Edge cases

When one input is zero at the start, the loop does not execute and the other value is returned directly. gcd(0, n) = n and gcd(n, 0) = n, because every integer divides zero, making n the largest common divisor of the pair. The case gcd(0, 0) is undefined — every positive integer divides zero, so there is no finite greatest common divisor — and the implementation returns None. When a equals b, both are odd (since the both-even and one-even cases reduce at least one argument before reaching equal values), the subtraction produces zero, and the loop exits at the next iteration.

## Worked example

Trace gcd(24, 18). Both are even, so both are halved and k records the common factor:

$$\gcd(24,\ 18) = 2 \cdot \gcd(12,\ 9) \qquad k = 1$$

Now 12 is even and 9 is odd. The factor of 2 in 12 is not shared, so 12 is halved without updating k:

$$\gcd(12,\ 9) \;\to\; \gcd(6,\ 9)$$

Still 6 even, 9 odd, and 6 is halved again:

$$\gcd(6,\ 9) \;\to\; \gcd(3,\ 9)$$

Both are now odd and 9 > 3, so the larger is replaced by the difference:

$$\gcd(3,\ 9) \;\to\; \gcd(3,\ 6)$$

The result 6 is even, confirming the guaranteed shift. Now 3 is odd and 6 is even, so 6 is halved:

$$\gcd(3,\ 6) \;\to\; \gcd(3,\ 3)$$

Both odd and equal, so the subtraction produces zero:

$$\gcd(3,\ 3) \;\to\; \gcd(3,\ 0)$$

One argument is zero; the survivor is 3. Restoring the stripped factor:

$$\text{result} = 3 \cdot 2^1 = 6$$

Verification: 24 = 4 × 6 and 18 = 3 × 6, and since 4 and 3 share no common factor, no divisor larger than 6 divides both. ✓

The step from gcd(3, 9) to gcd(3, 6) and then immediately to gcd(3, 3) illustrates the key structural guarantee: the subtraction produced an even number (6 = 9 − 3), and the next step was forced to be a shift. This is why subtraction and shift must interleave, and why the bit length of the pair decreases steadily.

## Contrast with Euclidean GCD

The Euclidean algorithm can eliminate many bits in a single division. For example:

$$\gcd(1000,\ 3) \;\to\; \gcd(3,\ 1)$$

One step discards almost the entire bit length of the first argument. Binary GCD cannot do this — without division it must work bit by bit. The asymptotic complexity is the same, but the constant factor differs: Euclidean GCD is typically faster on hardware with cheap division, while binary GCD is preferable when division is slow or unavailable.
