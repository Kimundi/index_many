use std::ops::Range;

use index_many::simple_result::{GetManyError, GetManyErrorKind};

pub type Elem = usize;
pub const LEN: usize = 3;

macro_rules! generate {
    ($(
        $id:literal: fn $name:ident($(
            $arg:ident: $argty:ty
        ),* $(,)?) -> $ret:ty $blk:block
    )*) => {
        $(
            pub unsafe fn $name($(
                $arg: $argty
            ),*) -> $ret $blk
        )*

        pub const FUNCTIONS: &[(i32, &'static str)] = &[
            $(
                ($id, stringify!($name)),
            )*
        ];
    }
}

generate! {
    1: fn option_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        index_many::simple::get_many_mut(slice, indices)
    }
    1: fn option_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        index_many::generic::get_many_mut(slice, indices)
    }
    1: fn option_usize_trait(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Option<[&mut Elem; LEN]> {
        index_many::slice_index::get_many_mut(slice, indices)
    }
    1: fn option_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> Option<[&mut [Elem]; LEN]> {
        index_many::slice_index::get_many_mut(slice, indices)
    }
    1: fn result_simple(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Result<[&mut Elem; LEN], GetManyError<LEN>> {
        index_many::simple_result::get_many_mut(slice, indices)
    }
    1: fn result_kind(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Result<[&mut Elem; LEN], GetManyErrorKind> {
        index_many::simple_result::get_many_mut(slice, indices).map_err(|e| e.kind())
    }
    1: fn result_option(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        index_many::simple_result::get_many_mut(slice, indices).ok()
    }


    2: fn checked_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::simple::index_many_mut(slice, indices)
    }
    2: fn checked_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::generic::index_many_mut(slice, indices)
    }
    2: fn checked_usize_trait(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::slice_index::index_many_mut(slice, indices)
    }
    2: fn checked_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> [&mut [Elem]; LEN] {
        index_many::slice_index::index_many_mut(slice, indices)
    }
    2: fn checked_presorted(
        slice: &mut [Elem],
        indices: index_many::generic::PresortedIndices<LEN>,
    ) -> [&mut Elem; LEN] {
        index_many::generic::index_many_mut(slice, indices)
    }

    2: fn checked_unsorted(
        slice: &mut [Elem],
        indices: index_many::generic::UnsortedIndices<LEN>,
    ) -> [&mut Elem; LEN] {
        index_many::generic::index_many_mut(slice, indices)
    }


    3: fn unchecked_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::simple::index_many_mut_unchecked(slice, indices)
    }
    3: fn unchecked_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::generic::index_many_mut_unchecked(slice, indices)
    }
    3: fn unchecked_usize_trait(
        slice: &mut [Elem], indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::slice_index::get_many_unchecked_mut(slice, indices)
    }
    3: fn unchecked_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> [&mut [Elem]; LEN] {
        index_many::slice_index::get_many_unchecked_mut(slice, indices)
    }


    4: fn unwrap_option_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::simple::get_many_mut(slice, indices).unwrap()
    }
    4: fn unwrap_option_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::generic::get_many_mut(slice, indices).unwrap()
    }
    4: fn unwrap_result(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        index_many::simple_result::get_many_mut(slice, indices).unwrap()
    }


}
