use std::ops::{Range, RangeInclusive};

use super::*;

#[inline(never)]
#[cold]
#[track_caller]
fn range_check_fail(indices: &[Range<usize>], len: usize) -> ! {
    // Check that all ranges are valid
    for (i, idx) in indices.iter().enumerate() {
        if idx.start > idx.end {
            panic!(
                "Range {:?} starts at {}, but ends at {} (ranges {:?}, at {})",
                idx, idx.start, idx.end, indices, i,
            );
        }
    }

    // Check that the ranges are sorted and don't overlap
    for [a, b] in indices.array_windows() {
        if a.end > b.start {
            panic!("Ranges {:?} overlap or are not sorted", indices);
        }
    }

    // Check that the ranges are in bound
    for (i, idx) in indices.iter().enumerate() {
        if idx.end > len {
            panic!(
                "Range {:?} is out of bounds of slice with len {} (ranges {:?}, at {})",
                idx, len, indices, i,
            );
        }
    }

    // Fallthrough case, in case we missed anything above
    panic!(
        "Ranges {:?} are invalid for a slice with len {}",
        indices, len
    );
}

#[inline]
fn check_range_indices_valid<const N: usize>(indices: &[Range<usize>; N], len: usize) -> bool {
    let mut valid = true;

    for [a, b] in indices.array_windows() {
        valid &= a.start <= a.end;
        valid &= a.end <= b.start;
    }

    if let Some(a) = indices.last() {
        valid &= a.start <= a.end;
        valid &= a.end <= len;
    }

    valid
}

impl<T, const N: usize> SliceIndices<[T], N> for Range<usize> {
    type Output = [T];

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
        if check_range_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_indices_valid`
            unsafe { Some(Self::get_many_unchecked(indices, slice)) }
        } else {
            None
        }
    }
    fn get_many_mut(indices: [Self; N], slice: &mut [T]) -> Option<[&mut Self::Output; N]> {
        if check_range_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_indices_valid`
            unsafe { Some(Self::get_many_unchecked_mut(indices, slice)) }
        } else {
            None
        }
    }

    fn index_many(indices: [Self; N], slice: &[T]) -> [&Self::Output; N] {
        if check_range_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_indices_valid`
            unsafe { Self::get_many_unchecked(indices, slice) }
        } else {
            range_check_fail(&indices, slice.len())
        }
    }
    fn index_many_mut(indices: [Self; N], slice: &mut [T]) -> [&mut Self::Output; N] {
        if check_range_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_indices_valid`
            unsafe { Self::get_many_unchecked_mut(indices, slice) }
        } else {
            range_check_fail(&indices, slice.len())
        }
    }
}

#[inline(never)]
#[cold]
#[track_caller]
fn range_inclusive_check_fail(indices: &[RangeInclusive<usize>], len: usize) -> ! {
    // Check that all ranges are valid
    for (i, idx) in indices.iter().enumerate() {
        if idx.start() > idx.end() {
            panic!(
                "Range {:?} starts at {}, but ends at {} (ranges {:?}, at {})",
                idx,
                idx.start(),
                idx.end(),
                indices,
                i,
            );
        }
    }

    // Check that the ranges are sorted and don't overlap
    for [a, b] in indices.array_windows() {
        if a.end() >= b.start() {
            panic!("Ranges {:?} overlap or are not sorted", indices);
        }
    }

    // Check that the ranges are in bound
    for (i, idx) in indices.iter().enumerate() {
        if idx.end() >= &len {
            panic!(
                "Range {:?} is out of bounds of slice with len {} (ranges {:?}, at {})",
                idx, len, indices, i,
            );
        }
    }

    // Fallthrough case, in case we missed anything above
    panic!(
        "Ranges {:?} are invalid for a slice with len {}",
        indices, len
    );
}

#[inline]
fn check_range_inclusive_indices_valid<const N: usize>(
    indices: &[RangeInclusive<usize>; N],
    len: usize,
) -> bool {
    let mut valid = true;

    for [a, b] in indices.array_windows() {
        valid &= a.start() <= a.end();
        valid &= a.end() < b.start();
    }

    if let Some(a) = indices.last() {
        valid &= a.start() <= a.end();
        valid &= a.end() < &len;
    }

    valid
}

impl<T, const N: usize> SliceIndices<[T], N> for RangeInclusive<usize> {
    type Output = [T];

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
        if check_range_inclusive_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_inclusive_indices_valid`
            unsafe { Some(Self::get_many_unchecked(indices, slice)) }
        } else {
            None
        }
    }
    fn get_many_mut(indices: [Self; N], slice: &mut [T]) -> Option<[&mut Self::Output; N]> {
        if check_range_inclusive_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_inclusive_indices_valid`
            unsafe { Some(Self::get_many_unchecked_mut(indices, slice)) }
        } else {
            None
        }
    }

    fn index_many(indices: [Self; N], slice: &[T]) -> [&Self::Output; N] {
        if check_range_inclusive_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_inclusive_indices_valid`
            unsafe { Self::get_many_unchecked(indices, slice) }
        } else {
            range_inclusive_check_fail(&indices, slice.len())
        }
    }
    fn index_many_mut(indices: [Self; N], slice: &mut [T]) -> [&mut Self::Output; N] {
        if check_range_inclusive_indices_valid(&indices, slice.len()) {
            // SAFETY: We checked that the slices are valid
            // with `check_range_inclusive_indices_valid`
            unsafe { Self::get_many_unchecked_mut(indices, slice) }
        } else {
            range_inclusive_check_fail(&indices, slice.len())
        }
    }
}
