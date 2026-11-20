#![allow(unused)]
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::new_without_default)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::zero_repeat_side_effects)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::collapsible_if)]
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Range, Sub};
const M107: i64 = 1000000007_i64;
const M998: i64 = 998244353_i64;
const IINF: i32 = (1_i32 << 29) - 1;
const LINF: i64 = (1_i64 << 61) - 1;
const UINF: usize = (1_usize << 29) - 1;
const FSBLANK: u8 = 32;
const FSMINUS: u8 = 45;
const FSZERO: u8 = 48;
pub struct FastScanner {
    buffer: Vec<u8>,
    index: usize,
}
impl FastScanner {
    pub fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        return FastScanner {
            buffer: input,
            index: 0,
        };
    }
    pub fn skip_blanks(&mut self) {
        while self.index < self.buffer.len() && self.buffer[self.index] <= FSBLANK {
            self.index += 1;
        }
    }
    pub fn readint(&mut self) -> i32 {
        self.skip_blanks();
        let mut sign: i32 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == FSMINUS {
            sign = -1;
            self.index += 1;
        }
        let mut value: i32 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as i32;
            self.index += 1;
        }
        return value * sign;
    }
    pub fn readlong(&mut self) -> i64 {
        self.skip_blanks();
        let mut sign: i64 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == FSMINUS {
            sign = -1;
            self.index += 1;
        }
        let mut value: i64 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as i64;
            self.index += 1;
        }
        return value * sign;
    }
    pub fn readi128(&mut self) -> i128 {
        self.skip_blanks();
        let mut sign: i128 = 1;
        if self.index < self.buffer.len() && self.buffer[self.index] == FSMINUS {
            sign = -1;
            self.index += 1;
        }
        let mut value: i128 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as i128;
            self.index += 1;
        }
        return value * sign;
    }
    pub fn readuint(&mut self) -> u32 {
        self.skip_blanks();
        let mut value: u32 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as u32;
            self.index += 1;
        }
        return value;
    }
    pub fn readulong(&mut self) -> u64 {
        self.skip_blanks();
        let mut value: u64 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as u64;
            self.index += 1;
        }
        return value;
    }
    pub fn readu128(&mut self) -> u128 {
        self.skip_blanks();
        let mut value: u128 = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as u128;
            self.index += 1;
        }
        return value;
    }
    pub fn readusize(&mut self) -> usize {
        self.skip_blanks();
        let mut value: usize = 0;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            value = value * 10 + (self.buffer[self.index] - FSZERO) as usize;
            self.index += 1;
        }
        return value;
    }
    pub fn read(&mut self) -> String {
        self.skip_blanks();
        let start: usize = self.index;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            self.index += 1;
        }
        return String::from_utf8(self.buffer[start..self.index].to_vec()).unwrap();
    }
    pub fn readints(&mut self, n: usize) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readint());
        }
        return result;
    }
    pub fn readlongs(&mut self, n: usize) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readlong());
        }
        return result;
    }
    pub fn readi128s(&mut self, n: usize) -> Vec<i128> {
        let mut result: Vec<i128> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readi128());
        }
        return result;
    }
    pub fn readuints(&mut self, n: usize) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readuint());
        }
        return result;
    }
    pub fn readulongs(&mut self, n: usize) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readulong());
        }
        return result;
    }
    pub fn readu128s(&mut self, n: usize) -> Vec<u128> {
        let mut result: Vec<u128> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readu128());
        }
        return result;
    }
    pub fn readusizes(&mut self, n: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readusize());
        }
        return result;
    }
    pub fn reads(&mut self, n: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.read());
        }
        return result;
    }
    pub fn readbytes(&mut self) -> Vec<u8> {
        self.skip_blanks();
        let start: usize = self.index;
        while self.index < self.buffer.len() && self.buffer[self.index] > FSBLANK {
            self.index += 1;
        }
        return self.buffer[start..self.index].to_vec();
    }
    pub fn readbytess(&mut self, n: usize) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.readbytes());
        }
        return result;
    }
    pub fn readintt2(&mut self) -> (i32, i32) {
        return (self.readint(), self.readint());
    }
    pub fn readintt3(&mut self) -> (i32, i32, i32) {
        return (self.readint(), self.readint(), self.readint());
    }
    pub fn readlongt2(&mut self) -> (i64, i64) {
        return (self.readlong(), self.readlong());
    }
    pub fn readlongt3(&mut self) -> (i64, i64, i64) {
        return (self.readlong(), self.readlong(), self.readlong());
    }
    pub fn readusizet2(&mut self) -> (usize, usize) {
        return (self.readusize(), self.readusize());
    }
    pub fn readusizet3(&mut self) -> (usize, usize, usize) {
        return (self.readusize(), self.readusize(), self.readusize());
    }
}
pub struct FastWriter {
    writer: BufWriter<io::Stdout>,
}
impl FastWriter {
    pub fn new() -> Self {
        return FastWriter {
            writer: BufWriter::new(io::stdout()),
        };
    }
    pub fn write<T: Display>(&mut self, value: T) {
        write!(self.writer, "{}", value).unwrap();
    }
    pub fn writeline<T: Display>(&mut self, value: T) {
        writeln!(self.writer, "{}", value).unwrap();
    }
    pub fn newline(&mut self) {
        writeln!(self.writer).unwrap();
    }
    pub fn writelist<T: Display>(&mut self, values: &[T]) {
        self.writelist_sep(values, " ");
    }
    pub fn writelist_sep<T: Display>(&mut self, values: &[T], separator: &str) {
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                self.write(separator);
            }
            self.write(value);
        }
        self.newline();
    }
    pub fn writelist2<T: Display, R: AsRef<[T]>>(&mut self, values: &[R]) {
        self.writelist2_sep(values, " ");
    }
    pub fn writelist2_sep<T: Display, R: AsRef<[T]>>(&mut self, values: &[R], separator: &str) {
        for row in values.iter() {
            let row_values = row.as_ref();
            self.writelist_sep(row_values, separator);
        }
    }
    pub fn writeiter<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Display,
    {
        self.writeiter_sep(iter, " ");
    }
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
    }
}
#[derive(Clone, Copy)]
struct TreapRandom {
    seed: u64,
}
impl TreapRandom {
    #[inline(always)]
    fn new(seed: u64) -> Self {
        return Self {
            seed,
        };
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        return z ^ (z >> 31);
    }
}
struct ImplicitTreapNode {
    value: i64,
    priority: u64,
    size: usize,
    sum: i64,
    max_value: i64,
    min_value: i64,
    lazy_add: i64,
    reverse: bool,
    left: Option<Box<ImplicitTreapNode>>,
    right: Option<Box<ImplicitTreapNode>>,
}
impl ImplicitTreapNode {
    #[inline(always)]
    fn new(value: i64, priority: u64) -> Self {
        return Self {
            value,
            priority,
            size: 1,
            sum: value,
            max_value: value,
            min_value: value,
            lazy_add: 0,
            reverse: false,
            left: None,
            right: None,
        };
    }
}
#[derive(Default)]
pub struct ImplicitTreap {
    root: Option<Box<ImplicitTreapNode>>,
    random: TreapRandom,
}
impl Default for TreapRandom {
    #[inline(always)]
    fn default() -> Self {
        return Self::new(0xCAFEBABEDEADBEEF);
    }
}
impl ImplicitTreap {
    #[inline(always)]
    pub fn new() -> Self {
        return Self {
            root: None,
            random: TreapRandom::new(0xCAFEBABEDEADBEEF),
        };
    }
    #[inline(always)]
    pub fn with_seed(seed: u64) -> Self {
        return Self {
            root: None,
            random: TreapRandom::new(seed),
        };
    }
    pub fn from_slice(values: &[i64]) -> Self {
        let mut treap = Self::new();
        for &value in values {
            treap.push_back(value);
        }
        return treap;
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        return Self::size_of(&self.root);
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.root.is_none();
    }
    #[inline(always)]
    pub fn clear(&mut self) {
        self.root = None;
    }
    #[inline(always)]
    pub fn split(mut self, left_len: usize) -> (Self, Self) {
        let k = min(left_len, self.len());
        let (left_root, right_root) = Self::split_node(self.root.take(), k);
        let left = Self {
            root: left_root,
            random: TreapRandom::new(self.random.seed ^ 0x1111111111111111),
        };
        let right = Self {
            root: right_root,
            random: TreapRandom::new(self.random.seed ^ 0x2222222222222222),
        };
        return (left, right);
    }
    #[inline(always)]
    pub fn merge(mut left: Self, mut right: Self) -> Self {
        let root = Self::merge_node(left.root.take(), right.root.take());
        return Self {
            root,
            random: TreapRandom::new(left.random.seed ^ right.random.seed ^ 0x3333333333333333),
        };
    }
    #[inline(always)]
    pub fn append(&mut self, mut other: Self) {
        self.root = Self::merge_node(self.root.take(), other.root.take());
    }
    #[inline(always)]
    pub fn push_back(&mut self, value: i64) {
        let node = Some(Box::new(ImplicitTreapNode::new(value, self.random.next_u64())));
        self.root = Self::merge_node(self.root.take(), node);
    }
    #[inline(always)]
    pub fn push_back_with_priority(&mut self, value: i64, priority: u64) {
        let node = Some(Box::new(ImplicitTreapNode::new(value, priority)));
        self.root = Self::merge_node(self.root.take(), node);
    }
    #[inline(always)]
    pub fn push_front(&mut self, value: i64) {
        let node = Some(Box::new(ImplicitTreapNode::new(value, self.random.next_u64())));
        self.root = Self::merge_node(node, self.root.take());
    }
    #[inline(always)]
    pub fn push_front_with_priority(&mut self, value: i64, priority: u64) {
        let node = Some(Box::new(ImplicitTreapNode::new(value, priority)));
        self.root = Self::merge_node(node, self.root.take());
    }
    #[inline(always)]
    pub fn insert(&mut self, index: usize, value: i64) {
        let k = min(index, self.len());
        let (left, right) = Self::split_node(self.root.take(), k);
        let node = Some(Box::new(ImplicitTreapNode::new(value, self.random.next_u64())));
        self.root = Self::merge_node(Self::merge_node(left, node), right);
    }
    #[inline(always)]
    pub fn insert_with_priority(&mut self, index: usize, value: i64, priority: u64) {
        let k = min(index, self.len());
        let (left, right) = Self::split_node(self.root.take(), k);
        let node = Some(Box::new(ImplicitTreapNode::new(value, priority)));
        self.root = Self::merge_node(Self::merge_node(left, node), right);
    }
    #[inline(always)]
    pub fn remove_at(&mut self, index: usize) -> Option<i64> {
        if self.len() <= index {
            return None;
        }
        let (left, rest) = Self::split_node(self.root.take(), index);
        let (middle, right) = Self::split_node(rest, 1);
        let answer = middle.map(|mut node| {
            Self::push_node(&mut node);
            node.value
        });
        self.root = Self::merge_node(left, right);
        return answer;
    }
    #[inline(always)]
    pub fn remove_range(&mut self, left: usize, right: usize) -> Vec<i64> {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return Vec::new();
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (b, c) = Self::split_node(bc, r - l);
        let mut removed = Self {
            root: b,
            random: self.random,
        };
        let answer = removed.to_vec();
        self.root = Self::merge_node(a, c);
        return answer;
    }
    #[inline(always)]
    pub fn get(&mut self, index: usize) -> Option<i64> {
        if self.len() <= index {
            return None;
        }
        return Self::get_node(&mut self.root, index);
    }
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: i64) -> bool {
        if self.len() <= index {
            return false;
        }
        let (left, rest) = Self::split_node(self.root.take(), index);
        let (mut middle, right) = Self::split_node(rest, 1);
        if let Some(node) = middle.as_mut() {
            Self::push_node(node);
            node.value = value;
            node.sum = value;
            node.max_value = value;
            node.min_value = value;
            node.lazy_add = 0;
            Self::update_node(node);
        }
        self.root = Self::merge_node(Self::merge_node(left, middle), right);
        return true;
    }
    #[inline(always)]
    pub fn range_add(&mut self, left: usize, right: usize, delta: i64) {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return;
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (mut b, c) = Self::split_node(bc, r - l);
        if let Some(node) = b.as_mut() {
            Self::apply_add(node, delta);
        }
        self.root = Self::merge_node(Self::merge_node(a, b), c);
    }
    #[inline(always)]
    pub fn range_reverse(&mut self, left: usize, right: usize) {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return;
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (mut b, c) = Self::split_node(bc, r - l);
        if let Some(node) = b.as_mut() {
            Self::apply_reverse(node);
        }
        self.root = Self::merge_node(Self::merge_node(a, b), c);
    }
    #[inline(always)]
    pub fn range_max(&mut self, left: usize, right: usize) -> Option<i64> {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return None;
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (b, c) = Self::split_node(bc, r - l);
        let answer = b.as_ref().map(|node| node.max_value);
        self.root = Self::merge_node(Self::merge_node(a, b), c);
        return answer;
    }
    #[inline(always)]
    pub fn range_min(&mut self, left: usize, right: usize) -> Option<i64> {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return None;
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (b, c) = Self::split_node(bc, r - l);
        let answer = b.as_ref().map(|node| node.min_value);
        self.root = Self::merge_node(Self::merge_node(a, b), c);
        return answer;
    }
    #[inline(always)]
    pub fn range_sum(&mut self, left: usize, right: usize) -> i64 {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return 0;
        }
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (b, c) = Self::split_node(bc, r - l);
        let answer = b.as_ref().map_or(0, |node| node.sum);
        self.root = Self::merge_node(Self::merge_node(a, b), c);
        return answer;
    }
    #[inline(always)]
    pub fn all_max(&self) -> Option<i64> {
        return self.root.as_ref().map(|node| node.max_value);
    }
    #[inline(always)]
    pub fn all_min(&self) -> Option<i64> {
        return self.root.as_ref().map(|node| node.min_value);
    }
    #[inline(always)]
    pub fn all_sum(&self) -> i64 {
        return self.root.as_ref().map_or(0, |node| node.sum);
    }
    #[inline(always)]
    pub fn rotate_right_range(&mut self, left: usize, right: usize, k: usize) {
        let (l, r) = self.normalize_range(left, right);
        let len = r - l;
        if len == 0 {
            return;
        }
        let shift = k % len;
        if shift == 0 {
            return;
        }
        let (a, bcd) = Self::split_node(self.root.take(), l);
        let (bc, d) = Self::split_node(bcd, len);
        let (b, c) = Self::split_node(bc, len - shift);
        let rotated = Self::merge_node(c, b);
        self.root = Self::merge_node(Self::merge_node(a, rotated), d);
    }
    #[inline(always)]
    pub fn rotate_left_range(&mut self, left: usize, right: usize, k: usize) {
        let (l, r) = self.normalize_range(left, right);
        let len = r - l;
        if len == 0 {
            return;
        }
        let shift = k % len;
        if shift == 0 {
            return;
        }
        self.rotate_right_range(l, r, len - shift);
    }
    pub fn move_range(&mut self, left: usize, right: usize, to: usize) {
        let (l, r) = self.normalize_range(left, right);
        if l == r {
            return;
        }
        let len = r - l;
        let (a, bc) = Self::split_node(self.root.take(), l);
        let (b, c) = Self::split_node(bc, len);
        let without = Self::merge_node(a, c);
        let without_len = Self::size_of(&without);
        let insert_pos = min(to, without_len);
        let (left_part, right_part) = Self::split_node(without, insert_pos);
        self.root = Self::merge_node(Self::merge_node(left_part, b), right_part);
    }
    #[inline(always)]
    pub fn to_vec(&mut self) -> Vec<i64> {
        let mut result = Vec::with_capacity(self.len());
        Self::collect_values(&mut self.root, &mut result);
        return result;
    }
    #[inline(always)]
    fn normalize_range(&self, left: usize, right: usize) -> (usize, usize) {
        let len = self.len();
        let l = min(left, len);
        let r = min(right, len);
        if r < l {
            return (l, l);
        }
        return (l, r);
    }
    #[inline(always)]
    fn size_of(node: &Option<Box<ImplicitTreapNode>>) -> usize {
        return node.as_ref().map_or(0, |x| x.size);
    }
    #[inline(always)]
    fn sum_of(node: &Option<Box<ImplicitTreapNode>>) -> i64 {
        return node.as_ref().map_or(0, |x| x.sum);
    }
    #[inline(always)]
    fn max_of(node: &Option<Box<ImplicitTreapNode>>) -> i64 {
        return node.as_ref().map_or(i64::MIN, |x| x.max_value);
    }
    #[inline(always)]
    fn min_of(node: &Option<Box<ImplicitTreapNode>>) -> i64 {
        return node.as_ref().map_or(i64::MAX, |x| x.min_value);
    }
    #[inline(always)]
    fn update_node(node: &mut Box<ImplicitTreapNode>) {
        node.size = 1 + Self::size_of(&node.left) + Self::size_of(&node.right);
        node.sum = node.value + Self::sum_of(&node.left) + Self::sum_of(&node.right);
        node.max_value = max(node.value, max(Self::max_of(&node.left), Self::max_of(&node.right)));
        node.min_value = min(node.value, min(Self::min_of(&node.left), Self::min_of(&node.right)));
    }
    #[inline(always)]
    fn apply_add(node: &mut Box<ImplicitTreapNode>, delta: i64) {
        node.value += delta;
        node.sum += delta * node.size as i64;
        node.max_value += delta;
        node.min_value += delta;
        node.lazy_add += delta;
    }
    #[inline(always)]
    fn apply_reverse(node: &mut Box<ImplicitTreapNode>) {
        swap(&mut node.left, &mut node.right);
        node.reverse = !node.reverse;
    }
    #[inline(always)]
    fn push_node(node: &mut Box<ImplicitTreapNode>) {
        if node.reverse {
            if let Some(left) = node.left.as_mut() {
                Self::apply_reverse(left);
            }
            if let Some(right) = node.right.as_mut() {
                Self::apply_reverse(right);
            }
            node.reverse = false;
        }
        if node.lazy_add != 0 {
            let delta = node.lazy_add;
            if let Some(left) = node.left.as_mut() {
                Self::apply_add(left, delta);
            }
            if let Some(right) = node.right.as_mut() {
                Self::apply_add(right, delta);
            }
            node.lazy_add = 0;
        }
    }
    fn merge_node(
        left: Option<Box<ImplicitTreapNode>>,
        right: Option<Box<ImplicitTreapNode>>,
    ) -> Option<Box<ImplicitTreapNode>> {
        match (left, right) {
            (None, r) => return r,
            (l, None) => return l,
            (Some(mut l), Some(mut r)) => {
                if l.priority > r.priority {
                    Self::push_node(&mut l);
                    l.right = Self::merge_node(l.right.take(), Some(r));
                    Self::update_node(&mut l);
                    return Some(l);
                } else {
                    Self::push_node(&mut r);
                    r.left = Self::merge_node(Some(l), r.left.take());
                    Self::update_node(&mut r);
                    return Some(r);
                }
            }
        }
    }
    fn split_node(
        root: Option<Box<ImplicitTreapNode>>,
        k: usize,
    ) -> (Option<Box<ImplicitTreapNode>>, Option<Box<ImplicitTreapNode>>) {
        let Some(mut node) = root else {
            return (None, None);
        };
        Self::push_node(&mut node);
        let left_size = Self::size_of(&node.left);
        if k <= left_size {
            let (a, b) = Self::split_node(node.left.take(), k);
            node.left = b;
            Self::update_node(&mut node);
            return (a, Some(node));
        } else {
            let (a, b) = Self::split_node(node.right.take(), k - left_size - 1);
            node.right = a;
            Self::update_node(&mut node);
            return (Some(node), b);
        }
    }
    fn get_node(root: &mut Option<Box<ImplicitTreapNode>>, mut index: usize) -> Option<i64> {
        let node = root.as_mut()?;
        Self::push_node(node);
        let left_size = Self::size_of(&node.left);
        if index < left_size {
            return Self::get_node(&mut node.left, index);
        }
        index -= left_size;
        if index == 0 {
            return Some(node.value);
        }
        return Self::get_node(&mut node.right, index - 1);
    }
    fn collect_values(root: &mut Option<Box<ImplicitTreapNode>>, result: &mut Vec<i64>) {
        let Some(node) = root.as_mut() else {
            return;
        };
        Self::push_node(node);
        Self::collect_values(&mut node.left, result);
        result.push(node.value);
        Self::collect_values(&mut node.right, result);
        Self::update_node(node);
    }
}
fn main() {
    let mut scanner = FastScanner::new();
    let mut output = FastWriter::new();
    let mut t: i32 = 1;
    while t > 0 {
        solve3(&mut scanner, &mut output);
        t -= 1;
    }
}
fn solve1(fs: &mut FastScanner, out: &mut FastWriter) {
    let n = fs.readusize();
    let qnum = fs.readusize();
    let mut treap = ImplicitTreap::from_slice(&vec![0_i64; n]);
    for _ in 0..qnum {
        let q = fs.readusize();
        if q == 0 {
            let i = fs.readusize() - 1;
            let x = fs.readlong();
            treap.range_add(i, i + 1, x);
        } else {
            let s = fs.readusize() - 1;
            let t = fs.readusize();
            let sum = treap.range_sum(s, t);
            out.writeline(sum);
        }
    }
}
fn solve2(fs: &mut FastScanner, out: &mut FastWriter) {
    let n = fs.readusize();
    let qnum = fs.readusize();
    let mut treap = ImplicitTreap::from_slice(&vec![0_i64; n]);
    for _ in 0..qnum {
        let q = fs.readusize();
        if q == 0 {
            let s = fs.readusize() - 1;
            let t = fs.readusize() - 1;
            let x = fs.readlong();
            treap.range_add(s, t + 1, x);
        } else {
            let s = fs.readusize() - 1;
            let t = fs.readusize() - 1;
            let sum = treap.range_sum(s, t + 1);
            out.writeline(sum);
        }
    }
}
fn solve3(fs: &mut FastScanner, out: &mut FastWriter) {
    let n = fs.readusize();
    let qnum = fs.readusize();
    let mut treap = ImplicitTreap::from_slice(&vec![0_i64; n]);
    for _ in 0..qnum {
        let q = fs.readusize();
        if q == 0 {
            let s = fs.readusize();
            let t = fs.readusize();
            let x = fs.readlong();
            treap.range_add(s, t + 1, x);
        } else {
            let s = fs.readusize();
            let t = fs.readusize();
            let min = treap.range_min(s, t + 1);
            out.writeline(min.unwrap());
        }
    }
}
