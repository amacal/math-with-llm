# Karatsuba Multiplication

## The core idea

Karatsuba multiplication is a divide-and-conquer algorithm that multiplies two n-chunk big integers in O(n^log_2(3)) time, beating the O(n^2) of schoolbook by reducing the number of recursive sub-multiplications from four to three. The saving comes entirely from one algebraic identity, not from any clever data structure.

The starting point is the same split used in schoolbook arithmetic. Given two numbers A and B represented in base B = 2^64, we pick a pivot n (roughly half the length of the inputs) and write each number as a high and low half. Formally:

$$A = A_{hi} \cdot (2^{64})^n + A_{lo} \qquad B = B_{hi} \cdot (2^{64})^n + B_{lo}$$

Expanding A * B by the distributive law gives four terms.

$$A \cdot B = A_{hi} B_{hi} \cdot (2^{64})^{2n} + (A_{hi} B_{lo} + A_{lo} B_{hi}) \cdot (2^{64})^n + A_{lo} B_{lo}$$

There are four sub-multiplications here: A_hi*B_hi, A_hi*B_lo, A_lo*B_hi, and A_lo*B_lo. Naively recursing on all four gives T(n) = 4T(n/2) + O(n), which solves to O(n^log_2(4)) = O(n^2) — no improvement over schoolbook.

## The algebraic trick

The Karatsuba trick eliminates one of the four sub-multiplications by observing that the middle term A_hi*B_lo + A_lo*B_hi can be recovered from three products that we already need. Name the three key products:

$$z_0 = A_{lo} \cdot B_{lo} \qquad z_2 = A_{hi} \cdot B_{hi} \qquad z_1 = (A_{lo} + A_{hi})(B_{lo} + B_{hi}) - z_0 - z_2$$

Expanding (A_lo + A_hi)(B_lo + B_hi) by the distributive law gives z_0 + z_2 + A_hi*B_lo + A_lo*B_hi, so subtracting z_0 and z_2 leaves exactly the middle cross term. This means z_1 = A_hi*B_lo + A_lo*B_hi, computed with only one multiplication instead of two. The full product then assembles as:

$$A \cdot B = z_2 \cdot (2^{64})^{2n} + z_1 \cdot (2^{64})^n + z_0$$

Multiplication by (2^64)^k in the Vec<u64> representation is not arithmetic at all — it is just prepending k zero chunks to the low end of the vector. The only true arithmetic operations are the three sub-multiplications and a handful of additions and subtractions, all of which cost O(n).

## Safety of the z1 subtraction

Computing z_1 requires two subtractions of big unsigned integers, which could underflow if done in the wrong order. The safety argument is: since z_1 = A_hi*B_lo + A_lo*B_hi >= 0 and z_0 >= 0, it follows that (A_lo + A_hi)(B_lo + B_hi) = z_0 + z_1 + z_2 >= z_2, so the first subtraction (removing z_2) is always safe. After removing z_2 the remainder is z_1 + z_0 >= z_0, so the second subtraction (removing z_0) is also always safe. Subtracting z_2 first and z_0 second is therefore guaranteed not to underflow.

## Complexity

There are three recursive calls on sub-problems of size roughly n/2, plus O(n) work for additions, subtractions, and shifts. This gives the recurrence:

$$T(n) = 3 \cdot T(n/2) + O(n)$$

The Master Theorem handles recurrences of the form T(n) = a*T(n/b) + O(n^c). The key comparison is between log_b(a) and c: when log_b(a) > c the recursive work dominates; when log_b(a) < c the top-level work dominates; when they are equal both contribute equally. Here a = 3, b = 2, c = 1, and log_2(3) ≈ 1.585 > 1, so the recursive work dominates and T(n) = O(n^log_2(3)). This is strictly better than the schoolbook O(n^2), and the entire improvement rests on reducing a = 4 (naive) to a = 3 (Karatsuba). Had we kept four sub-multiplications, log_2(4) = 2 would have recovered exactly O(n^2).

## Base case and the pivot strategy

The recursion reduces sub-problem size only when the pivot is at least 1, which requires min(len_a, len_b) >= 2. When either input has a single chunk, splitting would produce a zero high half and leave the same problem to recurse on — infinite recursion. The implementation resolves this by padding the shorter operand to the length of the longer one before splitting, ensuring both halves are always non-empty and the pivot is always >= 1. The true base case is both operands having exactly one chunk, at which point mul128 computes the exact 128-bit product directly.

The pivot is chosen as min(len_a, len_b) / 2 after padding has equalized the lengths, so both splits land at the same position and the sub-problems are as balanced as possible.

## Worked example

Take A = [3, 5] and B = [7, 11] in base 2^64, meaning A = 3 + 5*(2^64) and B = 7 + 11*(2^64). With pivot n = 1:

A_lo = 3, A_hi = 5, B_lo = 7, B_hi = 11

Both inputs have length 2 and the sub-problems have length 1, so the three products hit the base case directly:

$$z_0 = 3 \cdot 7 = 21 \qquad z_2 = 5 \cdot 11 = 55$$

$$z_1 = (3 + 5)(7 + 11) - 21 - 55 = 8 \cdot 18 - 76 = 144 - 76 = 68$$

The final assembly is:

$$A \cdot B = 55 \cdot (2^{64})^2 + 68 \cdot 2^{64} + 21$$

To verify: expanding directly, (3 + 5*2^64)(7 + 11*2^64) = 21 + 33*2^64 + 35*2^64 + 55*(2^64)^2 = 21 + 68*2^64 + 55*(2^64)^2, which matches.
