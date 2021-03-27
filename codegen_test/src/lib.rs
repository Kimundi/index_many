pub type Elem = usize;
pub const LEN: usize = 3;

pub fn index_many_mut_hardcoded(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    index_many::index_many_mut(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_unchecked(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> [&mut Elem; LEN] {
    index_many::index_many_mut_unchecked(slice, indices)
}

pub fn index_many_mut_hardcoded_sorted(
    slice: &mut [Elem],
    indices: index_many::SortedIndices<3>,
) -> [&mut Elem; LEN] {
    index_many::index_many_mut(slice, indices)
}

pub fn index_many_mut_hardcoded_unsorted(
    slice: &mut [Elem],
    indices: index_many::UnsortedIndices<3>,
) -> [&mut Elem; LEN] {
    index_many::index_many_mut(slice, indices)
}
