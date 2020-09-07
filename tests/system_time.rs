#![cfg(feature = "std")]
#![warn(rust_2018_idioms)]

// https://github.com/rust-lang/rust/blob/master/src/libstd/time.rs

use easytime::{Duration, SystemTime};

macro_rules! assert_almost_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        if a != b {
            let (a, b) = if a > b { (a, b) } else { (b, a) };
            assert!(a - Duration::new(0, 1000) <= b, "{:?} is not almost equal to {:?}", a, b);
        }
    }};
}

#[test]
fn system_time_math() {
    let a = SystemTime::now();
    let _b = SystemTime::now();

    let second = Duration::new(1, 0);
    assert_almost_eq!(a.duration_since(a - second), second);
    assert_eq!(a.duration_since(a + second).into_inner(), None);

    assert_almost_eq!(a - second + second, a);

    // A difference of 80 and 800 years cannot fit inside a 32-bit time_t
    if !(cfg!(unix) && std::mem::size_of::<libc::time_t>() <= 4) {
        let eighty_years = second * 60 * 60 * 24 * 365 * 80;
        assert_almost_eq!(a - eighty_years + eighty_years, a);
        assert_almost_eq!(a - (eighty_years * 10) + (eighty_years * 10), a);
    }

    let one_second_from_epoch = SystemTime::UNIX_EPOCH + Duration::new(1, 0);
    let one_second_from_epoch2 =
        SystemTime::UNIX_EPOCH + Duration::new(0, 500_000_000) + Duration::new(0, 500_000_000);
    assert_eq!(one_second_from_epoch, one_second_from_epoch2);

    // checked_add_duration will not panic on overflow
    let mut maybe_t = SystemTime::UNIX_EPOCH;
    let max_duration = Duration::from_secs(u64::max_value());
    // in case `SystemTime` can store `>= UNIX_EPOCH + max_duration`.
    for _ in 0..2 {
        maybe_t += max_duration;
    }
    assert_eq!(maybe_t.into_inner(), None);

    // checked_add_duration calculates the right time and will work for another year
    let year = Duration::from_secs(60 * 60 * 24 * 365);
    assert_eq!(a + year, a + year);
}

#[test]
fn system_time_elapsed() {
    let a = SystemTime::now();
    let _ = a.elapsed();
}

#[test]
fn since_epoch() {
    let ts = SystemTime::now();
    let a = ts.duration_since(SystemTime::UNIX_EPOCH);
    let b = ts.duration_since(SystemTime::UNIX_EPOCH - Duration::new(1, 0));
    assert!(b > a);
    assert_eq!(b - a, Duration::new(1, 0));

    let thirty_years = Duration::new(1, 0) * 60 * 60 * 24 * 365 * 30;

    // Right now for CI this test is run in an emulator, and apparently the
    // aarch64 emulator's sense of time is that we're still living in the
    // 70s.
    //
    // Otherwise let's assume that we're all running computers later than
    // 2000.
    if !cfg!(target_arch = "aarch64") {
        assert!(a > thirty_years);
    }

    // let's assume that we're all running computers earlier than 2090.
    // Should give us ~70 years to fix this!
    let hundred_twenty_years = thirty_years * 4;
    assert!(a < hundred_twenty_years);
}
