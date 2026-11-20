// 素因数分解
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
//                                      素因数分解
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/prime_factorize.rs

/// Miller-Rabin で使う基数
/// i64 の正整数範囲ならこの基数セットで十分
const MILLER_RABIN_BASES: [i64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

/// 最大公約数
#[inline(always)]
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
} // end of method

/// 繰り返し二乗法
#[inline(always)]
pub fn mod_pow(mut a: i128, mut n: i128, m: i64) -> i64 {
    let modulo = m as i128;

    a %= modulo;
    let mut result = 1_i128;
    while n > 0 {
        if (n & 1) == 1 {
            result = (result * a) % modulo;
        }

        n >>= 1;
        a = (a * a) % modulo;
    }
    return result as i64;
} // end of method

/// Miller-Rabinの素数判定
#[inline(always)]
pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // n - 1 = 2^s * d に分解
    let mut s = 0_i64;
    let mut d = n - 1;
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }

    for base in MILLER_RABIN_BASES {
        // base が n の倍数なら、この基数では判定材料にならない
        let a = base % n;
        if a == 0 {
            continue;
        }

        // x = a^d mod n
        let mut x = mod_pow(a as i128, d as i128, n) as i128;

        // xが1 or -1ならこの基数では素数
        if x == 1 || x == (n - 1) as i128 {
            continue;
        }

        let mut ok = false;
        // x, x^2, x^4 ... を見て, どこかで-1になるかを確認
        for _ in 1..s {
            x = (x * x) % n as i128;
            if x == (n - 1) as i128 {
                ok = true;
                break;
            }
        }

        // どこでも-1にならなければ合成数
        if !ok {
            return false;
        }
    }

    return true;
} // end of method

/// Pollard Rho で使う関数
/// f(x) = x^2 + c mod n
#[inline(always)]
pub fn pollard_func(x: i128, c: i128, n: i128) -> i128 {
    return (x * x + c) % n;
} // end of method

/// Pollard Rho法
/// nが合成数のとき, 非自明な約数を1つ返す
#[inline(always)]
pub fn pollard(n: i64) -> i64 {
    if n % 2 == 0 {
        return 2;
    }

    // 素数なら自身
    if is_prime(n) {
        return n;
    }

    // cを変えながら試す
    let modulo = n as i128;
    let mut c = 1_i128;
    loop {
        // Floydの循環検出
        let mut x = 2_i128;
        let mut y = 2_i128;
        let mut d = 1_i64;

        while d == 1 {
            x = pollard_func(x, c, modulo);
            y = pollard_func(pollard_func(y, c, modulo), c, modulo);

            // |x - y| を取る
            let diff = if x >= y { x - y } else { y - x };

            // gcd(|x - y|, n) を見る
            d = gcd((diff % modulo) as i64, n);
        }

        // n以外の約数が見つかった
        if d != n {
            return d;
        }

        // 失敗したので別の多項式
        c += 1;
    }
} // end of method

/// 素因数分解の再帰部分
#[inline(always)]
pub fn prime_factorize_inner(n: i64, result: &mut Vec<i64>) {
    if n == 1 {
        return;
    }

    // 素数なら追加
    if is_prime(n) {
        result.push(n);
        return;
    }

    // 約数を1つ
    let p = pollard(n);

    // p側とn/p側それぞれ分解
    prime_factorize_inner(p, result);
    prime_factorize_inner(n / p, result);
} // end of method

/// Pollard Rho による素因数分解
#[inline(always)]
pub fn prime_factorize(n: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    if n <= 1 {
        return result;
    }

    prime_factorize_inner(n, &mut result);

    // 昇順に
    result.sort_unstable();
    return result;
} // end of method
