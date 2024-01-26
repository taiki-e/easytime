// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by easytime-internal-codegen
// (gen_assert_impl function at tools/codegen/src/main.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(
    dead_code,
    unused_macros,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]
fn assert_send<T: ?Sized + Send>() {}
fn assert_sync<T: ?Sized + Sync>() {}
fn assert_unpin<T: ?Sized + Unpin>() {}
fn assert_unwind_safe<T: ?Sized + std::panic::UnwindSafe>() {}
fn assert_ref_unwind_safe<T: ?Sized + std::panic::RefUnwindSafe>() {}
const _: fn() = || {
    assert_send::<crate::duration::Duration>();
    assert_sync::<crate::duration::Duration>();
    assert_unpin::<crate::duration::Duration>();
    assert_unwind_safe::<crate::duration::Duration>();
    assert_ref_unwind_safe::<crate::duration::Duration>();
    assert_send::<crate::error::TryFromTimeError>();
    assert_sync::<crate::error::TryFromTimeError>();
    assert_unpin::<crate::error::TryFromTimeError>();
    assert_unwind_safe::<crate::error::TryFromTimeError>();
    assert_ref_unwind_safe::<crate::error::TryFromTimeError>();
    assert_send::<crate::instant::Instant>();
    assert_sync::<crate::instant::Instant>();
    assert_unpin::<crate::instant::Instant>();
    assert_unwind_safe::<crate::instant::Instant>();
    assert_ref_unwind_safe::<crate::instant::Instant>();
};
