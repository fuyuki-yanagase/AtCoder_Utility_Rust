// Fenwick Tree
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
//                                     Fenwick Tree
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/fenwick_tree.rs

/// Fenwick Treeに乗せるために必要な性質
///
/// - Copy : Fenwick Tree内で値を何度も足す
/// - AddAssign : += で加算
/// - Sub : range_sumの計算に利用
pub trait FenwickValue: Copy + AddAssign<Self> + Sub<Output = Self> {}

/// 条件を満たす型には自動で FenwickValue を実装する
impl<T> FenwickValue for T where T: Copy + AddAssign<Self> + Sub<Output = Self> {}

/// Fenwick Tree
///
/// - 1点加算
/// - 先頭からの累積和
/// - 半開区間 [left, right) の区間和
///
/// 外部から渡す index は 0-indexed
/// 内部では Fenwick Tree の都合で 1-indexed に変換する
///
/// 参考 : https://algo-logic.info/binary-indexed-tree/
pub struct FenwickTree<T>
where
    T: FenwickValue,
{
    size: usize,

    /// 加算の単位元
    identity: T,

    /// tree[i] → 特定の範囲の合計
    /// 1-indexed, 長さはsize + 1
    tree: Vec<T>,
} // end of struct

impl<T> FenwickTree<T>
where
    T: FenwickValue,
{
    /// 要素数 size のFenwickTreeを作成
    pub fn with_identity(size: usize, identity: T) -> Self {
        return Self {
            size,
            identity,
            tree: vec![identity; size + 1],
        };
    } // end of constructor

    /// index番目にvalueを加算 (0-indexed)
    #[inline(always)]
    pub fn add(&mut self, index: usize, value: T) {
        // 範囲外アクセスを明確に防ぐ(実行速度は落ちるがデバッグ用)
        debug_assert!(index < self.size);

        // 1-indexed
        let mut i = index + 1;

        // 範囲を増やしなら。 iの1が立っているbitの最小を足していく
        while i <= self.size {
            self.tree[i] += value;

            // i & -i と同義, usignedでの対応
            i += i & (!i + 1);
        }

        return;
    } // end of method

    /// [0, right) の合計を返す (right : 0-indexed)
    #[inline(always)]
    pub fn sum_prefix(&self, right: usize) -> T {
        // [0, right) なので right == size は許可する
        debug_assert!(right <= self.size);

        let mut i = right;
        let mut result = self.identity;

        // 関係する区間を集計
        while i > 0 {
            result += self.tree[i];

            // i & -i と同義。 iの1が立っている最小のbitを引いていく
            i -= i & (!i + 1);
        }

        return result;
    } // end of method

    /// [left, right) の合計を返す (left,right : 0-indexed)
    #[inline(always)]
    pub fn range_sum(&self, left: usize, right: usize) -> T {
        // 半開区間 [left, right) として正しい範囲か確認する
        debug_assert!(left <= right);
        debug_assert!(right <= self.size);

        return self.sum_prefix(right) - self.sum_prefix(left);
    } // end of method

    /// index番目の値を取得する (0-indexed)
    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        debug_assert!(index < self.size);

        return self.range_sum(index, index + 1);
    } // end of method
} // end of impl

impl<T> FenwickTree<T>
where
    T: FenwickValue + Default,
{
    /// Default::default() を単位元として Fenwick Tree を作成する
    ///
    /// i64 なら 0
    /// usize なら 0
    /// f64 なら 0.0
    pub fn new(size: usize) -> Self {
        return Self::with_identity(size, T::default());
    } // end of constructor
} // end of impl

// 0/1の存在管理に特化した拡張
impl FenwickTree<i32> {
    /// 0/1のバイト列からO(N)で構築
    pub fn from_binary_bytes(s: &[u8]) -> Self {
        let n = s.len();
        let mut ft = Self::new(n);

        // 各要素の値を設定
        for i in 1..=n {
            ft.tree[i] = (s[i - 1] == b'1') as i32;
        }

        // Fenwick Treeを線形時間で構築
        for i in 1..=n {
            let parent = i + (i & (!i + 1));

            if parent <= n {
                let value = ft.tree[i];
                ft.tree[parent] += value;
            }
        }

        return ft;
    }

    /// 0-indexedでk番目の1の位置を返す
    ///
    /// 計算量 O(log N)
    /// 各要素が非負であることが前提
    pub fn kth(&self, k: usize) -> Option<usize> {
        let total = self.sum_prefix(self.size) as usize;

        if k >= total {
            return None;
        }

        // 求める累積和（1-indexed）
        let mut target = (k + 1) as i32;
        let mut index = 0;

        // size以下の最大の2のべき乗
        let mut bit = 1usize;
        while bit <= self.size / 2 {
            bit <<= 1;
        }

        // Fenwick Treeの内部配列を使って探索
        while bit > 0 {
            let next = index + bit;

            if next <= self.size && self.tree[next] < target {
                target -= self.tree[next];
                index = next;
            }

            bit >>= 1;
        }

        return Some(index);
    }
}
