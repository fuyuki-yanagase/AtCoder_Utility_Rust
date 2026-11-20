// 偏角(比較可能)
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
//                                    偏角(比較可能)
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/spiral_angle.rs

/// 回転数を含めた偏角
#[derive(Debug, Clone, Copy)]
pub struct SpiralAngle {
    x: i64,
    y: i64,
    turn: i64,
} // end of struct SpiralAngle

impl SpiralAngle {
    /// ベクトルと回転数から生成
    /// ゼロベクトルは偏角を定義できないので一旦禁止
    pub fn new(x: i64, y: i64, turn: i64) -> Self {
        assert!(x != 0 || y != 0, "ゼロベクトルは偏角を定義できません(´・ω・`)");
        #[rustfmt::skip]
        return Self { x, y, turn };
    } // end of constructor

    /// 回転数0で生成
    #[rustfmt::skip]
	#[inline(always)]
    pub fn from_xy(x: i64, y: i64) -> Self { return Self::new(x, y, 0); }

    /// 方向ベクトルを取得
    #[rustfmt::skip]
	#[inline(always)]
    pub fn vector(&self) -> (i64, i64) { return (self.x, self.y); }

    /// 回転数を取得
    #[rustfmt::skip]
	#[inline(always)]
    pub fn turn(&self) -> i64 { return self.turn; }

    /// 回転数を指定して新しい角度を生成
    #[rustfmt::skip]
	#[inline(always)]
    pub fn with_turn(self, turn: i64) -> Self { return Self {turn, ..self} }

    /// 回転数を加算(i64を超えたらNone)
    #[inline(always)]
    pub fn checked_add_turns(self, delta: i64) -> Option<Self> {
        return Some(Self {
            turn: self.turn.checked_add(delta)?,
            ..self
        });
    } // end of method

    /// 方向ベクトル同士の外積を取得
    /// 正 : 相手の方向が反時計周り側 (0° < 差 < 180°)
    /// 負 : 時計回り側  (0° < 差 < 180°)
    /// 同じ方向 or 逆方向
    pub fn cross(&self, other: &Self) -> i128 {
        return (self.x as i128) * (other.y as i128) - (self.y as i128) * (other.x as i128);
    } // end of method

    /// 回転数を含む角度をラジアンで取得
    /// 浮動小数点による近似値。
    /// 厳密な角度比較にはOrdを使用すること。
    #[inline(always)]
    pub fn radians(&self) -> f64 {
        // let angle = (self.y as f64).atan2(self.x as f64).rem_euclid(TAU);
        let mut angle = (self.y as f64).atan2(self.x as f64);
        if angle < 0.0 {
            angle += TAU;
        }
        // 丸めでTAUになっても0に戻さず、ほぼ360°という近似値を保つ。

        return self.turn as f64 * TAU + angle;
    } // end of method

    /// 回転数を含む角度を度数法で取得
    #[rustfmt::skip]
    pub fn degrees(&self) -> f64 { return self.radians().to_degrees(); }

    /// 回転数を無視して同じ方向か判定
    #[inline(always)]
    pub fn same_direction(&self, other: &Self) -> bool {
        return self.cmp_direction(other) == Ordering::Equal;
    } // end of method

    /// 偏角を2つの半平面に分割(上下)
    /// 0: [0, 180)
    /// 1: [180, 360)
    #[inline(always)]
    fn half(&self) -> u8 {
        if self.y > 0 || (self.y == 0 && self.x > 0) {
            return 0;
        } else {
            return 1;
        }
    } // end of method

    /// 回転数を無視して偏角を比較
    #[inline(always)]
    fn cmp_direction(&self, other: &Self) -> Ordering {
        // 半平面を比較
        let order = self.half().cmp(&other.half());
        returnif!(order != Ordering::Equal, order);

        // 同じ半平面なら外積で大小関係
        return match self.cross(other).cmp(&0) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        };
    } // end of method

    /// 反時計回りに90°回転(オーバーフロー時はNone)
    #[inline(always)]
    pub fn checked_rotate90(self) -> Option<Self> {
        let rotated = Self {
            x: self.y.checked_neg()?,
            y: self.x,
            ..self
        };
        return self.carry_ccw(rotated);
    } // end of method

    /// 時計回りに90°回転(オーバーフロー時はNone)
    #[inline(always)]
    pub fn checked_rotate90_cw(self) -> Option<Self> {
        let rotated = Self {
            x: self.y,
            y: self.x.checked_neg()?,
            ..self
        };
        return self.carry_cw(rotated);
    } // end of method

    /// 反時計回りに180°回転(+180°、オーバーフロー時はNone)
    #[inline(always)]
    pub fn checked_rotate180(self) -> Option<Self> {
        let rotated = Self {
            x: self.x.checked_neg()?,
            y: self.y.checked_neg()?,
            ..self
        };
        return self.carry_ccw(rotated);
    } // end of method

    /// 反時計回りに90°回転(オーバーフロー時はパニック)
    #[rustfmt::skip]
    #[inline(always)]
    pub fn rotate90(self) -> Self { return self.checked_rotate90().expect("rotate90でオーバーフロー"); }

    /// 時計回りに90°回転(オーバーフロー時はパニック)
    #[rustfmt::skip]
    #[inline(always)]
    pub fn rotate90_cw(self) -> Self { return self.checked_rotate90_cw().expect("rotate90_cwでオーバーフロー"); }

    /// 反時計回りに180°回転(+180°、オーバーフロー時はパニック)
    #[rustfmt::skip]
	#[inline(always)]
    pub fn rotate180(self) -> Self { return self.checked_rotate180().expect("rotate180でオーバーフロー"); }

    /// 回転数を含めた角度差が180°未満か
    /// 同じ角度(差0°):true, ちょうど180°:false
    #[inline(always)]
    pub fn within_half_turn(&self, other: &Self) -> bool {
        let (lo, hi) = if self <= other { (self, other) } else { (other, self) };

        // i128で差を取ってオーバーフロー回避
        let dt = hi.turn as i128 - lo.turn as i128;
        return match dt {
            // 同じ周回: hiがloから反時計回りに180°未満
            0 => lo.same_direction(hi) || lo.cross(hi) > 0,
            // 1周差: 0°をまたいで、かつ180°未満
            1 => hi.cmp_direction(lo) == Ordering::Less && lo.cross(hi) > 0,
            // 2周差以上は必ず360°以上
            _ => false,
        };
    } // end of method

    /// 反時計回り回転後の回転数補正
    /// 方向が小さくなった → 360°をまたいだ
    #[inline(always)]
    fn carry_ccw(self, rotated: Self) -> Option<Self> {
        if rotated.cmp_direction(&self) == Ordering::Less {
            return rotated.checked_add_turns(1);
        }
        return Some(rotated);
    } // end of method

    /// 時計回り回転後の補正
    /// 方向が大きくなった → 0°をまたいだ
    #[inline(always)]
    fn carry_cw(self, rotated: Self) -> Option<Self> {
        if rotated.cmp_direction(&self) == Ordering::Greater {
            return rotated.checked_add_turns(-1);
        }
        return Some(rotated);
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
