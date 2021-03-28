use super::*;

impl<T, const N: usize> SliceIndices<[T], N> for usize {
    type Output = T;

    unsafe fn get_many_unchecked(indices: [Self; N], slice: &[T]) -> [&Self::Output; N] {
        crate::get_many_internal(slice, indices)
    }
    unsafe fn get_many_unchecked_mut(
        indices: [Self; N],
        slice: &mut [T],
    ) -> [&mut Self::Output; N] {
        crate::get_many_internal_mut(slice, indices)
    }

    fn get_many(indices: [Self; N], slice: &[T]) -> Option<[&Self::Output; N]> {
        crate::simple::get_many(slice, indices)
    }
    fn get_many_mut(indices: [Self; N], slice: &mut [T]) -> Option<[&mut Self::Output; N]> {
        crate::simple::get_many_mut(slice, indices)
    }

    fn index_many(indices: [Self; N], slice: &[T]) -> [&Self::Output; N] {
        crate::simple::index_many(slice, indices)
    }
    fn index_many_mut(indices: [Self; N], slice: &mut [T]) -> [&mut Self::Output; N] {
        crate::simple::index_many_mut(slice, indices)
    }
}
