use assembly_line::*;

#[test]
fn test_production_rate_per_hour_1() {
    assert_eq!(994.5, production_rate_per_hour(5 as u8));
}

#[test]
fn test_production_rate_per_hour_2() {
    assert_eq!(1591.2, production_rate_per_hour(8 as u8));
}

#[test]
fn test_working_items_per_minute_1() {
    assert_eq!(26, working_items_per_minute(8 as u8));
}

