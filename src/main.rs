// 実行テンプレート
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
//                                   実行テンプレート
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs
// Template URL https://fuyuki-connect.net/posts/atcoder-serial-01-roadmap

/// 高速標準入力
pub struct FastScanner {
    buffer: Vec<u8>,   // 入力全体をバイト列に
    index: usize,      // 現在の読んでいる位置
    interactive: bool, // インタラクティブ問題として動かすか
}
impl FastScanner {
    const BLANK: u8 = b' '; // 空白文字の境界 (32)
    const MINUS: u8 = b'-'; // "-" (45)
    const ZERO: u8 = b'0'; // "0" (48)

    pub fn new() -> Self {
        // 標準入力を取得
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();

        return FastScanner {
            buffer: input,
            index: 0,
            interactive: false,
        };
    } // end of constructor

    /// インタラクティブ問題用
    /// 質問を出力したら必ず out.flush() をすること
    pub fn new_interactive() -> Self {
        return FastScanner {
            buffer: Vec::new(),
            index: 0,
            interactive: true,
        };
    } // end of constructor

    /// インタラクティブ問題用にバッファに再入力
    #[inline(always)]
    pub fn refill(&mut self) -> bool {
        self.buffer.clear();
        self.index = 0;
        let mut line = String::new();

        // ジャッジから次の1行まで待機
        let read_size = io::stdin().read_line(&mut line).unwrap();

        if read_size == 0 {
            return false;
        }
        self.buffer = line.into_bytes();
        return true;
    } // end of method

    /// 空白, 改行, タブなどを読み飛ばす
    #[inline(always)]
    pub fn skip_blanks(&mut self) {
        if !self.interactive {
            while self.index < self.buffer.len() && self.buffer[self.index] <= Self::BLANK {
                self.index += 1;
            }
        } else {
            self.skip_blanks_interactive();
        }

        debug_assert!(self.index < self.buffer.len(), "入力を読みすぎていますʕ•ᴥ•ʔ");
    } // end of method

    /// 空白, 改行, タブなどを読み飛ばす
    #[inline(always)]
    pub fn skip_blanks_interactive(&mut self) {
        loop {
            // 現在のバッファ内の空白を読み飛ばす
            while self.index < self.buffer.len() && self.buffer[self.index] <= Self::BLANK {
                self.index += 1;
            }
            if self.index < self.buffer.len() {
                return;
            }
            // インタラクティブなら次の1行を待つ
            if !self.refill() {
                panic!("標準入力が終了");
            }
        }
    } // end of method

    /// i32で1つ変数を読みこみ
    #[inline(always)]
    pub fn readint(&mut self) -> i32 {
        self.skip_blanks();

        // 正負
        let mut sign: i32 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == Self::MINUS {
            sign = -1;
            self.index += 1;
        }

        // 数字を読んでintに
        let mut value: i32 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as i32;
            self.index += 1;
        }

        return value * sign;
    } // end of readint

    /// i64で1つ変数を読みこみ
    #[inline(always)]
    pub fn readlong(&mut self) -> i64 {
        self.skip_blanks();

        // 正負
        let mut sign: i64 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == Self::MINUS {
            sign = -1;
            self.index += 1;
        }

        // 数字を読んでlongに
        let mut value: i64 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as i64;
            self.index += 1;
        }

        return value * sign;
    } // end of readlong

    /// i128で1つ変数を読みこみ
    #[inline(always)]
    pub fn readi128(&mut self) -> i128 {
        self.skip_blanks();

        // 正負
        let mut sign: i128 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == Self::MINUS {
            sign = -1;
            self.index += 1;
        }

        // 数字を読んでi128に
        let mut value: i128 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as i128;
            self.index += 1;
        }

        return value * sign;
    } // end of readi128

    /// u32で1つ変数を読み込み
    #[inline(always)]
    pub fn readuint(&mut self) -> u32 {
        self.skip_blanks();

        // 非負整数として読む
        let mut value: u32 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as u32;
            self.index += 1;
        }

        return value;
    } // end of readuint

    /// u64で1つ変数を読み込み
    #[inline(always)]
    pub fn readulong(&mut self) -> u64 {
        self.skip_blanks();

        // 非負整数として読む
        let mut value: u64 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as u64;
            self.index += 1;
        }

        return value;
    } // end of readulong

    /// u128で1つ変数を読み込み
    #[inline(always)]
    pub fn readu128(&mut self) -> u128 {
        self.skip_blanks();

        // 非負整数として読む
        let mut value: u128 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as u128;
            self.index += 1;
        }

        return value;
    } // end of readu128

    /// usizeで1つ変数を読み込み
    #[inline(always)]
    pub fn readusize(&mut self) -> usize {
        self.skip_blanks();

        // 非負整数として読む
        let mut value: usize = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            value = value * 10 + (self.buffer[self.index] - Self::ZERO) as usize;
            self.index += 1;
        }

        return value;
    } // end of readusize

    /// f64で1つ変数を読み込み
    #[inline(always)]
    pub fn readdouble(&mut self) -> f64 {
        self.skip_blanks();

        // 数値文字列の開始
        let start: usize = self.index;

        // 次の空白まで
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            self.index += 1;
        }

        // 読み取ったバイト列を文字列として解釈してf64に
        return std::str::from_utf8(&self.buffer[start..self.index]).unwrap().parse::<f64>().unwrap();
    } // end of readdouble

    /// 文字列を入力
    #[inline(always)]
    pub fn read(&mut self) -> String {
        self.skip_blanks();

        // 文字列の開始位置
        let start: usize = self.index;

        // 次の空白まで
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            self.index += 1;
        }

        // UTF-8としてStringに
        return String::from_utf8(self.buffer[start..self.index].to_vec()).unwrap();
    } // end of read

    /// i32の配列を読みこみ
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readints(&mut self, n: usize) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readint()); }
        return result;
    } // end of readints

    /// i64の配列を読みこみ
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readlongs(&mut self, n: usize) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readlong()); }
        return result;
    } // end of readlongs

    /// i128の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readi128s(&mut self, n: usize) -> Vec<i128> {
        let mut result: Vec<i128> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readi128()); }
        return result;
    } // end of readi128s

    /// u32の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readuints(&mut self, n: usize) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readuint()); }
        return result;
    } // end of readuints

    /// u64の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readulongs(&mut self, n: usize) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readulong()); }
        return result;
    } // end of readulongs

    /// u128の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readu128s(&mut self, n: usize) -> Vec<u128> {
        let mut result: Vec<u128> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readu128()); }
        return result;
    } // end of readu128s

    /// usizeの配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readusizes(&mut self, n: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readusize()); }
        return result;
    } // end of readusizes

    /// f64の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readdoubles(&mut self, n: usize) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readdouble()); }
        return result;
    } // end of readdoubles

    /// 差分とパディングを指定してi32の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readintswith(&mut self, n: usize, diff: i32, padding_left: usize, padding_right: usize, padding_value: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(padding_left + n + padding_right);
        // 前方padding
        result.extend(std::iter::repeat_n(padding_value, padding_left));

        // 本体
        for _ in 0..n {
            result.push(self.readint() + diff);
        }

        // 後方padding
        result.extend(std::iter::repeat_n(padding_value, padding_right));

        return result;
    } // end of readintswith

    /// 差分とパディング指定してi64の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readlongswith(&mut self, n: usize, diff: i64, padding_left: usize, padding_right: usize, padding_value: i64) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readlong() + diff); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readlongswith

    /// 差分とパディング指定してi128の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readi128swith(&mut self, n: usize, diff: i128, padding_left: usize, padding_right: usize, padding_value: i128) -> Vec<i128> {
        let mut result: Vec<i128> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readi128() + diff); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readi128swith

    /// 差分とパディング指定してu32の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readuintswith(&mut self, n: usize, diff: i32, padding_left: usize, padding_right: usize, padding_value: u32) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readuint().wrapping_add_signed(diff)); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readuintswith

    /// 差分とパディング指定してu64の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readulongswith(&mut self, n: usize, diff: i64, padding_left: usize, padding_right: usize, padding_value: u64) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readulong().wrapping_add_signed(diff)); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readulongswith

    /// 差分とパディング指定してu128の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readu128swith(&mut self, n: usize, diff: i128, padding_left: usize, padding_right: usize, padding_value: u128) -> Vec<u128> {
        let mut result: Vec<u128> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readu128().wrapping_add_signed(diff)); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readu128swith

    /// 差分とパディング指定してusizeの配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readusizeswith(&mut self, n: usize, diff: isize, padding_left: usize, padding_right: usize, padding_value: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readusize().wrapping_add_signed(diff)); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readusizeswith

    /// 差分とパディング指定してf64の配列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readdoubleswith(&mut self, n: usize, diff: f64, padding_left: usize, padding_right: usize, padding_value: f64) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::with_capacity(padding_left + n + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        for _ in 0..n { result.push(self.readdouble() + diff); }
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readdoubleswith

    /// 文字列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn reads(&mut self, n: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.read()); }
        return result;
    } // end of reads

    /// バイト列を読みこみ
    #[inline(always)]
    pub fn readbytes(&mut self) -> Vec<u8> {
        self.skip_blanks();

        // 文字列の開始位置
        let start: usize = self.index;

        // 次の空白まで
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK {
            self.index += 1;
        }

        return self.buffer[start..self.index].to_vec();
    } // end of readbytes

    /// 差分とパディングを指定してbyte列を読み込み
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readbyteswith(&mut self, diff: i8, padding_left: usize, padding_right: usize, padding_value: u8) -> Vec<u8> {
        self.skip_blanks();

        // 文字列の開始位置から次の空白まで
        let start: usize = self.index;
        while self.index < self.buffer.len() && self.buffer[self.index] > Self::BLANK { self.index += 1; }
        let body: &[u8] = &self.buffer[start..self.index];

        let mut result: Vec<u8> = Vec::with_capacity(padding_left + body.len() + padding_right);
        result.extend(std::iter::repeat_n(padding_value, padding_left));
        result.extend(body.iter().map(|&c| c.wrapping_add_signed(diff)));
        result.extend(std::iter::repeat_n(padding_value, padding_right));
        return result;
    } // end of readbyteswith

    /// バイト列を二次元配列で読み込み(グリッド問題など)
    #[rustfmt::skip]
    #[inline(always)]
    pub fn readbytess(&mut self, n: usize) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n { result.push(self.readbytes()); }
        return result;
    } // end of readbytess

    /// タプル読み込み
    pub fn readintt2(&mut self) -> (i32, i32) {
        return (self.readint(), self.readint());
    } // end of readintt2
    pub fn readintt3(&mut self) -> (i32, i32, i32) {
        return (self.readint(), self.readint(), self.readint());
    } // end of readintt3
    pub fn readintt4(&mut self) -> (i32, i32, i32, i32) {
        return (self.readint(), self.readint(), self.readint(), self.readint());
    } // end of readintt4
    pub fn readlongt2(&mut self) -> (i64, i64) {
        return (self.readlong(), self.readlong());
    } // end of readlongt2
    pub fn readlongt3(&mut self) -> (i64, i64, i64) {
        return (self.readlong(), self.readlong(), self.readlong());
    } // end of readlongt3
    pub fn readlongt4(&mut self) -> (i64, i64, i64, i64) {
        return (self.readlong(), self.readlong(), self.readlong(), self.readlong());
    } // end of readlongt4
    pub fn readusizet2(&mut self) -> (usize, usize) {
        return (self.readusize(), self.readusize());
    } // end of readusizet2
    pub fn readusizet3(&mut self) -> (usize, usize, usize) {
        return (self.readusize(), self.readusize(), self.readusize());
    } // end of readusizet3
    pub fn readusizet4(&mut self) -> (usize, usize, usize, usize) {
        return (self.readusize(), self.readusize(), self.readusize(), self.readusize());
    } // end of readusizet4
} // end of FastScanner

/// 高速標準出力
pub struct FastWriter {
    writer: BufWriter<io::Stdout>,
}
impl FastWriter {
    pub fn new() -> Self {
        return FastWriter {
            writer: BufWriter::new(io::stdout()),
        };
    } // end of constructor

    /// 改行なしで1つ出力
    #[inline(always)]
    pub fn write<T: Display>(&mut self, value: T) {
        write!(self.writer, "{}", value).unwrap();
    } // end of write

    /// Debug形式で改行なし出力
    /// タプルやHashMapなど
    #[inline(always)]
    pub fn writedebug<T: Debug + ?Sized>(&mut self, value: &T) {
        write!(self.writer, "{:?}", value).unwrap();
    } // end of method

    /// Debug形式で改行あり出力
    /// タプルやHashMapなど
    #[inline(always)]
    pub fn writedebugline<T: Debug + ?Sized>(&mut self, value: &T) {
        writeln!(self.writer, "{:?}", value).unwrap();
    } // end of writedebugline

    /// 改行ありで1つ出力
    #[inline(always)]
    pub fn writeline<T: Display>(&mut self, value: T) {
        writeln!(self.writer, "{}", value).unwrap();
    } // end of writeline

    /// 改行のみ出力
    #[inline(always)]
    pub fn newline(&mut self) {
        writeln!(self.writer).unwrap();
    } // end of newline

    /// 空白区切りでVecや配列のスライスを出力
    #[inline(always)]
    pub fn writelist<T: Display>(&mut self, values: &[T]) {
        self.writelist_sep(values, " ");
    } // end of writelist

    /// 区切り文字をつけてVecや配列のスライスを出力
    #[inline(always)]
    pub fn writelist_sep<T: Display>(&mut self, values: &[T], separator: &str) {
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                self.write(separator);
            }
            self.write(value);
        }
        self.newline();
    } // end of writelist_sep

    /// 区切り文字をつけて二次元配列を出力
    #[inline(always)]
    pub fn writelist2<T: Display, R: AsRef<[T]>>(&mut self, values: &[R]) {
        self.writelist2_sep(values, " ");
    } // end of writelist2

    /// 区切り文字をつけて二次元配列を出力
    /// rowごとに1行ずつ出力する
    /// separatorは同じ行の要素間にだけ入れる
    #[inline(always)]
    pub fn writelist2_sep<T: Display, R: AsRef<[T]>>(&mut self, values: &[R], separator: &str) {
        // 二次元配列の各行を順番に見る
        for row in values.iter() {
            // Vec<T> や [T; N] を &[T] として扱う
            let row_values = row.as_ref();

            // 1行分を区切り文字つきで出力する
            self.writelist_sep(row_values, separator);
        }
    } // end of writelist2_sep

    /// 空白区切りでイテレータを出力
    #[inline(always)]
    pub fn writeiter<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Display,
    {
        self.writeiter_sep(iter, " ");
    } // end of writeiter

    /// 区切り文字をつけてイテレータを出力
    #[inline(always)]
    pub fn writeiter_sep<I, T>(&mut self, iter: I, separator: &str)
    where
        I: IntoIterator<Item = T>,
        T: Display,
    {
        let mut isfirst = true;
        for value in iter {
            if isfirst {
                isfirst = false;
            } else {
                self.write(separator);
            }
            self.write(value);
        }
        self.newline();
    } // end of writeiter_sep

    /// バッファに残っている出力を強制的に標準出力へ流す
    #[inline(always)]
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    } // end of flush
} // end of FastWriter

fn main() {
    const INTERACTIVE: bool = false;
    let mut fs = if INTERACTIVE { FastScanner::new_interactive() } else { FastScanner::new() };
    let mut out = FastWriter::new();
    let mut t: i32 = 1;
    for _ in 0..t {
        solve(&mut fs, &mut out);
    }
    out.flush();
}

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                        ModInt
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

/// 固定法の整数。対応する法は 2..=2^31-1
/// value は常に 0..MOD
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModInt<const MOD: u32> {
    value: u32,
}

impl<const MOD: u32> ModInt<MOD> {
    // 使用時に定数評価されるため、演算ごとの実行時チェック不要
    const CHECK_MOD: () = assert!(2 <= MOD && MOD <= i32::MAX as u32);

    pub const ZERO: Self = {
        let () = Self::CHECK_MOD;
        Self {
            value: 0,
        }
    };

    pub const ONE: Self = {
        let () = Self::CHECK_MOD;
        Self {
            value: 1,
        }
    };

    /// 整数を正規化して生成。負の値にも対応
    #[inline]
    pub fn new<T: Into<Self>>(value: T) -> Self {
        return value.into();
    }

    /// 正規化済みの値から生成。剰余計算は行わない
    /// value >= MOD は debug ビルドでのみ panic（release ではチェックしない）
    #[inline]
    pub const fn raw(value: u32) -> Self {
        let () = Self::CHECK_MOD;
        debug_assert!(value < MOD, "値は法未満にしてねー(´・ω・｀)");
        return Self {
            value,
        };
    }

    #[inline]
    pub const fn val(self) -> u32 {
        return self.value;
    }

    #[inline]
    pub const fn modulus() -> u32 {
        let () = Self::CHECK_MOD;
        return MOD;
    }

    // ---------- 演算の本体（const fn なのでコンパイル時テーブルにも使える） ----------

    #[inline]
    const fn add_impl(self, rhs: Self) -> Self {
        // MOD <= 2^31-1 のため u32 に収まる。cmov になる
        let v = self.value + rhs.value;
        return Self {
            value: if v >= MOD { v - MOD } else { v },
        };
    }

    #[inline]
    const fn sub_impl(self, rhs: Self) -> Self {
        let value = if self.value >= rhs.value { self.value - rhs.value } else { self.value + MOD - rhs.value };
        return Self {
            value,
        };
    }

    #[inline]
    const fn mul_impl(self, rhs: Self) -> Self {
        // MOD はコンパイル時定数なので、% は逆数乗算に最適化される
        return Self {
            value: (self.value as u64 * rhs.value as u64 % MOD as u64) as u32,
        };
    }

    #[inline]
    const fn div_impl(self, rhs: Self) -> Self {
        return self.mul_impl(rhs.inv());
    }

    #[inline]
    const fn neg_impl(self) -> Self {
        return Self {
            value: if self.value == 0 { 0 } else { MOD - self.value },
        };
    }

    /// 繰り返し二乗法 O(log exp)。0^0 = 1
    #[inline]
    pub const fn pow(self, mut exponent: u64) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while exponent > 0 {
            if (exponent & 1) != 0 {
                result = result.mul_impl(base);
            }
            base = base.mul_impl(base);
            exponent >>= 1;
        }
        return result;
    }

    /// 逆元 O(log MOD)。存在しなければ None
    /// 法が合成数でも gcd(value, MOD) == 1 なら使用可能
    pub const fn checked_inv(self) -> Option<Self> {
        // 除算は 32bit で行い、係数だけ i64 で持つ
        let mut a = MOD;
        let mut b = self.value;
        let mut x = 0_i64;
        let mut y = 1_i64;

        while b != 0 {
            let q = a / b;
            let r = a - q * b;
            a = b;
            b = r;
            let ny = x - q as i64 * y;
            x = y;
            y = ny;
        }
        returnif!(a != 1, None);

        // この係数は -MOD < x < MOD に収まるので剰余計算不要
        let value = if x < 0 { x + MOD as i64 } else { x };
        return Some(Self {
            value: value as u32,
        });
    }

    /// 逆元。存在しなければ panic
    #[inline]
    pub const fn inv(self) -> Self {
        return match self.checked_inv() {
            Some(v) => v,
            None => panic!("逆元が存在しないよー(´・ω・｀)"),
        };
    }
}

// ---------- 整数からの変換 ----------

// 入力型の範囲を保てる整数型で正規化する。
// 普通の整数の変換に i128/u128 の剰余計算を持ち込まない。
macro_rules! impl_modint_from {
    ($wide:ty; $($t:ty),+ $(,)?) => {
        $(
            impl<const MOD: u32> From<$t> for ModInt<MOD> {
                #[inline]
                fn from(value: $t) -> Self {
                    let () = Self::CHECK_MOD;
                    Self {
                        value: (value as $wide).rem_euclid(MOD as $wide) as u32,
                    }
                }
            }
        )+
    };
}

impl_modint_from!(i64; i8, i16, i32, i64, isize);
impl_modint_from!(u64; u8, u16, u32, u64, usize);
impl_modint_from!(i128; i128);
impl_modint_from!(u128; u128);

// `a + &b` を書けるようにする
impl<'a, const MOD: u32> From<&'a ModInt<MOD>> for ModInt<MOD> {
    #[inline]
    fn from(value: &'a ModInt<MOD>) -> Self {
        return *value;
    }
}

// ---------- 四則演算 ----------
// 右辺は Into<ModInt> なら何でも可: x + y, x * 2, x -= 1, x / &y など
// 左辺は値と参照の両方に対応: &x + 1 など

macro_rules! impl_modint_binop {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $body:ident) => {
        impl<const MOD: u32, T: Into<ModInt<MOD>>> $trait<T> for ModInt<MOD> {
            type Output = ModInt<MOD>;
            #[inline]
            fn $method(self, rhs: T) -> ModInt<MOD> {
                return self.$body(rhs.into());
            }
        }

        impl<'a, const MOD: u32, T: Into<ModInt<MOD>>> $trait<T> for &'a ModInt<MOD> {
            type Output = ModInt<MOD>;
            #[inline]
            fn $method(self, rhs: T) -> ModInt<MOD> {
                return (*self).$body(rhs.into());
            }
        }

        impl<const MOD: u32, T: Into<ModInt<MOD>>> $assign_trait<T> for ModInt<MOD> {
            #[inline]
            fn $assign_method(&mut self, rhs: T) {
                *self = (*self).$body(rhs.into());
            }
        }
    };
}

impl_modint_binop!(Add, add, AddAssign, add_assign, add_impl);
impl_modint_binop!(Sub, sub, SubAssign, sub_assign, sub_impl);
impl_modint_binop!(Mul, mul, MulAssign, mul_assign, mul_impl);
impl_modint_binop!(Div, div, DivAssign, div_assign, div_impl);

impl<const MOD: u32> Neg for ModInt<MOD> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        return self.neg_impl();
    }
}

impl<'a, const MOD: u32> Neg for &'a ModInt<MOD> {
    type Output = ModInt<MOD>;
    #[inline]
    fn neg(self) -> ModInt<MOD> {
        return (*self).neg_impl();
    }
}

// ---------- イテレータ ----------

impl<const MOD: u32> Sum for ModInt<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        return iter.fold(Self::ZERO, |a, b| a.add_impl(b));
    }
}

impl<'a, const MOD: u32> Sum<&'a Self> for ModInt<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        return iter.fold(Self::ZERO, |a, b| a.add_impl(*b));
    }
}

impl<const MOD: u32> Product for ModInt<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        return iter.fold(Self::ONE, |a, b| a.mul_impl(b));
    }
}

impl<'a, const MOD: u32> Product<&'a Self> for ModInt<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        return iter.fold(Self::ONE, |a, b| a.mul_impl(*b));
    }
}

// ---------- その他のトレイト ----------

impl<const MOD: u32> Default for ModInt<MOD> {
    #[inline]
    fn default() -> Self {
        return Self::ZERO;
    }
}

impl<const MOD: u32> fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::Display::fmt(&self.value, f);
    }
}

// dbg! で値だけ表示する
impl<const MOD: u32> fmt::Debug for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return fmt::Debug::fmt(&self.value, f);
    }
}

// ---------- 数字列からの生成 ----------

impl<const MOD: u32> ModInt<MOD> {
    /// 10進の数字列（先頭に '+' / '-' 可）から生成。
    /// Vec<u8>, &[u8], &Vec<u8>, &str, String など AsRef<[u8]> なら何でも渡せる。
    /// 1桁ずつ処理するので、i64 に収まらない巨大な数字列も読める
    pub fn try_from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, String> {
        let bytes = bytes.as_ref();
        let (neg, digits) = match bytes.split_first() {
            Some((b'-', rest)) => (true, rest),
            Some((b'+', rest)) => (false, rest),
            _ => (false, bytes),
        };

        let error = || format!("数値じゃないよー(´・ω・｀): {:?}", String::from_utf8_lossy(bytes));
        returnif!(digits.is_empty(), Err(error()));

        let ten = Self::raw(10 % MOD);
        let mut v = Self::ZERO;
        for &c in digits {
            if !c.is_ascii_digit() {
                return Err(error());
            }
            v = v.mul_impl(ten).add_impl(Self::raw((c - b'0') as u32 % MOD));
        }
        return Ok(if neg { v.neg_impl() } else { v });
    }

    /// try_from_bytes の panic 版。不正な入力なら panic
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Self {
        return match Self::try_from_bytes(bytes) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
    }
}

/// proconio の input! で直接読める
impl<const MOD: u32> FromStr for ModInt<MOD> {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::try_from_bytes(s);
    }
}

// ======================================================================================
//
//                               Permutation / Combination
//
// ======================================================================================

/// 階乗と逆階乗を前計算し、順列 nPr と組合せ nCr を O(1) で求める。
///
/// 前提：MOD は素数で、max_n < MOD であること。
/// 構築：O(max_n + log MOD)、メモリ：O(max_n)
/// 各クエリ：O(1)
///
/// ```
/// use utility_rust::modint::Combinatorics;
///
/// let comb = Combinatorics::<998244353>::new(10);
/// assert_eq!(comb.permutation(5, 2).val(), 20);
/// assert_eq!(comb.combination(5, 2).val(), 10);
/// ```
pub struct Combinatorics<const MOD: u32> {
    fact: Vec<ModInt<MOD>>,
    inv_fact: Vec<ModInt<MOD>>,
}

impl<const MOD: u32> Combinatorics<MOD> {
    /// 0! から max_n! までの階乗・逆階乗を構築する。
    pub fn new(max_n: usize) -> Self {
        // 素数 MOD のもとで n! の逆元を利用するので、n < MOD が必要。
        assert!(max_n < MOD as usize, "max_n は MOD 未満にしてください");

        // fact[i] = i!
        let mut fact = vec![ModInt::<MOD>::ONE; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i;
        }

        // inv_fact[i] = 1 / i!
        // 逆元は max_n! に対して一度だけ計算し、残りは逆順に求める。
        let mut inv_fact = vec![ModInt::<MOD>::ONE; max_n + 1];
        inv_fact[max_n] = fact[max_n].inv();
        for i in (1..=max_n).rev() {
            inv_fact[i - 1] = inv_fact[i] * i;
        }

        return Self {
            fact,
            inv_fact,
        };
    }

    /// n! を O(1) で取得する。
    #[inline]
    pub fn factorial(&self, n: usize) -> ModInt<MOD> {
        return self.fact[n];
    }

    /// nPr = n! / (n-r)! を O(1) で取得する。
    /// r > n の場合は 0 を返す。
    #[inline]
    pub fn permutation(&self, n: usize, r: usize) -> ModInt<MOD> {
        assert!(n < self.fact.len(), "n が前計算の範囲を超えています");
        if r > n {
            return ModInt::<MOD>::ZERO;
        }
        return self.fact[n] * self.inv_fact[n - r];
    }

    /// nCr = n! / (r! * (n-r)!) を O(1) で取得する。
    /// r > n の場合は 0 を返す。
    #[inline]
    pub fn combination(&self, n: usize, r: usize) -> ModInt<MOD> {
        assert!(n < self.fact.len(), "n が前計算の範囲を超えています");
        if r > n {
            return ModInt::<MOD>::ZERO;
        }
        return self.fact[n] * self.inv_fact[r] * self.inv_fact[n - r];
    }
}

pub type ModInt998 = ModInt<998244353>;
pub type ModInt107 = ModInt<1000000007>;

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/main.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/main.rs

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

fn solve(fs: &mut FastScanner, out: &mut FastWriter) {
    let (n, m) = fs.readusizet2();
    let mut arr = vec![HashMap::<usize, i64>::new(); n];
    let mut counts = vec![0_i64; n];
    for _ in 0..m {
        let a = fs.readusize() - 1;
        let b = fs.readusize() - 1;
        *arr[a].entry(b).or_insert(0) += 1;
        counts[a] += 1;
    }

    let values: Vec<_> = counts.iter().enumerate().map(|(i, &value)| indexed::IndexedValue::new(value, i)).collect();
    let mut lazy = lazy_from_operator::<indexed::RangeAddMaxIndex<i64>>(&values);

    for i in 0..n {
        let len = counts[i];
        lazy.update(i, i + 1, -len);
        for (&key, &value) in arr[i].iter() {
            lazy.update(key, key + 1, value);
        }
        // out.writedebugline(&lazy.to_vec());
        out.writeline(lazy.query(0, n).index + 1);
        for (&key, &value) in arr[i].iter() {
            lazy.update(key, key + 1, -value);
        }
        lazy.update(i, i + 1, len);
    }
}
