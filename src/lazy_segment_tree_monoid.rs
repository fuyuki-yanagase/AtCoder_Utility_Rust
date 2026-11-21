// 遅延セグメント木（モノイド）
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

pub use crate::lazy_segment_tree::{
    LazySegmentTreeOperator, MaxIndex, MinIndex, MinMaxValue, MulUsize, RangeAddMax, RangeAddMin, RangeAddSum,
    RangeAssignMax, RangeAssignMaxIndex, RangeAssignMin, RangeAssignMinIndex, RangeAssignSum, SumCount, ZeroValue,
};
use crate::modint::*;
use crate::monoid_group_abelian::*;

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                             遅延セグメント木（モノイド）
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/lazy_segment_tree_monoid.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/lazy_segment_tree_monoid.rs

/// モノイド版の遅延セグメント木。
///
/// M: 値の集約。combine(left, right)は配列の左右の順番。
/// F: 更新の合成。combine(new, old)はnew ∘ old。
/// Map: 更新を集約値へ作用させる関数。
///
/// 半開区間 [left, right)、0-index。
///
/// 構築・to_vec: O(N)
/// update/query/get/set: O(log N)
/// all: O(1)
/// 空間: O(N)
///
/// 各演算とCloneがO(1)の場合の計算量。
/// 空配列にも対応する。
pub struct LazySegmentTreeMonoid<M, F, Map>
where
    M: Monoid,
    F: Monoid,
{
    original_len: usize,
    leaf_len: usize,
    log: u32,

    node: Vec<M::S>,

    /// 内部ノードだけに保持, Noneは未伝播の更新が無い
    lazy: Vec<Option<F::S>>,

    value_monoid: M,
    update_monoid: F,
    mapping: Map,
} // end of struct LazySegmentTreeMonoid

impl<M, F, Map> LazySegmentTreeMonoid<M, F, Map>
where
    M: Monoid,
    F: Monoid,
    M::S: Clone,
    F::S: Clone,
    Map: Fn(&F::S, &mut M::S),
{
    pub fn new(values: &[M::S], value_monoid: M, update_monoid: F, mapping: Map) -> Self {
        let n = values.len();
        let size = n.max(1).checked_next_power_of_two().expect("配列が大きすぎるよー(´・ω・｀)");
        let capacity = size.checked_mul(2).expect("配列が大きすぎるよー(´・ω・｀)");
        let mut node = Vec::with_capacity(capacity);

        // 内部ノード
        node.extend((0..capacity).map(|_| value_monoid.identity()));
        node[size..size + n].clone_from_slice(values);

        // 葉から構築
        for k in (1..size).rev() {
            node[k] = value_monoid.combine(&node[k << 1], &node[k << 1 | 1]);
        }
        let lazy = (0..size).map(|_| None).collect();
        return Self {
            original_len: n,
            leaf_len: size,
            log: size.trailing_zeros(),
            node,
            lazy,
            value_monoid,
            update_monoid,
            mapping,
        };
    } // end of constructor

    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.original_len;
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.original_len == 0;
    }

    /// 全体の集約値を借用
    #[inline(always)]
    pub fn all(&self) -> &M::S {
        return &self.node[1];
    }

    /// 遅延更新を反映して葉を借用
    #[inline(always)]
    pub fn get(&mut self, index: usize) -> &M::S {
        assert!(index < self.original_len, "添字が範囲外だよー(´・ω・｀)");
        let k = self.leaf_len + index;
        for h in (1..=self.log).rev() {
            self.push(k >> h);
        }
        return &self.node[k];
    }

    /// 1要素を置き換え
    pub fn set(&mut self, index: usize, value: M::S) {
        assert!(index < self.original_len, "添字が範囲外だよー(´・ω・｀)");
        let k = self.leaf_len + index;
        for h in (1..=self.log).rev() {
            self.push(k >> h);
        }
        self.node[k] = value;
        for h in 1..=self.log {
            self.pull(k >> h);
        }
    }

    /// 半開区間 [left, right) に更新fを作用
    #[inline(always)]
    pub fn update(&mut self, left: usize, right: usize, f: F::S) {
        assert!(left <= right && right <= self.original_len, "区間範囲外だよー(´・ω・｀)");
        returnif!(left >= right);

        let mut l = left + self.leaf_len;
        let mut r = right + self.leaf_len;
        self.push_boundaries(l, r);

        let (l0, r0) = (l, r);
        while l < r {
            if (l & 1) != 0 {
                self.apply(l, &f);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                self.apply(r, &f);
            }
            l >>= 1;
            r >>= 1;
        }

        // 完全被覆したノードをpullすると遅延更新が消える
        // 境界をまたぐ祖先だけ再構築
        for h in l0.trailing_zeros() + 1..=self.log {
            self.pull(l0 >> h);
        }
        for h in r0.trailing_zeros() + 1..=self.log {
            self.pull((r0 - 1) >> h);
        }
    }

    /// 半開区間 [left, right) の集約値を生成
    #[inline(always)]
    pub fn query(&mut self, left: usize, right: usize) -> M::S {
        assert!(left <= right && right <= self.original_len, "区間範囲外だよー(´・ω・｀)");
        returnif!(left >= right, self.value_monoid.identity());

        let mut l = left + self.leaf_len;
        let mut r = right + self.leaf_len;
        self.push_boundaries(l, r);

        let mut a = self.value_monoid.identity();
        let mut b = self.value_monoid.identity();

        while l < r {
            if (l & 1) != 0 {
                a = self.value_monoid.combine(&a, &self.node[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                b = self.value_monoid.combine(&self.node[r], &b);
            }
            l >>= 1;
            r >>= 1;
        }

        // 非可換演算でも左右の順序を維持
        return self.value_monoid.combine(&a, &b);
    }

    /// 全葉に伝播し実要素のスライスを借用
    #[inline(always)]
    pub fn as_slice(&mut self) -> &[M::S] {
        self.push_all();
        return &self.node[self.leaf_len..self.leaf_len + self.original_len];
    }

    /// 実要素を複製して返す
    #[inline(always)]
    pub fn to_vec(&mut self) -> Vec<M::S>
    where
        M::S: Clone,
    {
        return self.as_slice().to_vec();
    }

    /// 木を消費し、実要素を所有権ごと返す
    pub fn into_vec(mut self) -> Vec<M::S> {
        self.push_all();
        // 内部ノード領域を除去、葉を先頭へ移動
        drop(self.node.drain(..self.leaf_len));

        // 2冪への拡張で追加した葉を削除
        self.node.truncate(self.original_len);

        return self.node;
    }

    /// 下から引っ張って現在のノード対応
    #[inline(always)]
    fn pull(&mut self, k: usize) {
        self.node[k] = self.value_monoid.combine(&self.node[k << 1], &self.node[k << 1 | 1]);
    }

    /// 更新を借用して作用
    #[inline(always)]
    fn apply(&mut self, k: usize, f: &F::S) {
        (self.mapping)(f, &mut self.node[k]);
        if k < self.leaf_len {
            let next = match &self.lazy[k] {
                Some(old) => self.update_monoid.combine(f, old),
                // Cloneの代わりにモノイド演算で所有値を生成
                None => self.update_monoid.combine(f, &self.update_monoid.identity()),
            };
            self.lazy[k] = Some(next);
        }
    }

    /// 更新の所有権を受け取って作用
    #[inline(always)]
    fn apply_owned(&mut self, k: usize, f: F::S) {
        (self.mapping)(&f, &mut self.node[k]);
        if k < self.leaf_len {
            self.lazy[k] = Some(match self.lazy[k].take() {
                Some(old) => self.update_monoid.combine(&f, &old),
                None => f,
            });
        }
    }

    /// 要素追加
    #[inline(always)]
    fn push(&mut self, k: usize) {
        if let Some(f) = self.lazy[k].take() {
            self.apply(k << 1, &f);
            self.apply_owned(k << 1 | 1, f); // 片方の子どには更新の所有権を移動できる
        }
    }

    /// 区間境界をまたぐ祖先だけ上から伝播
    #[inline(always)]
    fn push_boundaries(&mut self, l: usize, r: usize) {
        for h in (l.trailing_zeros() + 1..=self.log).rev() {
            self.push(l >> h);
        }
        for h in (r.trailing_zeros() + 1..=self.log).rev() {
            self.push((r - 1) >> h);
        }
    }

    /// 全部上から伝播
    #[inline(always)]
    fn push_all(&mut self) {
        for k in 1..self.leaf_len {
            self.push(k);
        }
    }
}

/// 既存の演算定義を値の集約用Monoidに接続
pub struct LazyValueMonoid<Op>(std::marker::PhantomData<fn() -> Op>);

impl<Op: LazySegmentTreeOperator> Monoid for LazyValueMonoid<Op> {
    type S = Op::Value;

    #[inline(always)]
    fn identity(&self) -> Self::S {
        return Op::identity();
    }

    #[inline(always)]
    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S {
        return Op::operate(*left, *right);
    }
}

/// 既存の演算定義を、更新の合成用Monoidへ接続する。
pub struct LazyUpdateMonoid<Op>(std::marker::PhantomData<fn() -> Op>);

impl<Op: LazySegmentTreeOperator> Monoid for LazyUpdateMonoid<Op> {
    type S = Op::Lazy;

    #[inline(always)]
    fn identity(&self) -> Self::S {
        return Op::lazy_identity();
    }

    #[inline(always)]
    fn combine(&self, new: &Self::S, old: &Self::S) -> Self::S {
        // new ∘ old
        return Op::composition(*new, *old);
    }
}

/// 既存の演算定義から反復型の遅延セグメント木を構築する。
/// 返すクロージャはvaluesを借用しない。
pub fn lazy_from_operator<Op>(
    values: &[Op::Value],
) -> LazySegmentTreeMonoid<LazyValueMonoid<Op>, LazyUpdateMonoid<Op>, impl Fn(&Op::Lazy, &mut Op::Value) + use<Op>>
where
    Op: LazySegmentTreeOperator,
{
    return LazySegmentTreeMonoid::new(
        values,
        LazyValueMonoid::<Op>(std::marker::PhantomData),
        LazyUpdateMonoid::<Op>(std::marker::PhantomData),
        |f: &Op::Lazy, value: &mut Op::Value| {
            *value = Op::mapping(*f, *value);
        },
    );
}

/// 区間の最大値・最小値と、その位置を取得する演算。
///
/// [`super::lazy_from_operator`] に演算型と、元の添字で初期化した葉を渡す。
/// 同値なら既定で [`Left`] が最小の添字を選び、[`Right`] が最大の添字を選ぶ。
/// 加算では途中結果が型の範囲内に収まり、加算によって大小関係が保たれることを前提とする。
///
/// ```
/// use utility_rust::lazy_segment_tree_monoid::indexed::{MaxIndex, RangeAssignMaxIndex, Right};
/// use utility_rust::lazy_segment_tree_monoid::lazy_from_operator;
///
/// let leaves = [MaxIndex::new(9_i64, 0), MaxIndex::new(3, 1), MaxIndex::new(5, 2)];
/// let mut tree = lazy_from_operator::<RangeAssignMaxIndex<i64, Right>>(&leaves);
/// tree.update(0, 3, Some(4));
/// let result = tree.query(0, 3);
/// assert_eq!((result.value, result.index), (4, 2));
/// ```
pub mod indexed {
    use super::{LazySegmentTreeOperator, MinMaxValue, ZeroValue};
    use std::cmp::Ordering;
    use std::marker::PhantomData;

    /// 同じ値を持つ候補から端の添字を選ぶ。常に小さい方か常に大きい方を選ぶこと。
    pub trait TieBreak {
        fn select(a: usize, b: usize) -> usize;
    }

    /// 同値の場合に最も左の添字を選ぶ。
    pub struct Left;

    impl TieBreak for Left {
        #[inline]
        fn select(a: usize, b: usize) -> usize {
            a.min(b)
        }
    }

    /// 同値の場合に最も右の添字を選ぶ。
    pub struct Right;

    impl TieBreak for Right {
        #[inline]
        fn select(a: usize, b: usize) -> usize {
            a.max(b)
        }
    }

    /// 極値と、その極値が現れる添字。空区間では `index == usize::MAX`。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct IndexedValue<T> {
        pub value: T,
        pub index: usize,
        // 代入後は全要素が同値になるため、代入前の極値の位置とは別に両端を保持する。
        left: usize,
        right: usize,
    }

    /// 最大値とその位置を持つノード。
    pub type MaxIndex<T> = IndexedValue<T>;
    /// 最小値とその位置を持つノード。
    pub type MinIndex<T> = IndexedValue<T>;

    impl<T> IndexedValue<T> {
        /// 1要素を初期化する。`usize::MAX` は空区間に予約する。
        #[inline]
        pub fn new(value: T, index: usize) -> Self {
            assert!(index != usize::MAX, "usize::MAXは空区間の添字です");
            Self {
                value,
                index,
                left: index,
                right: index,
            }
        }

        /// 実際の要素を含まない集約かを返す。
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.left == usize::MAX
        }

        #[inline]
        fn empty(value: T) -> Self {
            Self {
                value,
                index: usize::MAX,
                left: usize::MAX,
                right: usize::MAX,
            }
        }

        #[inline]
        fn combine<P: TieBreak>(a: Self, b: Self, preferred: Ordering) -> Self
        where
            T: Ord,
        {
            // 数値の極限と実要素が同値でも、空ノードを候補にしない。
            if a.is_empty() {
                return b;
            }
            if b.is_empty() {
                return a;
            }
            let order = a.value.cmp(&b.value);
            let (value, index) = if order == Ordering::Equal {
                (a.value, P::select(a.index, b.index))
            } else if order == preferred {
                (a.value, a.index)
            } else {
                (b.value, b.index)
            };
            Self {
                value,
                index,
                left: a.left.min(b.left),
                right: a.right.max(b.right),
            }
        }
    }

    macro_rules! define_indexed_operators {
        ($add:ident, $assign:ident, $identity:ident, $preferred:ident, $add_doc:literal, $assign_doc:literal) => {
            #[doc = $add_doc]
            pub struct $add<T, P = Left>(PhantomData<fn() -> (T, P)>);

            impl<T: MinMaxValue + ZeroValue + Ord, P: TieBreak> LazySegmentTreeOperator for $add<T, P> {
                type Value = IndexedValue<T>;
                type Lazy = T;

                #[inline]
                fn identity() -> Self::Value {
                    IndexedValue::empty(T::$identity())
                }

                #[inline]
                fn lazy_identity() -> T {
                    T::zero()
                }

                #[inline]
                fn operate(a: Self::Value, b: Self::Value) -> Self::Value {
                    IndexedValue::combine::<P>(a, b, Ordering::$preferred)
                }
                #[inline]
                fn mapping(delta: T, mut node: Self::Value) -> Self::Value {
                    if !node.is_empty() {
                        node.value = node.value + delta;
                    }
                    node
                }
                #[inline]
                fn composition(new: T, old: T) -> T {
                    old + new
                }
            }

            #[doc = $assign_doc]
            pub struct $assign<T, P = Left>(PhantomData<fn() -> (T, P)>);

            impl<T: MinMaxValue + Ord, P: TieBreak> LazySegmentTreeOperator for $assign<T, P> {
                type Value = IndexedValue<T>;
                type Lazy = Option<T>;

                #[inline]
                fn identity() -> Self::Value {
                    IndexedValue::empty(T::$identity())
                }

                #[inline]
                fn lazy_identity() -> Option<T> {
                    None
                }

                #[inline]
                fn operate(a: Self::Value, b: Self::Value) -> Self::Value {
                    IndexedValue::combine::<P>(a, b, Ordering::$preferred)
                }
                #[inline]
                fn mapping(update: Option<T>, mut node: Self::Value) -> Self::Value {
                    if let Some(value) = update {
                        if !node.is_empty() {
                            node.value = value;
                            node.index = P::select(node.left, node.right);
                        }
                    }
                    node
                }
                #[inline]
                fn composition(new: Option<T>, old: Option<T>) -> Option<T> {
                    new.or(old)
                }
            }
        };
    }

    define_indexed_operators!(
        RangeAddMaxIndex,
        RangeAssignMaxIndex,
        max_identity,
        Greater,
        "区間加算と最大値・添字の取得。更新値は加算量。",
        "区間代入と最大値・添字の取得。`Some(value)` で代入し、`None` は更新なし。"
    );
    define_indexed_operators!(
        RangeAddMinIndex,
        RangeAssignMinIndex,
        min_identity,
        Less,
        "区間加算と最小値・添字の取得。更新値は加算量。",
        "区間代入と最小値・添字の取得。`Some(value)` で代入し、`None` は更新なし。"
    );
}

// ======================================================================================
//
//                               便利モノイド演算
//
// ======================================================================================

/// 法 MOD上の一次関数 f(x) = ax + b
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Affine<const MOD: u32> {
    a: ModInt<MOD>,
    b: ModInt<MOD>,
}

impl<const MOD: u32> Affine<MOD> {
    /// 整数・ModInt のどちらからも生成できる。
    #[inline(always)]
    pub fn new<A: Into<ModInt<MOD>>, B: Into<ModInt<MOD>>>(a: A, b: B) -> Self {
        return Self {
            a: a.into(),
            b: b.into(),
        };
    }

    /// 法 MOD 上で ax + b を計算する。
    #[inline(always)]
    pub fn eval<T: Into<ModInt<MOD>>>(&self, x: T) -> ModInt<MOD> {
        return self.a * x + self.b;
    }
}

/// 配列の左から右へ一次関数を適用
/// combine(l, r) = right∘left
pub struct AffineComposite<const MOD: u32>;

impl<const MOD: u32> Monoid for AffineComposite<MOD> {
    type S = Affine<MOD>;

    #[inline]
    fn identity(&self) -> Self::S {
        return Affine::new(ModInt::<MOD>::ONE, ModInt::<MOD>::ZERO);
    }

    #[inline]
    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S {
        return Affine {
            a: right.a * left.a,
            b: right.a * left.b + right.b,
        };
    }
}

/// 法 MOD 上で区間アフィン変換と区間和を扱う。
///
/// 葉には `SumCount::new(value, 1)` を使い、更新値には `Affine::new(a, b)` を渡す。
/// 各要素への ax + b は、区間和に対して a * sum + b * count として作用する。
pub struct RangeAffineSum<const MOD: u32>;

impl<const MOD: u32> LazySegmentTreeOperator for RangeAffineSum<MOD> {
    type Value = SumCount<ModInt<MOD>>;
    type Lazy = Affine<MOD>;

    #[inline]
    fn identity() -> Self::Value {
        SumCount::new(ModInt::ZERO, 0)
    }

    #[inline]
    fn lazy_identity() -> Self::Lazy {
        AffineComposite::<MOD>.identity()
    }

    #[inline]
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value {
        SumCount::new(left.sum + right.sum, left.count + right.count)
    }

    #[inline]
    fn mapping(f: Self::Lazy, node: Self::Value) -> Self::Value {
        SumCount::new(f.a * node.sum + f.b * node.count, node.count)
    }

    #[inline]
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy {
        // AffineComposite は第1引数を先に作用させる。
        AffineComposite::<MOD>.combine(&current_f, &new_f)
    }
}

/// 上位 TOP 件と下位 BOTTOM 件を持つノード。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopBottom<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool = false> {
    top: [Option<T>; TOP],
    bottom: [Option<T>; BOTTOM],
    len: usize,
}

impl<T: Clone, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool> TopBottom<T, TOP, BOTTOM, DISTINCT> {
    /// 元の配列の1要素を葉にする
    #[inline(always)]
    pub fn leaf(value: T) -> Self {
        let mut top = std::array::from_fn(|_| None);
        let mut bottom = std::array::from_fn(|_| None);
        if let Some(first) = top.first_mut() {
            *first = Some(value.clone());
        }
        if let Some(first) = bottom.first_mut() {
            *first = Some(value);
        }
        return Self {
            top,
            bottom,
            len: 1,
        };
    }

    #[inline(always)]
    pub fn top(&self) -> impl Iterator<Item = &T> {
        return self.top.iter().map_while(Option::as_ref);
    }

    #[inline(always)]
    pub fn bottom(&self) -> impl Iterator<Item = &T> {
        return self.bottom.iter().map_while(Option::as_ref);
    }

    /// 区間全体の要素数
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.len;
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }
}

/// TOP, BOTTOMのいずれかを0にするとそっち側を省略可能
pub struct TopBottomMonoid<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool = false>(
    std::marker::PhantomData<fn() -> T>,
);

impl<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool> TopBottomMonoid<T, TOP, BOTTOM, DISTINCT> {
    #[inline(always)]
    pub fn new() -> Self {
        return Self(std::marker::PhantomData);
    }
}

pub type DistinctTopBottomMonoid<T, const TOP: usize, const BOTTOM: usize> = TopBottomMonoid<T, TOP, BOTTOM, true>;

impl<T: Ord + Clone, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool> Monoid
    for TopBottomMonoid<T, TOP, BOTTOM, DISTINCT>
{
    type S = TopBottom<T, TOP, BOTTOM, DISTINCT>;

    #[inline(always)]
    fn identity(&self) -> Self::S {
        return TopBottom {
            top: std::array::from_fn(|_| None),
            bottom: std::array::from_fn(|_| None),
            len: 0,
        };
    }

    #[inline(always)]
    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S {
        return TopBottom {
            top: merge_prefix::<T, TOP, DISTINCT>(&left.top, &right.top, |a, b| a >= b, |a, b| a == b),
            bottom: merge_prefix::<T, BOTTOM, DISTINCT>(&left.bottom, &right.bottom, |a, b| a <= b, |a, b| a == b),
            len: left.len + right.len,
        };
    }
}

#[inline(always)]
fn merge_prefix<T: Clone, const K: usize, const DISTINCT: bool>(
    left: &[Option<T>; K],
    right: &[Option<T>; K],
    left_first: impl Fn(&T, &T) -> bool,
    same_value: impl Fn(&T, &T) -> bool,
) -> [Option<T>; K] {
    let mut left = left.iter().map_while(Option::as_ref).peekable();
    let mut right = right.iter().map_while(Option::as_ref).peekable();
    let mut last = None;

    return std::array::from_fn(|_| {
        loop {
            let value = match (left.peek(), right.peek()) {
                (Some(a), Some(b)) if left_first(a, b) => left.next(),
                (_, Some(_)) => right.next(),
                _ => left.next(),
            }?;

            // 添字付きの場合も、重複の判定には値だけを使う。
            continueif!(DISTINCT && last.is_some_and(|previous| same_value(previous, value)));
            last = Some(value);
            return Some(value.clone());
        }
    });
}

/// 区間への更新、Identityは何もしない
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopBottomUpdate<T> {
    Identity,
    Add(T),
    Assign(T),
}

pub struct TopBottomUpdateMonoid<T>(std::marker::PhantomData<fn() -> T>);

impl<T: Copy + std::ops::Add<Output = T>> Monoid for TopBottomUpdateMonoid<T> {
    type S = TopBottomUpdate<T>;

    #[inline(always)]
    fn identity(&self) -> Self::S {
        return TopBottomUpdate::Identity;
    }

    #[inline(always)]
    fn combine(&self, new: &Self::S, old: &Self::S) -> Self::S {
        use TopBottomUpdate::{Add, Assign, Identity};

        return match (*new, *old) {
            (Identity, old) => old,
            (Assign(value), _) => Assign(value),
            (Add(delta), Identity) => Add(delta),
            (Add(delta), Add(previous)) => Add(previous + delta),
            (Add(delta), Assign(value)) => Assign(value + delta),
        };
    }
}

/// 区間加算・代入ができる上位・下位集約の木を作る。
/// 加算は大小関係を厳密に保ち、値と遅延中の加算量が型の範囲に収まること。
#[inline(always)]
pub fn lazy_top_bottom<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool>(
    values: &[T],
) -> LazySegmentTreeMonoid<
    TopBottomMonoid<T, TOP, BOTTOM, DISTINCT>,
    TopBottomUpdateMonoid<T>,
    impl Fn(&TopBottomUpdate<T>, &mut TopBottom<T, TOP, BOTTOM, DISTINCT>) + use<T, TOP, BOTTOM, DISTINCT>,
>
where
    T: Copy + Ord + std::ops::Add<Output = T>,
{
    let leaves: Vec<_> = values.iter().copied().map(TopBottom::leaf).collect();

    return LazySegmentTreeMonoid::new(
        &leaves,
        TopBottomMonoid::<T, TOP, BOTTOM, DISTINCT>::new(),
        TopBottomUpdateMonoid(std::marker::PhantomData),
        |update: &TopBottomUpdate<T>, node: &mut TopBottom<T, TOP, BOTTOM, DISTINCT>| {
            return match *update {
                TopBottomUpdate::Identity => {}
                TopBottomUpdate::Add(delta) => {
                    // 同じ量を加えても順位と重複の関係は変わらない
                    for value in node.top.iter_mut().chain(node.bottom.iter_mut()).flatten() {
                        *value = *value + delta;
                    }
                }
                TopBottomUpdate::Assign(value) => {
                    // 代入後は全要素が同じ値。重複なしなら1件だけ残す
                    let count = if DISTINCT { node.len.min(1) } else { node.len };
                    node.top = std::array::from_fn(|i| (i < count).then_some(value));
                    node.bottom = std::array::from_fn(|i| (i < count).then_some(value));
                }
            };
        },
    );
}

/// 添字付きの上位・下位集約から取得した1件。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopBottomEntry<T> {
    pub value: T,
    pub index: usize,
}

/// 添字付きの上位 TOP 件と下位 BOTTOM 件。各件の添字は元配列の0始まり。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedTopBottom<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool = false> {
    top: [Option<TopBottomEntry<T>>; TOP],
    bottom: [Option<TopBottomEntry<T>>; BOTTOM],
    // 代入で全要素が同値になったとき、更新前の順位に関係なく端の添字を返すために保持する。
    top_indices: [Option<usize>; TOP],
    bottom_indices: [Option<usize>; BOTTOM],
    len: usize,
}

impl<T: Copy, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool>
    IndexedTopBottom<T, TOP, BOTTOM, DISTINCT>
{
    /// 一点置換用の葉を作る。indexにはsetの対象と同じ添字を渡す。
    pub fn leaf(value: T, index: usize) -> Self {
        let entry = TopBottomEntry { value, index };
        Self {
            top: std::array::from_fn(|i| (i == 0).then_some(entry)),
            bottom: std::array::from_fn(|i| (i == 0).then_some(entry)),
            top_indices: std::array::from_fn(|i| (i == 0).then_some(index)),
            bottom_indices: std::array::from_fn(|i| (i == 0).then_some(index)),
            len: 1,
        }
    }

    pub fn top(&self) -> impl Iterator<Item = &TopBottomEntry<T>> {
        self.top.iter().map_while(Option::as_ref)
    }

    pub fn bottom(&self) -> impl Iterator<Item = &TopBottomEntry<T>> {
        self.bottom.iter().map_while(Option::as_ref)
    }

    /// 重複を含む区間全体の要素数。
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 上位と下位の同値時の優先方向を別々に選べる集約。
///
/// `TopTie` と `BottomTie` の既定はどちらも [`indexed::Left`]。
/// `DISTINCT = true` の場合、各値につき指定方向の端の添字を1件保持する。
pub struct IndexedTopBottomMonoid<
    T,
    const TOP: usize,
    const BOTTOM: usize,
    const DISTINCT: bool = false,
    TopTie = indexed::Left,
    BottomTie = indexed::Left,
>(std::marker::PhantomData<fn() -> (T, TopTie, BottomTie)>);

impl<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool, TopTie, BottomTie> Monoid
    for IndexedTopBottomMonoid<T, TOP, BOTTOM, DISTINCT, TopTie, BottomTie>
where
    T: Copy + Ord,
    TopTie: indexed::TieBreak,
    BottomTie: indexed::TieBreak,
{
    type S = IndexedTopBottom<T, TOP, BOTTOM, DISTINCT>;

    fn identity(&self) -> Self::S {
        IndexedTopBottom {
            top: [None; TOP],
            bottom: [None; BOTTOM],
            top_indices: [None; TOP],
            bottom_indices: [None; BOTTOM],
            len: 0,
        }
    }

    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S {
        IndexedTopBottom {
            top: merge_prefix::<_, TOP, DISTINCT>(
                &left.top,
                &right.top,
                |a, b| match a.value.cmp(&b.value) {
                    Ordering::Equal => TopTie::select(a.index, b.index) == a.index,
                    order => order == Ordering::Greater,
                },
                |a, b| a.value == b.value,
            ),
            bottom: merge_prefix::<_, BOTTOM, DISTINCT>(
                &left.bottom,
                &right.bottom,
                |a, b| match a.value.cmp(&b.value) {
                    Ordering::Equal => BottomTie::select(a.index, b.index) == a.index,
                    order => order == Ordering::Less,
                },
                |a, b| a.value == b.value,
            ),
            top_indices: merge_prefix::<_, TOP, false>(
                &left.top_indices,
                &right.top_indices,
                |a, b| TopTie::select(*a, *b) == *a,
                |a, b| a == b,
            ),
            bottom_indices: merge_prefix::<_, BOTTOM, false>(
                &left.bottom_indices,
                &right.bottom_indices,
                |a, b| BottomTie::select(*a, *b) == *a,
                |a, b| a == b,
            ),
            len: left.len + right.len,
        }
    }
}

impl<T, const TOP: usize, const BOTTOM: usize, const DISTINCT: bool, TopTie, BottomTie>
    IndexedTopBottomMonoid<T, TOP, BOTTOM, DISTINCT, TopTie, BottomTie>
where
    T: Copy + Ord + Add<Output = T>,
    TopTie: indexed::TieBreak,
    BottomTie: indexed::TieBreak,
{
    /// 区間加算・代入に対応する木を作る。加算の前提は [`lazy_top_bottom`] と同じ。
    pub fn build(
        values: &[T],
    ) -> LazySegmentTreeMonoid<
        Self,
        TopBottomUpdateMonoid<T>,
        impl Fn(&TopBottomUpdate<T>, &mut IndexedTopBottom<T, TOP, BOTTOM, DISTINCT>)
            + use<T, TOP, BOTTOM, DISTINCT, TopTie, BottomTie>,
    > {
        let leaves: Vec<_> = values
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| IndexedTopBottom::leaf(value, index))
            .collect();
        LazySegmentTreeMonoid::new(
            &leaves,
            Self(std::marker::PhantomData),
            TopBottomUpdateMonoid(std::marker::PhantomData),
            |update: &TopBottomUpdate<T>, node: &mut IndexedTopBottom<T, TOP, BOTTOM, DISTINCT>| {
                match *update {
                    TopBottomUpdate::Identity => {}
                    TopBottomUpdate::Add(delta) => {
                        for entry in node.top.iter_mut().chain(node.bottom.iter_mut()).flatten() {
                            entry.value = entry.value + delta;
                        }
                    }
                    TopBottomUpdate::Assign(value) => {
                        let assigned = |i, index: Option<usize>| {
                            if DISTINCT && i != 0 {
                                return None;
                            }
                            index.map(|index| TopBottomEntry { value, index })
                        };
                        node.top = std::array::from_fn(|i| assigned(i, node.top_indices[i]));
                        node.bottom = std::array::from_fn(|i| assigned(i, node.bottom_indices[i]));
                    }
                }
            },
        )
    }
}
