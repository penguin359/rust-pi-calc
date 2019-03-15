#![allow(unused)]
extern crate num_bigint;
extern crate bigdecimal;

use num_bigint::{ToBigInt, BigInt, Sign};
use bigdecimal::BigDecimal;


#[test]
fn big_decimal_test() {
}


fn calc_pi(maxK: u32, prec: u64, disp: u64) -> i32 {  // parameter defaults chosen to gain 1000+ digits within a few seconds
//    gc().prec = prec
    //let mut (K, M, L, X, S) = (6, 1, 13591409.to_bigint().unwrap(), 1, 13591409);
    let mut K = BigDecimal::from(6);
    let mut M = BigDecimal::from(1);
    let mut L = BigDecimal::from(13591409);
    let mut X = BigDecimal::from(1);
    let mut S = BigDecimal::from(13591409);
    for k in 1..maxK+1 {
        //M = (K.pow(3) - BigDecimal::from(16)*K) * M;  // k**3
        M = (BigDecimal::from(1)*&K*&K*&K - BigDecimal::from(16)*&K) * &M;  // k**3
        L += BigDecimal::from(545140134i64);
        //X *= BigDecimal::new(BigInt::from_radix_be(Sign::Minus, b"262537412640768000", 10).unwrap(), 0);
        X *= BigDecimal::from(-262537412640768000i64);
        S += &M * &L / &X;
        K += BigDecimal::from(12);
    }
    let pi = BigDecimal::from(426880) * BigDecimal::from(10005).sqrt().unwrap() / S;
//    pi = Dec(str(pi)[:disp]);  // drop few digits of precision for accuracy
    println!("PI(maxK={} iterations, gc().prec={}, disp={} digits) =\n{}", maxK, prec, disp, pi);
//    return pi;
    0
}

fn main() {
    println!("Hello, world!");

    let pi = calc_pi(70, 1008, 1007);
    println!("\nFor greater precision and more digits (takes a few extra seconds) - Try");
    println!("Pi = PI(317,4501,4500)");
    println!("Pi = PI(353,5022,5020)");
}
