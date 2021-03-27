use super::Indices;

#[derive(Copy, Clone)]
pub struct UnsortedIndices<const N: usize>(pub [usize; N]);

unsafe impl<const N: usize> Indices<N> for UnsortedIndices<N> {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        self.0
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        for (i, &idx) in self.0.iter().enumerate() {
            valid &= idx < len;
            for &idx2 in &self.0[..i] {
                valid &= idx != idx2;
            }
        }

        valid
    }

    #[inline(always)]
    fn cause_invalid_panic(&self, len: usize) -> ! {
        crate::bound_check_failed(&self.0, len)
    }
}
