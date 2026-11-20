// HL分解の使用例と旧binの回帰テスト。本体は公開モジュールを使う。
#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

use utility_rust::heavy_light_decomposition::HeavyLightDecomposition;
use utility_rust::segment_tree::{SegmentTree, SumNum};

fn example() {
    // HL分解本体の構造を確認
    test_hld_basic_structure();

    // 頂点値のパス和を確認
    test_hld_vertex_sum();

    // 辺値のパス和を確認
    test_hld_edge_sum();

    // 同じ頂点同士のクエリを確認
    test_hld_same_node();

    // 部分木区間をセグ木で確認
    test_hld_subtree_sum();

    println!("HeavyLightDecomposition tests passed.");
}

/// テスト用の木を作成する
///
/// 木:
///
/// ```text
///     0
///     |
///     1
///   /   \
///  2     3
///        |
///        4
/// ```
fn build_sample_hld() -> HeavyLightDecomposition {
    let n = 5;

    let mut hld = HeavyLightDecomposition::new(n);

    hld.add_edge(0, 1);
    hld.add_edge(1, 2);
    hld.add_edge(1, 3);
    hld.add_edge(3, 4);

    hld.build(0);

    return hld;
}

/// 頂点値をHL番号順に並べ替えたセグメント木を作成する
fn build_sample_vertex_segment_tree(hld: &HeavyLightDecomposition) -> SegmentTree<i64, SumNum<i64>> {
    // 元の頂点番号に対応する値
    //
    // node 0: 10
    // node 1: 20
    // node 2: 30
    // node 3: 40
    // node 4: 50
    let node_values = [10_i64, 20, 30, 40, 50];

    // セグ木にはHL番号順で値を置く
    let mut hld_values = vec![0_i64; node_values.len()];

    for node in 0..node_values.len() {
        let index = hld.get_index(node);
        hld_values[index] = node_values[node];
    }

    let seg = SegmentTree::<i64, SumNum<i64>>::new(&hld_values);

    return seg;
}

/// 辺値を子側の頂点に置いたセグメント木を作成する
fn build_sample_edge_segment_tree(hld: &HeavyLightDecomposition) -> SegmentTree<i64, SumNum<i64>> {
    let n = 5;

    // 辺:
    //
    // 0-1: 100
    // 1-2: 200
    // 1-3: 300
    // 3-4: 400
    //
    // 辺重みは子側の頂点に置く
    let mut hld_values = vec![0_i64; n];

    hld_values[hld.get_index(1)] = 100;
    hld_values[hld.get_index(2)] = 200;
    hld_values[hld.get_index(3)] = 300;
    hld_values[hld.get_index(4)] = 400;

    let seg = SegmentTree::<i64, SumNum<i64>>::new(&hld_values);

    return seg;
}

/// HL分解の基本情報を確認する
fn test_hld_basic_structure() {
    let hld = build_sample_hld();

    // parent の確認
    assert_eq!(hld.get_parent(0), 0, "parent of 0");
    assert_eq!(hld.get_parent(1), 0, "parent of 1");
    assert_eq!(hld.get_parent(2), 1, "parent of 2");
    assert_eq!(hld.get_parent(3), 1, "parent of 3");
    assert_eq!(hld.get_parent(4), 3, "parent of 4");

    // depth の確認
    assert_eq!(hld.get_depth(0), 0, "depth of 0");
    assert_eq!(hld.get_depth(1), 1, "depth of 1");
    assert_eq!(hld.get_depth(2), 2, "depth of 2");
    assert_eq!(hld.get_depth(3), 2, "depth of 3");
    assert_eq!(hld.get_depth(4), 3, "depth of 4");

    // subtree size の確認
    assert_eq!(hld.get_size(0), 5, "size of 0");
    assert_eq!(hld.get_size(1), 4, "size of 1");
    assert_eq!(hld.get_size(2), 1, "size of 2");
    assert_eq!(hld.get_size(3), 2, "size of 3");
    assert_eq!(hld.get_size(4), 1, "size of 4");

    // get_index と get_node が逆変換になっていることを確認
    for node in 0..5 {
        let index = hld.get_index(node);
        let restored_node = hld.get_node(index);

        assert_eq!(restored_node, node, "index reverse check");
    }

    // LCA の確認
    assert_eq!(hld.lca(2, 4), 1, "lca of 2 and 4");
    assert_eq!(hld.lca(2, 3), 1, "lca of 2 and 3");
    assert_eq!(hld.lca(3, 4), 3, "lca of 3 and 4");
    assert_eq!(hld.lca(0, 4), 0, "lca of 0 and 4");

    // 辺数距離の確認
    assert_eq!(hld.distance(2, 4), 3, "distance of 2 and 4");
    assert_eq!(hld.distance(0, 4), 3, "distance of 0 and 4");
    assert_eq!(hld.distance(3, 4), 1, "distance of 3 and 4");
}

/// 頂点値のパス和を確認する
fn test_hld_vertex_sum() {
    let hld = build_sample_hld();
    let seg = build_sample_vertex_segment_tree(&hld);

    // 頂点 2 から頂点 4 までのパス:
    //
    // 2 - 1 - 3 - 4
    //
    // 合計は 30 + 20 + 40 + 50 = 140
    let mut answer = 0_i64;

    let ranges = hld.path_query_vertex(2, 4);

    // 現在の実装とこの木では、以下の2区間に分解される
    assert_eq!(ranges, vec![(4, 5), (1, 4)], "vertex path ranges");

    for (left, right) in ranges {
        answer += seg.query(left, right);
    }

    assert_eq!(answer, 140, "vertex path sum 2 to 4");
}

/// 辺値のパス和を確認する
fn test_hld_edge_sum() {
    let hld = build_sample_hld();
    let seg = build_sample_edge_segment_tree(&hld);

    // 頂点 2 から頂点 4 までのパス:
    //
    // edge 2-1 + edge 1-3 + edge 3-4
    //
    // 200 + 300 + 400 = 900
    let mut answer = 0_i64;

    let ranges = hld.path_query_edge(2, 4);

    // 現在の実装とこの木では、以下の2区間に分解される
    assert_eq!(ranges, vec![(4, 5), (2, 4)], "edge path ranges");

    for (left, right) in ranges {
        answer += seg.query(left, right);
    }

    assert_eq!(answer, 900, "edge path sum 2 to 4");
}

/// 同じ頂点同士のクエリを確認する
fn test_hld_same_node() {
    let hld = build_sample_hld();

    let vertex_seg = build_sample_vertex_segment_tree(&hld);
    let edge_seg = build_sample_edge_segment_tree(&hld);

    // 頂点値では、自分自身の値を含む
    let mut vertex_answer = 0_i64;
    for (left, right) in hld.path_query_vertex(3, 3) {
        vertex_answer += vertex_seg.query(left, right);
    }

    assert_eq!(vertex_answer, 40, "same node vertex path sum");

    // 辺値では、同じ頂点同士のパスに辺がないので0
    let mut edge_answer = 0_i64;
    for (left, right) in hld.path_query_edge(3, 3) {
        edge_answer += edge_seg.query(left, right);
    }

    assert_eq!(edge_answer, 0, "same node edge path sum");
}

/// 部分木区間をセグ木で確認する
fn test_hld_subtree_sum() {
    let hld = build_sample_hld();
    let seg = build_sample_vertex_segment_tree(&hld);

    // node 0 の部分木:
    // 0, 1, 2, 3, 4
    // 10 + 20 + 30 + 40 + 50 = 150
    let (left0, right0) = hld.subtree_range(0);
    let sum0 = seg.query(left0, right0);
    assert_eq!(sum0, 150, "subtree sum of 0");

    // node 1 の部分木:
    // 1, 2, 3, 4
    // 20 + 30 + 40 + 50 = 140
    let (left1, right1) = hld.subtree_range(1);
    let sum1 = seg.query(left1, right1);
    assert_eq!(sum1, 140, "subtree sum of 1");

    // node 3 の部分木:
    // 3, 4
    // 40 + 50 = 90
    let (left3, right3) = hld.subtree_range(3);
    let sum3 = seg.query(left3, right3);
    assert_eq!(sum3, 90, "subtree sum of 3");

    // node 2 の部分木:
    // 2
    // 30
    let (left2, right2) = hld.subtree_range(2);
    let sum2 = seg.query(left2, right2);
    assert_eq!(sum2, 30, "subtree sum of 2");
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}

#[test]
fn subtree_and_single_vertex() {
    let mut tree = HeavyLightDecomposition::new(4);
    for (u, v) in [(0, 1), (1, 2), (1, 3)] {
        tree.add_edge(u, v);
    }
    tree.build(0);
    assert_eq!(tree.lca(2, 3), 1);
    assert_eq!(tree.get_parent(2), 1);
    assert_eq!(tree.get_depth(2), 2);
    let (l, r) = tree.subtree_range(1);
    let mut vertices: Vec<_> = (l..r).map(|i| tree.get_node(i)).collect();
    vertices.sort();
    assert_eq!(vertices, vec![1, 2, 3]);
    let mut singleton = HeavyLightDecomposition::new(1);
    singleton.build(0);
    assert_eq!(singleton.subtree_range(0), (0, 1));
    assert_eq!(singleton.lca(0, 0), 0);
    assert!(singleton.path_query_edge(0, 0).is_empty());
}
