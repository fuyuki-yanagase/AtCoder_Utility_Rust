// topological_sortの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::topological_sort::*;

fn test() {
    // ------------------------------------------------------------
    // Test 1
    //
    // 0 -> 1 -> 3
    //  \-> 2 -/
    //
    // 期待される順序の一例:
    // [0, 1, 2, 3]
    // ------------------------------------------------------------
    {
        let mut graph = vec![vec![]; 4];

        graph[0].push(1);
        graph[0].push(2);
        graph[1].push(3);
        graph[2].push(3);

        let result = topological_sort(&graph);

        assert_eq!(result, Some(vec![0, 1, 2, 3]));
    }

    // ------------------------------------------------------------
    // Test 2
    //
    // 0 -> 2
    // 1 -> 2
    //
    // 0 と 1 はどちらを先に処理してもよいが、
    // この実装では頂点番号順にキューへ入るので
    // [0, 1, 2] になる。
    // ------------------------------------------------------------
    {
        let mut graph = vec![vec![]; 3];

        graph[0].push(2);
        graph[1].push(2);

        let result = topological_sort(&graph);

        assert_eq!(result, Some(vec![0, 1, 2]));
    }

    // ------------------------------------------------------------
    // Test 3
    //
    // 辺が存在しないケース。
    // 全頂点の入次数が 0 なので番号順になる。
    // ------------------------------------------------------------
    {
        let graph = vec![vec![]; 5];

        let result = topological_sort(&graph);

        assert_eq!(result, Some(vec![0, 1, 2, 3, 4]));
    }

    // ------------------------------------------------------------
    // Test 4
    //
    // 0 -> 1 -> 2
    // ^         |
    // |---------|
    //
    // 閉路が存在するため None。
    // ------------------------------------------------------------
    {
        let mut graph = vec![vec![]; 3];

        graph[0].push(1);
        graph[1].push(2);
        graph[2].push(0);

        let result = topological_sort(&graph);

        assert_eq!(result, None);
    }

    // ------------------------------------------------------------
    // Test 5
    //
    // 一部だけ閉路がある場合。
    //
    // 0 -> 1
    //
    // 2 -> 3
    // ^    |
    // |----|
    //
    // グラフ全体として DAG ではないので None。
    // ------------------------------------------------------------
    {
        let mut graph = vec![vec![]; 4];

        graph[0].push(1);

        graph[2].push(3);
        graph[3].push(2);

        let result = topological_sort(&graph);

        assert_eq!(result, None);
    }

    // ------------------------------------------------------------
    // Test 6
    //
    // 頂点が1つだけ。
    // ------------------------------------------------------------
    {
        let graph = vec![vec![]; 1];

        let result = topological_sort(&graph);

        assert_eq!(result, Some(vec![0]));
    }

    println!("All tests passed.");
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
