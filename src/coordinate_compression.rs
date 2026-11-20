// 座標圧縮
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
//                                       座標圧縮
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/coordinate_compression.rs

// 静的な座標圧縮
///
/// new / with_indices: O(N log N)
/// index / lower_bound / upper_bound / range: O(log M)
///
/// Mは異なる座標の個数。
/// 比較・移動・破棄をO(1)とした計算量。
///
/// 構築後に座標を追加する場合は再構築が必要。
#[derive(Debug, Clone)]
pub struct CoordinateCompression<T> {
    values: Vec<T>,
}

impl<T: Ord> CoordinateCompression<T> {
    /// 入力を借用し、構築時にだけ複製する。
    pub fn new(values: &[T]) -> Self
    where
        T: Clone,
    {
        let mut values = values.to_vec();
        values.sort_unstable();
        values.dedup();
        return Self {
            values,
        };
    }

    /// newと同じ処理。
    pub fn from_slice(values: &[T]) -> Self
    where
        T: Clone,
    {
        Self::new(values)
    }

    /// 座標表と、入力順の圧縮済み添字を同時に生成する。
    /// ソート後の走査で添字を確定するため、
    /// 各入力値を二分探索し直す必要がない。
    /// 入力値ではなく添字をソートする。
    pub fn with_indices(values: &[T]) -> (Self, Vec<usize>)
    where
        T: Clone,
    {
        let n = values.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by(|&left, &right| values[left].cmp(&values[right]));

        let mut unique = Vec::with_capacity(n);
        let mut indices = vec![0; n];
        for original_index in order {
            let value = &values[original_index];
            if unique.last() != Some(value) {
                unique.push(value.clone());
            }
            indices[original_index] = unique.len() - 1;
        }
        return (
            Self {
                values: unique,
            },
            indices,
        );
    }

    /// 登録済み座標の添字を返す
    /// 未登録ならNone
    #[inline(always)]
    pub fn index(&self, value: &T) -> Option<usize> {
        return self.values.binary_search(value).ok();
    }

    /// value以上の最初の位置
    /// 存在しなければlen
    #[inline(always)]
    pub fn lower_bound(&self, value: &T) -> usize {
        return self.values.partition_point(|current| current < value);
    }

    /// valueより大きい最初の位置
    /// 存在しなければlen
    #[inline(always)]
    pub fn upper_bound(&self, value: &T) -> usize {
        return self.values.partition_point(|current| current <= value);
    }

    /// 元の半開区間[left, right)を、
    /// 圧縮後の半開区間に変換する。
    ///
    /// 端点は登録されていなくてもよい。
    /// left > rightならpanic。
    #[inline(always)]
    pub fn range(&self, left: &T, right: &T) -> Range<usize> {
        assert!(left <= right, "区間の左端が右端を超えてるよー(´・ω・｀)");
        return self.lower_bound(left)..self.lower_bound(right);
    }
}

impl<T> CoordinateCompression<T> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.values.len();
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.values.is_empty();
    }

    /// 圧縮後の添字から元の座標を借用.\
    /// 範囲外ならNone
    pub fn get(&self, index: usize) -> Option<&T> {
        return self.values.get(index);
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        return &self.values;
    }

    /// ソート・重複除去済みの座標を複製
    #[inline(always)]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        return self.values.to_vec();
    }

    /// 内部のVecをそのまま帰す
    #[inline(always)]
    pub fn into_vec(self) -> Vec<T> {
        return self.values;
    }
}

impl<T> Index<usize> for CoordinateCompression<T> {
    type Output = T;

    /// 圧縮後の添字から元の座標を借用する。
    /// 範囲外ならpanic。
    #[inline(always)]
    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}
