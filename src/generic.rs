use std::ops::Deref;

pub unsafe trait Indices<const N: usize>: Copy {
    fn to_indices(&self) -> [usize; N];
    fn is_in_bounds(&self, len: usize) -> bool;
}

unsafe impl<const N: usize> Indices<N> for [usize; N] {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        *self
    }

    #[inline]
    fn is_in_bounds(&self, len: usize) -> bool {
        let mut valid = true;

        for &[a, b] in self.array_windows() {
            valid &= a < b;
        }

        if let Some(&idx) = self.last() {
            valid &= idx < len;
        }

        valid
    }
}

#[derive(Copy, Clone)]
pub struct SortedIndices<const N: usize> {
    indices: [usize; N],
}

impl<const N: usize> Deref for SortedIndices<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

pub struct SortedIndicesError {
    _private: (),
}

impl<const N: usize> SortedIndices<N> {
    pub fn new(indices: [usize; N]) -> Result<Self, SortedIndicesError> {
        let mut valid = true;
        for &[a, b] in indices.array_windows() {
            valid &= a < b;
        }
        if valid {
            Ok(Self { indices })
        } else {
            Err(SortedIndicesError { _private: () })
        }
    }
}

unsafe impl<const N: usize> Indices<N> for SortedIndices<N> {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        self.indices
    }

    #[inline]
    fn is_in_bounds(&self, len: usize) -> bool {
        let mut valid = true;

        if let Some(&idx) = self.indices.last() {
            valid &= idx < len;
        }

        valid
    }
}

#[derive(Copy, Clone)]
pub struct UnsortedIndices<const N: usize>(pub [usize; N]);

unsafe impl<const N: usize> Indices<N> for UnsortedIndices<N> {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        self.0
    }

    #[inline]
    fn is_in_bounds(&self, len: usize) -> bool {
        let mut valid = true;

        for (i, &idx) in self.0.iter().enumerate() {
            valid &= idx < len;
            for &idx2 in &self.0[..i] {
                valid &= idx != idx2;
            }
        }

        valid
    }
}

pub unsafe fn index_many_unchecked<'a, T, I: Indices<N>, const N: usize>(
    slice: &'a [T],
    indices: I,
) -> [&'a T; N] {
    crate::index_many_internal(slice, indices.to_indices())
}

pub unsafe fn index_many_mut_unchecked<'a, T, I: Indices<N>, const N: usize>(
    slice: &'a mut [T],
    indices: I,
) -> [&'a mut T; N] {
    crate::index_many_mut_internal(slice, indices.to_indices())
}

pub fn get_many<'a, T, I: Indices<N>, const N: usize>(slice: &[T], indices: I) -> Option<[&T; N]> {
    if !indices.is_in_bounds(slice.len()) {
        return None;
    }
    unsafe { Some(index_many_unchecked(slice, indices)) }
}

pub fn get_many_mut<'a, T, I: Indices<N>, const N: usize>(
    slice: &mut [T],
    indices: I,
) -> Option<[&mut T; N]> {
    if !indices.is_in_bounds(slice.len()) {
        return None;
    }
    unsafe { Some(index_many_mut_unchecked(slice, indices)) }
}

pub fn index_many<'a, T, I: Indices<N>, const N: usize>(slice: &[T], indices: I) -> [&T; N] {
    let len = slice.len();

    match get_many(slice, indices) {
        Some(s) => s,
        None => {
            let tmp = indices.to_indices();
            crate::sorted_bound_check_failed(&tmp, len)
        }
    }
}

pub fn index_many_mut<'a, T, I: Indices<N>, const N: usize>(
    slice: &mut [T],
    indices: I,
) -> [&mut T; N] {
    let len = slice.len();

    match get_many_mut(slice, indices) {
        Some(s) => s,
        None => {
            let tmp = indices.to_indices();
            crate::sorted_bound_check_failed(&tmp, len)
        }
    }
}

pub trait SliceExt {
    type Item;

    unsafe fn index_many_unchecked<const N: usize>(&self, indices: [usize; N]) -> [&Self::Item; N];
    unsafe fn index_many_mut_unchecked<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> [&mut Self::Item; N];

    fn get_many<const N: usize>(&self, indices: [usize; N]) -> Option<[&Self::Item; N]>;
    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N])
        -> Option<[&mut Self::Item; N]>;

    fn index_many<const N: usize>(&self, indices: [usize; N]) -> [&Self::Item; N];
    fn index_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut Self::Item; N];
}

impl<T> SliceExt for [T] {
    type Item = T;

    unsafe fn index_many_unchecked<const N: usize>(&self, indices: [usize; N]) -> [&Self::Item; N] {
        index_many_unchecked(self, indices)
    }
    unsafe fn index_many_mut_unchecked<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> [&mut Self::Item; N] {
        index_many_mut_unchecked(self, indices)
    }

    fn get_many<const N: usize>(&self, indices: [usize; N]) -> Option<[&Self::Item; N]> {
        get_many(self, indices)
    }
    fn get_many_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Option<[&mut Self::Item; N]> {
        get_many_mut(self, indices)
    }

    fn index_many<const N: usize>(&self, indices: [usize; N]) -> [&Self::Item; N] {
        index_many(self, indices)
    }
    fn index_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut Self::Item; N] {
        index_many_mut(self, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    // #[should_panic(expected = "index 5 is out of bounds")]
    #[should_panic]
    fn test_mut_oob_nonempty() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [5]);
    }

    #[test]
    // #[should_panic(expected = "index 5 is out of bounds")]
    #[should_panic]
    fn test_ref_oob_nonempty() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [5]);
    }

    #[test]
    // #[should_panic(expected = "index 0 is out of bounds")]
    #[should_panic]
    fn test_mut_oob_empty() {
        let mut v: Vec<i32> = vec![];
        index_many_mut(&mut v, [0]);
    }

    #[test]
    // #[should_panic(expected = "index 0 is out of bounds")]
    #[should_panic]
    fn test_ref_oob_empty() {
        let v: Vec<i32> = vec![];
        index_many(&v, [0]);
    }

    #[test]
    // #[should_panic(expected = "indices [3, 1, 9] are not unique or sorted in ascending order")]
    #[should_panic]
    fn test_mut_unsorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [3, 1, 9]);
    }

    #[test]
    // #[should_panic(expected = "indices [3, 1, 9] are not unique or sorted in ascending order")]
    #[should_panic]
    fn test_ref_unsorted() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [3, 1, 9]);
    }
}
