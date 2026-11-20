// bfs_01の使用例・テスト。本体をコピーせず、公開モジュールを検証する。

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

use utility_rust::bfs_01::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_vertex() {
        assert_eq!(zero_one_bfs(&vec![vec![]], 0), vec![0]);
    }

    #[test]
    fn unreachable_vertices_and_nonzero_start() {
        let graph = vec![vec![(0, 0)], vec![(2, 1)], vec![], vec![]];
        assert_eq!(zero_one_bfs(&graph, 1), vec![UINF, 0, 1, UINF]);
    }

    #[test]
    fn zero_cost_self_loop() {
        assert_eq!(zero_one_bfs(&vec![vec![(0, 0)]], 0), vec![0]);
    }

    #[test]
    fn zero_cost_cycle() {
        let graph = vec![vec![(1, 0)], vec![(0, 0), (2, 1)], vec![(1, 0)]];
        assert_eq!(zero_one_bfs(&graph, 0), vec![0, 0, 1]);
    }

    #[test]
    fn equal_distance_paths_and_parallel_edges() {
        let graph = vec![vec![(1, 0), (2, 0), (1, 0)], vec![(3, 1), (3, 1)], vec![(3, 1)], vec![]];
        assert_eq!(zero_one_bfs(&graph, 0), vec![0, 0, 0, 1]);
    }

    #[test]
    fn later_shortening_and_stale_queue_entries() {
        // 1と3は距離1で末尾へ入った後に距離0へ改善され、古い要素が残る。
        let graph = vec![vec![(1, 1), (2, 0), (3, 1)], vec![(3, 0)], vec![(1, 0)], vec![(4, 1)], vec![]];
        assert_eq!(zero_one_bfs(&graph, 0), vec![0, 0, 0, 0, 1]);
    }

    #[test]
    fn exhaustive_small_graphs_match_floyd_warshall() {
        // 各有向辺（自己ループを含む）を「なし・0・1」で全列挙する。
        // 独立した全点対最短路と全始点を照合し、隣接辺の逆順でも確認する。
        for n in 1usize..=3 {
            for encoding in 0..3usize.pow((n * n) as u32) {
                let mut graph = vec![vec![]; n];
                let mut expected = vec![vec![None; n]; n];
                let mut remaining = encoding;
                for from in 0..n {
                    expected[from][from] = Some(0usize);
                    for to in 0..n {
                        let edge = remaining % 3;
                        remaining /= 3;
                        if edge != 0 {
                            let cost = edge - 1;
                            graph[from].push((to, cost));
                            expected[from][to] = Some(expected[from][to].map_or(cost, |d| d.min(cost)));
                        }
                    }
                }
                for via in 0..n {
                    for from in 0..n {
                        for to in 0..n {
                            if let (Some(left), Some(right)) = (expected[from][via], expected[via][to]) {
                                let candidate = left + right;
                                expected[from][to] = Some(expected[from][to].map_or(candidate, |d| d.min(candidate)));
                            }
                        }
                    }
                }
                for reverse in [false, true] {
                    if reverse {
                        for edges in &mut graph {
                            edges.reverse();
                        }
                    }
                    for start in 0..n {
                        let distances: Vec<usize> = expected[start].iter().map(|d| d.unwrap_or(UINF)).collect();
                        assert_eq!(
                            zero_one_bfs(&graph, start),
                            distances,
                            "n={n}, encoding={encoding}, start={start}, reverse={reverse}"
                        );
                    }
                }
            }
        }
    }
}

fn example() {
    assert_eq!(zero_one_bfs(&vec![vec![(1, 0)], vec![]], 0), vec![0, 0]);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
