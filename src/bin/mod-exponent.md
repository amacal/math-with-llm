# Modular Exponentiation

Modular exponentiation computes a^b mod m efficiently. The naive approach — multiplying a by itself b times — requires O(b) multiplications, which is completely impractical when b is large (a 64-bit exponent would require up to 2^64 multiplications). The insight that makes fast computation possible is that the binary representation of the exponent tells us exactly which powers of a to combine.

## The Square-and-Multiply Algorithm

Any exponent b can be written in binary as a sum of distinct powers of two. Because exponentiation distributes over this sum via multiplication, we have

$$a^b = \prod_{i : \text{bit } i \text{ of } b \text{ is } 1} a^{2^i}$$

The right-hand side involves powers of a at positions where b has a 1-bit. The key observation is that consecutive powers in this sequence are related by squaring: a^(2^(i+1)) = (a^(2^i))^2. This means we can generate all needed powers with a single pass through the bits, squaring at each step, and accumulating into the result whenever the current bit is set.

The algorithm maintains two running values. The variable `power` holds a^(2^n) mod m, where n is the number of bits already processed, and `result` holds the partial product of all powers corresponding to set bits seen so far. At each step, if the current (least significant remaining) bit of the exponent is 1, `power` is folded into `result`; then `power` is squared to advance to the next bit position; finally the exponent is right-shifted by one. Both operations — the conditional multiplication and the squaring — are taken mod m immediately to keep values bounded.

The reason intermediate values must be reduced mod m at every step is the congruence identity

$$(a \cdot b) \bmod m = ((a \bmod m) \cdot (b \bmod m)) \bmod m$$

This means reducing after each operation preserves the final result. Without it, the intermediate product could grow to (m-1)^2, which for m near u64::MAX would overflow a 64-bit integer. The implementation casts both operands to u128 before multiplying, guaranteeing the product fits before the modular reduction brings it back to u64 range.

## Loop Invariant and Correctness

The invariant maintained at the start of each iteration, after n bits have been processed, is

$$\texttt{result} = a^{(\texttt{exp\_original} \;\&\; (2^n - 1))} \bmod m \qquad \texttt{power} = a^{2^n} \bmod m$$

Here exp\_original & (2^n - 1) is the value of the original exponent formed by its n least significant bits. When all bits have been processed, n equals the bit-length of the original exponent, and (2^n - 1) covers every bit, so result = a^exp mod m exactly. This is the answer.

## Edge Cases

Three boundary conditions were explored. When exp = 0, the loop never executes and result retains its initial value of 1, which is correct since a^0 = 1 for any a and any modulus greater than 1. When modulus = 1, every integer is congruent to 0 mod 1, so the result should be 0; the implementation returns 0 because result starts at 1 and the very first conditional multiplication produces 1 % 1 = 0, which then propagates correctly — or more directly, power = base % 1 = 0, so any multiplication involving power yields 0. When modulus = 0, the operation is undefined; the function returns None.

## Complexity

The loop executes exactly once per bit of the exponent, so it runs floor(log2(b)) + 1 times. Each iteration does a constant number of multiplications and one bit-shift. The total complexity is

$$O(\log b)$$

For the largest possible u64 exponent, 2^64 - 1, this means at most 64 iterations — completely tractable.

## Worked Example

Compute 3^13 mod 7. First write 13 in binary: 13 = 8 + 4 + 1 = 1101 in binary, so bits 0, 2, and 3 are set.

Start: result = 1, power = 3 % 7 = 3, exp = 13.

Iteration 1 (bit 0 = 1): result = 1 * 3 = 3 mod 7 = 3. Square: power = 3 * 3 = 9 mod 7 = 2. Shift: exp = 6.

Iteration 2 (bit 0 of 6 = 0): result unchanged = 3. Square: power = 2 * 2 = 4 mod 7 = 4. Shift: exp = 3.

Iteration 3 (bit 0 of 3 = 1): result = 3 * 4 = 12 mod 7 = 5. Square: power = 4 * 4 = 16 mod 7 = 2. Shift: exp = 1.

Iteration 4 (bit 0 of 1 = 1): result = 5 * 2 = 10 mod 7 = 3. Square: power = 2 * 2 = 4 mod 7 = 4. Shift: exp = 0.

Loop ends. Result = 3. Verification: 3^13 = 1594323, and 1594323 mod 7 = 3. Correct.
