//! This module contains example functions with the generated assembly in
//! their docs.

use crate::*;

#[allow(unused_imports)]
use simple_result::{GetManyError, GetManyErrorKind};
#[allow(unused_imports)]
use std::ops::Range;
pub type Elem = usize;
pub const LEN: usize = 3;

/// Body: `{ simple::get_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::option_simple:
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_4
///  mov     r8, qword, ptr, [r9]
///  mov     rcx, qword, ptr, [r9, +, 8]
///  cmp     r8, rcx
///  jae     .LBB0_4
///  cmp     rcx, r10
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r8]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rax], r8
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rax], 0
///  ret
/// ```
pub unsafe fn option_simple(slice: &mut [Elem], indices: [usize; LEN]) -> Option<[&mut Elem; LEN]> {
    simple::get_many_mut(slice, indices)
}

/// Body: `{ generic::get_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::option_generic:
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_4
///  mov     r8, qword, ptr, [r9]
///  mov     rcx, qword, ptr, [r9, +, 8]
///  cmp     r8, rcx
///  jae     .LBB0_4
///  cmp     rcx, r10
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r8]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rax], r8
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rax], 0
///  ret
/// ```
pub unsafe fn option_generic(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Option<[&mut Elem; LEN]> {
    generic::get_many_mut(slice, indices)
}

/// Body: `{ slice_index::get_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::option_usize_trait:
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_4
///  mov     r8, qword, ptr, [r9]
///  mov     rcx, qword, ptr, [r9, +, 8]
///  cmp     r8, rcx
///  jae     .LBB0_4
///  cmp     rcx, r10
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r8]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rax], r8
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rax], 0
///  ret
/// ```
pub unsafe fn option_usize_trait(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Option<[&mut Elem; LEN]> {
    slice_index::get_many_mut(slice, indices)
}

/// Body: `{ slice_index::get_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::option_range_trait:
///  push    rsi
///  push    rdi
///  push    rbx
///  mov     rax, rcx
///  mov     rcx, qword, ptr, [r9, +, 40]
///  cmp     rcx, r8
///  ja      .LBB0_6
///  mov     r10, qword, ptr, [r9]
///  mov     rsi, qword, ptr, [r9, +, 8]
///  mov     r8, rsi
///  sub     r8, r10
///  jb      .LBB0_6
///  mov     r11, qword, ptr, [r9, +, 16]
///  cmp     rsi, r11
///  ja      .LBB0_6
///  mov     rbx, qword, ptr, [r9, +, 24]
///  mov     rdi, rbx
///  sub     rdi, r11
///  jb      .LBB0_6
///  mov     rsi, qword, ptr, [r9, +, 32]
///  cmp     rbx, rsi
///  ja      .LBB0_6
///  sub     rcx, rsi
///  jb      .LBB0_6
///  lea     rbx, [rdx, +, 8*r10]
///  lea     r9, [rdx, +, 8*r11]
///  lea     rdx, [rdx, +, 8*rsi]
///  mov     qword, ptr, [rax], rbx
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], r9
///  mov     qword, ptr, [rax, +, 24], rdi
///  mov     qword, ptr, [rax, +, 32], rdx
///  mov     qword, ptr, [rax, +, 40], rcx
///  jmp     .LBB0_8
/// .LBB0_6:
///  mov     qword, ptr, [rax], 0
/// .LBB0_8:
///  pop     rbx
///  pop     rdi
///  pop     rsi
///  ret
/// ```
pub unsafe fn option_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> Option<[&mut [Elem]; LEN]> {
    slice_index::get_many_mut(slice, indices)
}

/// Body: `{ simple_result::get_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::result_simple:
///  mov     r11, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     rcx, qword, ptr, [r9, +, 16]
///  cmp     rcx, r8
///  jae     .LBB0_4
///  cmp     r11, r10
///  jae     .LBB0_4
///  cmp     r10, rcx
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r11]
///  lea     r9, [rdx, +, 8*r10]
///  lea     rcx, [rdx, +, 8*rcx]
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], r9
///  mov     qword, ptr, [rax, +, 24], rcx
///  xor     ecx, ecx
///  mov     qword, ptr, [rax], rcx
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rax, +, 8], r11
///  mov     qword, ptr, [rax, +, 16], r10
///  mov     qword, ptr, [rax, +, 24], rcx
///  mov     qword, ptr, [rax, +, 32], r8
///  mov     ecx, 1
///  mov     qword, ptr, [rax], rcx
///  ret
/// ```
pub unsafe fn result_simple(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Result<[&mut Elem; LEN], GetManyError<LEN>> {
    simple_result::get_many_mut(slice, indices)
}

/// Body: `{ simple_result::get_many_mut(slice, indices).map_err(|e| e.kind()) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::result_kind:
///  sub     rsp, 72
///  mov     rax, rcx
///  mov     r10, qword, ptr, [r9]
///  mov     r11, qword, ptr, [r9, +, 8]
///  mov     rcx, qword, ptr, [r9, +, 16]
///  cmp     rcx, r8
///  jae     .LBB0_4
///  cmp     r10, r11
///  jae     .LBB0_4
///  cmp     r11, rcx
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r10]
///  lea     r9, [rdx, +, 8*r11]
///  lea     rcx, [rdx, +, 8*rcx]
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], r9
///  mov     qword, ptr, [rax, +, 24], rcx
///  xor     ecx, ecx
///  mov     qword, ptr, [rax], rcx
///  add     rsp, 72
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], r11
///  mov     qword, ptr, [rsp, +, 56], rcx
///  mov     qword, ptr, [rsp, +, 64], r8
///  cmp     r10, r11
///  jne     .LBB0_7
///  xor     edx, edx
/// .LBB0_6:
///  lea     r9, [rdx, +, 1]
///  mov     ecx, 2
///  jmp     .LBB0_10
/// .LBB0_7:
///  cmp     r10, r11
///  jbe     .LBB0_11
///  mov     r9d, 1
///  xor     edx, edx
/// .LBB0_9:
///  mov     ecx, 1
/// .LBB0_10:
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], rdx
///  mov     qword, ptr, [rax, +, 24], r9
///  mov     qword, ptr, [rax, +, 32], r8
///  mov     ecx, 1
///  mov     qword, ptr, [rax], rcx
///  add     rsp, 72
///  ret
/// .LBB0_11:
///  mov     edx, 1
///  cmp     r11, qword, ptr, [rsp, +, 56]
///  je      .LBB0_6
///  mov     edx, 1
///  mov     r9d, 2
///  ja      .LBB0_9
///  mov     r9, qword, ptr, [rsp, +, 40]
///  cmp     r9, r8
///  jae     .LBB0_17
///  mov     r9, qword, ptr, [rsp, +, 48]
///  cmp     r9, r8
///  jae     .LBB0_18
///  mov     r9, qword, ptr, [rsp, +, 56]
///  cmp     r9, r8
///  jb      .LBB0_19
///  mov     edx, 2
///  xor     ecx, ecx
///  jmp     .LBB0_10
/// .LBB0_17:
///  xor     edx, edx
/// .LBB0_18:
///  xor     ecx, ecx
///  jmp     .LBB0_10
/// .LBB0_19:
///  lea     rcx, [rip, +, __unnamed_1]
///  lea     r8, [rip, +, __unnamed_2]
///  mov     edx, 40
///  call    core::panicking::panic
///  ud2
/// ```
pub unsafe fn result_kind(
    slice: &mut [Elem],
    indices: [usize; LEN],
) -> Result<[&mut Elem; LEN], GetManyErrorKind> {
    simple_result::get_many_mut(slice, indices).map_err(|e| e.kind())
}

/// Body: `{ simple_result::get_many_mut(slice, indices).ok() }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::result_option:
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_4
///  mov     r8, qword, ptr, [r9]
///  mov     rcx, qword, ptr, [r9, +, 8]
///  cmp     r8, rcx
///  jae     .LBB0_4
///  cmp     rcx, r10
///  jae     .LBB0_4
///  lea     r8, [rdx, +, 8*r8]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rax], r8
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// .LBB0_4:
///  mov     qword, ptr, [rax], 0
///  ret
/// ```
pub unsafe fn result_option(slice: &mut [Elem], indices: [usize; LEN]) -> Option<[&mut Elem; LEN]> {
    simple_result::get_many_mut(slice, indices).ok()
}

/// Body: `{ simple::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_simple:
///  sub     rsp, 56
///  mov     r10, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r8
///  jae     .LBB0_3
///  cmp     r10, rax
///  jae     .LBB0_3
///  cmp     rax, r9
///  jae     .LBB0_3
///  lea     r8, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_3:
///  mov     qword, ptr, [rsp, +, 32], r10
///  mov     qword, ptr, [rsp, +, 40], rax
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::sorted_bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_simple(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    simple::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_generic:
///  sub     rsp, 56
///  mov     r10, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r8
///  jae     .LBB0_3
///  cmp     r10, rax
///  jae     .LBB0_3
///  cmp     rax, r9
///  jae     .LBB0_3
///  lea     r8, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_3:
///  mov     qword, ptr, [rsp, +, 32], r10
///  mov     qword, ptr, [rsp, +, 40], rax
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::sorted_bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_generic(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ slice_index::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_usize_trait:
///  sub     rsp, 56
///  mov     r10, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r8
///  jae     .LBB0_3
///  cmp     r10, rax
///  jae     .LBB0_3
///  cmp     rax, r9
///  jae     .LBB0_3
///  lea     r8, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_3:
///  mov     qword, ptr, [rsp, +, 32], r10
///  mov     qword, ptr, [rsp, +, 40], rax
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::sorted_bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_usize_trait(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    slice_index::index_many_mut(slice, indices)
}

/// Body: `{ slice_index::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_range_trait:
///  push    rsi
///  push    rdi
///  push    rbx
///  sub     rsp, 80
///  movups  xmm0, xmmword, ptr, [r9]
///  movups  xmm1, xmmword, ptr, [r9, +, 16]
///  movups  xmm2, xmmword, ptr, [r9, +, 32]
///  movaps  xmmword, ptr, [rsp, +, 32], xmm0
///  movaps  xmmword, ptr, [rsp, +, 48], xmm1
///  movaps  xmmword, ptr, [rsp, +, 64], xmm2
///  mov     r11, qword, ptr, [rsp, +, 72]
///  cmp     r11, r8
///  ja      .LBB0_6
///  mov     r10, qword, ptr, [rsp, +, 32]
///  mov     rsi, qword, ptr, [rsp, +, 40]
///  mov     r9, rsi
///  sub     r9, r10
///  jb      .LBB0_6
///  mov     rax, qword, ptr, [rsp, +, 48]
///  cmp     rsi, rax
///  ja      .LBB0_6
///  mov     rbx, qword, ptr, [rsp, +, 56]
///  mov     rsi, rbx
///  sub     rsi, rax
///  jb      .LBB0_6
///  mov     rdi, qword, ptr, [rsp, +, 64]
///  cmp     rbx, rdi
///  ja      .LBB0_6
///  sub     r11, rdi
///  jb      .LBB0_6
///  lea     rbx, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*rdi]
///  mov     qword, ptr, [rcx], rbx
///  mov     qword, ptr, [rcx, +, 8], r9
///  mov     qword, ptr, [rcx, +, 16], rax
///  mov     qword, ptr, [rcx, +, 24], rsi
///  mov     qword, ptr, [rcx, +, 32], rdx
///  mov     qword, ptr, [rcx, +, 40], r11
///  mov     rax, rcx
///  add     rsp, 80
///  pop     rbx
///  pop     rdi
///  pop     rsi
///  ret
/// .LBB0_6:
///  lea     r9, [rip, +, __unnamed_1]
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::slice_index::ranges::range_check_fail
///  ud2
/// ```
pub unsafe fn checked_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> [&mut [Elem]; LEN] {
    slice_index::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_presorted:
///  sub     rsp, 56
///  mov     r11, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     rax, qword, ptr, [r9, +, 16]
///  cmp     rax, r8
///  jae     .LBB0_1
///  lea     r8, [rdx, +, 8*r11]
///  lea     r9, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], r9
///  mov     qword, ptr, [rcx, +, 16], rax
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_1:
///  mov     qword, ptr, [rsp, +, 32], r11
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], rax
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_presorted(
    slice: &mut [Elem],
    indices: generic::PresortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted:
///  sub     rsp, 56
///  mov     rax, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r10
///  je      .LBB0_6
///  cmp     r9, rax
///  je      .LBB0_6
///  cmp     r9, r8
///  jae     .LBB0_6
///  cmp     r10, rax
///  je      .LBB0_6
///  cmp     rax, r8
///  jae     .LBB0_6
///  cmp     r10, r8
///  jae     .LBB0_6
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_6:
///  mov     qword, ptr, [rsp, +, 32], rax
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<LEN>,
) -> [&mut Elem; LEN] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_0:
/// ```
pub unsafe fn checked_unsorted_0(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<0>,
) -> [&mut Elem; 0] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_1:
///  sub     rsp, 40
///  cmp     r8, rdx
///  jae     .LBB0_1
///  lea     rax, [rcx, +, 8*r8]
///  add     rsp, 40
///  ret
/// .LBB0_1:
///  mov     rax, rdx
///  mov     qword, ptr, [rsp, +, 32], r8
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 1
///  mov     r8, rax
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_1(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<1>,
) -> [&mut Elem; 1] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_2:
///  push    rsi
///  sub     rsp, 48
///  mov     r10, rdx
///  cmp     r8, rdx
///  setb    dl
///  cmp     r9, r10
///  setb    al
///  and     al, dl
///  cmp     r9, r8
///  setne   r11b
///  lea     rdx, [rcx, +, 8*r9]
///  xor     esi, esi
///  test    r11b, al
///  cmove   rdx, rsi
///  lea     rax, [rcx, +, 8*r8]
///  cmove   rax, rsi
///  test    rax, rax
///  je      .LBB0_1
///  add     rsp, 48
///  pop     rsi
///  ret
/// .LBB0_1:
///  mov     qword, ptr, [rsp, +, 32], r8
///  mov     qword, ptr, [rsp, +, 40], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 2
///  mov     r8, r10
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_2(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<2>,
) -> [&mut Elem; 2] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_3:
///  sub     rsp, 56
///  mov     rax, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r10
///  je      .LBB0_6
///  cmp     r9, rax
///  je      .LBB0_6
///  cmp     r9, r8
///  jae     .LBB0_6
///  cmp     r10, rax
///  je      .LBB0_6
///  cmp     rax, r8
///  jae     .LBB0_6
///  cmp     r10, r8
///  jae     .LBB0_6
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_6:
///  mov     qword, ptr, [rsp, +, 32], rax
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_3(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<3>,
) -> [&mut Elem; 3] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_4:
///  sub     rsp, 72
///  mov     rax, qword, ptr, [r9]
///  mov     r11, qword, ptr, [r9, +, 8]
///  mov     r10, qword, ptr, [r9, +, 16]
///  mov     r9, qword, ptr, [r9, +, 24]
///  cmp     r9, r10
///  je      .LBB0_10
///  cmp     r9, r11
///  je      .LBB0_10
///  cmp     r9, rax
///  je      .LBB0_10
///  cmp     r9, r8
///  jae     .LBB0_10
///  cmp     r10, r11
///  je      .LBB0_10
///  cmp     r10, rax
///  je      .LBB0_10
///  cmp     r10, r8
///  jae     .LBB0_10
///  cmp     r11, rax
///  je      .LBB0_10
///  cmp     rax, r8
///  jae     .LBB0_10
///  cmp     r11, r8
///  jae     .LBB0_10
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r11]
///  lea     r10, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], r10
///  mov     qword, ptr, [rcx, +, 24], rdx
///  mov     rax, rcx
///  add     rsp, 72
///  ret
/// .LBB0_10:
///  mov     qword, ptr, [rsp, +, 40], rax
///  mov     qword, ptr, [rsp, +, 48], r11
///  mov     qword, ptr, [rsp, +, 56], r10
///  mov     qword, ptr, [rsp, +, 64], r9
///  lea     rcx, [rsp, +, 40]
///  mov     edx, 4
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_4(
    slice: &mut [Elem],
    indices: generic::UnsortedIndices<4>,
) -> [&mut Elem; 4] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ simple::index_many_mut_unchecked(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unchecked_simple:
///  mov     rcx, qword, ptr, [r9]
///  mov     r8, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     r8, [rdx, +, 8*r8]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rax], rcx
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// ```
pub unsafe fn unchecked_simple(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    simple::index_many_mut_unchecked(slice, indices)
}

/// Body: `{ generic::index_many_mut_unchecked(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unchecked_generic:
///  mov     rcx, qword, ptr, [r9]
///  mov     r8, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     r8, [rdx, +, 8*r8]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rax], rcx
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// ```
pub unsafe fn unchecked_generic(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    generic::index_many_mut_unchecked(slice, indices)
}

/// Body: `{ slice_index::get_many_unchecked_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unchecked_usize_trait:
///  mov     rcx, qword, ptr, [r9]
///  mov     r8, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  lea     rcx, [rdx, +, 8*rcx]
///  lea     r8, [rdx, +, 8*r8]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rax], rcx
///  mov     qword, ptr, [rax, +, 8], r8
///  mov     qword, ptr, [rax, +, 16], rdx
///  ret
/// ```
pub unsafe fn unchecked_usize_trait(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    slice_index::get_many_unchecked_mut(slice, indices)
}

/// Body: `{ slice_index::get_many_unchecked_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unchecked_range_trait:
///  push    rsi
///  push    rdi
///  mov     rax, rcx
///  mov     r8, qword, ptr, [r9]
///  mov     rcx, qword, ptr, [r9, +, 8]
///  mov     r10, qword, ptr, [r9, +, 16]
///  mov     r11, qword, ptr, [r9, +, 24]
///  mov     rdi, qword, ptr, [r9, +, 32]
///  mov     rsi, qword, ptr, [r9, +, 40]
///  lea     r9, [rdx, +, 8*r8]
///  sub     rcx, r8
///  lea     r8, [rdx, +, 8*r10]
///  sub     r11, r10
///  lea     rdx, [rdx, +, 8*rdi]
///  sub     rsi, rdi
///  mov     qword, ptr, [rax], r9
///  mov     qword, ptr, [rax, +, 8], rcx
///  mov     qword, ptr, [rax, +, 16], r8
///  mov     qword, ptr, [rax, +, 24], r11
///  mov     qword, ptr, [rax, +, 32], rdx
///  mov     qword, ptr, [rax, +, 40], rsi
///  pop     rdi
///  pop     rsi
///  ret
/// ```
pub unsafe fn unchecked_range_trait(
    slice: &mut [Elem],
    indices: [Range<usize>; LEN],
) -> [&mut [Elem]; LEN] {
    slice_index::get_many_unchecked_mut(slice, indices)
}

/// Body: `{ simple::get_many_mut(slice, indices).unwrap() }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unwrap_option_simple:
///  sub     rsp, 40
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_3
///  mov     r8, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  cmp     r8, rax
///  jae     .LBB0_3
///  cmp     rax, r10
///  jae     .LBB0_3
///  lea     r8, [rdx, +, 8*r8]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 40
///  ret
/// .LBB0_3:
///  lea     rcx, [rip, +, __unnamed_1]
///  lea     r8, [rip, +, __unnamed_2]
///  mov     edx, 43
///  call    core::panicking::panic
///  ud2
/// ```
pub unsafe fn unwrap_option_simple(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    simple::get_many_mut(slice, indices).unwrap()
}

/// Body: `{ generic::get_many_mut(slice, indices).unwrap() }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unwrap_option_generic:
///  sub     rsp, 40
///  mov     r10, qword, ptr, [r9, +, 16]
///  cmp     r10, r8
///  jae     .LBB0_3
///  mov     r8, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  cmp     r8, rax
///  jae     .LBB0_3
///  cmp     rax, r10
///  jae     .LBB0_3
///  lea     r8, [rdx, +, 8*r8]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r10]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 40
///  ret
/// .LBB0_3:
///  lea     rcx, [rip, +, __unnamed_1]
///  lea     r8, [rip, +, __unnamed_2]
///  mov     edx, 43
///  call    core::panicking::panic
///  ud2
/// ```
pub unsafe fn unwrap_option_generic(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    generic::get_many_mut(slice, indices).unwrap()
}

/// Body: `{ simple_result::get_many_mut(slice, indices).unwrap() }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::unwrap_result:
///  sub     rsp, 72
///  mov     r10, qword, ptr, [r9]
///  mov     rax, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r8
///  jae     .LBB4_3
///  cmp     r10, rax
///  jae     .LBB4_3
///  cmp     rax, r9
///  jae     .LBB4_3
///  lea     r8, [rdx, +, 8*r10]
///  lea     rax, [rdx, +, 8*rax]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], r8
///  mov     qword, ptr, [rcx, +, 8], rax
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 72
///  ret
/// .LBB4_3:
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], rax
///  mov     qword, ptr, [rsp, +, 56], r9
///  mov     qword, ptr, [rsp, +, 64], r8
///  lea     rax, [rip, +, __unnamed_6]
///  mov     qword, ptr, [rsp, +, 32], rax
///  lea     rcx, [rip, +, __unnamed_7]
///  lea     r9, [rip, +, __unnamed_8]
///  lea     r8, [rsp, +, 40]
///  mov     edx, 43
///  call    core::result::unwrap_failed
///  ud2
/// ```
pub unsafe fn unwrap_result(slice: &mut [Elem], indices: [usize; LEN]) -> [&mut Elem; LEN] {
    simple_result::get_many_mut(slice, indices).unwrap()
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_specialized_0:
/// ```
pub unsafe fn checked_unsorted_specialized_0(
    slice: &mut [Elem],
    indices: generic::UnsortedSpecializedIndices<0>,
) -> [&mut Elem; 0] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_specialized_1:
///  sub     rsp, 40
///  cmp     r8, rdx
///  jae     .LBB0_1
///  lea     rax, [rcx, +, 8*r8]
///  add     rsp, 40
///  ret
/// .LBB0_1:
///  mov     rax, rdx
///  mov     qword, ptr, [rsp, +, 32], r8
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 1
///  mov     r8, rax
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_specialized_1(
    slice: &mut [Elem],
    indices: generic::UnsortedSpecializedIndices<1>,
) -> [&mut Elem; 1] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_specialized_2:
///  push    rsi
///  sub     rsp, 48
///  mov     r10, rdx
///  cmp     r8, r9
///  setne   dl
///  cmp     r8, r10
///  setb    al
///  and     al, dl
///  cmp     r9, r10
///  setb    r11b
///  lea     rdx, [rcx, +, 8*r9]
///  xor     esi, esi
///  test    r11b, al
///  cmove   rdx, rsi
///  lea     rax, [rcx, +, 8*r8]
///  cmove   rax, rsi
///  test    rax, rax
///  je      .LBB0_1
///  add     rsp, 48
///  pop     rsi
///  ret
/// .LBB0_1:
///  mov     qword, ptr, [rsp, +, 32], r8
///  mov     qword, ptr, [rsp, +, 40], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 2
///  mov     r8, r10
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_specialized_2(
    slice: &mut [Elem],
    indices: generic::UnsortedSpecializedIndices<2>,
) -> [&mut Elem; 2] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_specialized_3:
///  sub     rsp, 56
///  mov     rax, qword, ptr, [r9]
///  mov     r10, qword, ptr, [r9, +, 8]
///  mov     r9, qword, ptr, [r9, +, 16]
///  cmp     r9, r10
///  je      .LBB0_6
///  cmp     r9, rax
///  je      .LBB0_6
///  cmp     r9, r8
///  jae     .LBB0_6
///  cmp     r10, rax
///  je      .LBB0_6
///  cmp     rax, r8
///  jae     .LBB0_6
///  cmp     r10, r8
///  jae     .LBB0_6
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], rdx
///  mov     rax, rcx
///  add     rsp, 56
///  ret
/// .LBB0_6:
///  mov     qword, ptr, [rsp, +, 32], rax
///  mov     qword, ptr, [rsp, +, 40], r10
///  mov     qword, ptr, [rsp, +, 48], r9
///  lea     rcx, [rsp, +, 32]
///  mov     edx, 3
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_specialized_3(
    slice: &mut [Elem],
    indices: generic::UnsortedSpecializedIndices<3>,
) -> [&mut Elem; 3] {
    generic::index_many_mut(slice, indices)
}

/// Body: `{ generic::index_many_mut(slice, indices) }`
///
/// # Assembly (x86_64)
/// ```x86asm
/// codegen_crate::checked_unsorted_specialized_4:
///  sub     rsp, 72
///  mov     rax, qword, ptr, [r9]
///  mov     r11, qword, ptr, [r9, +, 8]
///  mov     r10, qword, ptr, [r9, +, 16]
///  mov     r9, qword, ptr, [r9, +, 24]
///  cmp     r9, r10
///  je      .LBB0_10
///  cmp     r9, r11
///  je      .LBB0_10
///  cmp     r9, rax
///  je      .LBB0_10
///  cmp     r9, r8
///  jae     .LBB0_10
///  cmp     r10, r11
///  je      .LBB0_10
///  cmp     r10, rax
///  je      .LBB0_10
///  cmp     r10, r8
///  jae     .LBB0_10
///  cmp     r11, rax
///  je      .LBB0_10
///  cmp     rax, r8
///  jae     .LBB0_10
///  cmp     r11, r8
///  jae     .LBB0_10
///  lea     rax, [rdx, +, 8*rax]
///  lea     r8, [rdx, +, 8*r11]
///  lea     r10, [rdx, +, 8*r10]
///  lea     rdx, [rdx, +, 8*r9]
///  mov     qword, ptr, [rcx], rax
///  mov     qword, ptr, [rcx, +, 8], r8
///  mov     qword, ptr, [rcx, +, 16], r10
///  mov     qword, ptr, [rcx, +, 24], rdx
///  mov     rax, rcx
///  add     rsp, 72
///  ret
/// .LBB0_10:
///  mov     qword, ptr, [rsp, +, 40], rax
///  mov     qword, ptr, [rsp, +, 48], r11
///  mov     qword, ptr, [rsp, +, 56], r10
///  mov     qword, ptr, [rsp, +, 64], r9
///  lea     rcx, [rsp, +, 40]
///  mov     edx, 4
///  call    index_many::bound_check_failed
///  ud2
/// ```
pub unsafe fn checked_unsorted_specialized_4(
    slice: &mut [Elem],
    indices: generic::UnsortedSpecializedIndices<4>,
) -> [&mut Elem; 4] {
    generic::index_many_mut(slice, indices)
}
