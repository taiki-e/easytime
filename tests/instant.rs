// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "std")]

// https://github.com/rust-lang/rust/blob/1.63.0/library/std/src/time/tests.rs
mod std_tests {
    #![allow(clippy::eq_op)]

    use easytime::{Duration, Instant};

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = ($a, $b);
            if a != b {
                let (a, b) = if a > b { (a, b) } else { (b, a) };
                assert!(
                    a - Duration::from_micros(1) <= b,
                    "{:?} is not almost equal to {:?}",
                    a,
                    b
                );
            }
        }};
    }

    #[test]
    fn none() {
        assert!(Instant::NONE.is_none());
    }

    #[test]
    fn instant_monotonic() {
        let a = Instant::now();
        loop {
            let b = Instant::now();
            assert!(b >= a);
            if b > a {
                break;
            }
        }
    }

    #[test]
    fn instant_monotonic_concurrent() -> std::thread::Result<()> {
        let threads: Vec<_> = (0..8)
            .map(|_| {
                std::thread::spawn(|| {
                    let mut old = Instant::now();
                    let count = if cfg!(miri) { 1_000 } else { 5_000_000 };
                    for _ in 0..count {
                        let new = Instant::now();
                        assert!(new >= old);
                        old = new;
                    }
                })
            })
            .collect();
        for t in threads {
            t.join()?;
        }
        Ok(())
    }

    #[test]
    fn instant_elapsed() {
        let a = Instant::now();
        let _ = a.elapsed();
    }

    #[test]
    fn instant_math() {
        let a = Instant::now();
        let b = Instant::now();
        println!("a: {a:?}");
        println!("b: {b:?}");
        let dur = b.duration_since(a);
        println!("dur: {dur:?}");
        assert_almost_eq!(b - dur, a);
        assert_almost_eq!(a + dur, b);

        let second = Duration::new(1, 0);
        assert_almost_eq!(a - second + second, a);

        // checked_add_duration will not panic on overflow
        let mut maybe_t = Instant::now();
        let max_duration = Duration::from_secs(u64::MAX);
        // in case `Instant` can store `>= now + max_duration`.
        for _ in 0..2 {
            maybe_t += max_duration;
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
        let now = Instant::now();
        let earlier = now - Duration::new(1, 0);
        let later = now + Duration::new(1, 0);
        // https://github.com/rust-lang/rust/commit/9d8ef1160747a4d033f21803770641f2deb32b25
        assert_eq!(earlier.duration_since(now), Duration::ZERO);
        assert_eq!(later.duration_since(now), Duration::new(1, 0));
        assert_eq!(now.duration_since(now), Duration::ZERO);
    }
}
