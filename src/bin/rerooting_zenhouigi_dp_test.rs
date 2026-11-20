// rerooting_zenhouigi_dpの使用例・テスト。

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

use utility_rust::monoid_group_abelian::FnMonoid;
use utility_rust::rerooting_zenhouigi_dp::Rerooting;

fn example() {
    let mut tree = Rerooting::new(3);
    tree.add_edge(0, 1, 2_i64);
    tree.add_edge(1, 2, 3_i64);
    let maximum = FnMonoid::new(|| 0_i64, |a: &i64, b: &i64| *a.max(b));
    let distances = tree.solve(&maximum, |_, &v| v, |_, _, &cost, &v| v + cost);
    assert_eq!(distances, vec![5, 3, 5]);
    println!("各頂点からの最遠距離: {distances:?}");
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn empty_singleton_and_distances_match_traversal() {
    let sum = FnMonoid::new(|| (0_usize, 0_i64), |a: &(usize, i64), b: &(usize, i64)| (a.0 + b.0, a.1 + b.1));
    for n in [0, 1, 2, 7, 30] {
        let mut tree = Rerooting::new(n);
        let mut graph = vec![vec![]; n];
        for v in 1..n {
            let parent = (v * 17 + 3) % v;
            let weight = (v % 5 + 1) as i64;
            tree.add_edge(parent, v, weight);
            graph[parent].push((v, weight));
            graph[v].push((parent, weight));
        }
        let result = tree.solve(
            &sum,
            |_, &(count, distance)| (count + 1, distance),
            |_, _, &weight, &(count, distance)| (count, distance + count as i64 * weight),
        );
        for root in 0..n {
            let mut stack = vec![(root, n, 0)];
            let mut expected = 0;
            while let Some((v, parent, distance)) = stack.pop() {
                expected += distance;
                for &(to, weight) in &graph[v] {
                    if to != parent {
                        stack.push((to, v, distance + weight));
                    }
                }
            }
            assert_eq!(result[root], (n, expected));
        }
        assert_eq!(result.len(), n);
    }
}
#[test]
fn noncommutative_monoid_preserves_neighbor_order() {
    let mut tree = Rerooting::new(3);
    tree.add_edge(0, 2, ());
    tree.add_edge(0, 1, ());
    let concat = FnMonoid::new(String::new, |a: &String, b: &String| format!("{a}{b}"));
    let result = tree.solve(
        &concat,
        |v, merged| format!("{v}({merged})"),
        |_, _, _, value| value.to_string(),
    );
    assert_eq!(result, vec!["0(2()1())", "1(0(2()))", "2(0(1()))"]);
}
