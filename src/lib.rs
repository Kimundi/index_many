#![feature(array_windows)]
#![feature(slice_ptr_get)]
#![feature(is_sorted)]
#![feature(unchecked_math)]
#![doc = include_str!("../README.md")]

use std::{array::IntoIter, mem::MaybeUninit, slice::SliceIndex};

pub mod generic;
pub mod simple;
pub mod slice_index;
pub mod with_result;

unsafe fn get_many_internal<'a, T, I: SliceIndex<[T]>, const N: usize>(
    slice: *const [T],
    indices: [I; N],
) -> [&'a I::Output; N] {
    let mut arr: MaybeUninit<[&'a I::Output; N]> = MaybeUninit::uninit();
    let arr_ptr = arr.as_mut_ptr();
    for (i, idx) in IntoIter::new(indices).enumerate() {
        *(*arr_ptr).get_unchecked_mut(i) = &*slice.get_unchecked(idx);
    }
    arr.assume_init()
}

unsafe fn get_many_internal_mut<'a, T, I: SliceIndex<[T]>, const N: usize>(
    slice: *mut [T],
    indices: [I; N],
) -> [&'a mut I::Output; N] {
    let mut arr: MaybeUninit<[&'a mut I::Output; N]> = MaybeUninit::uninit();
    let arr_ptr = arr.as_mut_ptr();
    for (i, idx) in IntoIter::new(indices).enumerate() {
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
