// SPDX-License-Identifier: Apache-2.0 OR MIT

#[inline]
pub(crate) fn pair_and_then<A, B, C, F>(x: Option<A>, y: Option<B>, f: F) -> Option<C>
where
    F: FnOnce(A, B) -> Option<C>,
{
    match (x, y) {
        (Some(x), Some(y)) => f(x, y),
        _ => None,
    }
}
