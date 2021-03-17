#![feature(array_windows)]
#![feature(array_map)]
#![feature(maybe_uninit_extra)]

#[inline(never)]
fn check_indices_sorted_failed(indices: &[usize]) -> ! {
    panic!(
        "indices {:?} are not unique and sorted in ascending order",
        indices
    );
}

#[inline(never)]
fn check_indices_bound_failed(max_idx: usize) -> ! {
    panic!("at least index {} is out of bounds", max_idx);
}

#[inline]
fn check_indices(indices: &[usize], len: usize) {
    if let Some(&idx) = indices.get(0) {
        let mut max_idx = idx;
        let mut sorted = true;

        for &[a, b] in indices.array_windows() {
            if a >= b {
                sorted = false;
                break;
            }
            max_idx = b;
        }

        if max_idx >= len {
            check_indices_bound_failed(max_idx);
        }

        if !sorted {
            check_indices_sorted_failed(&indices);
        }
    }
}

pub fn index_many<'a, T, const N: usize>(slice: &'a [T], indices: [usize; N]) -> [&'a T; N] {
    check_indices(&indices, slice.len());
    unsafe {
        let mut arr: [*const T; N] = [std::ptr::null(); N];
        for (dst, idx) in arr.iter_mut().zip(indices.iter().copied()) {
            *dst = slice.get_unchecked(idx);
        }
        arr.map(|v| &*v)
    }
}

pub fn index_many_mut<'a, T, const N: usize>(
    slice: &'a mut [T],
    indices: [usize; N],
) -> [&'a mut T; N] {
    check_indices(&indices, slice.len());
    unsafe {
        let mut arr: [*mut T; N] = [std::ptr::null_mut(); N];
        for (dst, idx) in arr.iter_mut().zip(indices.iter().copied()) {
            *dst = slice.get_unchecked_mut(idx);
        }
        arr.map(|v| &mut *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
        *a += 10;
        *b += 100;
        *c += 1000;
        assert_eq!(v, vec![11, 2, 103, 4, 1005]);
    }

    #[test]
    fn test_empty() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [] = index_many_mut(&mut v, []);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_single_first() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [0]);
        *a += 10;
        assert_eq!(v, vec![11, 2, 3, 4, 5]);
    }

    #[test]
    fn test_single_last() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a] = index_many_mut(&mut v, [4]);
        *a += 10;
        assert_eq!(v, vec![1, 2, 3, 4, 15]);
    }

    #[test]
    #[should_panic]
    fn test_oob_nonempty() {
        let mut v = vec![1, 2, 3, 4, 5];
        index_many_mut(&mut v, [5]);
    }

    #[test]
    #[should_panic]
    fn test_oob_empty() {
        let mut v: Vec<i32> = vec![];
        index_many_mut(&mut v, [0]);
    }
}
