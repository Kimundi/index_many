#![feature(array_windows)]
#![feature(slice_ptr_get)]
#![feature(is_sorted)]

use std::mem::MaybeUninit;

pub mod generic;
pub mod simple;

unsafe fn index_many_internal<'a, T, const N: usize>(
    slice: *const [T],
    indices: [usize; N],
) -> [&'a T; N] {
    let mut arr: MaybeUninit<[&'a T; N]> = MaybeUninit::uninit();
    // Get a pointer to the first array element, for ease of writing to it by offset.
    let arr_ptr = arr.as_mut_ptr() as *mut &'a T;
    let mut i = 0;
    // You can't beat `while i < N` for performance when `N` is a constant-generic parameter.
    while i < N {
        arr_ptr
            .add(i)
            .write(&*slice.get_unchecked(*indices.get_unchecked(i)));
        i += 1;
    }
    // All the elements in `arr` are now definitely initialized, so we can safely call `assume_init`.
    arr.assume_init()
}

unsafe fn index_many_mut_internal<'a, T, const N: usize>(
    slice: *mut [T],
    indices: [usize; N],
) -> [&'a mut T; N] {
    let mut arr: MaybeUninit<[&'a mut T; N]> = MaybeUninit::uninit();
    // Get a pointer to the first array element, for ease of writing to it by offset.
    let arr_ptr = arr.as_mut_ptr() as *mut &'a mut T;
    let mut i = 0;
    // You can't beat `while i < N` for performance when `N` is a constant-generic parameter.
    while i < N {
        arr_ptr
            .add(i)
            .write(&mut *slice.get_unchecked_mut(*indices.get_unchecked(i)));
        i += 1;
    }
    // All the elements in `arr` are now definitely initialized, so we can safely call `assume_init`.
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
