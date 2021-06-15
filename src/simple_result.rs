//! A variant of the simple API that returns a `Result` instead.

#[inline]
fn check_indices_valid(indices: &[usize], len: usize) -> bool {
    let mut valid = true;

    for &[a, b] in indices.array_windows() {
        valid &= a < b;
    }

    if let Some(&idx) = indices.last() {
        valid &= idx < len;
    }

    valid
}

pub unsafe fn index_many_unchecked<'a, T, const N: usize>(
    slice: &'a [T],
    indices: [usize; N],
) -> [&'a T; N] {
    crate::get_many_internal(slice, indices)
}

pub unsafe fn index_many_mut_unchecked<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: [usize; N],
) -> [&'a mut T; N] {
    crate::get_many_internal_mut(slice, indices)
}

pub struct GetManyError<const N: usize> {
    indices: [usize; N],
    len: usize,
}

pub enum GetManyErrorKind {
    OutOfBounds {
        many_idx: usize,
        slice_idx: usize,
        slice_len: usize,
    },
    NotSorted {
        many_idx_1: usize,
        many_idx_2: usize,
    },
    NotUnique {
        many_idx_1: usize,
        many_idx_2: usize,
    },
}

impl<const N: usize> GetManyError<N> {
    pub fn kind(&self) -> GetManyErrorKind {
        for (i, &[a, b]) in self.indices.array_windows().enumerate() {
            if a == b {
                return GetManyErrorKind::NotUnique {
                    many_idx_1: i,
                    many_idx_2: i + 1,
                };
            } else if a > b {
                return GetManyErrorKind::NotSorted {
                    many_idx_1: i,
                    many_idx_2: i + 1,
                };
            }
        }
        for (many_idx, slice_idx) in self.indices.iter().copied().enumerate() {
            if slice_idx >= self.len {
                return GetManyErrorKind::OutOfBounds {
                    many_idx,
                    slice_idx,
                    slice_len: self.len,
                };
            }
        }
        unreachable!()
    }
}

pub fn get_many<'a, T, const N: usize>(
    slice: &[T],
    indices: [usize; N],
) -> Result<[&T; N], GetManyError<N>> {
    if !check_indices_valid(&indices, slice.len()) {
        return Err(GetManyError {
            indices,
            len: slice.len(),
        });
    }
    unsafe { Ok(index_many_unchecked(slice, indices)) }
}

pub fn get_many_mut<'a, T, const N: usize>(
    slice: &mut [T],
    indices: [usize; N],
) -> Result<[&mut T; N], GetManyError<N>> {
    if !check_indices_valid(&indices, slice.len()) {
        return Err(GetManyError {
            indices,
            len: slice.len(),
        });
    }
    unsafe { Ok(index_many_mut_unchecked(slice, indices)) }
}

pub fn index_many<'a, T, const N: usize>(slice: &[T], indices: [usize; N]) -> [&T; N] {
    let len = slice.len();
    match get_many(slice, indices) {
        Ok(s) => s,
        Err(_) => {
            let tmp = indices;
            crate::sorted_bound_check_failed(&tmp, len)
        }
    }
}

pub fn index_many_mut<'a, T, const N: usize>(slice: &mut [T], indices: [usize; N]) -> [&mut T; N] {
    let len = slice.len();
    match get_many_mut(slice, indices) {
        Ok(s) => s,
        Err(_) => {
            let tmp = indices;
            crate::sorted_bound_check_failed(&tmp, len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_normal() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
        *b += 100;
        *a += 10;
        *c += 1000;
        std::mem::swap(a, b);
        assert_eq!(v, vec![103, 2, 11, 4, 1005]);
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
    #[should_panic(expected = "Indices [3, 1, 4] are not sorted")]
    fn test_mut_unsorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [3, 1, 4]);
    }

    #[test]
    #[should_panic(expected = "Indices [3, 1, 4] are not sorted")]
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
