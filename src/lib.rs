extern crate regex;

use regex::Regex;

use std::u32;
use std::ops::Add;
use std::ops::Mul;

static MAX_VALUE: u32 = u32::MAX;

pub struct BigUint {
    v: Vec<u32>,
}

impl BigUint {
    pub fn from_hex_string(arb_hex: String) -> Result<BigUint, &'static str> {
        let mut hex = arb_hex.to_lowercase();
        let regex = Regex::new("^[123456789abcdef]{1}[0123456789abcdef]*$").unwrap();
        if !regex.is_match(&hex) {
            return Err("String is not a valid hexadecimal number");
        }
        let mut vec = Vec::with_capacity(hex.len() / 8 + 1);
        loop {
            let mut num = 0;
            if hex.len() > 8 {
                let len = hex.len();
                let tail = hex.split_off(len - 8);
                num = u32::from_str_radix(&tail, 16).unwrap();
            } else {
                num = u32::from_str_radix(&hex, 16).unwrap();
                hex.split_off(0);
            }

            vec.push(num);
            if hex.len() == 0 {
                break;
            }
        }
        let num = BigUint { v: vec };
        Ok(num)
    }

    pub fn to_hex_string(&self) -> String {
        let mut string = String::with_capacity(self.v.len() * 8);
        for i in (0..(self.v.len())).rev() {
            string.push_str(&format!("{:x}", self.v[i]));
        }
        string
    }
}

impl Add for BigUint {
    type Output = BigUint;

    fn add(self, other: BigUint) -> BigUint {
        let v1 = self.v;
        let v2 = other.v;
        let mut carry = 0;
        let mut res = Vec::with_capacity(std::cmp::max(v1.len(), v2.len()) + 1);
        let mut i = 0;
        while carry > 0 || i < v1.len() || i < v2.len() {
            let mut current_value: u64 = carry;
            carry = 0;
            if i < v1.len() {
                current_value += v1[i] as u64;
            }
            if i < v2.len() {
                current_value += v2[i] as u64;
            }
            if current_value > MAX_VALUE as u64 {
                current_value -= MAX_VALUE as u64 + 1;
                carry += 1;
            }
            res.push(current_value as u32);
            i += 1;
        }
        BigUint { v: res }
    }
}

impl Mul for BigUint {
    type Output = BigUint;

    fn mul(self, other: BigUint) -> BigUint {
        let v1 = self.v;
        let v2 = other.v;
        let mut carry = 0;
        let mut res = vec![0; v1.len() * v2.len() + 1];
        let mut i = 0;
        
        BigUint { v: res }
    }
}


// #[test]
// #[should_panic]
// fn validate() {
//     BigUint::from_hex_string("ololo".to_string()).unwrap();
// }

#[test]
fn validate2() {
    let bigint = BigUint::from_hex_string("abcde1234abfa".to_string()).unwrap();
}

// #[test]
// #[should_panic]
// fn validate3() {
//     BigUint::from_hex_string("0abcde123456f".to_string()).unwrap();
// }

#[test]
fn assert_equality() {
    let hex_strings = vec!["abcde1234abfa",
                           "86e961e7511699fb856888b52ee3f3858ff9fe5c2fa576a210fbb25d556",
                           "d68b84131cb92d63a1d58dc18fb1661c5fb14bb183b7e3d29fefc6b34aa6"];

    for i in hex_strings {
        let big_num = BigUint::from_hex_string(i.to_string().clone()).unwrap();
        assert_eq!(i, big_num.to_hex_string());
    }
}

#[test]
fn assert_addition() {
    let first =
        BigUint::from_hex_string("86e961e7511699fb856888b52ee3f3858ff9fe5c2fa576a210fbb25d556"
                .to_string())
            .unwrap();
    let second =
        BigUint::from_hex_string("d68b84131cb92d63a1d58dc18fb1661c5fb14bb183b7e3d29fefc6b34aa6"
                .to_string())
            .unwrap();

    assert_eq!((first + second).to_hex_string(),
               "defa1a3191ca97035a2c164ce29fa554b8b0eb9746b23b3cc0ff81d91ffc")
}

#[test]
fn assert_multiplication() {
    let first =
        BigUint::from_hex_string("86e961e7511699fb856888b52ee3f3858ff9fe5c2fa576a210fbb25d556"
                .to_string())
            .unwrap();
    let second =
        BigUint::from_hex_string("d68b84131cb92d63a1d58dc18fb1661c5fb14bb183b7e3d29fefc6b34aa6"
                .to_string())
            .unwrap();

    assert_eq!((first * second).to_hex_string(),
               "71109e29ef5293411df1d9d6170aefbd32189914b0e5ceda8a6e9d3baf30492d5d793498d9003cf39f7fb006d5e04f37d2436fb1f7db00af95531c4")
}