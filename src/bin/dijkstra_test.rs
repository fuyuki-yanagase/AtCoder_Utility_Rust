// dijkstraの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::dijkstra::*;

#[test]
fn test_dijkstra_single_vertex() {
    let graph = vec![vec![]];
    assert_eq!(dijkstra(&graph, 0), vec![0]);
}

#[test]
fn test_dijkstra_no_edges() {
    let inf = i64::MAX / 4;
    let graph = vec![vec![]; 4];
    assert_eq!(dijkstra(&graph, 2), vec![inf, inf, 0, inf]);
}

#[test]
fn test_dijkstra_directed_graph() {
    let inf = i64::MAX / 4;
    let graph = vec![vec![(1, 2), (2, 5)], vec![(2, 1), (3, 6)], vec![(3, 3)], vec![], vec![]];

    assert_eq!(dijkstra(&graph, 0), vec![0, 2, 3, 6, inf]);
    assert_eq!(dijkstra(&graph, 1), vec![inf, 0, 1, 4, inf]);
    assert_eq!(dijkstra(&graph, 3), vec![inf, inf, inf, 0, inf]);
}

#[test]
fn test_dijkstra_undirected_graph() {
    let graph = vec![vec![(1, 4), (2, 10)], vec![(0, 4), (2, 2)], vec![(0, 10), (1, 2), (3, 3)], vec![(2, 3)]];

    assert_eq!(dijkstra(&graph, 0), vec![0, 4, 6, 9]);
    assert_eq!(dijkstra(&graph, 3), vec![9, 5, 3, 0]);
}

#[test]
fn test_dijkstra_stale_queue_entries() {
    let graph = vec![vec![(1, 100), (2, 10), (3, 1)], vec![(4, 1)], vec![(1, 1)], vec![(2, 1)], vec![]];

    assert_eq!(dijkstra(&graph, 0), vec![0, 3, 2, 1, 4]);
}

#[test]
fn test_dijkstra_zero_cost_cycle_and_equal_paths() {
    let graph = vec![vec![(1, 0), (2, 0)], vec![(2, 0), (3, 5)], vec![(0, 0), (3, 5)], vec![]];

    assert_eq!(dijkstra(&graph, 0), vec![0, 0, 0, 5]);
}

#[test]
fn test_dijkstra_parallel_edges_and_self_loops() {
    let graph = vec![vec![(0, 0), (0, 7), (1, 10), (1, 3), (1, 8)], vec![(1, 0), (2, 4)], vec![(2, 2)]];

    assert_eq!(dijkstra(&graph, 0), vec![0, 3, 7]);
}

#[test]
fn test_dijkstra_large_costs() {
    let inf = i64::MAX / 4;
    let graph = vec![vec![(1, inf - 2)], vec![(2, 1)], vec![]];

    assert_eq!(dijkstra(&graph, 0), vec![0, inf - 2, inf - 1]);
}

fn example() {
    assert_eq!(dijkstra(&vec![vec![(1, 3)], vec![]], 0), vec![0, 3]);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
