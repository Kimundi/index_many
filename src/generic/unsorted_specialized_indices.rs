use super::Indices;

/// This type allows any order for the indices, as long as they are unique.
///
/// This requires `O(N^2)` comparisons to check if the indices are unique and
/// in bounds of a slice.
///
/// This variant includes manual optimizations for N <= 3, though they don't seem
/// to improve the codegen.
///
/// # Example codegen
/// See [`crate::_doc_assembly::checked_unsorted_specialized_3()`]
#[derive(Copy, Clone)]
pub struct UnsortedSpecializedIndices<const N: usize>(pub [usize; N]);

unsafe impl<const N: usize> Indices<N> for UnsortedSpecializedIndices<N> {
    #[inline]
    fn to_raw_indices(&self) -> [usize; N] {
        self.0
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        match N {
            0 => {}
            1 => {
                let a = self.0[0];
                valid = a < len;
            }
            2 => {
                let a = self.0[0];
                let b = self.0[1];
                valid &= a != b;
                valid &= a < len;
                valid &= b < len;
            }
            3 => {
                let a = self.0[0];
                let b = self.0[1];
                let c = self.0[2];

                valid &= a < len;

                valid &= b < len;
                valid &= b != a;

                valid &= c < len;
                valid &= c != a;
                valid &= c != b;
            }
            _ => {
                for (i, &idx) in self.0.iter().enumerate() {
                    valid &= idx < len;
                    for &idx2 in &self.0[..i] {
                        valid &= idx != idx2;
                    }
                }
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
    use super::UnsortedSpecializedIndices;

    fn index_many<'a, T, const N: usize>(slice: &[T], indices: [usize; N]) -> [&T; N] {
        let indices = UnsortedSpecializedIndices(indices);
        super::super::index_many(slice, indices)
    }

    fn index_many_mut<'a, T, const N: usize>(slice: &mut [T], indices: [usize; N]) -> [&mut T; N] {
        let indices = UnsortedSpecializedIndices(indices);
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
    fn test_ref_unsorted_variable() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(index_many(&v, [3, 1, 4]), [&3, &1, &4]);
        assert_eq!(index_many(&v, [3, 1]), [&3, &1]);
        assert_eq!(index_many(&v, [1]), [&1]);
        assert_eq!(index_many(&v, []), [&999_i32; 0]);
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
