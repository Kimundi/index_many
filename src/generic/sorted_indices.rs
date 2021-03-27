use super::Indices;

unsafe impl<const N: usize> Indices<N> for [usize; N] {
    #[inline]
    fn to_indices(&self) -> [usize; N] {
        *self
    }

    #[inline]
    fn is_valid(&self, len: usize) -> bool {
        let mut valid = true;

        for &[a, b] in self.array_windows() {
            valid &= a < b;
        }

        if let Some(&idx) = self.last() {
            valid &= idx < len;
        }

        valid
    }

    #[inline(always)]
    fn cause_invalid_panic(&self, len: usize) -> ! {
        crate::sorted_bound_check_failed(self, len)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_mut_normal() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
        *a += 10;
        *b += 100;
        *c += 1000;
        assert_eq!(v, vec![11, 2, 103, 4, 1005]);
    }

    #[test]
    fn test_ref_normal() {
        let v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many(&v, [0, 2, 4]);
        assert_eq!(a, &1);
        assert_eq!(b, &3);
        assert_eq!(c, &5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_empty() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [] = index_many_mut(&mut v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ref_empty() {
        let v = vec![1, 2, 3, 4, 5];
        let [] = index_many(&v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_single_first() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [0]);
        *a += 10;
        assert_eq!(v, vec![11, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ref_single_first() {
        let v = vec![1, 2, 3, 4, 5];
        let [a] = index_many(&v, [0]);
        assert_eq!(a, &1);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mut_single_last() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [4]);
        *a += 10;
        assert_eq!(v, vec![1, 2, 3, 4, 15]);
    }

    #[test]
    fn test_ref_single_last() {
        let v = vec![1, 2, 3, 4, 5];
        let [a] = index_many(&v, [4]);
        assert_eq!(a, &5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    // #[should_panic(expected = "index 5 is out of bounds")]
    #[should_panic]
    fn test_mut_oob_nonempty() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [5]);
    }

    #[test]
    // #[should_panic(expected = "index 5 is out of bounds")]
    #[should_panic]
    fn test_ref_oob_nonempty() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [5]);
    }

    #[test]
    // #[should_panic(expected = "index 0 is out of bounds")]
    #[should_panic]
    fn test_mut_oob_empty() {
        let mut v: Vec<i32> = vec![];
        index_many_mut(&mut v, [0]);
    }

    #[test]
    // #[should_panic(expected = "index 0 is out of bounds")]
    #[should_panic]
    fn test_ref_oob_empty() {
        let v: Vec<i32> = vec![];
        index_many(&v, [0]);
    }

    #[test]
    // #[should_panic(expected = "indices [3, 1, 9] are not unique or sorted in ascending order")]
    #[should_panic]
    fn test_mut_unsorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [3, 1, 4]);
    }

    #[test]
    // #[should_panic(expected = "indices [3, 1, 9] are not unique or sorted in ascending order")]
    #[should_panic]
    fn test_ref_unsorted() {
        let v = vec![1, 2, 3, 4, 5];
        index_many(&v, [3, 1, 4]);
    }
}
