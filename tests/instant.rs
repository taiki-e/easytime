#![cfg(feature = "std")]

// https://github.com/rust-lang/rust/blob/master/src/libstd/time.rs

use easytime::{Duration, Instant};

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        if a != b {
            let (a, b) = if a > b { (a, b) } else { (b, a) };
            assert!(
                a - Duration::new(0, 1000) <= b,
                "{:?} is not almost equal to {:?}",
                a,
                b
            );
        }
    }};
}

#[test]
fn instant_monotonic() {
    let a = Instant::now();
    let b = Instant::now();
    assert!(b >= a);
}

#[test]
fn instant_elapsed() {
    let a = Instant::now();
    a.elapsed();
}

#[test]
fn instant_math() {
    let a = Instant::now();
    let b = Instant::now();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    let dur = b.duration_since(a);
    println!("dur: {:?}", dur);
    assert_almost_eq!(b - dur, a);
    assert_almost_eq!(a + dur, b);

    let second = Duration::new(1, 0);
    assert_almost_eq!(a - second + second, a);

    // checked_add_duration will not panic on overflow
    let mut maybe_t = Instant::now();
    let max_duration = Duration::from_secs(u64::max_value());
    // in case `Instant` can store `>= now + max_duration`.
    for _ in 0..2 {
        maybe_t = maybe_t + max_duration;
    }
    assert_eq!(maybe_t.into_inner(), None);

    // checked_add_duration calculates the right time and will work for another year
    let year = Duration::from_secs(60 * 60 * 24 * 365);
    assert_eq!(a + year, a + year);
}

#[test]
fn instant_math_is_associative() {
    let now = Instant::now();
    let offset = Duration::from_millis(5);
    // Changing the order of instant math shouldn't change the results,
    // especially when the expression reduces to X + identity.
    assert_eq!((now + offset) - now, (now - now) + offset);
}

#[test]
fn instant_duration_untrusted() {
    let a = Instant::now();
    let ret = (a - Duration::new(1, 0)).duration_since(a);
    assert_eq!(ret.into_inner(), None);
}
