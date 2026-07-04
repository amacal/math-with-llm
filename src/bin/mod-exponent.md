# Modular Exponentiation

## Overview

Modular exponentiation computes a^b mod m efficiently, without ever forming the full-size number a^b. The naive approach — multiplying a by itself b times — requires O(b) multiplications, which becomes hopeless once b is large: a 64-bit exponent would demand up to roughly 2^64 multiplications. The fix comes from noticing that most of that work is redundant. To compute a^6, for instance, there is no need to multiply a by itself five times in sequence; since 6 = 4 + 2 in binary, a^6 is just a^4 times a^2, and both of those powers come from repeated squaring — a^2, then a^4 — starting from a itself. Two squarings and one multiplication replace five multiplications, and the saving grows with b: doubling the number of bits in b roughly doubles the work instead of squaring it.

## The square-and-multiply algorithm

Any exponent b can be written in binary as a sum of distinct powers of two. Because exponentiation distributes over this sum via multiplication, we have

$$a^b = \prod_{i : \text{bit } i \text{ of } b \text{ is } 1} a^{2^i}$$

The right-hand side involves powers of a at positions where b has a 1-bit. The key observation is that consecutive powers in this sequence are related by squaring: a^(2^(i+1)) = (a^(2^i))^2. This means all needed powers can be generated with a single pass through the bits, squaring at each step and accumulating into the result whenever the current bit is set.

The algorithm maintains two running values. The variable `power` holds a^(2^n) mod m, where n is the number of bits already processed, and `result` holds the partial product of all powers corresponding to set bits seen so far. At each step, if the current (least significant remaining) bit of the exponent is 1, `power` is folded into `result`; then `power` is squared to advance to the next bit position; finally the exponent is right-shifted by one. Both operations — the conditional multiplication and the squaring — are taken mod m immediately to keep values bounded.

The reason intermediate values must be reduced mod m at every step is the congruence identity

$$(a \cdot b) \bmod m = ((a \bmod m) \cdot (b \bmod m)) \bmod m$$

This means reducing after each operation preserves the final result. Without it, the intermediate product could grow to (m-1)^2, which for m near u64::MAX would overflow a 64-bit integer. The implementation casts both operands to u128 before multiplying, guaranteeing the product fits before the modular reduction brings it back to u64 range.

## Correctness

The invariant maintained at the start of each iteration, after n bits have been processed, is

$$\texttt{result} = a^{(\texttt{exp\_original} \;\&\; (2^n - 1))} \bmod m \qquad \texttt{power} = a^{2^n} \bmod m$$

Here exp\_original & (2^n - 1) is the value formed by the original exponent's n least significant bits — masking off everything above bit n-1. When all bits have been processed, n equals the bit-length of the original exponent, and (2^n - 1) covers every bit, so result = a^exp mod m exactly. This is the answer, and it falls out of the invariant rather than needing a separate argument: correctness of the loop reduces to correctness of the invariant plus the observation that the invariant becomes the goal statement once the loop has consumed every bit.

## Complexity

The loop executes exactly once per bit of the exponent, so it runs floor(log2(b)) + 1 times. Each iteration does a constant number of multiplications and one bit-shift. The total complexity is

$$O(\log b)$$

For the largest possible u64 exponent, 2^64 - 1, this means at most 64 iterations — completely tractable, in sharp contrast to the naive O(b) approach this session started from.

## Edge cases

Three boundary conditions were explored. When exp = 0, the loop never executes and result retains its initial value of 1, which is correct since a^0 = 1 for any a and any modulus greater than 1. When modulus = 1, every integer is congruent to 0 mod 1, so the result should be 0; the implementation returns 0 because power = base % 1 = 0 from the first iteration onward, so any multiplication involving power yields 0. When modulus = 0, the operation is undefined, and the function returns None rather than producing a meaningless result.

## Worked example

Compute 3^13 mod 7. First write 13 in binary: 13 = 8 + 4 + 1 = 1101 in binary, so bits 0, 2, and 3 are set.

Start: result = 1, power = 3 % 7 = 3, exp = 13.

Iteration 1 (bit 0 = 1): result = 1 * 3 = 3 mod 7 = 3. Square: power = 3 * 3 = 9 mod 7 = 2. Shift: exp = 6.

Iteration 2 (bit 0 of 6 = 0): result unchanged = 3. Square: power = 2 * 2 = 4 mod 7 = 4. Shift: exp = 3.

Iteration 3 (bit 0 of 3 = 1): result = 3 * 4 = 12 mod 7 = 5. Square: power = 4 * 4 = 16 mod 7 = 2. Shift: exp = 1.

Iteration 4 (bit 0 of 1 = 1): result = 5 * 2 = 10 mod 7 = 3. Square: power = 2 * 2 = 4 mod 7 = 4. Shift: exp = 0.

Loop ends. Result = 3. Verification: 3^13 = 1594323, and 1594323 mod 7 = 3. Correct.
