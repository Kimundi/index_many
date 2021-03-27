mod presorted_indices;
mod sorted_indices;
mod unsorted_indices;

pub use presorted_indices::PresortedIndices;
pub use presorted_indices::PresortedIndicesError;
pub use unsorted_indices::UnsortedIndices;

pub unsafe trait Indices<const N: usize>: Copy {
    fn to_indices(&self) -> [usize; N];
    fn is_valid(&self, len: usize) -> bool;
    fn cause_invalid_panic(&self, len: usize) -> !;
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
