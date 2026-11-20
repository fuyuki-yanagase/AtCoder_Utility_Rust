// Heavy-Light分解
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
//                                   Heavy-Light分解
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/heavy_light_decomposition.rs

/// 木をパスの連続区間に分解
///
/// - 頂点 v の HL 番号を取得
/// - HL 番号から元の頂点番号を復元
/// - u-v パスを O(logN) 個の半開区間に分解
/// - 部分木を半開区間で取得
/// - LCA(最近共通祖先) を取得
///
/// - 頂点 : 0-indexed
/// - パス区間はセグ木と同じく半開区間 [left, right)
/// - 頂点値クエリ : path_query_vertex
/// - 辺値クエリ　 : path_query_edge
pub struct HeavyLightDecomposition {
    /// 頂点数
    n: usize,

    /// 根
    root: usize,

    /// 元の木
    graph: Vec<Vec<usize>>,

    /// 親
    parent: Vec<usize>,

    /// 深さ
    depth: Vec<usize>,

    /// 部分木サイズ
    size: Vec<usize>,

    /// heavy child, 最も部分木サイズが大きい子
    heavy: Vec<Option<usize>>,

    /// 各頂点が属する heavy path の先頭
    head: Vec<usize>,

    /// 元の頂点番号 → HL番号
    index: Vec<usize>,

    /// HL番号 → 元の頂点番号
    reverse: Vec<usize>,

    /// 次に割り当てるHL番号
    current_index: usize,
} // end of struct

impl HeavyLightDecomposition {
    /// n頂点のHL分解を構築
    pub fn new(n: usize) -> Self {
        return Self {
            n,
            root: 0,
            graph: vec![Vec::new(); n],
            parent: vec![usize::MAX; n],
            depth: vec![0; n],
            size: vec![0; n],
            heavy: vec![None; n],
            head: vec![0; n],
            index: vec![0; n],
            reverse: vec![0; n],
            current_index: 0,
        };
    } // end of constructor

    /// 無向辺を追加
    #[inline(always)]
    pub fn add_edge(&mut self, u: usize, v: usize) {
        debug_assert!(u < self.n, "u is out of range");
        debug_assert!(v < self.n, "v is out of range");

        self.graph[u].push(v);
        self.graph[v].push(u);
    } // end of method

    /// HL分解を構築
    /// add_edgeが終わってから一回だけ呼ぶ
    pub fn build(&mut self, root: usize) {
        debug_assert!(root < self.n, "root is out of range");

        self.root = root;
        self.parent[root] = root;
        self.depth[root] = 0;

        self.dfs_size(root, root);
        self.current_index = 0;
        self.dfs_decompose(root, root);
    } // end of method

    /// 部分木サイズとheavy child計算
    fn dfs_size(&mut self, node: usize, parent: usize) {
        self.size[node] = 1;
        self.heavy[node] = None;

        let mut max_subtree_size = 0;

        // cloneせずに頑張る
        let degree = self.graph[node].len();

        for i in 0..degree {
            let next = self.graph[node][i];
            if next == parent {
                continue;
            }

            self.parent[next] = node;
            self.depth[next] = self.depth[node] + 1;

            self.dfs_size(next, node);

            self.size[node] += self.size[next];

            // 最も大きい子をheavy child
            if self.size[next] > max_subtree_size {
                max_subtree_size = self.size[next];
                self.heavy[node] = Some(next);
            }
        }
    } // end of method

    /// heavy pathごとにHL番号を割り当て
    fn dfs_decompose(&mut self, node: usize, head: usize) {
        self.head[node] = head;
        self.index[node] = self.current_index;
        self.reverse[self.current_index] = node;
        self.current_index += 1;

        // heavy childを先に処理
        // → heavy pathが連続区間に
        if let Some(heavy_child) = self.heavy[node] {
            self.dfs_decompose(heavy_child, head);
        }

        // light edgeの子は, 新しいheav pathの先頭になる
        let degree = self.graph[node].len();
        for i in 0..degree {
            let next = self.graph[node][i];
            if next == self.parent[node] {
                continue;
            }
            if Some(next) == self.heavy[node] {
                continue;
            }

            self.dfs_decompose(next, next);
        }
    } // end of method

    /// 元の頂点番号からHL番号を返す
    #[inline(always)]
    pub fn get_index(&self, node: usize) -> usize {
        debug_assert!(node < self.n, "node is out of range");

        return self.index[node];
    } // end of method

    /// HL番号から元の頂点番号を返す
    #[inline(always)]
    pub fn get_node(&self, index: usize) -> usize {
        debug_assert!(index < self.n, "index is out of range");

        return self.reverse[index];
    } // end of method

    /// 親を返す
    #[inline(always)]
    pub fn get_parent(&self, node: usize) -> usize {
        debug_assert!(node < self.n, "node is out of range");

        return self.parent[node];
    } // end of method

    /// 深さを返す
    #[inline(always)]
    pub fn get_depth(&self, node: usize) -> usize {
        debug_assert!(node < self.n, "node is out of range");

        return self.depth[node];
    } // end of method

    /// 部分木サイズを返す
    #[inline(always)]
    pub fn get_size(&self, node: usize) -> usize {
        debug_assert!(node < self.n, "node is out of range");

        return self.size[node];
    } // end of method

    /// 部分木を表す半開区間を返す
    /// nodeの部分木はHL番号上で [left, right) になる
    #[inline(always)]
    pub fn subtree_range(&self, node: usize) -> (usize, usize) {
        debug_assert!(node < self.n, "node is out of range");

        let left = self.index[node];
        let right = self.index[node] + self.size[node];
        return (left, right);
    } // end of method

    /// LCA (最近共通祖先) を返す
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        debug_assert!(u < self.n, "u is out of range");
        debug_assert!(v < self.n, "v is out of range");

        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                u = self.parent[self.head[u]];
            } else {
                v = self.parent[self.head[v]];
            }
        }

        if self.depth[u] <= self.depth[v] {
            return u;
        } else {
            return v;
        }
    } // end of method

    /// 2頂点間の辺の数を返す
    pub fn distance(&self, u: usize, v: usize) -> usize {
        let lca = self.lca(u, v);
        return self.depth[u] + self.depth[v] - 2 * self.depth[lca];
    } // end of method

    /// 頂点値用のパス区間を返す
    /// u-v パス上の頂点を表す半開区間 [left, right) のリストを返す
    /// セグ木にそのまま query(left, right)できるように
    #[inline(always)]
    pub fn path_query_vertex(&self, mut u: usize, mut v: usize) -> Vec<(usize, usize)> {
        debug_assert!(u < self.n, "u is out of range");
        debug_assert!(v < self.n, "v is out of range");

        let mut ranges = Vec::<(usize, usize)>::new();

        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                let left = self.index[self.head[u]];
                let right = self.index[u] + 1;
                ranges.push((left, right));

                u = self.parent[self.head[u]];
            } else {
                let left = self.index[self.head[v]];
                let right = self.index[v] + 1;
                ranges.push((left, right));

                v = self.parent[self.head[v]];
            }
        }

        let mut left = self.index[u];
        let mut right = self.index[v];
        if left > right {
            std::mem::swap(&mut left, &mut right);
        }

        // 同じheavy path上の最後の区間
        // 頂点値なのでLCAも含める
        ranges.push((left, right + 1));

        return ranges;
    } // end of method

    /// 辺値用のパスを返す
    /// 辺の値を「子側の頂点」持たせる前提
    ///
    ///  - edge parent-childの重みをchildのHL番号に置く
    ///
    /// u-vパス上の辺を表す半開区間 [left, right) のリストを返す
    #[inline(always)]
    pub fn path_query_edge(&self, mut u: usize, mut v: usize) -> Vec<(usize, usize)> {
        debug_assert!(u < self.n, "u is out of range");
        debug_assert!(v < self.n, "v is out of range");

        let mut ranges = Vec::<(usize, usize)>::new();

        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                let left = self.index[self.head[u]];
                let right = self.index[u] + 1;
                ranges.push((left, right));

                u = self.parent[self.head[u]];
            } else {
                let left = self.index[self.head[v]];
                let right = self.index[v] + 1;
                ranges.push((left, right));

                v = self.parent[self.head[v]];
            }
        }

        let mut left_node = u;
        let mut right_node = v;

        if self.depth[left_node] > self.depth[right_node] {
            std::mem::swap(&mut left_node, &mut right_node);
        }

        // 同じheavy path上ではleft_nodeがLCA側
        // 辺値ではLCAの頂点を含めない
        let left = self.index[left_node] + 1;
        let right = self.index[right_node] + 1;
        if left < right {
            ranges.push((left, right));
        }

        return ranges;
    } // end of method
} // end of impl HeavyLightDecomposition
