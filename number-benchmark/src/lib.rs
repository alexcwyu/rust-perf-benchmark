pub mod fixed_dec;
mod fixed_dec_lib;

use decimal::d128;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;


#[inline]
pub fn calc_list_f64(data_f64: &Vec<f64>) -> f64{
    let mut result = 0_f64;
    for i in 0..data_f64.len() {
        for k in (0..data_f64.len()).rev() {

            let left = data_f64[i];
            let right = data_f64[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}

#[inline]
pub fn calc_list_rust_decimal(data_rust_decimal: &Vec<rust_decimal::Decimal>) -> rust_decimal::Decimal {
    let mut result = rust_decimal::Decimal::new(0, 0);
    for i in 0..data_rust_decimal.len() {
        for k in (0..data_rust_decimal.len()).rev() {

            let left = data_rust_decimal[i];
            let right = data_rust_decimal[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}

#[inline]
pub fn calc_list_bigdecimal(data_rust_decimal: &Vec<bigdecimal::BigDecimal>) -> bigdecimal::BigDecimal {
    let mut result = bigdecimal::BigDecimal::from(0);
    for i in 0..data_rust_decimal.len() {
        for k in (0..data_rust_decimal.len()).rev() {

            let left = &data_rust_decimal[i];
            let right = &data_rust_decimal[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result


}

#[inline]
pub fn calc_list_fraction(data_rust_decimal: &Vec<fraction::Fraction>) -> fraction::Fraction {
    let mut result = fraction::Fraction::from(0);
    for i in 0..data_rust_decimal.len() {
        for k in (0..data_rust_decimal.len()).rev() {

            let left = data_rust_decimal[i];
            let right = data_rust_decimal[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}

#[inline]
pub fn calc_list_decimal(data_rust_decimal: &Vec<decimal::d128>) -> decimal::d128 {
    let mut result = d128!(0);
    for i in 0..data_rust_decimal.len() {
        for k in (0..data_rust_decimal.len()).rev() {

            let left = data_rust_decimal[i];
            let right = data_rust_decimal[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}

#[inline]
pub fn calc_list_fpdec(data_rust_decimal: &Vec<fpdec::Decimal>) -> fpdec::Decimal {
    let mut result = fpdec::Decimal::try_from(0_f64).unwrap();
    for i in 0..data_rust_decimal.len() {
        for k in (0..data_rust_decimal.len()).rev() {

            let left = data_rust_decimal[i];
            let right = data_rust_decimal[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}


#[inline]
pub fn calc_list_fixed(data_rust_fixed: &Vec<fixed::types::I64F64>) -> fixed::types::I64F64 {
    let mut result = fixed::types::I64F64::from_num(0);
    for i in 0..data_rust_fixed.len() {
        for k in (0..data_rust_fixed.len()).rev() {

            let left = data_rust_fixed[i];
            let right = data_rust_fixed[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}


#[inline]
pub fn calc_list_fixed2(data_rust_fixed: &Vec<fixed::types::I36F28>) -> fixed::types::I36F28 {
    let mut result = fixed::types::I36F28::from_num(0);
    for i in 0..data_rust_fixed.len() {
        for k in (0..data_rust_fixed.len()).rev() {

            let left = data_rust_fixed[i];
            let right = data_rust_fixed[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}


#[inline]
pub fn calc_list_fixed_decimal(data_rust_fixed: &Vec<fixed_dec::FixedDecimal>) -> fixed_dec::FixedDecimal {
    let mut result = fixed_dec::FixedDecimal::new(0_f64, 0);
    for i in 0..data_rust_fixed.len() {
        for k in (0..data_rust_fixed.len()).rev() {

            let left = data_rust_fixed[i];
            let right = data_rust_fixed[k];
            let add = left + right;
            let sub = left - right;
            let mul = left * right;
            let div = left / right;

            result = result + add + sub + mul + div;
        }
    }
    result
}

#[inline]
pub fn calc_all_f64(left : f64, right : f64) -> f64 {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_rust_decimal(left : rust_decimal::Decimal,
                       right : rust_decimal::Decimal)
                       -> rust_decimal::Decimal {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_bigdecimal(left : &bigdecimal::BigDecimal, right : &bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_fraction(left : fraction::Fraction, right : fraction::Fraction) -> fraction::Fraction {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}


#[inline]
pub fn calc_all_decimal(left : decimal::d128, right : decimal::d128) -> decimal::d128 {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_fpdec(left : fpdec::Decimal, right : fpdec::Decimal) -> fpdec::Decimal {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_fixed(left : fixed::types::I64F64, right : fixed::types::I64F64) -> fixed::types::I64F64 {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_fixed2(left : fixed::types::I36F28, right : fixed::types::I36F28) -> fixed::types::I36F28 {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_all_fixed_decimal(left : fixed_dec::FixedDecimal, right : fixed_dec::FixedDecimal) -> fixed_dec::FixedDecimal {
    let add = left + right;
    let sub = left - right;
    let mul = left * right;
    let div = left / right;

    add + sub + mul + div
}

#[inline]
pub fn calc_add_f64(left : f64, right : f64) -> f64 {
    left + right
}

#[inline]
pub fn calc_add_rust_decimal(left : rust_decimal::Decimal,
                           right : rust_decimal::Decimal)
                           -> rust_decimal::Decimal {
    left + right
}

#[inline]
pub fn calc_add_bigdecimal(left : &bigdecimal::BigDecimal, right : &bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    left + right
}

#[inline]
pub fn calc_add_fraction(left : fraction::Fraction, right : fraction::Fraction) -> fraction::Fraction {
    left + right
}


#[inline]
pub fn calc_add_decimal(left : decimal::d128, right : decimal::d128) -> decimal::d128 {
    left + right
}

#[inline]
pub fn calc_add_fpdec(left : fpdec::Decimal, right : fpdec::Decimal) -> fpdec::Decimal {
    left + right
}

#[inline]
pub fn calc_add_fixed(left : fixed::types::I64F64, right : fixed::types::I64F64) -> fixed::types::I64F64 {
    left + right
}

#[inline]
pub fn calc_add_fixed2(left : fixed::types::I36F28, right : fixed::types::I36F28) -> fixed::types::I36F28 {
    left + right
}



#[inline]
pub fn calc_add_fixed_decimal(left : fixed_dec::FixedDecimal, right : fixed_dec::FixedDecimal) -> fixed_dec::FixedDecimal {
    left + right
}

#[inline]
pub fn calc_sub_f64(left : f64, right : f64) -> f64 {
    left - right
}

#[inline]
pub fn calc_sub_rust_decimal(left : rust_decimal::Decimal,
                             right : rust_decimal::Decimal)
                             -> rust_decimal::Decimal {
    left - right
}

#[inline]
pub fn calc_sub_bigdecimal(left : &bigdecimal::BigDecimal, right : &bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    left - right
}

#[inline]
pub fn calc_sub_fraction(left : fraction::Fraction, right : fraction::Fraction) -> fraction::Fraction {
    left - right
}


#[inline]
pub fn calc_sub_decimal(left : decimal::d128, right : decimal::d128) -> decimal::d128 {
    left - right
}

#[inline]
pub fn calc_sub_fpdec(left : fpdec::Decimal, right : fpdec::Decimal) -> fpdec::Decimal {
    left - right
}

#[inline]
pub fn calc_sub_fixed(left : fixed::types::I64F64, right : fixed::types::I64F64) -> fixed::types::I64F64 {
    left - right
}


#[inline]
pub fn calc_sub_fixed2(left : fixed::types::I36F28, right : fixed::types::I36F28) -> fixed::types::I36F28 {
    left - right
}


#[inline]
pub fn calc_sub_fixed_decimal(left : fixed_dec::FixedDecimal, right : fixed_dec::FixedDecimal) -> fixed_dec::FixedDecimal {
     left - right
}


#[inline]
pub fn calc_mul_f64(left : f64, right : f64) -> f64 {
    left * right
}

#[inline]
pub fn calc_mul_rust_decimal(left : rust_decimal::Decimal,
                             right : rust_decimal::Decimal)
                             -> rust_decimal::Decimal {
    left * right
}

#[inline]
pub fn calc_mul_bigdecimal(left : &bigdecimal::BigDecimal, right : &bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    left * right
}

#[inline]
pub fn calc_mul_fraction(left : fraction::Fraction, right : fraction::Fraction) -> fraction::Fraction {
    left * right
}


#[inline]
pub fn calc_mul_decimal(left : decimal::d128, right : decimal::d128) -> decimal::d128 {
    left * right
}

#[inline]
pub fn calc_mul_fpdec(left : fpdec::Decimal, right : fpdec::Decimal) -> fpdec::Decimal {
    left * right
}

#[inline]
pub fn calc_mul_fixed(left : fixed::types::I64F64, right : fixed::types::I64F64) -> fixed::types::I64F64 {
    left * right
}

#[inline]
pub fn calc_mul_fixed2(left : fixed::types::I36F28, right : fixed::types::I36F28) -> fixed::types::I36F28 {
    left * right
}

#[inline]
pub fn calc_mul_fixed_decimal(left : fixed_dec::FixedDecimal, right : fixed_dec::FixedDecimal) -> fixed_dec::FixedDecimal {
    left * right
}


#[inline]
pub fn calc_div_f64(left : f64, right : f64) -> f64 {
    left / right
}

#[inline]
pub fn calc_div_rust_decimal(left : rust_decimal::Decimal,
                             right : rust_decimal::Decimal)
                             -> rust_decimal::Decimal {
    left / right
}

#[inline]
pub fn calc_div_bigdecimal(left : &bigdecimal::BigDecimal, right : &bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    left / right
}

#[inline]
pub fn calc_div_fraction(left : fraction::Fraction, right : fraction::Fraction) -> fraction::Fraction {
    left / right
}


#[inline]
pub fn calc_div_decimal(left : decimal::d128, right : decimal::d128) -> decimal::d128 {
    left / right
}

#[inline]
pub fn calc_div_fpdec(left : fpdec::Decimal, right : fpdec::Decimal) -> fpdec::Decimal {
    left / right
}

#[inline]
pub fn calc_div_fixed(left : fixed::types::I64F64, right : fixed::types::I64F64) -> fixed::types::I64F64 {
    left / right
}

#[inline]
pub fn calc_div_fixed2(left : fixed::types::I36F28, right : fixed::types::I36F28) -> fixed::types::I36F28 {
    left / right
}

#[inline]
pub fn calc_div_fixed_decimal(left : fixed_dec::FixedDecimal, right : fixed_dec::FixedDecimal) -> fixed_dec::FixedDecimal {
    left / right
}


#[cfg(test)]
mod tests {
    use decimal::d128;
    use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
    use super::*;
    use float_cmp::approx_eq;
    use rand::Rng;

    #[test]
    fn one_number() {
        // let mut rng = rand::thread_rng();
        // let left : f64 = rng.gen();
        // let right : f64 = rng.gen();

        let left : f64 = 1.0;
        let right : f64 = 2.0;

        let result_1 = calc_all_f64(left, right);
        let result_2 = calc_all_rust_decimal(rust_decimal::Decimal::from_f64(left).unwrap(), rust_decimal::Decimal::from_f64(right).unwrap()).to_f64().unwrap();
        let result_3 = calc_all_bigdecimal(&bigdecimal::BigDecimal::from_f64(left).unwrap(), &bigdecimal::BigDecimal::from_f64(right).unwrap()).to_f64().unwrap();
        let result_4 = fast_float::parse(calc_all_fpdec(fpdec::Decimal::try_from(left).unwrap(), fpdec::Decimal::try_from(right).unwrap()).to_string()).unwrap();
        let result_5 = calc_all_fixed(fixed::types::I64F64::from_num(left), fixed::types::I64F64::from_num(right)).to_num();
        let result_6 = calc_all_fixed2(fixed::types::I36F28::from_num(left), fixed::types::I36F28::from_num(right)).to_num();
        let result_7 = calc_all_fixed_decimal(fixed_dec::FixedDecimal::new(left, 8), fixed_dec::FixedDecimal::new(right, 8)).as_f64();

        println!("result_1={}", result_1);
        println!("result_2={}", result_2);
        println!("result_3={}", result_3);
        println!("result_4={}", result_4);
        println!("result_5={}", result_5);
        println!("result_6={}", result_6);
        println!("result_7={}", result_7);

        assert!( approx_eq!(f64, result_1, result_2, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_3, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_4, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_5, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_6, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_7, ulps = 2) );
    }

    #[test]
    fn all_numbers(){
        let size = 100;
        let mut data_f64: Vec<f64> = Vec::with_capacity(size);
        let mut data_rust_decimal: Vec<rust_decimal::Decimal> = Vec::with_capacity(size);
        let mut data_big_decimal: Vec<bigdecimal::BigDecimal> = Vec::with_capacity(size);
        let mut data_fpdec: Vec<fpdec::Decimal> = Vec::with_capacity(size);
        let mut data_fixed: Vec<fixed::types::I64F64> = Vec::with_capacity(size);
        let mut data_fixed2: Vec<fixed::types::I36F28> = Vec::with_capacity(size);
        let mut data_fixed_decimal: Vec<fixed_dec::FixedDecimal> = Vec::with_capacity(size);
        // let mut data_fraction: Vec<fraction::Fraction> = Vec::with_capacity(size);
        // let mut data_decimal: Vec<decimal::d128> = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for i in 0..size {
            let x : f64 = rng.gen();
            data_f64.push(x);
            data_rust_decimal.push(rust_decimal::Decimal::from_f64(x).unwrap());
            data_big_decimal.push(bigdecimal::BigDecimal::from_f64(x).unwrap());
            data_fpdec.push(fpdec::Decimal::try_from(x).unwrap());
            data_fixed.push(fixed::types::I64F64::from_num(x));
            data_fixed2.push(fixed::types::I36F28::from_num(x));
            data_fixed_decimal.push(fixed_dec::FixedDecimal::new(x, 8));
            // data_fraction.push(fraction::Fraction::from(x));
            // data_decimal.push(d128!(x));
        }
        let result_1 :f64 = calc_list_f64(&data_f64);
        let result_2 :f64 = calc_list_rust_decimal(&data_rust_decimal).to_f64().unwrap();
        let result_3 :f64 = calc_list_bigdecimal(&data_big_decimal).to_f64().unwrap();
        let result_4 :f64 = fast_float::parse(calc_list_fpdec(&data_fpdec).to_string()).unwrap();
        let result_5 :f64 = calc_list_fixed(&data_fixed).to_num();
        let result_6 :f64 = calc_list_fixed2(&data_fixed2).to_num();
        let result_7 :f64 = calc_list_fixed_decimal(&data_fixed_decimal).as_f64();
        // let result_8 = calc_list_fraction(&data_fraction).to_f64().unwrap();
        // let result_9 :f64 = fast_float::parse(calc_list_decimal(&data_decimal).to_string()).unwrap();


        println!("result_1={}", result_1);
        println!("result_2={}", result_2);
        println!("result_3={}", result_3);
        println!("result_4={}", result_4);
        println!("result_5={}", result_5);
        println!("result_6={}", result_6);
        println!("result_7={}", result_7);
        // println!("result_8={}", result_8);
        // println!("result_9={}", result_9);

        assert!( approx_eq!(f64, result_1, result_2, epsilon = 0.00003, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_3, epsilon = 0.00003, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_4, epsilon = 0.00003, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_5, epsilon = 0.00003, ulps = 2) );
        assert!( approx_eq!(f64, result_1, result_6, epsilon = 0.003, ulps = 8) );
        assert!( approx_eq!(f64, result_1, result_7, epsilon = 0.5, ulps = 5) );
    }
}

