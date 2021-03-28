//! A generic variant of the API, that can be indexed by any type implementing `Indices<N>`.
//!
//! The `Indices<N>` trait allows for more flexibility in its implementors:
//!
//! - `[usize; N]` works like the simple API.
//! - `PresortedIndices<N>` ensures statically that the indices are sorted.
//! - `UnsortedIndices<N>` allows any order for the indices.

mod presorted_indices;
mod sorted_indices;
mod unsorted_indices;

pub use presorted_indices::PresortedIndices;
pub use presorted_indices::PresortedIndicesError;
pub use unsorted_indices::UnsortedIndices;

pub unsafe trait Indices<const N: usize>: Copy {
    fn to_raw_indices(&self) -> [usize; N];
    fn is_valid(&self, len: usize) -> bool;
    fn cause_invalid_panic(&self, len: usize) -> !;
}

pub unsafe fn index_many_unchecked<'a, T, I: Indices<N>, const N: usize>(
    slice: &'a [T],
    indices: I,
) -> [&'a T; N] {
    crate::get_many_internal(slice, indices.to_raw_indices())
}

pub unsafe fn index_many_mut_unchecked<'a, T, I: Indices<N>, const N: usize>(
    slice: &'a mut [T],
    indices: I,
) -> [&'a mut T; N] {
    crate::get_many_internal_mut(slice, indices.to_raw_indices())
}

pub fn get_many<'a, T, I: Indices<N>, const N: usize>(slice: &[T], indices: I) -> Option<[&T; N]> {
    if !indices.is_valid(slice.len()) {
        return None;
    }
    unsafe { Some(index_many_unchecked(slice, indices)) }
}

pub fn get_many_mut<'a, T, I: Indices<N>, const N: usize>(
    slice: &mut [T],
    indices: I,
) -> Option<[&mut T; N]> {
    if !indices.is_valid(slice.len()) {
        return None;
    }
    unsafe { Some(index_many_mut_unchecked(slice, indices)) }
}

pub fn index_many<'a, T, I: Indices<N>, const N: usize>(slice: &[T], indices: I) -> [&T; N] {
    let len = slice.len();
    match get_many(slice, indices) {
        Some(s) => s,
        None => {
            let tmp = indices;
            tmp.cause_invalid_panic(len)
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
            let tmp = indices;
            tmp.cause_invalid_panic(len)
        }
    }
}
