pub fn index_many_mut_hardcoded(slice: &mut [usize], indices: [usize; 3]) -> [&mut usize; 3] {
    index_many::index_many_mut(slice, indices)
}

pub unsafe fn index_many_mut_hardcoded_unchecked(
    slice: &mut [usize],
    indices: [usize; 3],
) -> [&mut usize; 3] {
    index_many::index_many_mut_unchecked(slice, indices)
}
