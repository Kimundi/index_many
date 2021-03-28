pub type Elem = usize;
pub const LEN: usize = 3;

pub unsafe fn index_many_mut_hardcoded_checked_simple(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> [&mut Elem; LEN] {
    index_many::simple::index_many_mut(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_unchecked_simple(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> [&mut Elem; LEN] {
    index_many::simple::index_many_mut_unchecked(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_option_simple(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Option<[&mut Elem; LEN]> {
    index_many::simple::get_many_mut(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_checked(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_unchecked(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut_unchecked(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_option(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Option<[&mut Elem; LEN]> {
    index_many::generic::get_many_mut(slice, indices)
}

pub fn index_many_mut_hardcoded_presorted(
    slice: &mut [Elem],
    indices: index_many::generic::PresortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}

pub fn index_many_mut_hardcoded_unsorted(
    slice: &mut [Elem],
    indices: index_many::generic::UnsortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    index_many::generic::index_many_mut(slice, indices)
}
