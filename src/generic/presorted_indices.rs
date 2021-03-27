use std::ops::Deref;

use super::Indices;

#[derive(Copy, Clone)]
pub struct PresortedIndices<const N: usize> {
    indices: [usize; N],
}

impl<const N: usize> Deref for PresortedIndices<N> {
    type Target = [usize; N];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

pub struct PresortedIndicesError {
    _private: (),
}

impl<const N: usize> PresortedIndices<N> {
    pub fn new(indices: [usize; N]) -> Result<Self, PresortedIndicesError> {
        let mut valid = true;
        for &[a, b] in indices.array_windows() {
            valid &= a < b;
        }
        if valid {
            Ok(Self { indices })
        } else {
            Err(PresortedIndicesError { _private: () })
        }
    }
}

unsafe impl<const N: usize> Indices<N> for PresortedIndices<N> {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        self.indices
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        if let Some(&idx) = self.indices.last() {
            valid &= idx < len;
        }

        valid
    }

    #[inline(always)]
    fn cause_invalid_panic(&self, len: usize) -> ! {
        crate::bound_check_failed(&self.indices, len)
    }
}
