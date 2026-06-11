# Chinese Remainder Theorem (CRT)

## Problem statement
Given `x ≡ a₁ (mod m₁)` and `x ≡ a₂ (mod m₂)` with `gcd(m₁, m₂) = 1`, find the unique solution `x ∈ [0, m₁·m₂)`.

## Existence
As `x` runs through `[0, m₁·m₂)`, the pair `(x mod m₁, x mod m₂)` hits every combination in `[0, m₁) × [0, m₂)` exactly once — the coprime cycles don't repeat until `m₁·m₂`. So for any target pair `(a₁, a₂)`, a solution exists.

## Uniqueness
Suppose `x` and `y` both satisfy both congruences. Then:
- `m₁ | (x - y)` and `m₂ | (x - y)`
- Since `gcd(m₁, m₂) = 1`, both dividing means `m₁·m₂ | (x - y)`
- But `x, y ∈ [0, m₁·m₂)`, so `x - y ∈ (-(m₁·m₂), m₁·m₂)` — the only multiple of `m₁·m₂` in that open interval is `0`
- Therefore `x = y`

## Construction
Write `x = a₁ + k·m₁` (satisfies the first congruence for any integer `k`). Substitute into the second congruence:
- `a₁ + k·m₁ ≡ a₂ (mod m₂)`
- `k·m₁ ≡ (a₂ - a₁) (mod m₂)`
- `k ≡ (a₂ - a₁) · m₁⁻¹ (mod m₂)`

The inverse exists because `gcd(m₁, m₂) = 1`. Then `x = a₁ + k·m₁`, reduced to `[0, m₁·m₂)`.

## Unsigned subtraction trap
`(a₂ - a₁) mod m₂` requires care with `u64`. Two cases:
- `a₂ ≥ a₁`: compute `a₂ - a₁` directly (result is in `[0, m₂)` after normalization)
- `a₂ < a₁`: compute `(m₂ - (a₁ - a₂) % m₂) % m₂` — reduce before subtracting from `m₂`, then take one more `% m₂` to handle the case where `(a₁ - a₂)` is exactly divisible by `m₂`

The user discovered a subtle bug here: first wrote `y.1 - (x.0 - y.0)` (missing the inner `% y.1`), which failed when `x.0 - y.0 > y.1`. A misleading test with a wrong expected value masked the bug initially.

## Edge cases
- Non-coprime moduli: `mod_inverse` returns `None`, propagated up
- `m = 0`: caught explicitly before normalization
- Unnormalized inputs (`a ≥ m`): normalized at entry with `%`
- `m₁·m₂` overflow: caught with `checked_mul`
- `k·m₁` overflow: caught with `checked_mul`

## Complexity
O(log(min(m₁, m₂))) — dominated by the modular inverse call, which calls extended GCD.

## Relation to previous problems
Direct application of [[mod-inverse]], which is a direct application of [[gcd-euclidean-extended]]. CRT is the first result that uses modular inverse as a tool rather than as an end in itself. It is foundational for RSA key generation, efficient multi-precision arithmetic, and competitive programming number theory.
