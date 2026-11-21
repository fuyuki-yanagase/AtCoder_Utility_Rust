// 2次元ベクトル・偏角
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
//                                 2次元ベクトル・偏角
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/vec2_angle.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/vec2_angle.rs

// ======================================================================================
//
// 2次元ベクトル（Vec2）
//
// ======================================================================================

/// 加減算・符号反転・整数倍・回転など、i64で行う演算は中間値もi64内に収まる前提。
/// 内積・外積・距離・面積などの広域演算は、差・積・和・絶対値を含む全中間値がi128内に収まる前提。
/// 各成分がi64内でも、あらゆる組合せの積や和がi128内に収まるとは限らない。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
} // end of struct Vec2

impl Vec2 {
    #[rustfmt::skip]
    pub fn new(x: i64, y: i64) -> Self { Self {x, y} }

	#[rustfmt::skip]
	#[inline(always)]
    pub fn is_zero(self) -> bool { self.x == 0 && self.y == 0 }

    /// 内積
    #[inline(always)]
    pub fn dot(self, other: Self) -> i128 {
        return (self.x as i128) * (other.x as i128) + (self.y as i128) * (other.y as i128);
    } // end of method

    /// 外積
    #[inline(always)]
    pub fn cross(self, other: Self) -> i128 {
        return (self.x as i128) * (other.y as i128) - (self.y as i128) * (other.x as i128);
    } // end of method

    /// ノルムの二乗、ノルムの比較などに整数で
	#[rustfmt::skip]
	#[inline(always)]
    pub fn norm_sq(self) -> i128 { self.dot(self) }

    /// ノルム
	#[rustfmt::skip]
	#[inline(always)]
    pub fn norm(self) -> f64 { (self.norm_sq() as f64).sqrt() }

    /// 二点間の距離の二乗、比較などに
    #[inline(always)]
    pub fn distance_sq(self, other: Self) -> i128 {
        let delta = self.wide_diff(other);
        return Self::wide_dot(delta, delta);
    } // end of method

    /// ユークリッド距離、戻り値は近似値
    #[inline(always)]
    pub fn distance(self, other: Self) -> f64 {
        return (self.distance_sq(other) as f64).sqrt();
    } // end of method

    /// マンハッタン距離, |dx| + |dy|
    #[inline(always)]
    pub fn manhattan_distance(self, other: Self) -> i128 {
        let (dx, dy) = self.wide_diff(other);
        return dx.abs() + dy.abs();
    } // end of method

    /// チェビシェフ距離, max(|dx|, |dy|)
    #[inline(always)]
    pub fn chebyshev_distance(self, other: Self) -> i128 {
        let (dx, dy) = self.wide_diff(other);
        return dx.abs().max(dy.abs());
    } // end of method

    /// 反時計回りに90度回転
	#[rustfmt::skip]
	#[inline(always)]
    pub fn rotate90(self) -> Self { Self::new(-self.y, self.x) }

    /// 時計回りに90度回転
	#[rustfmt::skip]
	#[inline(always)]
    pub fn rotate90_cw(self) -> Self { Self::new(self.y, -self.x) }

    /// 180度回転
	#[rustfmt::skip]
	#[inline(always)]
    pub fn rotate180(self) -> Self { -self }

    /// 平行？ 両方が非ゼロの前提
	#[rustfmt::skip]
	#[inline(always)]
    pub fn is_parallel(self, other: Self) -> bool { self.cross(other) == 0 }

    /// 直交？ 両方が非ゼロの前提
	#[rustfmt::skip]
	#[inline(always)]
    pub fn is_perpendicular(self, other: Self) -> bool { self.dot(other) == 0 }

    /// 同じ方向？ どちらかがゼロならfalse
	#[rustfmt::skip]
	#[inline(always)]
    pub fn same_direction(self, other: Self) -> bool { self.cross(other) == 0 && self.dot(other) > 0 }

    /// 反対方向？ どちらかがゼロならfalse
	#[rustfmt::skip]
	#[inline(always)]
    pub fn opposite_direction(self, other: Self) -> bool { self.cross(other) == 0 && self.dot(other) < 0 }

    /// 三角形abcの符号付き面積の2倍
    /// 正なら反時計回り、負なら時計回り、0なら同一線上
	#[rustfmt::skip]
    #[inline(always)]
    pub fn orient(a: Self, b: Self, c: Self) -> i128 { Self::wide_cross(b.wide_diff(a), c.wide_diff(a)) }

    /// 三角形の面積の2倍。向きに依存しない
	#[rustfmt::skip]
	#[inline(always)]
    pub fn triangle_area2(a: Self, b: Self, c: Self) -> i128 { Self::orient(a, b, c).abs() }

    /// 三角形の面積。戻り値は近似値
	#[rustfmt::skip]
	#[inline(always)]
    pub fn triangle_area(a: Self, b: Self, c: Self) -> f64 { (Self::triangle_area2(a, b, c) as f64) / 2.0 }

    /// 3点が同一線上にあるか
	#[rustfmt::skip]
	#[inline(always)]
    pub fn is_collinear(a: Self, b: Self, c: Self) -> bool { Self::orient(a, b, c) == 0 }

    /// 自分が線分ab上にあるか。端点も含む
    /// a == b の場合も、その点との一致で判定可能
    #[inline(always)]
    pub fn is_on_segment(self, a: Self, b: Self) -> bool {
        return Self::is_collinear(a, b, self)
            && a.x.min(b.x) <= self.x
            && self.x <= a.x.max(b.x)
            && a.y.min(b.y) <= self.y
            && self.y <= a.y.max(b.y);
    } // end of method

    /// 線分abと線分cdが交差するか
    /// 端点接触、共線の重なり、点に退化した線分も含む
    #[inline(always)]
    pub fn segments_intersect(a: Self, b: Self, c: Self, d: Self) -> bool {
        let o1 = Self::orient(a, b, c);
        let o2 = Self::orient(a, b, d);
        let o3 = Self::orient(c, d, a);
        let o4 = Self::orient(c, d, b);

        // 共線の場合は、相手の線分上にあるか調べる。
        if (o1 == 0 && c.is_on_segment(a, b))
            || (o2 == 0 && d.is_on_segment(a, b))
            || (o3 == 0 && a.is_on_segment(c, d))
            || (o4 == 0 && b.is_on_segment(c, d))
        {
            return true;
        }

        // 大きな外積同士を直接掛けない。
        // signumで-1・0・1へ変換してから掛ける。
        return o1.signum() * o2.signum() < 0 && o3.signum() * o4.signum() < 0;
    } // end of method

    /// 自分と直線abの距離の二乗
    /// a != b が前提、戻り値は近似値
    #[inline(always)]
    pub fn distance_sq_to_line(self, a: Self, b: Self) -> f64 {
        let ab = b.wide_diff(a);
        let ap = self.wide_diff(a);

        // 面積をf64にしてから二乗
        let area2 = Self::wide_cross(ab, ap) as f64;
        return area2 * area2 / Self::wide_dot(ab, ab) as f64;
    } // end of method

    /// 自分と線分abの距離の二乗
    /// 端点・内部の判定は整数、戻り値は近似値
    #[inline(always)]
    pub fn distance_sq_to_segment(self, a: Self, b: Self) -> f64 {
        returnif!(a == b, self.distance_sq(a) as f64);

        let ab = b.wide_diff(a);
        let ap = self.wide_diff(a);
        let d = Self::wide_dot(ab, ab);
        let projection = Self::wide_dot(ap, ab);

        // 最も近い点が端点a
        returnif!(projection <= 0, self.distance_sq(a) as f64);

        // 最も近い点が端点b
        returnif!(projection >= d, self.distance_sq(b) as f64);

        // 垂線の足が線分の内部にある
        let area2 = Self::wide_cross(ab, ap) as f64;
        return area2 * area2 / d as f64;
    } // end of method

    /// 偏角が [0, π) なら0, [π, 2π) なら1
    #[inline(always)]
    fn half(self) -> u8 {
        return if self.y > 0 || (self.y == 0 && self.x > 0) { 0 } else { 1 };
    } // end of method

    /// 偏角比較, ゼロベクトル不可
    #[inline(always)]
    pub fn cmp_arg(self, other: Self) -> Ordering {
        assert!(!self.is_zero() && !other.is_zero());
        return self.half().cmp(&other.half()).then_with(|| 0.cmp(&self.cross(other)));
    } // end of method

    /// 内部補助: 引き算より先に各成分をi128に変換
    #[inline(always)]
    fn wide_diff(self, other: Self) -> (i128, i128) {
        return ((self.x as i128) - (other.x as i128), (self.y as i128) - (other.y as i128));
    } // end of method

    /// 内部補助: i128の成分同士の内積
    #[inline(always)]
    fn wide_dot(a: (i128, i128), b: (i128, i128)) -> i128 {
        return a.0 * b.0 + a.1 * b.1;
    } // end of method

    /// 内部補助: i128の成分同士の外積
    #[inline(always)]
    fn wide_cross(a: (i128, i128), b: (i128, i128)) -> i128 {
        return a.0 * b.1 - a.1 * b.0;
    } // end of method
} // end of impl Vec2

/// a + b
impl Add for Vec2 {
    type Output = Self;

	#[rustfmt::skip]
    fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y) }
} // end of Add for Vec2

/// a - b
impl Sub for Vec2 {
    type Output = Self;

	#[rustfmt::skip]
    fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y) }
} // end of Sub for Vec2

/// -a
impl Neg for Vec2 {
    type Output = Self;

	#[rustfmt::skip]
    fn neg(self) -> Self { Self::new(-self.x, -self.y) }
} // end of Neg for Vec2

/// a += b
impl AddAssign for Vec2 {
    #[rustfmt::skip]
    fn add_assign(&mut self, other: Self) { *self = *self + other }
} // end of AddAssign for Vec2

/// a -= b
impl SubAssign for Vec2 {
    #[rustfmt::skip]
    fn sub_assign(&mut self, other: Self) { *self = *self - other }
} // end of SubAssign for Vec2

/// a * b
impl Mul<i64> for Vec2 {
    type Output = Self;

	#[rustfmt::skip]
    fn mul(self, scalar: i64) -> Self { Self::new(self.x * scalar, self.y * scalar) }
} // end of Mul<i64> for Vec2

/// a *= b
impl MulAssign<i64> for Vec2 {
    #[rustfmt::skip]
    fn mul_assign(&mut self, scalar: i64) { *self = *self * scalar }
} // end of MulAssign<i64> for Vec2

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                        偏角(比較可能)
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/vec2_angle.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/vec2_angle.rs

/// 回転数を含めた偏角
///
/// dirは非ゼロ。方向演算はVec2と同じi64・i128の範囲内という前提。
/// 角度は turn * 2π + dir の偏角
///
/// 同じ回転数・偏角ならベクトルの長さは違って良い
#[derive(Debug, Clone, Copy)]
pub struct SpiralAngle {
    dir: Vec2,
    turn: i64,
} // end of struct SpiralAngle

impl SpiralAngle {
    /// 方向ベクトルと回転数から生成
	#[rustfmt::skip]
    pub fn new(dir: Vec2, turn: i64) -> Self {
        assert!(!dir.is_zero(), "ゼロベクトルは偏角を定義できません");
        return Self { dir, turn };
    } // end of constructor

    /// (x, y) から生成, 回転数は0
	#[rustfmt::skip]
	#[inline(always)]
    pub fn from_xy(x: i64, y: i64) -> Self { Self::new(Vec2::new(x, y), 0) }

    /// 方向ベクトル
	#[rustfmt::skip]
	#[inline(always)]
    pub fn vector(&self) -> Vec2 { self.dir }

    /// 回転数
	#[rustfmt::skip]
	#[inline(always)]
    pub fn turn(&self) -> i64 { self.turn }

    /// 回転数を指定して新しい角度を生成
	#[rustfmt::skip]
	#[inline(always)]
    pub fn with_turn(self, turn: i64) -> Self { Self{ turn, ..self } }

    /// 回転数を加算。i64に収まらなければNone
    #[inline(always)]
    pub fn checked_add_turns(self, delta: i64) -> Option<Self> {
        return Some(Self {
            turn: self.turn.checked_add(delta)?,
            ..self
        });
    } // end of method

    /// 方向ベクトル同士の外積
    /// 回転数を考慮しない
	#[rustfmt::skip]
	#[inline(always)]
    pub fn cross(&self, other: &Self) -> i128 { self.dir.cross(other.dir) }

    /// 回転数を含む角度をラジアンで取得
    /// 戻り値は浮動小数点の近似値
    /// 厳密な角度比較にOrdを使用
    #[inline(always)]
    pub fn radians(&self) -> f64 {
        let mut angle = (self.dir.y as f64).atan2(self.dir.x as f64);

        if angle < 0.0 {
            angle += TAU;
        }

        // 丸めでTAUになっても0に戻さない
        // ほぼ360°の方向をほぼ0°として扱うのを避ける
        return (self.turn as f64) * TAU + angle;
    } // end of method

    /// 回転数を含む角度を度数法で取得
    /// 戻り値は近似値
	#[rustfmt::skip]
	#[inline(always)]
    pub fn degrees(&self) -> f64 { self.radians().to_degrees() }

    /// 回転数とベクトルの長さを無視して同じ方向かを判定
    #[inline(always)]
    pub fn same_direction(&self, other: &Self) -> bool {
        return self.cmp_direction(other) == Ordering::Equal;
    } // end of method

    /// 回転数を無視して偏角を比較
    #[inline(always)]
    pub fn cmp_direction(&self, other: &Self) -> Ordering {
        // Vec2のOrdは座標順
        // 偏角順の比較にはcmp_argを使用
        return self.dir.cmp_arg(other.dir);
    } // end of method

    /// 反時計回りに90°回転
    /// 成分・回転数のオーバーフロー時はNone
	#[rustfmt::skip]
    #[inline(always)]
    pub fn checked_rotate90(self) -> Option<Self> {
        let dir = Vec2::new(self.dir.y.checked_neg()?, self.dir.x);
        return self.carry_ccw(Self { dir, ..self });
    } // end of method

	/// 時計回りに90°回転
	/// 成分・回転数のオーバーフロー時はNone
	#[rustfmt::skip]
	#[inline(always)]
	pub fn checked_rotate90_cw(self) -> Option<Self> { 
		let dir = Vec2::new(self.dir.y, self.dir.x.checked_neg()?);
		return self.carry_cw(Self { dir, ..self });
	} // end of method

	/// 反時計回りに180°回転
	/// 成分・回転数のオーバーフロー時はNone
	#[rustfmt::skip]
	#[inline(always)]
	pub fn checked_rotate180(self) -> Option<Self> { 
		let dir = Vec2::new(self.dir.x.checked_neg()?, self.dir.y.checked_neg()?);
		return self.carry_ccw(Self { dir, ..self });
	} // end of method

	/// 反時計回りに90°回転
	/// オーバーフロー時にはパニック
	#[rustfmt::skip]
	#[inline(always)]
	pub fn rotate90(self) -> Self { self.checked_rotate90().expect("rotate90でオーバーフロー") }

	/// 時計回りに90°回転
	/// オーバーフロー時にはパニック
	#[rustfmt::skip]
	#[inline(always)]
	pub fn rotate90_cw(self) -> Self { self.checked_rotate90_cw().expect("rotate90_cwでオーバーフロー") }

	/// 反時計回りに180°回転
	/// オーバーフロー時にはパニック
	#[rustfmt::skip]
	#[inline(always)]
	pub fn rotate180(self) -> Self { self.checked_rotate180().expect("rotate180でオーバーフロー") }

	/// 回転数を含めた角度差が180°未満か
	/// 差が0°ならtrue, ちょうど180°ならfalse
	/// 引数の順序に依存しない
	#[inline(always)]
	pub fn within_half_turn(&self, other:&Self) -> bool { 
		let (lo, hi) = if self <= other { 
			(self, other)
		}else { 
			(other, self)
		};

		// 差を取る前にi128に変換
		let dt  = (hi.turn as i128) - (lo.turn as i128);

		return match dt { 
			// 同じ周回で角度差が180°未満
			0 => lo.same_direction(hi) || lo.cross(hi) > 0,
			// 隣の周回へ0°をまたぎ、角度差が180°未満
			1 => hi.cmp_direction(lo) == Ordering::Less && lo.cross(hi ) > 0,
			_ => false,
		};
	} // end of method

    /// 反時計回りの回転で360°を跨いだら回転数を増加
    #[inline(always)]
    fn carry_ccw(self, rotated: Self) -> Option<Self> {
        return if rotated.cmp_direction(&self) == Ordering::Less {
            rotated.checked_add_turns(1)
        } else {
            Some(rotated)
        };
    } // end of method

    /// 時計回りの回転で0°を跨いだら回転数を減らす
    #[inline(always)]
    fn carry_cw(self, rotated: Self) -> Option<Self> {
        return if rotated.cmp_direction(&self) == Ordering::Greater {
            rotated.checked_add_turns(-1)
        } else {
            Some(rotated)
        };
    } // end of method
} // end of impl SpiralAngle

/// 角度として等しいか判定
/// ベクトルのノルムは考慮しない
impl PartialEq for SpiralAngle {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
} // end of impl PartialEq for SpiralAngle

impl Eq for SpiralAngle {}

// 回転数を優先, 次いで偏角で比較
impl Ord for SpiralAngle {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        return self.turn.cmp(&other.turn).then_with(|| self.cmp_direction(other));
    }
} // end of impl Ord for SpiralAngle

impl PartialOrd for SpiralAngle {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
} // end of impl PartialOrd for SpiralAngle
