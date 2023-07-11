#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_decimal::prelude::{FromPrimitive};
use number_benchmark::*;
use number_benchmark::fixed_dec;
use rand::Rng;

fn bench_fibs(c: &mut Criterion) {

    let size = 100;
    let mut data_f64: Vec<f64> = Vec::with_capacity(size);
    let mut data_rust_decimal: Vec<rust_decimal::Decimal> = Vec::with_capacity(size);
    let mut data_big_decimal: Vec<bigdecimal::BigDecimal> = Vec::with_capacity(size);
    let mut data_fpdec: Vec<fpdec::Decimal> = Vec::with_capacity(size);
    let mut data_fixed: Vec<fixed::types::I64F64> = Vec::with_capacity(size);
    let mut data_fixed2: Vec<fixed::types::I36F28> = Vec::with_capacity(size);
    let mut data_fixed_decimal: Vec<fixed_dec::FixedDecimal> = Vec::with_capacity(size);

    let mut rng = rand::thread_rng();
    for _i in 0..size {
        let x : f64 = rng.gen();
        data_f64.push(x);
        data_rust_decimal.push(rust_decimal::Decimal::from_f64(x).unwrap());
        data_big_decimal.push(bigdecimal::BigDecimal::from_f64(x).unwrap());
        data_fpdec.push(fpdec::Decimal::try_from(x).unwrap());
        data_fixed.push(fixed::types::I64F64::from_num(x));
        data_fixed2.push(fixed::types::I36F28::from_num(x));
        data_fixed_decimal.push(fixed_dec::FixedDecimal::new(x, 8));
    }

    let mut group1 = c.benchmark_group("number_benchmark add");
    group1.significance_level(0.1).sample_size(10_000);
    group1.bench_function("add f64", |b| b.iter(|| calc_add_f64(data_f64[0], data_f64[size-1])));
    group1.bench_function("add fixed_decimal", |b| b.iter(|| calc_add_fixed_decimal(data_fixed_decimal[0], data_fixed_decimal[size-1])));
    group1.bench_function("add fixed", |b| b.iter(|| calc_add_fixed(data_fixed[0], data_fixed[size-1])));
    group1.bench_function("add fixed2", |b| b.iter(|| calc_add_fixed2(data_fixed2[0], data_fixed2[size-1])));
    group1.bench_function("add fpdec", |b| b.iter(|| calc_add_fpdec(data_fpdec[0], data_fpdec[size-1])));
    group1.bench_function("add rust_decimal", |b| b.iter(|| calc_add_rust_decimal(data_rust_decimal[0], data_rust_decimal[size-1])));
    group1.bench_function("add big_decimal", |b| b.iter(|| calc_add_bigdecimal(&data_big_decimal[0], &data_big_decimal[size-1])));
    group1.finish();

    let mut group2 = c.benchmark_group("number_benchmark sub");
    group2.significance_level(0.1).sample_size(10_000);
    group2.bench_function("sub f64", |b| b.iter(|| calc_sub_f64(data_f64[0], data_f64[size-1])));
    group2.bench_function("sub fixed_decimal", |b| b.iter(|| calc_sub_fixed_decimal(data_fixed_decimal[0], data_fixed_decimal[size-1])));
    group2.bench_function("sub fixed", |b| b.iter(|| calc_sub_fixed(data_fixed[0], data_fixed[size-1])));
    group2.bench_function("sub fixed2", |b| b.iter(|| calc_sub_fixed2(data_fixed2[0], data_fixed2[size-1])));
    group2.bench_function("sub fpdec", |b| b.iter(|| calc_sub_fpdec(data_fpdec[0], data_fpdec[size-1])));
    group2.bench_function("sub rust_decimal", |b| b.iter(|| calc_sub_rust_decimal(data_rust_decimal[0], data_rust_decimal[size-1])));
    group2.bench_function("sub big_decimal", |b| b.iter(|| calc_sub_bigdecimal(&data_big_decimal[0], &data_big_decimal[size-1])));
    group2.finish();

    let mut group3 = c.benchmark_group("number_benchmark mul");
    group3.significance_level(0.1).sample_size(10_000);
    group3.bench_function("mul f64", |b| b.iter(|| calc_mul_f64(data_f64[0], data_f64[size-1])));
    group3.bench_function("mul fixed_decimal", |b| b.iter(|| calc_mul_fixed_decimal(data_fixed_decimal[0], data_fixed_decimal[size-1])));
    group3.bench_function("mul fixed", |b| b.iter(|| calc_mul_fixed(data_fixed[0], data_fixed[size-1])));
    group3.bench_function("mul fixed2", |b| b.iter(|| calc_mul_fixed2(data_fixed2[0], data_fixed2[size-1])));
    group3.bench_function("mul fpdec", |b| b.iter(|| calc_mul_fpdec(data_fpdec[0], data_fpdec[size-1])));
    group3.bench_function("mul rust_decimal", |b| b.iter(|| calc_mul_rust_decimal(data_rust_decimal[0], data_rust_decimal[size-1])));
    group3.bench_function("mul big_decimal", |b| b.iter(|| calc_mul_bigdecimal(&data_big_decimal[0], &data_big_decimal[size-1])));
    group3.finish();

    let mut group4 = c.benchmark_group("number_benchmark div");
    group4.significance_level(0.1).sample_size(10_000);
    group4.bench_function("div f64", |b| b.iter(|| calc_div_f64(data_f64[0], data_f64[size-1])));
    group4.bench_function("div fixed_decimal", |b| b.iter(|| calc_div_fixed_decimal(data_fixed_decimal[0], data_fixed_decimal[size-1])));
    group4.bench_function("div fixed", |b| b.iter(|| calc_div_fixed(data_fixed[0], data_fixed[size-1])));
    group4.bench_function("div fixed2", |b| b.iter(|| calc_div_fixed2(data_fixed2[0], data_fixed2[size-1])));
    group4.bench_function("div fpdec", |b| b.iter(|| calc_div_fpdec(data_fpdec[0], data_fpdec[size-1])));
    group4.bench_function("div rust_decimal", |b| b.iter(|| calc_div_rust_decimal(data_rust_decimal[0], data_rust_decimal[size-1])));
    group4.bench_function("div big_decimal", |b| b.iter(|| calc_div_bigdecimal(&data_big_decimal[0], &data_big_decimal[size-1])));
    group4.finish();


    let mut group5 = c.benchmark_group("number_benchmark all");
    group5.significance_level(0.1).sample_size(10_000);
    group5.bench_function("all f64", |b| b.iter(|| calc_all_f64(data_f64[0], data_f64[size-1])));
    group5.bench_function("all fixed_decimal", |b| b.iter(|| calc_all_fixed_decimal(data_fixed_decimal[0], data_fixed_decimal[size-1])));
    group5.bench_function("all fixed", |b| b.iter(|| calc_all_fixed(data_fixed[0], data_fixed[size-1])));
    group5.bench_function("all fixed2", |b| b.iter(|| calc_all_fixed2(data_fixed2[0], data_fixed2[size-1])));
    group5.bench_function("all fpdec", |b| b.iter(|| calc_all_fpdec(data_fpdec[0], data_fpdec[size-1])));
    group5.bench_function("all rust_decimal", |b| b.iter(|| calc_all_rust_decimal(data_rust_decimal[0], data_rust_decimal[size-1])));
    group5.bench_function("all big_decimal", |b| b.iter(|| calc_all_bigdecimal(&data_big_decimal[0], &data_big_decimal[size-1])));
    group5.finish();


    let mut group6 = c.benchmark_group("number_benchmark list");
    group6.significance_level(0.1).sample_size(100);
    group6.bench_function("list f64", |b| b.iter(|| calc_list_f64(&data_f64)));
    group6.bench_function("list fixed_decimal", |b| b.iter(|| calc_list_fixed_decimal(&data_fixed_decimal)));
    group6.bench_function("list fixed", |b| b.iter(|| calc_list_fixed(&data_fixed)));
    group6.bench_function("list fixed2", |b| b.iter(|| calc_list_fixed2(&data_fixed2)));
    group6.bench_function("list fpdec", |b| b.iter(|| calc_list_fpdec(&data_fpdec)));
    group6.bench_function("list rust_decimal", |b| b.iter(|| calc_list_rust_decimal(&data_rust_decimal)));
    group6.bench_function("list big_decimal", |b| b.iter(|| calc_list_bigdecimal(&data_big_decimal)));
    group6.finish();

}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);