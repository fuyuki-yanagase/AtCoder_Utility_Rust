// モノイド・群・アーベル群
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
//                               モノイド・群・アーベル群
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/monoid_group_abelian.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/monoid_group_abelian.rs

// ======================================================================================
//
// モノイド
//
// ======================================================================================

/// 結合律と単位元を持つモノイド演算
/// S は複数をまとめる型
/// メソッドを &self にすることで、設定値を持つモノイドも定義できる。
pub trait Monoid {
    type S;

    /// 単位元の生成
    fn identity(&self) -> Self::S;

    /// 左,右の順で結合
    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S;
} // end of trait Monoid

/// Monoid を実装する構造体を書かず、クロージャ演算を渡したい時に使用
/// identity : 単位元を毎回作る関数
/// combine : 2つの参照に演算を適用した値を返す
///
/// ```
/// use utility_rust::monoid_group_abelian::{FnMonoid, Monoid};
///
/// let sum = FnMonoid::new(
///     || 0_i64,
///     |a: &i64, b: &i64| *a + *b,
/// );
/// assert_eq!(sum.combine(&3, &4), 7);
/// ```
pub struct FnMonoid<S, FIdentity, FCombine> {
    identity: FIdentity,
    combine: FCombine,
    // 値を保持せずにモノイドの対象型 S を型システムに記録
    _marker: PhantomData<fn() -> S>,
} // end of struct FnMonoid

impl<S, FIdentity, FCombine> FnMonoid<S, FIdentity, FCombine>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
{
    /// 単位元生成関数と結合関数からモノイドを作成
    pub fn new(identity: FIdentity, combine: FCombine) -> Self {
        Self {
            identity,
            combine,
            _marker: PhantomData,
        }
    }
} // end of impl FnMonoid

impl<S, FIdentity, FCombine> Monoid for FnMonoid<S, FIdentity, FCombine>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
{
    type S = S;

    fn identity(&self) -> S {
        (self.identity)()
    }

    fn combine(&self, left: &S, right: &S) -> S {
        (self.combine)(left, right)
    }
} // end of impl Monoid

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                          群
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/monoid_group_abelian.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/monoid_group_abelian.rs

/// モノイドに逆元を追加した群
///
/// - 結合律: (a * b) * c = a * (b * c)
/// - 単位元: e * a = a * e = a
/// - 逆元: a * inverse(a) = inverse(a) * a = e
///
/// 交換律は要求しないため、非可換群に対応
/// 演算にはMonoidのcombineを使用
pub trait Group: Monoid {
    /// 指定した要素の逆元
    fn inverse(&self, value: &Self::S) -> Self::S;
} // end of trait Group

/// クロージャで群演算を定義するための構造体
///
/// - identity: 単位元を生成
/// - combine: 2つの値を左、右の順番に結合
/// - inverse: 値の逆元を生成
///
/// Monoid部分はFnMonoidに委譲
pub struct FnGroup<S, FIdentity, FCombine, FInverse> {
    monoid: FnMonoid<S, FIdentity, FCombine>,
    inverse: FInverse,
} // end of struct FnGroup

impl<S, FIdentity, FCombine, FInverse> FnGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    /// 単位元、結合演算、逆元の関数から群を構築
    pub fn new(identity: FIdentity, combine: FCombine, inverse: FInverse) -> Self {
        return Self {
            monoid: FnMonoid::new(identity, combine),
            inverse,
        };
    } // end of constructor
} // end of impl FnGroup

/// FnGroupにMonoidを実装
impl<S, FIdentity, FCombine, FInverse> Monoid for FnGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    type S = S;

    /// 単位元の生成
	#[rustfmt::skip]
	#[inline(always)]
    fn identity(&self) -> S { self.monoid.identity() }

    /// 左、右の順に結合
	#[rustfmt::skip]
	#[inline(always)]
    fn combine(&self, left: &S, right: &S) -> S { self.monoid.combine(left, right) }
} // end of impl Monoid

/// FnGroupにGroupを実装
impl<S, FIdentity, FCombine, FInverse> Group for FnGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    /// 逆元の生成
	#[rustfmt::skip]
	#[inline(always)]
    fn inverse(&self, value: &S) -> S { (self.inverse)(value) }
} // end of impl Group

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                  アーベル群(可換群)
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/monoid_group_abelian.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/monoid_group_abelian.rs

/// 可換群 (Abelian Group)
///
/// Groupの全性質に加えて交換律を満たす
///
/// - 結合律: (a * b) * c = a * (b * c)
/// - 単位元: e * a = a * e = a
/// - 逆元: a * inverse(a) = inverse(a) * a = e
/// - 交換律: a * b = b * a
///
/// Rustの型システムで交換律を検証できないかも
/// 実装の方で丁寧にカバーしてみる
pub trait AbelianGroup: Group {}

/// クロージャで可換群を定義するための構造体
///
/// - identity: 単位元
/// - combine: 結合
/// - inverse: 逆元
///
/// 内部ではFnGroupで
/// combineには交換律を満たす演算を渡す(Rustで検証できないので優しくしてあげる)
pub struct FnAbelianGroup<S, FIdentity, FCombine, FInverse> {
    group: FnGroup<S, FIdentity, FCombine, FInverse>,
} // end of struct FnAbelianGroup

impl<S, FIdentity, FCombine, FInverse> FnAbelianGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    /// 単位元、結合演算、逆元の関数から可換群を構築
    pub fn new(identity: FIdentity, combine: FCombine, inverse: FInverse) -> Self {
        return Self {
            group: FnGroup::new(identity, combine, inverse),
        };
    } // end of constructor
} // end of impl FnAbelianGroup

/// 可換群にもMonoidを実装
impl<S, FIdentity, FCombine, FInverse> Monoid for FnAbelianGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    type S = S;

    /// 単位元の生成
	#[rustfmt::skip]
    #[inline(always)]
    fn identity(&self) -> S { self.group.identity() }

    /// 2つの値を結合
	#[rustfmt::skip]
    #[inline(always)]
    fn combine(&self, left: &S, right: &S) -> S { self.group.combine(left, right) }
} // end of impl Monoid

/// 可換群にもGroupを実装
impl<S, FIdentity, FCombine, FInverse> Group for FnAbelianGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
    /// 逆元の生成
	#[rustfmt::skip]
    #[inline(always)]
    fn inverse(&self, value: &S) -> S { self.group.inverse(value) }
} // end of impl Group

/// 交換律を満たすことを宣言
impl<S, FIdentity, FCombine, FInverse> AbelianGroup for FnAbelianGroup<S, FIdentity, FCombine, FInverse>
where
    FIdentity: Fn() -> S,
    FCombine: Fn(&S, &S) -> S,
    FInverse: Fn(&S) -> S,
{
} // end of impl AbelianGroup
