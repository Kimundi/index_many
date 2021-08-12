macro_rules! generate {
    (
        header {
            $(
                $item:item
            )*
        }
        $(
            $id:literal: fn $name:ident($(
                $arg:ident: $argty:ty
            ),* $(,)?) -> $ret:ty $blk:block
        )*
    ) => {
        use index_many::*;

        $(
            $item
        )*

        $(
            pub unsafe fn $name($(
                $arg: $argty
            ),*) -> $ret $blk
        )*

        pub const HEADER: &str = stringify!(
            $(
                $item
            )*
        );
        pub const FUNCTIONS: &[(i32, &'static str, &'static str, &'static str)] = &[
            $(
                ($id, stringify!($name), stringify!(
                    pub unsafe fn $name($(
                        $arg: $argty
                    ),*) -> $ret $blk
                ), stringify!($blk)),
            )*
        ];
    }
}

generate! {
    header {
        #[allow(unused_imports)]
        use std::ops::Range;

        #[allow(unused_imports)]
        use simple_result::{GetManyError, GetManyErrorKind};

        pub type Elem = usize;
        pub const LEN: usize = 3;
    }

    1: fn option_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        simple::get_many_mut(slice, indices)
    }
    1: fn option_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        generic::get_many_mut(slice, indices)
    }
    1: fn option_usize_trait(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Option<[&mut Elem; LEN]> {
        slice_index::get_many_mut(slice, indices)
    }
    1: fn option_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> Option<[&mut [Elem]; LEN]> {
        slice_index::get_many_mut(slice, indices)
    }
    1: fn option_unsorted(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<LEN>,
    ) -> Option<[&mut Elem; LEN]> {
        generic::get_many_mut(slice, indices)
    }
    1: fn result_simple(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Result<[&mut Elem; LEN], GetManyError<LEN>> {
        simple_result::get_many_mut(slice, indices)
    }
    1: fn result_kind(
        slice: &mut [Elem],
        indices: [usize; LEN],
    ) -> Result<[&mut Elem; LEN], GetManyErrorKind> {
        simple_result::get_many_mut(slice, indices).map_err(|e| e.kind())
    }
    1: fn result_option(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> Option<[&mut Elem; LEN]> {
        simple_result::get_many_mut(slice, indices).ok()
    }


    2: fn checked_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        simple::index_many_mut(slice, indices)
    }
    2: fn checked_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_usize_trait(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        slice_index::index_many_mut(slice, indices)
    }
    2: fn checked_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> [&mut [Elem]; LEN] {
        slice_index::index_many_mut(slice, indices)
    }
    2: fn checked_presorted(
        slice: &mut [Elem],
        indices: generic::PresortedIndices<LEN>,
    ) -> [&mut Elem; LEN] {
        generic::index_many_mut(slice, indices)
    }

    2: fn checked_unsorted(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<LEN>,
    ) -> [&mut Elem; LEN] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_unsorted_0(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<0>,
    ) -> [&mut Elem; 0] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_unsorted_1(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<1>,
    ) -> [&mut Elem; 1] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_unsorted_2(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<2>,
    ) -> [&mut Elem; 2] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_unsorted_3(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<3>,
    ) -> [&mut Elem; 3] {
        generic::index_many_mut(slice, indices)
    }
    2: fn checked_unsorted_4(
        slice: &mut [Elem],
        indices: generic::UnsortedIndices<4>,
    ) -> [&mut Elem; 4] {
        generic::index_many_mut(slice, indices)
    }

    3: fn unchecked_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        simple::index_many_mut_unchecked(slice, indices)
    }
    3: fn unchecked_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        generic::index_many_mut_unchecked(slice, indices)
    }
    3: fn unchecked_usize_trait(
        slice: &mut [Elem], indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        slice_index::get_many_unchecked_mut(slice, indices)
    }
    3: fn unchecked_range_trait(
        slice: &mut [Elem],
        indices: [Range<usize>; LEN],
    ) -> [&mut [Elem]; LEN] {
        slice_index::get_many_unchecked_mut(slice, indices)
    }


    4: fn unwrap_option_simple(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        simple::get_many_mut(slice, indices).unwrap()
    }
    4: fn unwrap_option_generic(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        generic::get_many_mut(slice, indices).unwrap()
    }
    4: fn unwrap_result(
        slice: &mut [Elem],
        indices: [usize; LEN]
    ) -> [&mut Elem; LEN] {
        simple_result::get_many_mut(slice, indices).unwrap()
    }


    5: fn checked_unsorted_specialized_0(
        slice: &mut [Elem],
        indices: generic::UnsortedSpecializedIndices<0>,
    ) -> [&mut Elem; 0] {
        generic::index_many_mut(slice, indices)
    }
    5: fn checked_unsorted_specialized_1(
        slice: &mut [Elem],
        indices: generic::UnsortedSpecializedIndices<1>,
    ) -> [&mut Elem; 1] {
        generic::index_many_mut(slice, indices)
    }
    5: fn checked_unsorted_specialized_2(
        slice: &mut [Elem],
        indices: generic::UnsortedSpecializedIndices<2>,
    ) -> [&mut Elem; 2] {
        generic::index_many_mut(slice, indices)
    }
    5: fn checked_unsorted_specialized_3(
        slice: &mut [Elem],
        indices: generic::UnsortedSpecializedIndices<3>,
    ) -> [&mut Elem; 3] {
        generic::index_many_mut(slice, indices)
    }
    5: fn checked_unsorted_specialized_4(
        slice: &mut [Elem],
        indices: generic::UnsortedSpecializedIndices<4>,
    ) -> [&mut Elem; 4] {
        generic::index_many_mut(slice, indices)
    }
}
