// convolutionの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

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
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Range, Sub, SubAssign};

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

use num_complex::Complex64;
use utility_rust::convolution::*;
const NTT_MOD: u64 = 998244353;

fn example() {
    let mut a = vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0), Complex64::new(3.0, 0.0)];

    let values = fft(&mut a);
    let restored = ifft(&values);

    // aもゼロ埋めされているため、4要素同士で比較する。
    assert_eq!(restored.len(), a.len());

    for (j, value) in restored.iter().enumerate() {
        assert!((*value - a[j]).norm() < 1e-10);
        println!("a[{j}] = {value}");
    }

    // 2つの畳み込み
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0];
    let c = convolution_fft(&a, &b);
    let expected = [4.0, 13.0, 22.0, 15.0];
    assert_eq!(c.len(), expected.len());
    for (i, value) in c.iter().enumerate() {
        assert!((*value - expected[i]).abs() < 1e-9);
        println!("c[{i}] = {value}");
    }

    // ntt畳み込み
    let mut a = vec![1, 2, 3];
    let values = ntt(&mut a);
    assert_eq!(values, vec![6, 825_076_915, 2, 173_167_434],);
    println!("{values:?}");

    let mut a = vec![1, 2, 3];
    let values = ntt(&mut a);
    let restored = intt(&values);
    assert_eq!(restored, a);
    println!("{restored:?}");

    // ntt 2つ畳み込み
    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    let c = convolution_ntt(&a, &b);
    assert_eq!(c, vec![4, 13, 22, 15]);
    println!("{c:?}");
    // (p - 1) * 2 ≡ -2 ≡ p - 2 (mod p)
    assert_eq!(convolution_ntt(&[NTT_MOD - 1], &[2]), vec![NTT_MOD - 2],);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}

#[test]
fn empty_and_short_convolutions_match_definition() {
    for n in 0..9 {
        for m in 0..9 {
            let a: Vec<i64> = (0..n).map(|i| (i * 7 % 9) as i64 - 4).collect();
            let b: Vec<i64> = (0..m).map(|i| (i * 5 % 7) as i64 - 3).collect();
            let mut expected = vec![0; if n == 0 || m == 0 { 0 } else { n + m - 1 }];
            for i in 0..n {
                for j in 0..m {
                    expected[i + j] += a[i] * b[j];
                }
            }
            assert_eq!(convolution_naive_i64(&a, &b), expected);
            let real = convolution_fft(
                &a.iter().map(|&v| v as f64).collect::<Vec<_>>(),
                &b.iter().map(|&v| v as f64).collect::<Vec<_>>(),
            );
            assert_eq!(real.len(), expected.len());
            for (actual, &value) in real.iter().zip(&expected) {
                assert!((actual - value as f64).abs() < 1e-8);
            }
            let modular = convolution_ntt(
                &a.iter().map(|&v| v.rem_euclid(NTT_MOD as i64) as u64).collect::<Vec<_>>(),
                &b.iter().map(|&v| v.rem_euclid(NTT_MOD as i64) as u64).collect::<Vec<_>>(),
            );
            assert_eq!(
                modular,
                expected.iter().map(|v| v.rem_euclid(NTT_MOD as i64) as u64).collect::<Vec<_>>()
            );
        }
    }
}

// NTTや既存の畳み込みを使わず、各積をu128で計算する独立した検証基準。
// 入力を先にu32へ落とさないため、法以上やu64::MAXの正規化も検証できる。
fn naive_mod_oracle(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let modulus = NTT_MOD as u128;
    let mut result = vec![0_u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            let product = (x as u128 * y as u128) % modulus;
            result[i + j] = ((result[i + j] as u128 + product) % modulus) as u64;
        }
    }
    return result;
}

fn modular_input(len: usize, mut state: u64) -> Vec<u64> {
    return (0..len)
        .map(|i| {
            // 再現可能な入力。全幅の値と、mod正規化・符号変換の境界を交ぜる。
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            match i % 8 {
                0 => 0,
                1 => u64::MAX,
                2 => NTT_MOD,
                3 => NTT_MOD + 1,
                4 => NTT_MOD - 1,
                5 => (-(i as i64) - 1).rem_euclid(NTT_MOD as i64) as u64,
                6 => 2 * NTT_MOD,
                _ => state,
            }
        })
        .collect();
}

fn check_fast_against_oracle(a: &[u64], b: &[u64]) {
    let expected = naive_mod_oracle(a, b);
    let actual = convolution_ntt_fast(a, b);
    assert_eq!(actual, expected, "input lengths: {}, {}", a.len(), b.len());
    assert!(actual.iter().all(|&value| value < NTT_MOD));
}

#[test]
fn fast_ntt_empty_scalar_and_normalization() {
    let special = [0, 1, NTT_MOD - 1, NTT_MOD, NTT_MOD + 1, 2 * NTT_MOD, u64::MAX];
    check_fast_against_oracle(&[], &[]);
    check_fast_against_oracle(&special, &[]);
    check_fast_against_oracle(&[], &special);
    for &x in &special {
        for &y in &special {
            check_fast_against_oracle(&[x], &[y]);
        }
    }
}

#[test]
fn fast_ntt_naive_threshold_and_asymmetric_lengths() {
    // min(N, M) <= 16 の両向きと、17以上で変換へ切り替わる経路。
    for n in [1, 2, 15, 16, 17, 18, 31, 32, 33, 128, 129, 257] {
        for m in [1, 15, 16, 17, 19, 63, 128, 257] {
            check_fast_against_oracle(&modular_input(n, 0x1234), &modular_input(m, 0x5678));
        }
    }
}

#[test]
fn fast_ntt_power_of_two_boundaries() {
    // 結果長が2^kの直前・一致・直後。変換長64〜4096にわたり、
    // radix-4のみの偶数段とradix-2を含む奇数段、回転係数の更新を通す。
    for exponent in 5..=11 {
        let boundary = 1_usize << exponent;
        for result_len in [boundary - 1, boundary, boundary + 1] {
            let n = result_len.div_ceil(2);
            let m = result_len + 1 - n;
            check_fast_against_oracle(&modular_input(n, 0xabc), &modular_input(m, 0xdef));
        }
    }
}

#[test]
fn fast_ntt_zero_impulse_and_negative_coefficients() {
    let signed: Vec<i64> = (0..257).map(|i| i as i64 * 7919 - 1_000_000).collect();
    let a: Vec<u64> = signed.iter().map(|&x| x.rem_euclid(NTT_MOD as i64) as u64).collect();
    let b = modular_input(129, 0x9876);
    check_fast_against_oracle(&a, &b);
    check_fast_against_oracle(&vec![0; 257], &b);
    check_fast_against_oracle(&a, &vec![0; 129]);
    for position in [0, 64, 128] {
        let mut impulse = vec![0; 129];
        impulse[position] = u64::MAX;
        check_fast_against_oracle(&a, &impulse);
    }
}
