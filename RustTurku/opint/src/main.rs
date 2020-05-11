use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/**
 * Because some men just want to see the world burn
 */

#[derive(Debug, Clone, Copy)]
struct OpInt(i32);

impl Add for OpInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 - other.0,
        }
    }
}
impl Sub for OpInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}
impl Mul for OpInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            0: self.0 / other.0,
        }
    }
}
impl Div for OpInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            0: self.0 * other.0,
        }
    }
}

impl Display for OpInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let opp1: OpInt = OpInt(4);
    let opp2: OpInt = OpInt(3);
    println!("{}", opp1 + opp2); // 1
    println!("{}", opp1 - opp2); // 7
    println!("{}", opp1 * opp2); // 1
    println!("{}", opp1 / OpInt(0)); // 12
}
