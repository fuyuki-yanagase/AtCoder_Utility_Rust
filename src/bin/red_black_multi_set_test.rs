// red_black_multi_setの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::red_black_multi_set::*;

/// 使い方確認用の関数
///
/// mainから使用例を実行し、usage_exampleから同じ内容を自動検証する。
#[allow(dead_code)]
fn test() {
    // ======================================================================================
    //
    //                                1. 標準の Ord 比較で使う
    //
    // ======================================================================================

    // i64 は Ord を実装しているので、比較関数を渡さずに作れます。
    let mut set = OrdRBTreeMultiSet::<i64>::new();

    // 1個ずつ追加します。
    set.add(5);
    set.add(1);
    set.add(10);

    // MultiSet なので、同じキーを追加すると別ノードではなく node_count が増えます。
    set.add(3);
    set.add(3);
    set.add_count(3, 2);

    // 中身は [1, 3, 3, 3, 3, 5, 10] です。
    assert_eq!(set.count(), 7);
    assert_eq!(set.unique_count(), 4);
    assert_eq!(set.count_of(&3), 4);
    assert!(set.contains(&5));
    assert!(!set.contains(&100));

    // 最小・最大キーを取得します。
    assert_eq!(set.first(), Some(&1));
    assert_eq!(set.last(), Some(&10));
    assert_eq!(set.first_count(), 1);
    assert_eq!(set.last_count(), 1);

    // lower_bound は key 以上の最小キーです。
    assert_eq!(set.lower_bound(&0), Some(&1));
    assert_eq!(set.lower_bound(&3), Some(&3));
    assert_eq!(set.lower_bound(&4), Some(&5));
    assert_eq!(set.lower_bound(&11), None);

    // upper_bound は key より大きい最小キーです。
    assert_eq!(set.upper_bound(&3), Some(&5));
    assert_eq!(set.upper_bound(&10), None);

    // key 未満・key 以下の最大キーを取得します。
    assert_eq!(set.less_than_max(&3), Some(&1));
    assert_eq!(set.less_than_or_equal_max(&3), Some(&3));

    // 0-indexed で重複込みの k 番目を取得します。
    assert_eq!(set.get_at(0), Some(&1));
    assert_eq!(set.get_at(1), Some(&3));
    assert_eq!(set.get_at(4), Some(&3));
    assert_eq!(set.get_at(5), Some(&5));
    assert_eq!(set.get_at(6), Some(&10));
    assert_eq!(set.get_at(7), None);

    // 0-indexed でユニークキーの k 番目を取得します。
    assert_eq!(set.get_unique_at(0), Some(&1));
    assert_eq!(set.get_unique_at(1), Some(&3));
    assert_eq!(set.get_unique_at(2), Some(&5));
    assert_eq!(set.get_unique_at(3), Some(&10));
    assert_eq!(set.get_unique_at(4), None);

    // key より小さい要素数を数えます。
    assert_eq!(set.count_less_than(&3), 1);
    assert_eq!(set.count_less_than_or_equal(&3), 5);

    // ユニークキーとして key より小さい個数を数えます。
    assert_eq!(set.count_unique_less_than(&3), 1);
    assert_eq!(set.count_unique_less_than_or_equal(&3), 2);

    // key が何番目にあるかを取得します。
    assert_eq!(set.index_of(&3), Some(1));
    assert_eq!(set.index_range_of(&3), Some((1, 5)));
    assert_eq!(set.unique_index_of(&3), Some(1));

    // 昇順のキー列を Vec として取得します。
    let keys: Vec<i64> = set.keys().into_iter().copied().collect();
    assert_eq!(keys, vec![1, 3, 3, 3, 3, 5, 10]);

    // 昇順のユニークキー列を Vec として取得します。
    let unique_keys: Vec<i64> = set.unique_keys().into_iter().copied().collect();
    assert_eq!(unique_keys, vec![1, 3, 5, 10]);

    // ======================================================================================
    //
    //                                2. 削除の使い方
    //
    // ======================================================================================

    // 3を2個削除します。
    assert!(set.remove_count(&3, 2));
    assert_eq!(set.count(), 5);
    assert_eq!(set.count_of(&3), 2);
    assert_eq!(set.index_range_of(&3), Some((1, 3)));

    // 3を1個削除します。
    assert!(set.remove(&3));
    assert_eq!(set.count_of(&3), 1);

    // 3をすべて削除します。
    assert!(set.remove_all(&3));
    assert!(!set.contains(&3));
    assert_eq!(set.count(), 3);
    assert_eq!(set.unique_count(), 3);

    // 存在しないキーの削除は false です。
    assert!(!set.remove(&3));
    assert!(!set.remove_all(&3));

    // ======================================================================================
    //
    //                                3. 空きスロット再利用の確認
    //
    // ======================================================================================

    // remove_all でノードが消えると、そのスロットは Free になります。
    let slot_count_before_reuse: usize = set.slot_count();
    let free_count_before_reuse: usize = set.free_slot_count();
    assert!(free_count_before_reuse >= 1);

    // 新しいユニークキーを追加すると、Free スロットが優先的に再利用されます。
    set.add(7);

    // 空きスロットを再利用したので、内部 Vec のスロット数は増えません。
    assert_eq!(set.slot_count(), slot_count_before_reuse);
    assert_eq!(set.free_slot_count(), free_count_before_reuse - 1);
    assert!(set.contains(&7));

    // ======================================================================================
    //
    //                                4. 容量予約して使う
    //
    // ======================================================================================

    // 追加されるユニークキー数がある程度分かっている場合は、with_capacity が使えます。
    let mut reserved_set = OrdRBTreeMultiSet::<i64>::with_capacity(100);
    reserved_set.add(20);
    reserved_set.add(10);
    reserved_set.add(30);

    assert_eq!(reserved_set.count(), 3);
    assert_eq!(reserved_set.first(), Some(&10));
    assert_eq!(reserved_set.last(), Some(&30));
    assert!(reserved_set.capacity() >= 100);

    // ======================================================================================
    //
    //                                5. 比較関数を指定して使う
    //
    // ======================================================================================

    // 通常は OrdRBTreeMultiSet で十分ですが、特殊な順序にしたい場合は比較関数を渡します。
    // これは大きい値が先に来る降順の MultiSet です。
    let mut reverse_set = RBTreeMultiSet::with_compare(|left: &i64, right: &i64| right.cmp(left));

    reverse_set.add(10);
    reverse_set.add(30);
    reverse_set.add(20);
    reverse_set.add(20);

    assert_eq!(reverse_set.count(), 4);
    assert_eq!(reverse_set.unique_count(), 3);
    assert_eq!(reverse_set.count_of(&20), 2);

    // 比較関数上の先頭なので、降順では first が最大値になります。
    assert_eq!(reverse_set.first(), Some(&30));
    assert_eq!(reverse_set.last(), Some(&10));

    let reverse_keys: Vec<i64> = reverse_set.keys().into_iter().copied().collect();
    assert_eq!(reverse_keys, vec![30, 20, 20, 10]);

    // ======================================================================================
    //
    //                                6. clear の使い方
    //
    // ======================================================================================

    set.clear();
    assert!(set.is_empty());
    assert_eq!(set.count(), 0);
    assert_eq!(set.unique_count(), 0);
    assert_eq!(set.first(), None);
    assert_eq!(set.last(), None);
    assert_eq!(set.slot_count(), 0);
    assert_eq!(set.free_slot_count(), 0);
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
