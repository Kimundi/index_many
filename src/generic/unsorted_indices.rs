use super::Indices;

/// This type allows any order for the indices, as long as they are unique.
///
/// This requires `O(N^2)` comparisons to check if the indices are unique and
/// in bounds of a slice.
///
/// # Example codegen
///
/// ```rust
/// use index_many::generic::UnsortedIndices;
/// pub fn example(slice: &mut [usize], indices: UnsortedIndices<3>) -> [&mut usize; 3] {
///     index_many::generic::index_many_mut(slice, indices)
/// }
/// ```
///
/// ```nasm
/// example:
///  sub     rsp, 56
///  mov     rax, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r10
///  je      .LBB3_6
///  cmp     r9, rax
///  je      .LBB3_6
///  cmp     r9, r8
///  jae     .LBB3_6
///  cmp     r10, rax
///  je      .LBB3_6
///  cmp     rax, r8
///  jae     .LBB3_6
///  cmp     r10, r8
///  jae     .LBB3_6
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB3_6:
///  mov     qword, ptr, [rsp, +, 32], rax
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::bound_check_failed
///  ud2
/// ```
#[derive(Copy, Clone)]
pub struct UnsortedIndices<const N: usize>(pub [usize; N]);

unsafe impl<const N: usize> Indices<N> for UnsortedIndices<N> {
    #[inline]
    fn to_raw_indices(&self) -> [usize; N] {
        self.0
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        for (i, &idx) in self.0.iter().enumerate() {
            valid &= idx < len;
            for &idx2 in &self.0[..i] {
                valid &= idx != idx2;
            }
        }

        valid
    }

    #[inline(always)]
    fn cause_invalid_panic(&self, len: usize) -> ! {
        crate::bound_check_failed(&self.0, len)
    }
}

#[cfg(test)]
mod tests {
    use super::UnsortedIndices;

    fn index_many<'a, T, const N: usize>(slice: &[T], indices: [usize; N]) -> [&T; N] {
        let indices = UnsortedIndices(indices);
        super::super::index_many(slice, indices)
    }

    fn index_many_mut<'a, T, const N: usize>(slice: &mut [T], indices: [usize; N]) -> [&mut T; N] {
        let indices = UnsortedIndices(indices);
        super::super::index_many_mut(slice, indices)
    }

    #[test]
    fn test_mut_normal() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
        *a += 10;
        *b += 100;
        *c += 1000;
        assert_eq!(v, vec![11, 2, 103, 4, 1005]);
    }

    #[test]
    fn test_ref_normal() {
        let v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many(&v, [0, 2, 4]);
        assert_eq!(a, &1);
        assert_eq!(b, &3);
        assert_eq!(c, &5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_empty() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [] = index_many_mut(&mut v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ref_empty() {
        let v = vec![1, 2, 3, 4, 5];
        let [] = index_many(&v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_single_first() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [0]);
        *a += 10;
        assert_eq!(v, vec![11, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ref_single_first() {
        let v = vec![1, 2, 3, 4, 5];
        let [a] = index_many(&v, [0]);
        assert_eq!(a, &1);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_single_last() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [4]);
        *a += 10;
        assert_eq!(v, vec![1, 2, 3, 4, 15]);
    }

    #[test]
    fn test_ref_single_last() {
        let v = vec![1, 2, 3, 4, 5];
        let [a] = index_many(&v, [4]);
        assert_eq!(a, &5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic(
        expected = "Index 5 is out of bounds of slice with len 5 (indices [5], position 0)"
    )]
    fn test_mut_oob_nonempty() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [5]);
    }

    #[test]
    #[should_panic(
        expected = "Index 5 is out of bounds of slice with len 5 (indices [5], position 0)"
    )]
    fn test_ref_oob_nonempty() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [5]);
    }

    #[test]
    #[should_panic(
        expected = "Index 0 is out of bounds of slice with len 0 (indices [0], position 0)"
    )]
    fn test_mut_oob_empty() {
        let mut v: Vec<i32> = vec![];
        index_many_mut(&mut v, [0]);
    }

    #[test]
    #[should_panic(
        expected = "Index 0 is out of bounds of slice with len 0 (indices [0], position 0)"
    )]
    fn test_ref_oob_empty() {
        let v: Vec<i32> = vec![];
        index_many(&v, [0]);
    }

    #[test]
    fn test_mut_unsorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [3, 1, 4]);
    }

    #[test]
    fn test_ref_unsorted() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [3, 1, 4]);
    }

    #[test]
    #[should_panic(
        expected = "Index 3 appears more than once (indices [1, 3, 3, 4], position 1 and 2)"
    )]
    fn test_mut_duplicate() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [1, 3, 3, 4]);
    }

    #[test]
    #[should_panic(
        expected = "Index 3 appears more than once (indices [1, 3, 3, 4], position 1 and 2)"
    )]
    fn test_ref_duplicate() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [1, 3, 3, 4]);
    }
}
