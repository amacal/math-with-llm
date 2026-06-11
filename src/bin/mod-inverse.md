# Modular Inverse

## Key insight
The modular inverse of `a` modulo `m` is an integer `x` in `[0, m)` such that `a * x ≡ 1 (mod m)`. It is the multiplicative inverse in the ring of integers modulo `m` — the analogue of `1/a` in regular arithmetic, but restricted to integers.

## Existence condition
The inverse exists if and only if `gcd(a, m) = 1`. If `gcd(a, m) > 1`, then the remainders of `a * x mod m` cycle through values that are all multiples of `gcd(a, m)`, so 1 is never reached. The user discovered this by working through `a = 2`, `m = 4` by hand.

## Algorithm
A thin wrapper over extended GCD. Call `gcd(a, m)`:
- If the result is not `Some((1, _))`, return `None` — inverse does not exist.
- Otherwise, take the Bézout coefficient `x` for `a`. By Bézout: `x * a + y * m = 1`, so `x * a ≡ 1 (mod m)`.
- Normalise `x` to `[0, m)`: if `x >= 0`, return `x`; if `x < 0`, return `m - |x|`.

## Why normalisation works
`a * (x + m) = a * x + a * m ≡ a * x (mod m)`. Adding any multiple of `m` to `x` preserves the congruence, so `x + m` is equally valid. Since the Bézout coefficient satisfies `|x| < m`, a single addition of `m` is always enough to land in `[0, m)`.

## Why `m as i64` must be avoided
The naive expression `(x + m as i64) as u64 % m` overflows when `m > i64::MAX` because the cast wraps to a negative value. The fix is to branch on the sign of `x` and use `u64` subtraction: `m - (-x as u64)`.

## Edge cases
- `gcd(a, m) > 1`: no inverse, return `None`.
- `a = 0`: `gcd(0, m) = m`, so `None` unless `m = 1`.
- `m = 1`: everything is `0 mod 1`; `gcd(a, 1) = 1` for any `a`, so the inverse is `Some(0)` — the only element of `[0, 1)`.
- `m = 0`: `gcd(a, 0) = a`, so `None` unless `a = 1` (but `mod 0` is undefined anyway).
- Large inputs (`u64::MAX`): handled safely by branching on sign rather than casting `m` to `i64`.

## Loop invariant (inherited from extended GCD)
At every iteration: `a = xa * initial_a + ya * initial_b` and `b = xb * initial_a + yb * initial_b`. The invariant for `b` is broken in the final step when the quotient overflows `i64` — but `b` becomes 0 at that point and `(xb, yb)` are never used.

## Complexity
O(log(min(a, m))) — the wrapper adds O(1) work on top of extended GCD.

## Relation to previous problems
Direct application of [[gcd-euclidean-extended]]. The Bézout coefficient `x` is exactly the modular inverse when `gcd = 1`. This is the foundation for RSA key generation and modular arithmetic in cryptographic schemes.
