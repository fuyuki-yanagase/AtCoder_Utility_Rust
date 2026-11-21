// 遅延セグメント木
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
//                                   遅延セグメント木
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/lazy_segment_tree.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/lazy_segment_tree.rs

// LazySegmentTree

/// 遅延セグメント木に乗せる演算を表すtrait
///
/// Value :
///  - セグメント木のノードが持つ値
///  - min, max, sum, SumCount, 行列, 独自struct, ...
///
/// Lazy :
///  - 遅延評価で積む写像
///  - 加算なら数値
///  - 代入なら Option<T>
pub trait LazySegmentTreeOperator {
    /// セグメント木のノードが持つ値
    type Value: Copy;

    /// 遅延評価で積む写像
    /// 恒等写像かの比較用に PartialEq が必要
    type Lazy: Copy + PartialEq;

    /// Value側の単位元
    fn identity() -> Self::Value;

    /// Lazy側の恒等写像
    fn lazy_identity() -> Self::Lazy;

    /// 2つのノードを結合する
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value;

    /// 写像fをノードxに作用させる
    /// f(x) を返す
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value;

    /// 新しい写像 new_f を既存の写像 current_fに合成
    /// new_f ∘ current_f
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy;
} // end of trait

/// 遅延評価セグメント木
///
/// - 区間更新 update O(logN)
/// - 区間取得 query O(logN)
///
/// 外から渡すindexは 0-index
/// update/queryは半開区間 [left, right)
pub struct LazySegmentTree<Op>
where
    Op: LazySegmentTreeOperator,
{
    /// 元配列の長さ
    original_len: usize,

    /// 葉の数, 2のべき乗に拡張
    leaf_len: usize,

    /// セグメント木本体, root -> node[1]
    node: Vec<Op::Value>,

    /// 遅延評価を重ねる配列
    lazy: Vec<Op::Lazy>,

    /// Opを型として持つためのマーカー
    _marker: PhantomData<Op>,
} // end of struct

impl<Op> LazySegmentTree<Op>
where
    Op: LazySegmentTreeOperator,
{
    // ======================================================================================
    //
    //                                        構築・基本参照
    //
    // ======================================================================================

    /// 配列から遅延セグメント木を構築
    pub fn new(values: &[Op::Value]) -> Self {
        let original_len: usize = values.len();

        // 葉の数を2のべき乗に拡張
        let mut leaf_len: usize = 1_usize;
        while leaf_len < original_len {
            leaf_len <<= 1;
        }

        // nodeを単位元で初期化
        let mut node: Vec<Op::Value> = vec![Op::identity(); leaf_len << 1];

        // lazyを恒等写像で初期化
        let lazy: Vec<Op::Lazy> = vec![Op::lazy_identity(); leaf_len << 1];

        // 元配列を葉に
        node[leaf_len..(leaf_len + original_len)].copy_from_slice(values);

        // 下から親を構築
        for i in (1..leaf_len).rev() {
            node[i] = Op::operate(node[i << 1], node[(i << 1) | 1]);
        }

        return Self {
            original_len,
            leaf_len,
            node,
            lazy,
            _marker: PhantomData,
        };
    } // end of constructor

    /// 長さsizeの遅延セグメント木を単位元で構築
    /// すべての要素を Op::identity() で初期化する
    pub fn new_empty(size: usize) -> Self {
        let values: Vec<Op::Value> = vec![Op::identity(); size];
        return Self::new(&values);
    } // end of constructor

    /// 元配列の長さを返す
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.original_len;
    } // end of method

    /// 空かどうかを返す
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.original_len == 0;
    } // end of method

    /// 全体の値を返す
    /// apply_to_node方式では root である node[1] は常に最新値を持つ
    #[inline(always)]
    pub fn all(&self) -> Op::Value {
        return self.node[1];
    } // end of method

    // ======================================================================================
    //
    //                                       遅延セグ木メソッド
    //
    // ======================================================================================

    /// ノードkに写像fを作用
    /// この関数を呼んだらnode[k]は最新に
    /// lazy[k]には、まだ子へ降ろしていない写像fを合成
    #[inline(always)]
    fn apply_to_node(&mut self, k: usize, node_left: usize, node_right: usize, f: Op::Lazy) {
        // 現在のノードが持つ集約された値に写像fを反映
        self.node[k] = Op::mapping(f, self.node[k]);

        // 葉でなければ、子へ後から伝播させるので lazy[k] に写像を積んでおく
        if node_right - node_left > 1 {
            self.lazy[k] = Op::composition(f, self.lazy[k]);
        }
    } // end of method

    /// ノード k に積まれている遅延写像を子に降ろす
    /// node[k]自体には、すでにlazy[k]が反映済み
    ///  → node[k] の更新不要
    #[inline(always)]
    fn push(&mut self, k: usize, node_left: usize, node_right: usize) {
        // 遅延写像が恒等写像なら何もしない
        if self.lazy[k] == Op::lazy_identity() {
            return;
        }

        // 葉なら子が存在しないので何もしない
        if node_right - node_left == 1 {
            self.lazy[k] = Op::lazy_identity();
            return;
        }

        // 現在のノードに残っている遅延写像
        let f: Op::Lazy = self.lazy[k];

        // 左右の子に伝播
        let mid: usize = (node_left + node_right) >> 1;
        self.apply_to_node(k << 1, node_left, mid, f);
        self.apply_to_node((k << 1) | 1, mid, node_right, f);

        // 伝播済みなので自身のLazyを削除
        self.lazy[k] = Op::lazy_identity();
    } // end of method

    /// 子の値から親の値を作り直す
    #[inline(always)]
    fn pull(&mut self, k: usize) {
        self.node[k] = Op::operate(self.node[k << 1], self.node[(k << 1) | 1]);
    } // end of method

    /// 半開区間 [left, right) に写像fを作用
    #[inline(always)]
    pub fn update(&mut self, left: usize, right: usize, f: Op::Lazy) {
        debug_assert!(left <= right, "left must be less than or equal to right");
        debug_assert!(right <= self.original_len, "right is out of range");

        if left >= right {
            return;
        }

        self.update_internal(left, right, 1, 0, self.leaf_len, f);
    } // end of method

    /// updateの内部再帰, private
    /// update_left, update_right : 更新したい半開区間
    /// k : 現在のノード番号
    /// node_left, node_right : ノードkが担当する半開区間
    #[inline(always)]
    fn update_internal(
        &mut self,
        update_left: usize,
        update_right: usize,
        k: usize,
        node_left: usize,
        node_right: usize,
        f: Op::Lazy,
    ) {
        // 範囲外なら何もしない
        // apply to push と push/pull に分離したので先に評価可能
        if update_right <= node_left || node_right <= update_left {
            return;
        }

        // 完全に含まれるなら、自身に写像を直接作用
        if update_left <= node_left && node_right <= update_right {
            self.apply_to_node(k, node_left, node_right, f);
            return;
        }

        // 一部だけ重なる場合 → 遅延写像を子へ降ろす
        // 子に降りる前にpushして、子のノードが最新であることを保障
        self.push(k, node_left, node_right);

        // 左右の子に再帰
        let mid: usize = (node_left + node_right) >> 1;
        self.update_internal(update_left, update_right, k << 1, node_left, mid, f);
        self.update_internal(update_left, update_right, (k << 1) | 1, mid, node_right, f);

        // 子の更新が終わったので親を再構築
        self.pull(k);
    } // end of method

    /// 半開区間 [left, right) の値を取得する
    #[inline(always)]
    pub fn query(&mut self, left: usize, right: usize) -> Op::Value {
        debug_assert!(left <= right, "left must be less than or equal to right");
        debug_assert!(right <= self.original_len, "right is out of range");

        if left >= right {
            return Op::identity();
        }

        return self.query_internal(left, right, 1, 0, self.leaf_len);
    } // end of method

    /// queryの内部再帰, private
    #[inline(always)]
    fn query_internal(
        &mut self,
        query_left: usize,
        query_right: usize,
        k: usize,
        node_left: usize,
        node_right: usize,
    ) -> Op::Value {
        // 範囲外なら単位元を返す
        if query_right <= node_left || node_right <= query_left {
            return Op::identity();
        }

        // 完全に含まれるなら現在のノードの値を返す
        // node[k]は常に最新のはずなのでpush不要
        if query_left <= node_left && node_right <= query_right {
            return self.node[k];
        }

        // 一部だけ重なるなら遅延写像を降ろして子に再帰
        self.push(k, node_left, node_right);

        // 左右の子から取得
        let mid: usize = (node_left + node_right) >> 1;
        let left_value: Op::Value = self.query_internal(query_left, query_right, k << 1, node_left, mid);
        let right_value: Op::Value = self.query_internal(query_left, query_right, (k << 1) | 1, mid, node_right);

        // 取得した値を結合して返す
        return Op::operate(left_value, right_value);
    } // end of method

    /// 葉の値をすべて取得する (2のべき乗に拡張した部分は含まない)
    /// 遅延写像をすべて葉まで降ろすので O(N)
    pub fn to_vec(&mut self) -> Vec<Op::Value> {
        self.push_all(1, 0, self.leaf_len);
        return self.node[self.leaf_len..(self.leaf_len + self.original_len)].to_vec();
    } // end of method

    /// to_vecの内部再帰, private
    /// ノードkから下のすべての遅延写像を葉まで降ろす
    fn push_all(&mut self, k: usize, node_left: usize, node_right: usize) {
        // 葉に着いたら終了
        if node_right - node_left == 1 {
            return;
        }

        // 拡張部分だけのノードは見る必要がない
        if self.original_len <= node_left {
            return;
        }

        self.push(k, node_left, node_right);

        let mid: usize = (node_left + node_right) >> 1;
        self.push_all(k << 1, node_left, mid);
        self.push_all((k << 1) | 1, mid, node_right);
    } // end of method
} // end of impl LazySegmentTree

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                  Min/Max/Sum Traits
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/lazy_segment_tree.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/lazy_segment_tree.rs

/// min/max の単位元を返すtrait
/// Rust標準には 整数/浮動小数 まとめて最大/最小を返す共通traitが無いので自作
pub trait MinMaxValue: Copy + PartialOrd {
    /// 区間最小の単位元
    /// min(x, INF) = x になる値
    fn min_identity() -> Self;

    /// 区間最大の単位元
    /// max(x, -INF) = x になる値
    fn max_identity() -> Self;
} // end of trait

/// sum のゼロを返すtrait
/// Rust標準のDefaultでも可能, 意図を明確にするために min/max 同様に自作
pub trait ZeroValue: Copy + Add<Output = Self> {
    /// 加算の単位元
    /// x + 0 = x になる
    fn zero() -> Self;
} // end of trait

/// 整数型に MinMaxValue を実装するマクロ
macro_rules! impl_min_max_value_for_integer {
	($($t:ty), +) => {
		$(
			impl MinMaxValue for $t{
				#[inline(always)]
				fn min_identity() -> Self {
					return <$t>::MAX;
				}

				#[inline(always)]
				fn max_identity() -> Self {
					return <$t>::MIN;
				}
			}
		)+
	};
} // end of macro

/// 浮動小数型に MinMaxValue を実装するマクロ
macro_rules! impl_min_max_value_for_float {
	($($t: ty), +) => {
		$(
			impl MinMaxValue for $t{
				#[inline(always)]
				fn min_identity() -> Self{
					return <$t>::INFINITY;
				}

				#[inline(always)]
				fn max_identity() -> Self {
					return <$t>::NEG_INFINITY;
				}
			}
		)+
	};
} // end of macro

/// 数値型に ZeroValue を実装するマクロ
macro_rules! impl_zero_value {
	($($t:ty), +) => {
		$(
			impl ZeroValue for $t {
				#[inline(always)]
				fn zero() -> Self {
					return 0 as $t;
				}
			}
		)+
	};
} // end of macro

/// 整数型/浮動小数型 にmin/max/zeroを実装
impl_min_max_value_for_integer!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_min_max_value_for_float!(f32, f64);
impl_zero_value!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                               Lazy Segment Tree Traits
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/lazy_segment_tree.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/lazy_segment_tree.rs

/// 区間最大値と、その最大値が存在する index を持つ
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MaxIndex<T>
where
    T: Copy,
{
    /// 最大値
    pub value: T,

    /// 最大値が存在する index
    pub index: usize,

    /// ノードが担当する区間の左端 (代入用)
    pub left: usize,
}

impl<T> MaxIndex<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn new(value: T, index: usize) -> Self {
        return Self {
            value,
            index,
            left: index,
        };
    } // end of method
}

/// 区間最小値と、その最小値が存在する index を持つ
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MinIndex<T>
where
    T: Copy,
{
    /// 最小値
    pub value: T,

    /// 最小値が存在するindex
    pub index: usize,

    /// ノードが担当する区間の左端 (代入用)
    pub left: usize,
}

impl<T> MinIndex<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn new(value: T, index: usize) -> Self {
        return Self {
            value,
            index,
            left: index,
        };
    } // end of method
}

/// 区間最小 + 区間加算
/// Value = T
/// Lazy = T
///
/// update(l, r, x)
///  - [l, r) に x を加算
///
/// query(l, r):
///  - [l, r) の最小値
pub struct RangeAddMin<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAddMin<T>
where
    T: MinMaxValue + ZeroValue + Add<Output = T> + PartialEq,
{
    type Value = T;
    type Lazy = T;

    #[inline(always)]
    fn identity() -> Self::Value {
        return T::min_identity();
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return T::zero();
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        if left <= right {
            return left;
        } else {
            return right;
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        return x + f;
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        return new_f + current_f;
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAddMin

/// 区間最小 + 区間代入
///
/// Value = T
/// Lazy = Option<T>
///
/// update(l, r, Some(x)):
///  - [l, r) を x に代入
///
/// query(l, r):
///  - [l, r) の最小値
pub struct RangeAssignMin<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAssignMin<T>
where
    T: MinMaxValue,
{
    type Value = T;
    type Lazy = Option<T>;

    #[inline(always)]
    fn identity() -> Self::Value {
        return T::min_identity();
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return None;
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        if left <= right {
            return left;
        } else {
            return right;
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        match f {
            Some(value) => return value,
            None => return x,
        }
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        match new_f {
            Some(_) => {
                return new_f;
            }
            None => {
                return current_f;
            }
        }
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAssignMin

/// 区間最大 + 区間加算
/// Value = T
/// Lazy = T
///
/// update(l, r, x)
///  - [l, r) に x を加算
///
/// query(l, r):
///  - [l, r) の最大値
pub struct RangeAddMax<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAddMax<T>
where
    T: MinMaxValue + ZeroValue + Add<Output = T> + PartialEq,
{
    type Value = T;
    type Lazy = T;

    #[inline(always)]
    fn identity() -> Self::Value {
        return T::max_identity();
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return T::zero();
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        if left >= right {
            return left;
        } else {
            return right;
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        return x + f;
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        return new_f + current_f;
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAddMax

/// 区間最大 + 区間代入
///
/// Value = T
/// Lazy = Option<T>
///
/// update(l, r, Some(x)):
///  - [l, r) を x に代入
///
/// query(l, r):
///  - [l, r) の最大値
pub struct RangeAssignMax<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAssignMax<T>
where
    T: MinMaxValue,
{
    type Value = T;
    type Lazy = Option<T>;

    #[inline(always)]
    fn identity() -> Self::Value {
        return T::max_identity();
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return None;
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        if left >= right {
            return left;
        } else {
            return right;
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        match f {
            Some(value) => return value,
            None => return x,
        }
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        match new_f {
            Some(_) => {
                return new_f;
            }
            None => {
                return current_f;
            }
        }
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAssignMax

/// 区間最小(index付き) + 区間代入
///
/// value = MinIndex<T>
/// Lazy = Option<T>
///
/// update(l, r, Some(x)):
/// - [l, r) をxに代入
///
/// query(l, r):
/// - [l, r)の最小値とそのindex(同値なら最も左)
pub struct RangeAssignMinIndex<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAssignMinIndex<T>
where
    T: MinMaxValue,
{
    type Value = MinIndex<T>;
    type Lazy = Option<T>;

    #[inline(always)]
    fn identity() -> Self::Value {
        return MinIndex {
            value: T::min_identity(),
            index: usize::MAX,
            left: usize::MAX,
        };
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return None;
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        let node_left: usize = if left.left <= right.left { left.left } else { right.left };

        // 最小値が同じなら左側の index を優先
        if left.value < right.value || (left.value == right.value && left.index <= right.index) {
            return MinIndex {
                value: left.value,
                index: left.index,
                left: node_left,
            };
        } else {
            return MinIndex {
                value: right.value,
                index: right.index,
                left: node_left,
            };
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        match f {
            Some(value) => {
                // 単位元(パディング)には作用させない
                if x.left == usize::MAX {
                    return x;
                }
                // 区間全体が value になるので、index は区間の左端
                return MinIndex {
                    value,
                    index: x.left,
                    left: x.left,
                };
            }
            None => {
                return x;
            }
        }
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        match new_f {
            Some(_) => {
                return new_f;
            }
            None => {
                return current_f;
            }
        }
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAssignMinIndex

/// 区間最大(index付き) + 区間代入
///
/// Value = MaxIndex<T>
/// Lazy = Option<T>
///
/// update(l, r, Some(x)):
///  - [l, r) を x に代入
///
/// query(l, r):
///  - [l, r) の最大値と、その index (同値なら最も左)
pub struct RangeAssignMaxIndex<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of struct

impl<T> LazySegmentTreeOperator for RangeAssignMaxIndex<T>
where
    T: MinMaxValue,
{
    type Value = MaxIndex<T>;
    type Lazy = Option<T>;

    #[inline(always)]
    fn identity() -> Self::Value {
        return MaxIndex {
            value: T::max_identity(),
            index: usize::MAX,
            left: usize::MAX,
        };
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return None;
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        let node_left: usize = if left.left <= right.left { left.left } else { right.left };

        // 最大値が同じなら左側の index を優先
        if left.value > right.value || (left.value == right.value && left.index <= right.index) {
            return MaxIndex {
                value: left.value,
                index: left.index,
                left: node_left,
            };
        } else {
            return MaxIndex {
                value: right.value,
                index: right.index,
                left: node_left,
            };
        }
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        match f {
            Some(value) => {
                if x.left == usize::MAX {
                    return x;
                }
                return MaxIndex {
                    value,
                    index: x.left,
                    left: x.left,
                };
            }
            None => {
                return x;
            }
        }
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        match new_f {
            Some(_) => {
                return new_f;
            }
            None => {
                return current_f;
            }
        }
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAssignMaxIndex

/// 区間和用のノード
///
/// sum :
///  - 区間和
///
/// count:
///  - 区間に含まれる要素数
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SumCount<T>
where
    T: Copy,
{
    pub sum: T,
    pub count: usize,
} // end of struct

impl<T> SumCount<T>
where
    T: Copy,
{
    #[inline(always)]
    pub fn new(sum: T, count: usize) -> Self {
        return Self {
            sum,
            count,
        };
    } // end of method
} // end of impl SumCount

/// 値をusize倍するtrait
///
/// 区間和 + 区間加算において
///   -- 加算値 x 区間の長さ --
/// を計算するために使う
pub trait MulUsize: Copy {
    fn mul_usize(self, rhs: usize) -> Self;
} // end of trait

macro_rules! impl_mul_usize {
	($($t: ty), +) => {
		$(
			impl MulUsize for $t{
				#[inline(always)]
				fn mul_usize(self, rhs:  usize)->Self{
					return self * rhs as $t;
				}
			}
		)+
	};
} // end of macro

impl_mul_usize!(i8, i16, i32, i64, i128, isize);
impl_mul_usize!(u8, u16, u32, u64, u128, usize);
impl_mul_usize!(f32, f64);

/// 区間和 + 区間加算
///
/// Value = SumCount<T>
/// Lazy = T
///
/// update(l, r, x):
///  - [l, r) に xを加算
///
/// query(l, r).sum:
///  - [l, r) の区間和
pub struct RangeAddSum<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of constructor

impl<T> LazySegmentTreeOperator for RangeAddSum<T>
where
    T: ZeroValue + Add<Output = T> + MulUsize + PartialEq,
{
    type Value = SumCount<T>;
    type Lazy = T;

    #[inline(always)]
    fn identity() -> Self::Value {
        return SumCount::new(T::zero(), 0);
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return T::zero();
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        return SumCount::new(left.sum + right.sum, left.count + right.count);
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        return SumCount::new(x.sum + f.mul_usize(x.count), x.count);
    }

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        return new_f + current_f;
    } // end of method
} // end of impl LazySegmentTreeOperator for RangeAddSum

/// 区間和 + 区間代入
///
/// Value = SumCount<T>
/// Lazy = Option<T>
///
/// update(l, r, Some(x))
///  - [l, r) を x に代入
///
/// query(l, r).sum
///  - [l, r) の区間和
pub struct RangeAssignSum<T>
where
    T: Copy,
{
    _marker: PhantomData<T>,
} // end of constructor

impl<T> LazySegmentTreeOperator for RangeAssignSum<T>
where
    T: ZeroValue + Add<Output = T> + MulUsize + PartialEq,
{
    type Value = SumCount<T>;
    type Lazy = Option<T>;

    #[inline(always)]
    fn identity() -> Self::Value {
        return SumCount::new(T::zero(), 0);
    } // end of method

    #[inline(always)]
    fn lazy_identity() -> Self::Lazy {
        return None;
    } // end of method

    #[inline(always)]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        return SumCount::new(left.sum + right.sum, left.count + right.count);
    } // end of method

    #[inline(always)]
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value {
        match f {
            Some(value) => {
                return SumCount::new(value.mul_usize(x.count), x.count);
            }
            None => {
                return x;
            }
        }
    } // end of method

    #[inline(always)]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        match new_f {
            Some(_) => {
                return new_f;
            }
            None => {
                return current_f;
            }
        }
    } // end of method
}
