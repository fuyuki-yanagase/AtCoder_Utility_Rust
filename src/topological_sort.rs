// トポロジカルソート
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
//                                  トポロジカルソート
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/4f9d7e5464c603cf/src/topological_sort.rs
// Library URL : https://github.com/fuyuki-yanagase/AtCoder_Utility_Rust/blob/fb6f94b1d523ce47f174b8f69c53a8d0a8698fa5/src/topological_sort.rs

/// DAG のトポロジカルソートを行う。
/// graph[u] に u -> v の辺を持たせる。
///
/// 戻り値:
/// - DAGなら `Some(order)`
/// - 閉路があるなら `None`
///
/// 計算量:
/// O(N + M)
#[inline]
pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = graph.len();

    // 各頂点への入次数
    let mut indeg = vec![0_usize; n];
    for edges in graph {
        for &to in edges {
            indeg[to] += 1;
        }
    }

    // 入次数0の頂点を全部入れる
    let mut que = VecDeque::with_capacity(n);

    for i in 0..n {
        if indeg[i] == 0 {
            que.push_back(i);
        }
    }

    let mut order = Vec::with_capacity(n);

    while let Some(v) = que.pop_front() {
        order.push(v);

        for &to in &graph[v] {
            indeg[to] -= 1;

            if indeg[to] == 0 {
                que.push_back(to);
            }
        }
    }

    return if order.len() == n { Some(order) } else { None };
}
