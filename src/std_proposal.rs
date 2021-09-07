use std::{mem, num::NonZeroUsize};

pub trait SliceExt<T> {
    unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N])
        -> [&mut T; N];

    fn get_many_mut_opt<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]>;
    fn get_many_mut_res_simple<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorSimple<N>>;
    fn get_many_mut_res_direct<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorKind>;
    fn get_many_mut_res_indirect<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], Error<N>>;
    fn get_many_mut_res_indirect_niche<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorNiche<N>>;
}

impl<T> SliceExt<T> for [T] {
    #[inline]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> [&mut T; N] {
        // NB: This implementation is written as it is because any variation of
        // `indices.map(|i| self.get_unchecked_mut(i))` would make miri unhappy,
        // or generate worse code otherwise. This is also why we need to through
        // a raw pointer here.
        let slice: *mut [T] = self;
        let mut arr: mem::MaybeUninit<[&mut T; N]> = mem::MaybeUninit::uninit();
        let arr_ptr = arr.as_mut_ptr();

        // SAFETY: We expect `indices` to contain disjunct values that are
        // in bounds of `self`.
        unsafe {
            for i in 0..N {
                let idx = *indices.get_unchecked(i);
                *(*arr_ptr).get_unchecked_mut(i) = &mut *slice.get_unchecked_mut(idx);
            }
            arr.assume_init()
        }
    }

    #[inline]
    fn get_many_mut_opt<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]> {
        if !get_many_check_valid(&indices, self.len()) {
            return None;
        }
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Some(self.get_many_unchecked_mut(indices)) }
    }

    fn get_many_mut_res_simple<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorSimple<N>> {
        if !get_many_check_valid(&indices, self.len()) {
            return Err(ErrorSimple);
        }
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_many_unchecked_mut(indices)) }
    }

    fn get_many_mut_res_direct<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorKind> {
        get_many_check_valid_kinds(&indices, self.len())?;
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_many_unchecked_mut(indices)) }
    }

    fn get_many_mut_res_indirect<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], Error<N>> {
        if !get_many_check_valid(&indices, self.len()) {
            return Err(Error {
                indices,
                slice_len: self.len(),
            });
        }
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_many_unchecked_mut(indices)) }
    }

    fn get_many_mut_res_indirect_niche<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], ErrorNiche<N>> {
        if !get_many_check_valid(&indices, self.len()) {
            return Err(ErrorNiche {
                indices,
                slice_len: unsafe { SliceLenWithNiche(self.len().unchecked_add(2)) },
            });
        }
        // SAFETY: The `get_many_check_valid()` call checked that all indices
        // are disjunct and in bounds.
        unsafe { Ok(self.get_many_unchecked_mut(indices)) }
    }
}

/// This checks every index against each other, and against `len`.
///
/// This will do `binomial(N + 1, 2) = N * (N + 1) / 2 = 0, 1, 3, 6, 10, ..`
/// comparison operations.
fn get_many_check_valid<const N: usize>(indices: &[usize; N], len: usize) -> bool {
    // NB: The optimizer should inline the loops into a sequence
    // of instructions without additional branching.
    let mut valid = true;
    for (i, &idx) in indices.iter().enumerate() {
        valid &= idx < len;
        for &idx2 in &indices[..i] {
            valid &= idx != idx2;
        }
    }
    valid
}

// NB: The N here is there to be forward-compatible with adding more details
// to the error later
#[derive(Debug)]
pub struct ErrorSimple<const N: usize>;

pub struct Error<const N: usize> {
    indices: [usize; N],
    slice_len: usize,
}

#[rustc_layout_scalar_valid_range_start(2)]
#[rustc_nonnull_optimization_guaranteed]
struct SliceLenWithNiche(usize);

pub struct ErrorNiche<const N: usize> {
    indices: [usize; N],
    slice_len: SliceLenWithNiche,
}

pub enum ErrorKind {
    OutOfBounds,
    NotUnique,
}

/// This checks every index against each other, and against `len`.
///
/// This will do `binomial(N + 1, 2) = N * (N + 1) / 2 = 0, 1, 3, 6, 10, ..`
/// comparison operations.
fn get_many_check_valid_kinds<const N: usize>(
    indices: &[usize; N],
    len: usize,
) -> Result<(), ErrorKind> {
    // NB: The optimizer should inline the loops into a sequence
    // of instructions without additional branching.
    for (i, &idx) in indices.iter().enumerate() {
        if idx >= len {
            return Err(ErrorKind::OutOfBounds);
        }
        for &idx2 in &indices[..i] {
            if idx == idx2 {
                return Err(ErrorKind::NotUnique);
            }
        }
    }
    Ok(())
}

// TODO: write tests
