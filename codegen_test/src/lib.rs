use std::ops::Range;

pub type Elem = usize;
pub const LEN: usize = 3;

pub unsafe fn checked_simple(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::simple::index_many_mut(slice, indices)
}

pub unsafe fn unchecked_simple(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::simple::index_many_mut_unchecked(slice, indices)
}

pub unsafe fn option_simple(slice: &mut [Elem], indices: [usize; LEN]) -> Option<[&mut Elem; LEN]> {
    index_many::simple::get_many_mut(slice, indices)
}

pub unsafe fn checked(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}

pub unsafe fn unchecked(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut_unchecked(slice, indices)
}

pub unsafe fn option(slice: &mut [Elem], indices: [usize; LEN]) -> Option<[&mut Elem; LEN]> {
    index_many::generic::get_many_mut(slice, indices)
}

pub fn presorted(
    slice: &mut [Elem],
    indices: index_many::generic::PresortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}

pub fn unsorted(
    slice: &mut [Elem],
    indices: index_many::generic::UnsortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}

pub unsafe fn checked_usize_trait(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::slice_index::index_many_mut(slice, indices)
}

pub unsafe fn unchecked_usize_trait(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::slice_index::get_many_unchecked_mut(slice, indices)
}

pub unsafe fn option_usize_trait(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Option<[&mut Elem; LEN]> {
    index_many::slice_index::get_many_mut(slice, indices)
}

pub unsafe fn checked_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> [&mut [Elem]; LEN] {
    index_many::slice_index::index_many_mut(slice, indices)
}

pub unsafe fn unchecked_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> [&mut [Elem]; LEN] {
    index_many::slice_index::get_many_unchecked_mut(slice, indices)
}

pub unsafe fn option_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> Option<[&mut [Elem]; LEN]> {
    index_many::slice_index::get_many_mut(slice, indices)
}
