// 最大公約数・最小公倍数
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::zero_repeat_side_effects)]

use num_complex::*;
use num_rational::*;
use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::f64::consts::*;
use std::fmt;
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::iter::*;
use std::marker::*;
use std::mem::*;
use std::ops::*;
use std::str::FromStr;

// 1行でのbreak, continue, return
macro_rules! breakif {
    ($c:expr) => {
        if $c {
            break;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            break $l;
        }
    };
}
macro_rules! continueif {
    ($c:expr) => {
        if $c {
            continue;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            continue $l;
        }
    };
}
macro_rules! returnif {
    ($c:expr) => {
        if $c {
            return;
        }
    };
    ($c:expr, $v:expr) => {
        if $c {
            return $v;
        }
    };
}

const M107: i64 = 1000000007_i64;
const M998: i64 = 998244353_i64;
const L108: i64 = 100_000_000_i64;
const I108: i32 = 100_000_000_i32;
const L1018: i64 = 1_000_000_000_000_000_000_i64;
const IINF: i32 = (1_i32 << 29) - 1;
const LINF: i64 = (1_i64 << 61) - 1;
const UINF: usize = (1_usize << 61) - 1;

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                最大公約数・最小公倍数
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/gcd_lcm.rs

/// 最大公約数, デフォルトでi64
#[inline(always)]
pub fn gcd(a: i64, b: i64) -> i64 {
    return gcd_i64(a, b);
} // end of method

/// i32の最大公約数
#[inline(always)]
pub fn gcd_i32(mut a: i32, mut b: i32) -> i32 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    if a == 1 || b == 1 {
        return 1;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
} // end of method

/// i64の最大公約数
#[inline(always)]
pub fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    if a == 1 || b == 1 {
        return 1;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
} // end of method

/// i128の最大公約数
#[inline(always)]
pub fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    if a == 1 || b == 1 {
        return 1;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
} // end of method

/// 最小公倍数, デフォルトでi64
#[inline(always)]
pub fn lcm(a: i64, b: i64) -> i64 {
    return lcm_i64(a, b);
} // end of method

/// i32の最小公倍数
#[inline(always)]
pub fn lcm_i32(a: i32, b: i32) -> i32 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    let g = gcd_i32(a, b) as i64;
    let result = (a as i64 / g) * b as i64;
    return result as i32;
} // end of method

/// i64の最小公倍数
#[inline(always)]
pub fn lcm_i64(a: i64, b: i64) -> i64 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    let g = gcd_i64(a, b) as i128;
    let result = (a as i128 / g) * b as i128;
    return result as i64;
} // end of method

/// i128の最小公倍数
#[inline(always)]
pub fn lcm_i128(a: i128, b: i128) -> i128 {
    if a <= 0 || b <= 0 {
        return 0;
    }
    let g = gcd_i128(a, b);
    return a / g * b;
} // end of method
