#![cfg(feature = "std")]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(clippy::eq_op)]

// https://github.com/rust-lang/rust/blob/1.49.0/library/std/src/time/tests.rs
pub mod std_tests {
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
        assert!(Instant::none().is_none());
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
        assert_eq!(earlier.duration_since(now).into_inner(), None);
        assert_eq!(later.duration_since(now), Duration::new(1, 0));
        assert_eq!(now.duration_since(now), Duration::new(0, 0));
    }
}
