// エラトステネスの篩
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
//                                  エラトステネスの篩
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/eratos.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/eratos.rs


/// n 以下の素数を昇順で返す///
/// 奇数のみ保持・ビット詰め・区間篩による実装。
/// n < 2（負の数を含む）のときは空の Vec を返す。
///
/// 計算量: O(N log log N)
/// メモリ: 篩本体は O(√N)（区間 32 KiB 固定）+ 結果の Vec
pub fn eratos(n: i64) -> Vec<i64> {
    returnif!(n < 2, vec![]);
    let n = n as usize;
    let mut v = Vec::with_capacity(eratos_core::capacity(n));
    eratos_core::for_each_prime(n, |p| v.push(p as i64));
    return v;
}

/// n 以下の素数を昇順で返す（usize版）。
/// 中身はeratosと同じ
pub fn eratosusize(n: usize) -> Vec<usize> {
    returnif!(n < 2, vec![]);
    let mut v = Vec::with_capacity(eratos_core::capacity(n));
    eratos_core::for_each_prime(n, |p| v.push(p));
    return v;
}

/// eratos / eratosusize の内部実装。
/// 他の関数名と衝突しないようモジュールに閉じ込めている。
mod eratos_core {
    /// 1 区間で保持する奇数の個数（= ビット数）。32 KiB。
    const SEG_BITS: usize = 1 << 18;
    const SEG_WORDS: usize = SEG_BITS / 64;

    /// floor(sqrt(n))
    fn isqrt(n: usize) -> usize {
        let mut r = (n as f64).sqrt() as usize;
        while r.saturating_mul(r) > n {
            r -= 1;
        }
        while (r + 1).saturating_mul(r + 1) <= n {
            r += 1;
        }
        return r;
    }

    /// π(n) の上界（Dusart: π(x) < 1.25506 x / ln x）。Vec の事前確保用。
    pub fn capacity(n: usize) -> usize {
        returnif!(n < 2, 0);
        let x = n as f64;
        (x / x.ln() * 1.25506) as usize + 2
    }

    /// r 以下の奇素数（2 を除く）。区間篩の基底素数用。
    fn base_odd_primes(r: usize) -> Vec<usize> {
        returnif!(r < 3, vec![]);
        // 添字 i は整数 2i+1 に対応
        let m = (r + 1) >> 1;
        let mut s = vec![true; m];
        s[0] = false;
        let mut i = 1;
        loop {
            let p = (i << 1) + 1;
            if p > r / p {
                break;
            }
            if s[i] {
                s[(p * p) >> 1..].iter_mut().step_by(p).for_each(|x| *x = false);
            }
            i += 1;
        }
        (1..m).filter(|&i| s[i]).map(|i| (i << 1) + 1).collect()
    }

    /// n 以下の素数を昇順に f へ渡す。
    pub fn for_each_prime<F: FnMut(usize)>(n: usize, mut f: F) {
        returnif!(n < 2);
        f(2);

        // 添字 i は整数 2i+1 に対応。m は n 以下の奇数の個数。
        let m = (n + 1) >> 1;
        let odd_base = base_odd_primes(isqrt(n));
        // 各奇素数 p について「次に消す添字」。p*p から開始。
        let mut next: Vec<usize> = odd_base.iter().map(|&p| (p * p) >> 1).collect();

        let mut seg = vec![0u64; SEG_WORDS];
        let mut low = 0;
        while low < m {
            let high = (low + SEG_BITS).min(m);
            let len = high - low;
            let words = (len + 63) >> 6;
            let s = &mut seg[..words];

            // 全ビットを素数候補に。最後のワードは len を超えるビットを 0 に。
            s.fill(!0);
            let rem = len & 63;
            if rem != 0 {
                s[words - 1] = (1u64 << rem) - 1;
            }
            if low == 0 {
                s[0] &= !1; // 1 は素数ではない
            }

            // 合成数のビットを消す
            for (nx, &p) in next.iter_mut().zip(&odd_base) {
                let mut j = *nx;
                while j < high {
                    let o = j - low;
                    s[o >> 6] &= !(1u64 << (o & 63));
                    j += p;
                }
                *nx = j;
            }

            // 立っているビット（素数）だけを拾う
            for (wi, &word) in s.iter().enumerate() {
                let mut w = word;
                let base_idx = low + (wi << 6);
                while w != 0 {
                    let t = w.trailing_zeros() as usize;
                    f(((base_idx + t) << 1) + 1);
                    w &= w - 1; // 最下位の 1 を消す
                }
            }
            low = high;
        }
    }
}
