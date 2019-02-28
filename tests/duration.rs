// https://github.com/rust-lang/rust/blob/master/src/libcore/tests/time.rs

use core::time;

use easytime::Duration;

#[test]
fn creation() {
    assert!(Duration::from_secs(1) != Duration::from_secs(0));
    assert_eq!(
        Duration::from_secs(1) + Duration::from_secs(2),
        Duration::from_secs(3)
    );
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
    assert_eq!(
        Duration::new(0, 1_050_000_001).subsec_micros(),
        Some(50_000)
    );
    assert_eq!(Duration::from_secs(1).subsec_micros(), Some(0));
    assert_eq!(Duration::from_millis(999).subsec_micros(), Some(999_000));
    assert_eq!(Duration::from_millis(1001).subsec_micros(), Some(1_000));
    assert_eq!(
        Duration::from_micros(999_999).subsec_micros(),
        Some(999_999)
    );
    assert_eq!(Duration::from_micros(1_000_001).subsec_micros(), Some(1));
    assert_eq!(
        Duration::from_nanos(999_999_999).subsec_micros(),
        Some(999_999)
    );
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
    assert_eq!(
        Duration::from_micros(999_999).subsec_nanos(),
        Some(999_999_000)
    );
    assert_eq!(Duration::from_micros(1_000_001).subsec_nanos(), Some(1000));
    assert_eq!(
        Duration::from_nanos(999_999_999).subsec_nanos(),
        Some(999_999_999)
    );
    assert_eq!(Duration::from_nanos(1_000_000_001).subsec_nanos(), Some(1));
}

#[test]
fn add() {
    assert_eq!(
        Duration::new(0, 0) + Duration::new(0, 1),
        Duration::new(0, 1)
    );
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
    assert_eq!(
        (Duration::new(1, 0) + Duration::new(u64::max_value(), 0)).into_inner(),
        None
    );
}

#[test]
fn sub() {
    assert_eq!(
        Duration::new(0, 1) - Duration::new(0, 0),
        Duration::new(0, 1)
    );
    assert_eq!(
        Duration::new(0, 500_000_001) - Duration::new(0, 500_000_000),
        Duration::new(0, 1)
    );
    assert_eq!(
        Duration::new(1, 0) - Duration::new(0, 1),
        Duration::new(0, 999_999_999)
    );
}

#[test]
fn checked_sub() {
    let zero = Duration::new(0, 0);
    let one_nano = Duration::new(0, 1);
    let one_sec = Duration::new(1, 0);
    assert_eq!(
        (one_nano - zero).into_inner(),
        Some(time::Duration::new(0, 1))
    );
    assert_eq!(
        (one_sec - one_nano).into_inner(),
        Some(time::Duration::new(0, 999_999_999))
    );
    assert_eq!((zero - one_nano).into_inner(), None);
    assert_eq!((zero - one_sec).into_inner(), None);
}

#[test]
fn mul() {
    assert_eq!(Duration::new(0, 1) * 2, Duration::new(0, 2));
    assert_eq!(Duration::new(1, 1) * 3, Duration::new(3, 3));
    assert_eq!(Duration::new(0, 500_000_001) * 4, Duration::new(2, 4));
    assert_eq!(
        Duration::new(0, 500_000_001) * 4000,
        Duration::new(2000, 4000)
    );
}

#[test]
fn checked_mul() {
    assert_eq!(
        (Duration::new(0, 1) * 2).into_inner(),
        Some(time::Duration::new(0, 2))
    );
    assert_eq!(
        (Duration::new(1, 1) * 3).into_inner(),
        Some(time::Duration::new(3, 3))
    );
    assert_eq!(
        (Duration::new(0, 500_000_001) * 4).into_inner(),
        Some(time::Duration::new(2, 4))
    );
    assert_eq!(
        (Duration::new(0, 500_000_001) * 4000).into_inner(),
        Some(time::Duration::new(2000, 4000))
    );
    assert_eq!(
        (Duration::new(u64::max_value() - 1, 0) * 2).into_inner(),
        None
    );
}

#[test]
fn div() {
    assert_eq!(Duration::new(0, 1) / 2, Duration::new(0, 0));
    assert_eq!(Duration::new(1, 1) / 3, Duration::new(0, 333_333_333));
    assert_eq!(
        Duration::new(99, 999_999_000) / 100,
        Duration::new(0, 999_999_990)
    );
}

#[test]
fn checked_div() {
    assert_eq!(
        (Duration::new(2, 0) / 2).into_inner(),
        Some(time::Duration::new(1, 0))
    );
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

/* TODO duration_debug_impl https://github.com/rust-lang/rust/pull/50364
#[test]
fn debug_formatting_extreme_values() {
    assert_eq!(
        format!(
            "{:?}",
            Duration::new(18_446_744_073_709_551_615, 123_456_789)
        ),
        "18446744073709551615.123456789s"
    );
}

#[test]
fn debug_formatting_secs() {
    assert_eq!(format!("{:?}", Duration::new(7, 000_000_000)), "7s");
    assert_eq!(format!("{:?}", Duration::new(7, 100_000_000)), "7.1s");
    assert_eq!(format!("{:?}", Duration::new(7, 000_010_000)), "7.00001s");
    assert_eq!(
        format!("{:?}", Duration::new(7, 000_000_001)),
        "7.000000001s"
    );
    assert_eq!(
        format!("{:?}", Duration::new(7, 123_456_789)),
        "7.123456789s"
    );

    assert_eq!(format!("{:?}", Duration::new(88, 000_000_000)), "88s");
    assert_eq!(format!("{:?}", Duration::new(88, 100_000_000)), "88.1s");
    assert_eq!(format!("{:?}", Duration::new(88, 000_010_000)), "88.00001s");
    assert_eq!(
        format!("{:?}", Duration::new(88, 000_000_001)),
        "88.000000001s"
    );
    assert_eq!(
        format!("{:?}", Duration::new(88, 123_456_789)),
        "88.123456789s"
    );

    assert_eq!(format!("{:?}", Duration::new(999, 000_000_000)), "999s");
    assert_eq!(format!("{:?}", Duration::new(999, 100_000_000)), "999.1s");
    assert_eq!(
        format!("{:?}", Duration::new(999, 000_010_000)),
        "999.00001s"
    );
    assert_eq!(
        format!("{:?}", Duration::new(999, 000_000_001)),
        "999.000000001s"
    );
    assert_eq!(
        format!("{:?}", Duration::new(999, 123_456_789)),
        "999.123456789s"
    );
}

#[test]
fn debug_formatting_millis() {
    assert_eq!(format!("{:?}", Duration::new(0, 7_000_000)), "7ms");
    assert_eq!(format!("{:?}", Duration::new(0, 7_100_000)), "7.1ms");
    assert_eq!(format!("{:?}", Duration::new(0, 7_000_001)), "7.000001ms");
    assert_eq!(format!("{:?}", Duration::new(0, 7_123_456)), "7.123456ms");

    assert_eq!(format!("{:?}", Duration::new(0, 88_000_000)), "88ms");
    assert_eq!(format!("{:?}", Duration::new(0, 88_100_000)), "88.1ms");
    assert_eq!(format!("{:?}", Duration::new(0, 88_000_001)), "88.000001ms");
    assert_eq!(format!("{:?}", Duration::new(0, 88_123_456)), "88.123456ms");

    assert_eq!(format!("{:?}", Duration::new(0, 999_000_000)), "999ms");
    assert_eq!(format!("{:?}", Duration::new(0, 999_100_000)), "999.1ms");
    assert_eq!(
        format!("{:?}", Duration::new(0, 999_000_001)),
        "999.000001ms"
    );
    assert_eq!(
        format!("{:?}", Duration::new(0, 999_123_456)),
        "999.123456ms"
    );
}

#[test]
fn debug_formatting_micros() {
    assert_eq!(format!("{:?}", Duration::new(0, 7_000)), "7µs");
    assert_eq!(format!("{:?}", Duration::new(0, 7_100)), "7.1µs");
    assert_eq!(format!("{:?}", Duration::new(0, 7_001)), "7.001µs");
    assert_eq!(format!("{:?}", Duration::new(0, 7_123)), "7.123µs");

    assert_eq!(format!("{:?}", Duration::new(0, 88_000)), "88µs");
    assert_eq!(format!("{:?}", Duration::new(0, 88_100)), "88.1µs");
    assert_eq!(format!("{:?}", Duration::new(0, 88_001)), "88.001µs");
    assert_eq!(format!("{:?}", Duration::new(0, 88_123)), "88.123µs");

    assert_eq!(format!("{:?}", Duration::new(0, 999_000)), "999µs");
    assert_eq!(format!("{:?}", Duration::new(0, 999_100)), "999.1µs");
    assert_eq!(format!("{:?}", Duration::new(0, 999_001)), "999.001µs");
    assert_eq!(format!("{:?}", Duration::new(0, 999_123)), "999.123µs");
}

#[test]
fn debug_formatting_nanos() {
    assert_eq!(format!("{:?}", Duration::new(0, 0)), "0ns");
    assert_eq!(format!("{:?}", Duration::new(0, 1)), "1ns");
    assert_eq!(format!("{:?}", Duration::new(0, 88)), "88ns");
    assert_eq!(format!("{:?}", Duration::new(0, 999)), "999ns");
}

#[test]
fn debug_formatting_precision_zero() {
    assert_eq!(format!("{:.0?}", Duration::new(0, 0)), "0ns");
    assert_eq!(format!("{:.0?}", Duration::new(0, 123)), "123ns");

    assert_eq!(format!("{:.0?}", Duration::new(0, 1_001)), "1µs");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_499)), "1µs");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_500)), "2µs");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_999)), "2µs");

    assert_eq!(format!("{:.0?}", Duration::new(0, 1_000_001)), "1ms");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_499_999)), "1ms");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_500_000)), "2ms");
    assert_eq!(format!("{:.0?}", Duration::new(0, 1_999_999)), "2ms");

    assert_eq!(format!("{:.0?}", Duration::new(1, 000_000_001)), "1s");
    assert_eq!(format!("{:.0?}", Duration::new(1, 499_999_999)), "1s");
    assert_eq!(format!("{:.0?}", Duration::new(1, 500_000_000)), "2s");
    assert_eq!(format!("{:.0?}", Duration::new(1, 999_999_999)), "2s");
}

#[test]
fn debug_formatting_precision_two() {
    assert_eq!(format!("{:.2?}", Duration::new(0, 0)), "0.00ns");
    assert_eq!(format!("{:.2?}", Duration::new(0, 123)), "123.00ns");

    assert_eq!(format!("{:.2?}", Duration::new(0, 1_000)), "1.00µs");
    assert_eq!(format!("{:.2?}", Duration::new(0, 7_001)), "7.00µs");
    assert_eq!(format!("{:.2?}", Duration::new(0, 7_100)), "7.10µs");
    assert_eq!(format!("{:.2?}", Duration::new(0, 7_109)), "7.11µs");
    assert_eq!(format!("{:.2?}", Duration::new(0, 7_199)), "7.20µs");
    assert_eq!(format!("{:.2?}", Duration::new(0, 1_999)), "2.00µs");

    assert_eq!(format!("{:.2?}", Duration::new(0, 1_000_000)), "1.00ms");
    assert_eq!(format!("{:.2?}", Duration::new(0, 3_001_000)), "3.00ms");
    assert_eq!(format!("{:.2?}", Duration::new(0, 3_100_000)), "3.10ms");
    assert_eq!(format!("{:.2?}", Duration::new(0, 1_999_999)), "2.00ms");

    assert_eq!(format!("{:.2?}", Duration::new(1, 000_000_000)), "1.00s");
    assert_eq!(format!("{:.2?}", Duration::new(4, 001_000_000)), "4.00s");
    assert_eq!(format!("{:.2?}", Duration::new(2, 100_000_000)), "2.10s");
    assert_eq!(format!("{:.2?}", Duration::new(2, 104_990_000)), "2.10s");
    assert_eq!(format!("{:.2?}", Duration::new(2, 105_000_000)), "2.11s");
    assert_eq!(format!("{:.2?}", Duration::new(8, 999_999_999)), "9.00s");
}

#[test]
fn debug_formatting_precision_high() {
    assert_eq!(format!("{:.5?}", Duration::new(0, 23_678)), "23.67800µs");

    assert_eq!(
        format!("{:.9?}", Duration::new(1, 000_000_000)),
        "1.000000000s"
    );
    assert_eq!(
        format!("{:.10?}", Duration::new(4, 001_000_000)),
        "4.0010000000s"
    );
    assert_eq!(
        format!("{:.20?}", Duration::new(4, 001_000_000)),
        "4.00100000000000000000s"
    );
}
*/
