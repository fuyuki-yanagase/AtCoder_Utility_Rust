// priority_queueの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::zero_repeat_side_effects)]

use num_rational::Ratio;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::f64::consts::{PI, TAU};
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

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

use utility_rust::priority_queue::*;

fn test() {
    let mut pq = PriorityQueue::new(|a: &i64, b: &i64| {
        return b.cmp(a);
    });

    // let mut pq: PriorityQueue<i64, fn(&i64, &i64) -> Ordering> =
    // PriorityQueue::new(|a: &i64, b: &i64| -> Ordering {
    //     return b.cmp(a);
    // });

    pq.push(5);
    pq.push(1);
    pq.push(10);
    pq.push(3);

    while let Some(value) = pq.pop() {
        println!("{}", value);
    }
}

fn example() {
    test();
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}

#[test]
fn empty_duplicates_and_both_orders() {
    let mut min = OrdPriorityQueue::<i32>::new_min();
    let mut max = OrdPriorityQueue::<i32>::new_max();
    assert_eq!(min.pop(), None);
    for v in [3, -1, 3, 0] {
        min.push(v);
        max.push(v);
    }
    let mut ascending = Vec::new();
    let mut descending = Vec::new();
    while let Some(v) = min.pop() {
        ascending.push(v);
    }
    while let Some(v) = max.pop() {
        descending.push(v);
    }
    assert_eq!(ascending, vec![-1, 0, 3, 3]);
    assert_eq!(descending, vec![3, 3, 0, -1]);
    assert_eq!(max.pop(), None);
}

#[test]
fn closure_comparison_and_capacity() {
    // 比較でLessになる値を優先するので、通常のcmpで最小値が先頭になる。
    let mut queue = PriorityQueue::with_capacity(4, |a: &i64, b: &i64| a.cmp(b));
    assert!(queue.is_empty());
    assert_eq!(queue.peek(), None);
    for v in [3, 1, 5, 1] {
        queue.push(v);
    }
    assert_eq!(queue.len(), 4);
    assert_eq!(queue.peek(), Some(&1));
    let mut result = Vec::new();
    while let Some(v) = queue.pop() {
        result.push(v);
    }
    assert_eq!(result, vec![1, 1, 3, 5]);
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
}
