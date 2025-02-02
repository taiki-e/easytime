// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::time;

use easytime::Duration;

#[test]
fn none() {
    assert!(Duration::NONE.is_none());
}

#[test]
fn cmp() {
    assert!(Duration::from_secs(1) == Duration::from_secs(1));
    assert!(Duration::from_secs(1) != Duration::from_secs(0));
    assert!(Duration::from_secs(1) == time::Duration::from_secs(1));
    assert!(Duration::from_secs(1) != time::Duration::from_secs(0));
    assert!(time::Duration::from_secs(1) == Duration::from_secs(1));
    assert!(time::Duration::from_secs(1) != Duration::from_secs(0));
    assert!(Duration::from(None) != Duration::from_secs(1));
    assert!(Duration::from_secs(1) != Duration::from(None));
    assert!(Duration::from(None) != time::Duration::from_secs(1));
    assert!(time::Duration::from_secs(1) != Duration::from(None));
    assert!(Duration::from(None) == Duration::from(None)); // TODO: Is this not good?

    assert!(Duration::from_secs(1) > Duration::from_secs(0));
    assert!(Duration::from_secs(0) < Duration::from_secs(1));
    assert!(Duration::from_secs(1) >= Duration::from_secs(0));
    assert!(Duration::from_secs(0) <= Duration::from_secs(1));
    assert!(Duration::from_secs(1) > time::Duration::from_secs(0));
    assert!(Duration::from_secs(0) < time::Duration::from_secs(1));
    assert!(Duration::from_secs(1) >= time::Duration::from_secs(0));
    assert!(Duration::from_secs(0) <= time::Duration::from_secs(1));
    assert!(time::Duration::from_secs(1) > Duration::from_secs(0));
    assert!(time::Duration::from_secs(0) < Duration::from_secs(1));
    assert!(time::Duration::from_secs(1) >= Duration::from_secs(0));
    assert!(time::Duration::from_secs(0) <= Duration::from_secs(1));
}

// https://github.com/rust-lang/rust/blob/1.63.0/library/core/tests/time.rs
mod core_tests {
    #![allow(
        clippy::assertions_on_constants,
        clippy::items_after_statements,
        clippy::zero_prefixed_literal
    )]

    use std::time;

    use easytime::Duration;

    #[test]
    fn creation() {
        assert!(Duration::from_secs(1) != Duration::from_secs(0));
        assert_eq!(Duration::from_secs(1) + Duration::from_secs(2), Duration::from_secs(3));
        assert_eq!(
            Duration::from_millis(10) + Duration::from_secs(4),
            Duration::new(4, 10 * 1_000_000)
        );
        assert_eq!(Duration::from_millis(4000), Duration::new(4, 0));
    }

    #[test]
    fn secs() {
        assert_eq!(Duration::new(0, 0).as_secs(), Some(0));
        assert_eq!(Duration::new(0, 500_000_005).as_secs(), Some(0));
        assert_eq!(Duration::new(0, 1_050_000_001).as_secs(), Some(1));
        assert_eq!(Duration::from_secs(1).as_secs(), Some(1));
        assert_eq!(Duration::from_millis(999).as_secs(), Some(0));
        assert_eq!(Duration::from_millis(1001).as_secs(), Some(1));
        assert_eq!(Duration::from_micros(999_999).as_secs(), Some(0));
        assert_eq!(Duration::from_micros(1_000_001).as_secs(), Some(1));
        assert_eq!(Duration::from_nanos(999_999_999).as_secs(), Some(0));
        assert_eq!(Duration::from_nanos(1_000_000_001).as_secs(), Some(1));
    }

    #[test]
    fn millis() {
        assert_eq!(Duration::new(0, 0).subsec_millis(), Some(0));
        assert_eq!(Duration::new(0, 500_000_005).subsec_millis(), Some(500));
        assert_eq!(Duration::new(0, 1_050_000_001).subsec_millis(), Some(50));
        assert_eq!(Duration::from_secs(1).subsec_millis(), Some(0));
        assert_eq!(Duration::from_millis(999).subsec_millis(), Some(999));
        assert_eq!(Duration::from_millis(1001).subsec_millis(), Some(1));
        assert_eq!(Duration::from_micros(999_999).subsec_millis(), Some(999));
        assert_eq!(Duration::from_micros(1_001_000).subsec_millis(), Some(1));
        assert_eq!(Duration::from_nanos(999_999_999).subsec_millis(), Some(999));
        assert_eq!(Duration::from_nanos(1_001_000_000).subsec_millis(), Some(1));
    }

    #[test]
    fn micros() {
        assert_eq!(Duration::new(0, 0).subsec_micros(), Some(0));
        assert_eq!(Duration::new(0, 500_000_005).subsec_micros(), Some(500_000));
        assert_eq!(Duration::new(0, 1_050_000_001).subsec_micros(), Some(50_000));
        assert_eq!(Duration::from_secs(1).subsec_micros(), Some(0));
        assert_eq!(Duration::from_millis(999).subsec_micros(), Some(999_000));
        assert_eq!(Duration::from_millis(1001).subsec_micros(), Some(1_000));
        assert_eq!(Duration::from_micros(999_999).subsec_micros(), Some(999_999));
        assert_eq!(Duration::from_micros(1_000_001).subsec_micros(), Some(1));
        assert_eq!(Duration::from_nanos(999_999_999).subsec_micros(), Some(999_999));
        assert_eq!(Duration::from_nanos(1_000_001_000).subsec_micros(), Some(1));
    }

    #[test]
    fn nanos() {
        assert_eq!(Duration::new(0, 0).subsec_nanos(), Some(0));
        assert_eq!(Duration::new(0, 5).subsec_nanos(), Some(5));
        assert_eq!(Duration::new(0, 1_000_000_001).subsec_nanos(), Some(1));
        assert_eq!(Duration::from_secs(1).subsec_nanos(), Some(0));
        assert_eq!(Duration::from_millis(999).subsec_nanos(), Some(999_000_000));
        assert_eq!(Duration::from_millis(1001).subsec_nanos(), Some(1_000_000));
        assert_eq!(Duration::from_micros(999_999).subsec_nanos(), Some(999_999_000));
        assert_eq!(Duration::from_micros(1_000_001).subsec_nanos(), Some(1000));
        assert_eq!(Duration::from_nanos(999_999_999).subsec_nanos(), Some(999_999_999));
        assert_eq!(Duration::from_nanos(1_000_000_001).subsec_nanos(), Some(1));
    }

    #[test]
    fn add() {
        assert_eq!(Duration::new(0, 0) + Duration::new(0, 1), Duration::new(0, 1));
        assert_eq!(
            Duration::new(0, 500_000_000) + Duration::new(0, 500_000_001),
            Duration::new(1, 1)
        );
    }

    #[test]
    fn checked_add() {
        assert_eq!(
            (Duration::new(0, 0) + Duration::new(0, 1)).into_inner(),
            Some(time::Duration::new(0, 1))
        );
        assert_eq!(
            (Duration::new(0, 500_000_000) + Duration::new(0, 500_000_001)).into_inner(),
            Some(time::Duration::new(1, 1))
        );
        assert_eq!((Duration::new(1, 0) + Duration::new(u64::MAX, 0)).into_inner(), None);
    }

    #[test]
    fn sub() {
        assert_eq!(Duration::new(0, 1) - Duration::new(0, 0), Duration::new(0, 1));
        assert_eq!(
            Duration::new(0, 500_000_001) - Duration::new(0, 500_000_000),
            Duration::new(0, 1)
        );
        assert_eq!(Duration::new(1, 0) - Duration::new(0, 1), Duration::new(0, 999_999_999));
    }

    #[test]
    fn checked_sub() {
        let zero = Duration::new(0, 0);
        let one_nano = Duration::new(0, 1);
        let one_sec = Duration::new(1, 0);
        assert_eq!((one_nano - zero).into_inner(), Some(time::Duration::new(0, 1)));
        assert_eq!((one_sec - one_nano).into_inner(), Some(time::Duration::new(0, 999_999_999)));
        assert_eq!((zero - one_nano).into_inner(), None);
        assert_eq!((zero - one_sec).into_inner(), None);
    }

    #[test]
    fn mul() {
        assert_eq!(Duration::new(0, 1) * 2, Duration::new(0, 2));
        assert_eq!(Duration::new(1, 1) * 3, Duration::new(3, 3));
        assert_eq!(Duration::new(0, 500_000_001) * 4, Duration::new(2, 4));
        assert_eq!(Duration::new(0, 500_000_001) * 4000, Duration::new(2000, 4000));
    }

    #[test]
    fn checked_mul() {
        assert_eq!((Duration::new(0, 1) * 2).into_inner(), Some(time::Duration::new(0, 2)));
        assert_eq!((Duration::new(1, 1) * 3).into_inner(), Some(time::Duration::new(3, 3)));
        assert_eq!(
            (Duration::new(0, 500_000_001) * 4).into_inner(),
            Some(time::Duration::new(2, 4))
        );
        assert_eq!(
            (Duration::new(0, 500_000_001) * 4000).into_inner(),
            Some(time::Duration::new(2000, 4000))
        );
        assert_eq!((Duration::new(u64::MAX - 1, 0) * 2).into_inner(), None);
    }

    #[test]
    fn div() {
        assert_eq!(Duration::new(0, 1) / 2, Duration::new(0, 0));
        assert_eq!(Duration::new(1, 1) / 3, Duration::new(0, 333_333_333));
        assert_eq!(Duration::new(99, 999_999_000) / 100, Duration::new(0, 999_999_990));
    }

    #[test]
    fn checked_div() {
        assert_eq!((Duration::new(2, 0) / 2).into_inner(), Some(time::Duration::new(1, 0)));
        assert_eq!(
            (Duration::new(1, 0) / 2).into_inner(),
            Some(time::Duration::new(0, 500_000_000))
        );
        assert_eq!((Duration::new(2, 0) / 0).into_inner(), None);
    }

    /* TODO duration_sum
    #[test]
    fn correct_sum() {
        let durations = [
            Duration::new(1, 999_999_999),
            Duration::new(2, 999_999_999),
            Duration::new(0, 999_999_999),
            Duration::new(0, 999_999_999),
            Duration::new(0, 999_999_999),
            Duration::new(5, 0),
        ];
        let sum = durations.iter().sum::<Duration>();
        assert_eq!(sum, Duration::new(1 + 2 + 5 + 4, 1_000_000_000 - 5));
    }
    */

    // duration_debug_impl https://github.com/rust-lang/rust/pull/50364

    #[test]
    fn debug_formatting_extreme_values() {
        assert_eq!(
            format!("{:?}", Duration::new(18_446_744_073_709_551_615, 123_456_789)),
            "Some(18446744073709551615.123456789s)"
        );
    }

    #[test]
    fn debug_formatting_secs() {
        assert_eq!(format!("{:?}", Duration::new(7, 000_000_000)), "Some(7s)");
        assert_eq!(format!("{:?}", Duration::new(7, 100_000_000)), "Some(7.1s)");
        assert_eq!(format!("{:?}", Duration::new(7, 000_010_000)), "Some(7.00001s)");
        assert_eq!(format!("{:?}", Duration::new(7, 000_000_001)), "Some(7.000000001s)");
        assert_eq!(format!("{:?}", Duration::new(7, 123_456_789)), "Some(7.123456789s)");

        assert_eq!(format!("{:?}", Duration::new(88, 000_000_000)), "Some(88s)");
        assert_eq!(format!("{:?}", Duration::new(88, 100_000_000)), "Some(88.1s)");
        assert_eq!(format!("{:?}", Duration::new(88, 000_010_000)), "Some(88.00001s)");
        assert_eq!(format!("{:?}", Duration::new(88, 000_000_001)), "Some(88.000000001s)");
        assert_eq!(format!("{:?}", Duration::new(88, 123_456_789)), "Some(88.123456789s)");

        assert_eq!(format!("{:?}", Duration::new(999, 000_000_000)), "Some(999s)");
        assert_eq!(format!("{:?}", Duration::new(999, 100_000_000)), "Some(999.1s)");
        assert_eq!(format!("{:?}", Duration::new(999, 000_010_000)), "Some(999.00001s)");
        assert_eq!(format!("{:?}", Duration::new(999, 000_000_001)), "Some(999.000000001s)");
        assert_eq!(format!("{:?}", Duration::new(999, 123_456_789)), "Some(999.123456789s)");
    }

    #[test]
    fn debug_formatting_millis() {
        assert_eq!(format!("{:?}", Duration::new(0, 7_000_000)), "Some(7ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_100_000)), "Some(7.1ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_000_001)), "Some(7.000001ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_123_456)), "Some(7.123456ms)");

        assert_eq!(format!("{:?}", Duration::new(0, 88_000_000)), "Some(88ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_100_000)), "Some(88.1ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_000_001)), "Some(88.000001ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_123_456)), "Some(88.123456ms)");

        assert_eq!(format!("{:?}", Duration::new(0, 999_000_000)), "Some(999ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_100_000)), "Some(999.1ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_000_001)), "Some(999.000001ms)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_123_456)), "Some(999.123456ms)");
    }

    #[test]
    fn debug_formatting_micros() {
        assert_eq!(format!("{:?}", Duration::new(0, 7_000)), "Some(7µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_100)), "Some(7.1µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_001)), "Some(7.001µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 7_123)), "Some(7.123µs)");

        assert_eq!(format!("{:?}", Duration::new(0, 88_000)), "Some(88µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_100)), "Some(88.1µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_001)), "Some(88.001µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 88_123)), "Some(88.123µs)");

        assert_eq!(format!("{:?}", Duration::new(0, 999_000)), "Some(999µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_100)), "Some(999.1µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_001)), "Some(999.001µs)");
        assert_eq!(format!("{:?}", Duration::new(0, 999_123)), "Some(999.123µs)");
    }

    #[test]
    fn debug_formatting_nanos() {
        assert_eq!(format!("{:?}", Duration::new(0, 0)), "Some(0ns)");
        assert_eq!(format!("{:?}", Duration::new(0, 1)), "Some(1ns)");
        assert_eq!(format!("{:?}", Duration::new(0, 88)), "Some(88ns)");
        assert_eq!(format!("{:?}", Duration::new(0, 999)), "Some(999ns)");
    }

    #[test]
    fn debug_formatting_precision_zero() {
        assert_eq!(format!("{:.0?}", Duration::new(0, 0)), "Some(0ns)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 123)), "Some(123ns)");

        assert_eq!(format!("{:.0?}", Duration::new(0, 1_001)), "Some(1µs)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_499)), "Some(1µs)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_500)), "Some(2µs)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_999)), "Some(2µs)");

        assert_eq!(format!("{:.0?}", Duration::new(0, 1_000_001)), "Some(1ms)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_499_999)), "Some(1ms)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_500_000)), "Some(2ms)");
        assert_eq!(format!("{:.0?}", Duration::new(0, 1_999_999)), "Some(2ms)");

        assert_eq!(format!("{:.0?}", Duration::new(1, 000_000_001)), "Some(1s)");
        assert_eq!(format!("{:.0?}", Duration::new(1, 499_999_999)), "Some(1s)");
        assert_eq!(format!("{:.0?}", Duration::new(1, 500_000_000)), "Some(2s)");
        assert_eq!(format!("{:.0?}", Duration::new(1, 999_999_999)), "Some(2s)");
    }

    #[test]
    fn debug_formatting_precision_two() {
        assert_eq!(format!("{:.2?}", Duration::new(0, 0)), "Some(0.00ns)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 123)), "Some(123.00ns)");

        assert_eq!(format!("{:.2?}", Duration::new(0, 1_000)), "Some(1.00µs)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 7_001)), "Some(7.00µs)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 7_100)), "Some(7.10µs)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 7_109)), "Some(7.11µs)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 7_199)), "Some(7.20µs)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 1_999)), "Some(2.00µs)");

        assert_eq!(format!("{:.2?}", Duration::new(0, 1_000_000)), "Some(1.00ms)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 3_001_000)), "Some(3.00ms)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 3_100_000)), "Some(3.10ms)");
        assert_eq!(format!("{:.2?}", Duration::new(0, 1_999_999)), "Some(2.00ms)");

        assert_eq!(format!("{:.2?}", Duration::new(1, 000_000_000)), "Some(1.00s)");
        assert_eq!(format!("{:.2?}", Duration::new(4, 001_000_000)), "Some(4.00s)");
        assert_eq!(format!("{:.2?}", Duration::new(2, 100_000_000)), "Some(2.10s)");
        assert_eq!(format!("{:.2?}", Duration::new(2, 104_990_000)), "Some(2.10s)");
        assert_eq!(format!("{:.2?}", Duration::new(2, 105_000_000)), "Some(2.11s)");
        assert_eq!(format!("{:.2?}", Duration::new(8, 999_999_999)), "Some(9.00s)");
    }

    #[test]
    fn debug_formatting_padding() {
        assert_eq!("Some(0ns      )", format!("{:<9?}", Duration::new(0, 0)));
        assert_eq!("Some(      0ns)", format!("{:>9?}", Duration::new(0, 0)));
        assert_eq!("Some(   0ns   )", format!("{:^9?}", Duration::new(0, 0)));
        assert_eq!("Some(123ns    )", format!("{:<9.0?}", Duration::new(0, 123)));
        assert_eq!("Some(    123ns)", format!("{:>9.0?}", Duration::new(0, 123)));
        assert_eq!("Some(  123ns  )", format!("{:^9.0?}", Duration::new(0, 123)));
        assert_eq!("Some(123.0ns  )", format!("{:<9.1?}", Duration::new(0, 123)));
        assert_eq!("Some(  123.0ns)", format!("{:>9.1?}", Duration::new(0, 123)));
        assert_eq!("Some( 123.0ns )", format!("{:^9.1?}", Duration::new(0, 123)));
        assert_eq!("Some(7.1µs    )", format!("{:<9?}", Duration::new(0, 7_100)));
        assert_eq!("Some(    7.1µs)", format!("{:>9?}", Duration::new(0, 7_100)));
        assert_eq!("Some(  7.1µs  )", format!("{:^9?}", Duration::new(0, 7_100)));
        assert_eq!("Some(999.123456ms)", format!("{:<9?}", Duration::new(0, 999_123_456)));
        assert_eq!("Some(999.123456ms)", format!("{:>9?}", Duration::new(0, 999_123_456)));
        assert_eq!("Some(999.123456ms)", format!("{:^9?}", Duration::new(0, 999_123_456)));
        assert_eq!("Some(5s       )", format!("{:<9?}", Duration::new(5, 0)));
        assert_eq!("Some(       5s)", format!("{:>9?}", Duration::new(5, 0)));
        assert_eq!("Some(   5s    )", format!("{:^9?}", Duration::new(5, 0)));
        assert_eq!("Some(5.000000000000s)", format!("{:<9.12?}", Duration::new(5, 0)));
        assert_eq!("Some(5.000000000000s)", format!("{:>9.12?}", Duration::new(5, 0)));
        assert_eq!("Some(5.000000000000s)", format!("{:^9.12?}", Duration::new(5, 0)));

        // default alignment is left:
        assert_eq!("Some(5s       )", format!("{:9?}", Duration::new(5, 0)));
    }

    #[test]
    fn debug_formatting_precision_high() {
        assert_eq!(format!("{:.5?}", Duration::new(0, 23_678)), "Some(23.67800µs)");

        assert_eq!(format!("{:.9?}", Duration::new(1, 000_000_000)), "Some(1.000000000s)");
        assert_eq!(format!("{:.10?}", Duration::new(4, 001_000_000)), "Some(4.0010000000s)");
        assert_eq!(
            format!("{:.20?}", Duration::new(4, 001_000_000)),
            "Some(4.00100000000000000000s)"
        );
    }

    #[test]
    fn debug_formatting_none() {
        assert_eq!(format!("{:?}", Duration::new(0, 0) - Duration::new(0, 1)), "None");
    }

    const fn duration_second() -> Duration {
        Duration::from_secs(1)
    }

    #[test]
    fn duration_const() {
        // test that the methods of `Duration` are usable in a const context

        const DURATION: Duration = Duration::from_nanos(123_456_789);

        const SUB_SEC_MILLIS: Option<u32> = DURATION.subsec_millis();
        assert_eq!(SUB_SEC_MILLIS, Some(123));

        const SUB_SEC_MICROS: Option<u32> = DURATION.subsec_micros();
        assert_eq!(SUB_SEC_MICROS, Some(123_456));

        const SUB_SEC_NANOS: Option<u32> = DURATION.subsec_nanos();
        assert_eq!(SUB_SEC_NANOS, Some(123_456_789));

        const IS_ZERO: bool = Duration::ZERO.is_zero();
        assert!(IS_ZERO);

        const SECONDS: Option<u64> = duration_second().as_secs();
        assert_eq!(SECONDS, Some(1));

        const FROM_SECONDS: Duration = Duration::from_secs(1);
        assert_eq!(FROM_SECONDS, duration_second());

        // const SECONDS_F32: Option<f32> = duration_second().as_secs_f32();
        // assert_eq!(SECONDS_F32, Some(1.));

        // const FROM_SECONDS_F32: Duration = Duration::from_secs_f32(1.);
        // assert_eq!(FROM_SECONDS_F32, duration_second);

        // const SECONDS_F64: f64 = duration_second().as_secs_f64();
        // assert_eq!(SECONDS_F64, 1.);

        // const FROM_SECONDS_F64: Duration = Duration::from_secs_f64(1.);
        // assert_eq!(FROM_SECONDS_F64, duration_second());

        const MILLIS: Option<u128> = duration_second().as_millis();
        assert_eq!(MILLIS, Some(1_000));

        const FROM_MILLIS: Duration = Duration::from_millis(1_000);
        assert_eq!(FROM_MILLIS, duration_second());

        const MICROS: Option<u128> = duration_second().as_micros();
        assert_eq!(MICROS, Some(1_000_000));

        const FROM_MICROS: Duration = Duration::from_micros(1_000_000);
        assert_eq!(FROM_MICROS, duration_second());

        const NANOS: Option<u128> = duration_second().as_nanos();
        assert_eq!(NANOS, Some(1_000_000_000));

        const FROM_NANOS: Duration = Duration::from_nanos(1_000_000_000);
        assert_eq!(FROM_NANOS, duration_second());

        #[allow(dead_code)]
        const MAX: Duration = Duration::new(u64::MAX, 999_999_999);

        // const ADD: Duration = MAX + duration_second();
        // assert_eq!(ADD.into_inner(), None);

        // const SUB: Duration = Duration::ZERO - duration_second();
        // assert_eq!(SUB.into_inner(), None);

        // const MUL: Duration = duration_second() * 1;
        // assert_eq!(MUL, duration_second());

        // const MUL_F32: Duration = duration_second().mul_f32(1.);
        // assert_eq!(MUL_F32, duration_second());

        // const MUL_F64: Duration = duration_second().mul_f64(1.);
        // assert_eq!(MUL_F64, duration_second());

        // const DIV: Duration = duration_second() / 1;
        // assert_eq!(DIV, duration_second());

        // const DIV_F32: Duration = duration_second().div_f32(1.);
        // assert_eq!(DIV_F32, duration_second());

        // const DIV_F64: Duration = duration_second().div_f64(1.);
        // assert_eq!(DIV_F64, duration_second());

        // const DIV_DURATION_F32: f32 = duration_second().div_duration_f32(duration_second());
        // assert_eq!(DIV_DURATION_F32, 1.);

        // const DIV_DURATION_F64: f64 = duration_second().div_duration_f64(duration_second());
        // assert_eq!(DIV_DURATION_F64, 1.);
    }
}
