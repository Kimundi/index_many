use std::ops::Deref;

use super::Indices;

/// This type ensures statically that the indices are sorted and unique.
///
/// This ensures only a single comparison is needed to check if the indices
/// are in bounds of a slice.
///
/// # Example codegen
/// See [`crate::_doc_assembly::checked_presorted()`]
#[derive(Copy, Clone)]
pub struct PresortedIndices<const N: usize> {
    indices: [usize; N],
}

impl<const N: usize> Deref for PresortedIndices<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

#[derive(Debug)]
pub struct PresortedIndicesError {
    _private: (),
}

impl<const N: usize> PresortedIndices<N> {
    pub fn new(indices: [usize; N]) -> Result<Self, PresortedIndicesError> {
        let mut valid = true;
        for &[a, b] in indices.array_windows() {
            valid &= a < b;
        }
        if valid {
            Ok(Self { indices })
        } else {
            Err(PresortedIndicesError { _private: () })
        }
    }
}

unsafe impl<const N: usize> Indices<N> for PresortedIndices<N> {
    #[inline]
    fn to_raw_indices(&self) -> [usize; N] {
        self.indices
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        if let Some(&idx) = self.indices.last() {
            valid &= idx < len;
        }

        valid
    }

    #[inline(always)]
    fn cause_invalid_panic(&self, len: usize) -> ! {
        crate::bound_check_failed(&self.indices, len)
    }
}

#[cfg(test)]
mod tests {
    use super::PresortedIndices;

    fn index_many<'a, T, const N: usize>(slice: &[T], indices: [usize; N]) -> [&T; N] {
        let indices = PresortedIndices::new(indices).unwrap();
        assert!(indices.indices.is_sorted());
        super::super::index_many(slice, indices)
    }

    fn index_many_mut<'a, T, const N: usize>(slice: &mut [T], indices: [usize; N]) -> [&mut T; N] {
        let indices = PresortedIndices::new(indices).unwrap();
        assert!(indices.indices.is_sorted());
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
    fn test_unsorted() {
        assert!(PresortedIndices::new([3, 1, 4]).is_err())
    }

    #[test]
    fn test_duplicate() {
        assert!(PresortedIndices::new([1, 3, 3, 4]).is_err())
    }
}
