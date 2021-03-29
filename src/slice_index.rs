//! A variant of the API that also works with ranges.
//!
//! The elements have to be sorted, and not overlap. This ensures that
//! only `O(N)` comparisons are needed at runtime to verify the indices are in bounds.

mod ranges;
mod single_usize;

pub trait SliceIndices<T: ?Sized, const N: usize>: Sized {
    type Output: ?Sized;

    unsafe fn get_many_unchecked(this: [Self; N], slice: &T) -> [&Self::Output; N];
    unsafe fn get_many_unchecked_mut(this: [Self; N], slice: &mut T) -> [&mut Self::Output; N];

    fn get_many(this: [Self; N], slice: &T) -> Option<[&Self::Output; N]>;
    fn get_many_mut(this: [Self; N], slice: &mut T) -> Option<[&mut Self::Output; N]>;

    fn index_many(this: [Self; N], slice: &T) -> [&Self::Output; N];
    fn index_many_mut(this: [Self; N], slice: &mut T) -> [&mut Self::Output; N];
}

pub unsafe fn get_many_unchecked<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &'a [T],
    indices: [I; N],
) -> [&'a I::Output; N] {
    I::get_many_unchecked(indices, slice)
}

pub unsafe fn get_many_unchecked_mut<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &'a mut [T],
    indices: [I; N],
) -> [&'a mut I::Output; N] {
    I::get_many_unchecked_mut(indices, slice)
}

pub fn get_many<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &[T],
    indices: [I; N],
) -> Option<[&I::Output; N]> {
    I::get_many(indices, slice)
}

pub fn get_many_mut<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &mut [T],
    indices: [I; N],
) -> Option<[&mut I::Output; N]> {
    I::get_many_mut(indices, slice)
}

pub fn index_many<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &[T],
    indices: [I; N],
) -> [&I::Output; N] {
    I::index_many(indices, slice)
}

pub fn index_many_mut<'a, T, I: SliceIndices<[T], N>, const N: usize>(
    slice: &mut [T],
    indices: [I; N],
) -> [&mut I::Output; N] {
    I::index_many_mut(indices, slice)
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
        let [] = index_many_mut::<_, usize, 0>(&mut v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ref_empty() {
        let v = vec![1, 2, 3, 4, 5];
        let [] = index_many::<_, usize, 0>(&v, []);
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
