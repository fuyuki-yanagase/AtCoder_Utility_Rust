// permutation_combinationの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::zero_repeat_side_effects)]

use num_rational::Ratio;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::f64::consts::{PI, TAU};
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

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

use utility_rust::permutation_combination::*;

/// 動作確認用
fn example() {
    const M998: i64 = 998_244_353_i64;
    const M107: i64 = 1_000_000_007_i64;

    assert_eq!(mod_pow(2, 10, M107), 1024);
    assert_eq!(mod_pow(-2, 3, M107), 999_999_999);

    assert_eq!(nCk_direct(5, 3, M998), Some(10));
    assert_eq!(nPk_direct(5, 3, M998), 60);

    assert_eq!(nCk_direct(5, 0, M998), Some(1));
    assert_eq!(nCk_direct(5, 5, M998), Some(1));
    assert_eq!(nCk_direct(5, 6, M998), Some(0));

    assert_eq!(nPk_direct(5, 0, M998), 1);
    assert_eq!(nPk_direct(5, 5, M998), 120);
    assert_eq!(nPk_direct(5, 6, M998), 0);

    let inv = enum_inv(5, 7);
    assert_eq!(inv[1], 1);
    assert_eq!(inv[2], 4);
    assert_eq!(inv[3], 5);
    assert_eq!(inv[4], 2);
    assert_eq!(inv[5], 3);

    for i in 1..=5 {
        assert_eq!(mod_mul(i as i64, inv[i], 7), 1);
    }

    let fact = enum_fact(7, M998);
    assert_eq!(fact[0], 1);
    assert_eq!(fact[1], 1);
    assert_eq!(fact[2], 2);
    assert_eq!(fact[3], 6);
    assert_eq!(fact[4], 24);
    assert_eq!(fact[5], 120);

    let comb = ModCombination::new(100, M998);

    assert_eq!(comb.nCk(5, 3), 10);
    assert_eq!(comb.nCk(5, 0), 1);
    assert_eq!(comb.nCk(5, 5), 1);
    assert_eq!(comb.nCk(5, 6), 0);

    assert_eq!(comb.nPk(5, 3), 60);
    assert_eq!(comb.nPk(5, 0), 1);
    assert_eq!(comb.nPk(5, 5), 120);
    assert_eq!(comb.nPk(5, 6), 0);

    assert_eq!(comb.nCk(10, 2), 45);
    assert_eq!(comb.nPk(10, 2), 90);

    for n in 0..=20 {
        for k in 0..=22 {
            assert_eq!(comb.nCk(n, k), nCk_direct(n as i64, k as i64, M998).unwrap());
            assert_eq!(comb.nPk(n, k), nPk_direct(n as i64, k as i64, M998));
        }
    }

    for i in 0..=20 {
        let value = mod_mul(comb.factorial(i), comb.inverse_factorial(i), M998);
        assert_eq!(value, 1);
    }
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
