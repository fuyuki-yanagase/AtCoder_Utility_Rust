// strongly_connected_component_decompositionの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

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
#![allow(clippy::bool_assert_comparison)]

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

use utility_rust::strongly_connected_component_decomposition::*;

/// 外部クレート不要の簡易乱数（xorshift64）
struct TestXorShift(u64);

impl TestXorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// 0..n の乱数
    fn range(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// 辺リストから隣接リストを作る
fn test_build(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges {
        g[u].push(v);
    }
    g
}

/// 愚直解と照合して scc の結果を検証する
/// 1. 同じ成分 ⇔ 互いに到達可能
/// 2. 辺 u -> v について comp[u] <= comp[v]（トポロジカル順）
/// 3. 成分番号が 0..k で隙間なく使われている
fn test_check(graph: &[Vec<usize>], comp: &[usize]) {
    let n = graph.len();
    assert_eq!(comp.len(), n);

    // 推移閉包（ワーシャル・フロイド）で到達可能性を求める
    let mut reach = vec![vec![false; n]; n];
    for u in 0..n {
        reach[u][u] = true;
        for &v in &graph[u] {
            reach[u][v] = true;
        }
    }
    for k in 0..n {
        for i in 0..n {
            if reach[i][k] {
                for j in 0..n {
                    if reach[k][j] {
                        reach[i][j] = true;
                    }
                }
            }
        }
    }

    // 1. 同じ成分 ⇔ 互いに到達可能
    for u in 0..n {
        for v in 0..n {
            let same = comp[u] == comp[v];
            let mutual = reach[u][v] && reach[v][u];
            assert_eq!(same, mutual, "u={u}, v={v}, comp={comp:?}, graph={graph:?}");
        }
    }

    // 2. トポロジカル順
    for u in 0..n {
        for &v in &graph[u] {
            assert!(comp[u] <= comp[v], "edge {u}->{v}, comp={comp:?}, graph={graph:?}");
        }
    }

    // 3. 成分番号が 0..k で連続
    if n > 0 {
        let k = *comp.iter().max().unwrap() + 1;
        let mut used = vec![false; k];
        for &c in comp {
            used[c] = true;
        }
        assert!(used.iter().all(|&b| b), "comp={comp:?}");
    }
}

fn test_empty_graph() {
    let g: Vec<Vec<usize>> = vec![];
    assert_eq!(scc(&g), Vec::<usize>::new());
}

fn test_single_vertex() {
    let g = test_build(1, &[]);
    assert_eq!(scc(&g), vec![0]);
}

fn test_self_loop_and_multi_edge() {
    // 0 -> 0, 0 -> 1 (2本), 1 -> 1
    let g = test_build(2, &[(0, 0), (0, 1), (0, 1), (1, 1)]);
    assert_eq!(scc(&g), vec![0, 1]);
}

fn test_simple_cycle() {
    let g = test_build(3, &[(0, 1), (1, 2), (2, 0)]);
    assert_eq!(scc(&g), vec![0, 0, 0]);
}

fn test_dag_path() {
    // 2 -> 1 -> 0 （番号の逆向き）でもトポロジカル順に番号が振られるか
    let g = test_build(3, &[(2, 1), (1, 0)]);
    assert_eq!(scc(&g), vec![2, 1, 0]);
}

fn test_two_cycles_connected() {
    // {0,1} -> {2,3}
    let g = test_build(4, &[(0, 1), (1, 0), (1, 2), (2, 3), (3, 2)]);
    assert_eq!(scc(&g), vec![0, 0, 1, 1]);
}

fn test_disconnected() {
    // {0,1} と {2} と {3,4} がバラバラ
    let g = test_build(5, &[(0, 1), (1, 0), (3, 4), (4, 3)]);
    let comp = scc(&g);
    test_check(&g, &comp);
    assert_eq!(comp[0], comp[1]);
    assert_eq!(comp[3], comp[4]);
    assert_ne!(comp[0], comp[2]);
    assert_ne!(comp[0], comp[3]);
    assert_ne!(comp[2], comp[3]);
}

fn test_random_small_graphs() {
    let mut rng = TestXorShift(0x2545_F491_4F6C_DD1D);
    for _ in 0..3000 {
        let n = rng.range(10) + 1; // 1..=10 頂点
        let m = rng.range(n * n + 3); // 多重辺・自己ループも出る
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let u = rng.range(n);
            let v = rng.range(n);
            g[u].push(v);
        }
        let comp = scc(&g);
        test_check(&g, &comp);
    }
}

fn test_deep_path() {
    // 再帰DFSならスタックオーバーフローする深さ
    let n = 1_000_000;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let g = test_build(n, &edges);
    let comp = scc(&g);
    for i in 0..n {
        assert_eq!(comp[i], i);
    }
}

fn test_deep_cycle() {
    let n = 1_000_000;
    let edges: Vec<(usize, usize)> = (0..n).map(|i| (i, (i + 1) % n)).collect();
    let g = test_build(n, &edges);
    let comp = scc(&g);
    assert!(comp.iter().all(|&c| c == 0));
}

fn example() {
    let tests: [(&str, fn()); 10] = [
        ("empty_graph", test_empty_graph),
        ("single_vertex", test_single_vertex),
        ("self_loop_and_multi_edge", test_self_loop_and_multi_edge),
        ("simple_cycle", test_simple_cycle),
        ("dag_path", test_dag_path),
        ("two_cycles_connected", test_two_cycles_connected),
        ("disconnected", test_disconnected),
        ("random_small_graphs", test_random_small_graphs),
        ("deep_path", test_deep_path),
        ("deep_cycle", test_deep_cycle),
    ];

    for (name, f) in tests.iter() {
        f();
        println!("ok: {}", name);
    }
    println!("all tests passed");
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
