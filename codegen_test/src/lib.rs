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
