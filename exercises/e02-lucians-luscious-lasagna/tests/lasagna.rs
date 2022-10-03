use lucians_luscious_lasagna::*;

#[test]
fn expected_minutes_in_oven_1() {
    assert_eq!(40, expected_minutes_in_oven());
}

#[test]
fn remaining_minutes_in_oven_1() {
    assert_eq!(30, remaining_minutes_in_oven(10));
}

#[test]
fn preparation_time_in_minutes_1() {
    assert_eq!(30, preparation_time_in_minutes(15));
}

#[test]
fn elapsed_time_in_minutes_1() {
    assert_eq!(24, elapsed_time_in_minutes(2, 20));
}
