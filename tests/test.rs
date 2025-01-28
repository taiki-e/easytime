// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::mem;

use easytime::*;

// Test the size of public types. This is not intended to keep a specific size and
// is intended to be used only as a help in optimization.
#[test]
#[cfg_attr(any(not(target_pointer_width = "64"), miri), ignore)] // We set -Z randomize-layout for Miri.
fn size() {
    assert_eq!(mem::size_of::<Duration>(), 16);
    assert_eq!(mem::size_of::<TryFromTimeError>(), 0);
    assert_eq!(mem::size_of::<Instant>(), 16);
}
