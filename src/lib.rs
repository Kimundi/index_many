#![feature(array_windows)]
#![feature(slice_ptr_get)]
#![feature(is_sorted)]
#![feature(external_doc)]
#![doc(include = "../README.md")]

use std::mem::MaybeUninit;

pub mod generic;
pub mod simple;

unsafe fn index_many_internal<'a, T, const N: usize>(
    slice: *const [T],
    indices: [usize; N],
) -> [&'a T; N] {
    let mut arr: MaybeUninit<[&'a T; N]> = MaybeUninit::uninit();
    let arr_ptr = arr.as_mut_ptr();
    for i in 0..N {
        let idx = *indices.get_unchecked(i);
        *(*arr_ptr).get_unchecked_mut(i) = &*slice.get_unchecked(idx);
    }
    arr.assume_init()
}

unsafe fn index_many_mut_internal<'a, T, const N: usize>(
    slice: *mut [T],
    indices: [usize; N],
) -> [&'a mut T; N] {
    let mut arr: MaybeUninit<[&'a mut T; N]> = MaybeUninit::uninit();
    let arr_ptr = arr.as_mut_ptr();
    for i in 0..N {
        let idx = *indices.get_unchecked(i);
        *(*arr_ptr).get_unchecked_mut(i) = &mut *slice.get_unchecked_mut(idx);
    }
    arr.assume_init()
}

fn bound_check_failed(indices: &[usize], len: usize) -> ! {
    for (i, &idx) in indices.iter().enumerate() {
        if idx >= len {
            panic!(
                "Index {} is out of bounds of slice with len {} (indices {:?}, position {})",
                idx, len, indices, i,
            );
        }
    }

    // TODO: We might want to use a linear-time algorithm here instead
    for (i, &idx) in indices.iter().enumerate() {
        for (j, &idx2) in indices[..i].iter().enumerate() {
            if idx == idx2 {
                panic!(
                    "Index {} appears more than once (indices {:?}, position {} and {})",
                    idx, indices, j, i,
                );
            }
        }
    }

    // Fallthrough case
    panic!(
        "Indices {:?} are invalid for a slice with len {}",
        indices, len
    );
}

fn sorted_bound_check_failed(indices: &[usize], len: usize) -> ! {
    if !indices.is_sorted() {
        panic!("Indices {:?} are not sorted", indices);
    }
    bound_check_failed(indices, len)
}
