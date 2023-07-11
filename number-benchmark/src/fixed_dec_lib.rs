use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
};

use serde_json::{Result, Value};


pub const FIXED_PRECISION: u8 = 9;
pub const FIXED_SCALAR: f64 = 1_000_000_000.0; // 10.0**FIXED_PRECISION

#[must_use]
pub fn f64_to_fixed_i64(value: f64, precision: u8) -> i64 {
    assert!(precision <= FIXED_PRECISION, "precision exceeded maximum 9");
    let pow1 = 10_i64.pow(u32::from(precision));
    let pow2 = 10_i64.pow(u32::from(FIXED_PRECISION - precision));
    let rounded = (value * pow1 as f64).round() as i64;
    rounded * pow2
}

#[must_use]
pub fn f64_to_fixed_u64(value: f64, precision: u8) -> u64 {
    assert!(precision <= FIXED_PRECISION, "precision exceeded maximum 9");
    let pow1 = 10_u64.pow(u32::from(precision));
    let pow2 = 10_u64.pow(u32::from(FIXED_PRECISION - precision));
    let rounded = (value * pow1 as f64).round() as u64;
    rounded * pow2
}

#[must_use]
pub fn fixed_i64_to_f64(value: i64) -> f64 {
    (value as f64) * 0.000_000_001
}

#[must_use]
pub fn fixed_u64_to_f64(value: u64) -> f64 {
    (value as f64) * 0.000_000_001
}


/// Return the decimal precision inferred from the given string.
pub fn precision_from_str(s: &str) -> u8 {
    let lower_s = s.to_lowercase();
    // Handle scientific notation
    if lower_s.contains("e-") {
        return lower_s.split("e-").last().unwrap().parse::<u8>().unwrap();
    }
    if !lower_s.contains('.') {
        return 0;
    }
    return lower_s.split('.').last().unwrap().len() as u8;
}

#[no_mangle]
pub unsafe extern "C" fn precision_from_cstr(ptr: *const c_char) -> u8 {
    precision_from_str(&cstr_to_string(ptr))
}
#[must_use]
pub unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    assert!(!ptr.is_null(), "`ptr` was NULL");
    CStr::from_ptr(ptr)
        .to_str()
        .expect("CStr::from_ptr failed")
        .to_string()
}


////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest(precision, value,
        case(0, 0.0),
        case(1, 1.0),
        case(1, 1.1),
        case(9, 0.000_000_001),
        case(0, -0.0),
        case(1, -1.0),
        case(1, -1.1),
        case(9, -0.000_000_001),
    )]
    fn test_f64_to_fixed_i64_to_fixed(precision: u8, value: f64) {
        let fixed = f64_to_fixed_i64(value, precision);
        let result = fixed_i64_to_f64(fixed);
        assert_eq!(result, value);
    }

    #[rstest(
        precision,
        value,
        case(0, 0.0),
        case(1, 1.0),
        case(1, 1.1),
        case(9, 0.000_000_001)
    )]
    fn test_f64_to_fixed_u64_to_fixed(precision: u8, value: f64) {
        let fixed = f64_to_fixed_u64(value, precision);
        let result = fixed_u64_to_f64(fixed);
        assert_eq!(result, value);
    }

    #[rstest(
        precision,
        value,
        expected,
        case(0, 123_456.0, 123_456_000_000_000),
        case(0, 123_456.7, 123_457_000_000_000),
        case(0, 123_456.4, 123_456_000_000_000),
        case(1, 123_456.0, 123_456_000_000_000),
        case(1, 123_456.7, 123_456_700_000_000),
        case(1, 123_456.4, 123_456_400_000_000),
        case(2, 123_456.0, 123_456_000_000_000),
        case(2, 123_456.7, 123_456_700_000_000),
        case(2, 123_456.4, 123_456_400_000_000)
    )]
    fn test_f64_to_fixed_i64_with_precision(precision: u8, value: f64, expected: i64) {
        assert_eq!(f64_to_fixed_i64(value, precision), expected);
    }

    #[rstest(precision, value, expected,
        case(0, 5.5, 6_000_000_000),
        case(1, 5.55, 5_600_000_000),
        case(2, 5.555, 5_560_000_000),
        case(3, 5.5555, 5_556_000_000),
        case(4, 5.55555, 5_555_600_000),
        case(5, 5.555_555, 5_555_560_000),
        case(6, 5.555_555_5, 5_555_556_000),
        case(7, 5.555_555_55, 5_555_555_600),
        case(8, 5.555_555_555, 5_555_555_560),
        case(9, 5.555_555_555_5, 5_555_555_556),
        case(0, -5.5, -6_000_000_000),
        case(1, -5.55, -5_600_000_000),
        case(2, -5.555, -5_560_000_000),
        case(3, -5.5555, -5_556_000_000),
        case(4, -5.55555, -5_555_600_000),
        case(5, -5.555_555, -5_555_560_000),
        case(6, -5.555_555_5, -5_555_556_000),
        case(7, -5.555_555_55, -5_555_555_600),
        case(8, -5.555_555_555, -5_555_555_560),
        case(9, -5.555_555_555_5, -5_555_555_556),
    )]
    fn test_f64_to_fixed_i64(precision: u8, value: f64, expected: i64) {
        assert_eq!(f64_to_fixed_i64(value, precision), expected);
    }

    #[rstest(
        precision,
        value,
        expected,
        case(0, 5.5, 6_000_000_000),
        case(1, 5.55, 5_600_000_000),
        case(2, 5.555, 5_560_000_000),
        case(3, 5.5555, 5_556_000_000),
        case(4, 5.55555, 5_555_600_000),
        case(5, 5.555_555, 5_555_560_000),
        case(6, 5.555_555_5, 5_555_556_000),
        case(7, 5.555_555_55, 5_555_555_600),
        case(8, 5.555_555_555, 5_555_555_560),
        case(9, 5.555_555_555_5, 5_555_555_556)
    )]
    fn test_f64_to_fixed_u64(precision: u8, value: f64, expected: u64) {
        assert_eq!(f64_to_fixed_u64(value, precision), expected);
    }

    #[rstest]
    fn test_fixed_i64_to_f64(
        #[values(1, -1, 2, -2, 10, -10, 100, -100, 1_000, -1_000)] value: i64,
    ) {
        assert_eq!(fixed_i64_to_f64(value), value as f64 * 0.000_000_001);
    }

    #[rstest]
    fn test_fixed_u64_to_f64(
        #[values(
            0,
            1,
            2,
            3,
            10,
            100,
            1_000,
            10_000,
            100_000,
            1_000_000,
            10_000_000,
            100_000_000,
            1_000_000_000,
            10_000_000_000,
            100_000_000_000,
            1_000_000_000_000,
            10_000_000_000_000,
            100_000_000_000_000,
            1_000_000_000_000_000
        )]
        value: u64,
    ) {
        let result = fixed_u64_to_f64(value);
        assert_eq!(result, (value as f64) * 0.000_000_001);
    }
}
