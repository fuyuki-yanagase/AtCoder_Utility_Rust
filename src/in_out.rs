// 高速入出力
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
//                                      高速入出力
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/in_out.rs

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
