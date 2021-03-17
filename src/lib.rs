#![feature(array_windows)]
#![feature(array_map)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_uninit_array)]

use std::mem::MaybeUninit;

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
    let mut arr: [MaybeUninit<&'a T>; N] = MaybeUninit::uninit_array::<N>();

    for (dst, idx) in arr.iter_mut().zip(indices.iter().copied()) {
        dst.write(slice.get_unchecked(idx));
    }

    std::mem::transmute_copy::<_, [&'a T; N]>(&arr)
}

pub unsafe fn index_many_mut_unchecked<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: [usize; N],
) -> [&'a mut T; N] {
    let mut arr: [MaybeUninit<&'a mut T>; N] = MaybeUninit::uninit_array::<N>();

    for (dst, idx) in arr.iter_mut().zip(indices.iter().copied()) {
        dst.write(&mut *(slice.get_unchecked_mut(idx) as *mut _));
    }

    std::mem::transmute_copy::<_, [&'a mut T; N]>(&arr)
}

pub fn get_many<T, const N: usize>(slice: &[T], indices: [usize; N]) -> Option<[&T; N]> {
    if !check_indices_valid(&indices, slice.len()) {
        return None;
    }
    unsafe { Some(index_many_unchecked(slice, indices)) }
}

pub fn get_many_mut<T, const N: usize>(
    slice: &mut [T],
    indices: [usize; N],
) -> Option<[&mut T; N]> {
    if !check_indices_valid(&indices, slice.len()) {
        return None;
    }
    unsafe { Some(index_many_mut_unchecked(slice, indices)) }
}

pub fn index_many<T, const N: usize>(slice: &[T], indices: [usize; N]) -> [&T; N] {
    get_many(slice, indices).expect("indices not sorted or out of bounds")
}

pub fn index_many_mut<T, const N: usize>(slice: &mut [T], indices: [usize; N]) -> [&mut T; N] {
    get_many_mut(slice, indices).expect("indices not sorted or out of bounds")
}

pub trait SliceExt {
    type Item;

    unsafe fn index_many_unchecked<'a, const N: usize>(
        &'a self,
        indices: [usize; N],
    ) -> [&'a Self::Item; N];

    unsafe fn index_many_mut_unchecked<'a, const N: usize>(
        &'a mut self,
        indices: [usize; N],
    ) -> [&'a mut Self::Item; N];

    fn get_many<'a, const N: usize>(&'a self, indices: [usize; N]) -> Option<[&'a Self::Item; N]>;

    fn get_many_mut<'a, const N: usize>(
        &'a mut self,
        indices: [usize; N],
    ) -> Option<[&'a mut Self::Item; N]>;

    fn index_many<'a, const N: usize>(&'a self, indices: [usize; N]) -> [&'a Self::Item; N];

    fn index_many_mut<'a, const N: usize>(
        &'a mut self,
        indices: [usize; N],
    ) -> [&'a mut Self::Item; N];
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
    fn test_normal() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
        *a += 10;
        *b += 100;
        *c += 1000;
        assert_eq!(v, vec![11, 2, 103, 4, 1005]);
    }

    #[test]
    fn test_empty() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [] = index_many_mut(&mut v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_single_first() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [0]);
        *a += 10;
        assert_eq!(v, vec![11, 2, 3, 4, 5]);
    }

    #[test]
    fn test_single_last() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [4]);
        *a += 10;
        assert_eq!(v, vec![1, 2, 3, 4, 15]);
    }

    #[test]
    // #[should_panic(expected = "index 5 is out of bounds")]
    #[should_panic]
    fn test_oob_nonempty() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [5]);
    }

    #[test]
    // #[should_panic(expected = "index 0 is out of bounds")]
    #[should_panic]
    fn test_oob_empty() {
        let mut v: Vec<i32> = vec![];
        index_many_mut(&mut v, [0]);
    }

    #[test]
    // #[should_panic(expected = "indices [3, 1, 9] are not unique or sorted in ascending order")]
    #[should_panic]
    fn test_unsorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [3, 1, 9]);
    }
}
