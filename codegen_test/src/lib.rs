pub fn index_many_mut_hardcoded(slice: &mut [usize], indices: [usize; 3]) -> [&mut usize; 3] {
    index_many::index_many_mut(slice, indices)
}

pub fn index_many_mut_hardcoded_fix(slice: &mut [usize], [a, b, c]: [usize; 3]) -> [&mut usize; 3] {
    if !(a < b && b < c) {
        panic!("not unique");
    }
    let a: *mut usize = &mut slice[a];
    let b: *mut usize = &mut slice[b];
    let c: *mut usize = &mut slice[c];
    unsafe { [&mut *a, &mut *b, &mut *c] }
}
