// red_black_sorted_mapの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::red_black_sorted_map::*;

/// 使い方確認用の関数
///
/// mainから使用例を実行し、usage_exampleから同じ内容を自動検証する。
#[allow(dead_code)]
fn example() {
    // ======================================================================================
    //
    //                                1. 標準の Ord 比較で使う
    //
    // ======================================================================================

    // i64 は Ord を実装しているので、比較関数を渡さずに作れます。
    // 値の型は &str にしていますが, i64 や String など任意の型を使えます。
    let mut dictionary = OrdRBTreeSortedDictionary::<i64, &str>::new();

    // C# の SortedDictionary.Add に近い使い方です。
    // 同じキーがすでにある場合は false を返して追加しません。
    assert!(dictionary.add(5, "five"));
    assert!(dictionary.add(1, "one"));
    assert!(dictionary.add(10, "ten"));
    assert!(dictionary.add(3, "three"));
    assert!(!dictionary.add(3, "duplicate"));

    // 中身はキー昇順で [(1, one), (3, three), (5, five), (10, ten)] です。
    assert_eq!(dictionary.count(), 4);
    assert_eq!(dictionary.count_by_tree(), 4);
    assert!(dictionary.contains_key(&5));
    assert!(!dictionary.contains_key(&100));

    // get で値を取得します。
    assert_eq!(dictionary.get(&3), Some(&"three"));
    assert_eq!(dictionary.get(&100), None);

    // insert は新規追加または上書きです。
    // 既存キーの場合は古い値を返します。
    assert_eq!(dictionary.insert(3, "THREE"), Some("three"));
    assert_eq!(dictionary.get(&3), Some(&"THREE"));

    // 新規キーの場合は None を返します。
    assert_eq!(dictionary.insert(7, "seven"), None);
    assert_eq!(dictionary.count(), 5);

    // set は既存キーだけを上書きします。
    assert!(dictionary.set(&7, "SEVEN"));
    assert!(!dictionary.set(&100, "none"));
    assert_eq!(dictionary.get(&7), Some(&"SEVEN"));

    // get_mut を使うと値を直接書き換えられます。
    if let Some(value) = dictionary.get_mut(&5) {
        *value = "FIVE";
    }
    assert_eq!(dictionary.get(&5), Some(&"FIVE"));

    // 最小・最大キーを取得します。
    assert_eq!(dictionary.first_key(), Some(&1));
    assert_eq!(dictionary.last_key(), Some(&10));
    assert_eq!(dictionary.first_key_value(), Some((&1, &"one")));
    assert_eq!(dictionary.last_key_value(), Some((&10, &"ten")));

    // lower_bound は key 以上の最小キーです。
    assert_eq!(dictionary.lower_bound_key(&0), Some(&1));
    assert_eq!(dictionary.lower_bound_key(&3), Some(&3));
    assert_eq!(dictionary.lower_bound_key(&4), Some(&5));
    assert_eq!(dictionary.lower_bound_key(&11), None);

    // upper_bound は key より大きい最小キーです。
    assert_eq!(dictionary.upper_bound_key(&3), Some(&5));
    assert_eq!(dictionary.upper_bound_key(&10), None);

    // key 未満・key 以下の最大キーを取得します。
    assert_eq!(dictionary.less_than_max_key(&3), Some(&1));
    assert_eq!(dictionary.less_than_or_equal_max_key(&3), Some(&3));

    // 0-indexed で k 番目を取得します。
    assert_eq!(dictionary.get_at(0), Some((&1, &"one")));
    assert_eq!(dictionary.get_at(1), Some((&3, &"THREE")));
    assert_eq!(dictionary.get_at(2), Some((&5, &"FIVE")));
    assert_eq!(dictionary.get_at(3), Some((&7, &"SEVEN")));
    assert_eq!(dictionary.get_at(4), Some((&10, &"ten")));
    assert_eq!(dictionary.get_at(5), None);

    // key より小さいキー数を数えます。
    assert_eq!(dictionary.count_less_than(&3), 1);
    assert_eq!(dictionary.count_less_than_or_equal(&3), 2);

    // key が何番目にあるかを取得します。
    assert_eq!(dictionary.index_of(&3), Some(1));
    assert_eq!(dictionary.index_of(&7), Some(3));
    assert_eq!(dictionary.index_of(&100), None);

    // 昇順のキー列を Vec として取得します。
    let keys: Vec<i64> = dictionary.keys().into_iter().copied().collect();
    assert_eq!(keys, vec![1, 3, 5, 7, 10]);

    // 昇順の値列を Vec として取得します。
    let values: Vec<&str> = dictionary.values().into_iter().copied().collect();
    assert_eq!(values, vec!["one", "THREE", "FIVE", "SEVEN", "ten"]);

    // 昇順のキーと値を Vec として取得します。
    let items: Vec<(i64, &str)> = dictionary.items().into_iter().map(|(key, value)| (*key, *value)).collect();
    assert_eq!(items, vec![(1, "one"), (3, "THREE"), (5, "FIVE"), (7, "SEVEN"), (10, "ten")]);

    // 区間 [left_key, right_key] のキーと値を取得します。
    let range_items: Vec<(i64, &str)> = dictionary.range_items(&3, &7).into_iter().map(|(key, value)| (*key, *value)).collect();
    assert_eq!(range_items, vec![(3, "THREE"), (5, "FIVE"), (7, "SEVEN")]);

    // ======================================================================================
    //
    //                         get_or_insert / get_or_insert_with の確認
    //
    // ======================================================================================

    let mut vector_dictionary = OrdRBTreeSortedDictionary::<i64, Vec<String>>::new();

    // 10 が存在しないので、空の Vec<String> を作ってから push する
    vector_dictionary.get_or_insert_with(10, Vec::new).push(String::from("abc"));

    // 10 はすでに存在するので、既存の Vec<String> に push する
    vector_dictionary.get_or_insert_with(10, Vec::new).push(String::from("def"));

    // 20 が存在しないので、指定した Vec<String> を追加する
    vector_dictionary.get_or_insert(20, Vec::new()).push(String::from("ghi"));

    assert_eq!(vector_dictionary.count(), 2);
    assert_eq!(vector_dictionary.get(&10).unwrap().len(), 2);
    assert_eq!(vector_dictionary.get(&20).unwrap().len(), 1);

    // ======================================================================================
    //
    //                                2. 削除の使い方
    //
    // ======================================================================================

    // 3を削除します。
    assert!(dictionary.remove(&3));
    assert!(!dictionary.contains_key(&3));
    assert_eq!(dictionary.count(), 4);

    // 存在しないキーの削除は false です。
    assert!(!dictionary.remove(&3));

    // ======================================================================================
    //
    //                                3. 空きスロット再利用の確認
    //
    // ======================================================================================

    // remove でノードが消えると、そのスロットは Free になります。
    let slot_count_before_reuse: usize = dictionary.slot_count();
    let free_count_before_reuse: usize = dictionary.free_slot_count();
    assert!(free_count_before_reuse >= 1);

    // 新しいキーを追加すると、Free スロットが優先的に再利用されます。
    assert!(dictionary.add(8, "eight"));

    // 空きスロットを再利用したので、内部 Vec のスロット数は増えません。
    assert_eq!(dictionary.slot_count(), slot_count_before_reuse);
    assert_eq!(dictionary.free_slot_count(), free_count_before_reuse - 1);
    assert!(dictionary.contains_key(&8));

    // ======================================================================================
    //
    //                                4. 容量予約して使う
    //
    // ======================================================================================

    // 追加されるキー数がある程度分かっている場合は、with_capacity が使えます。
    let mut reserved_dictionary = OrdRBTreeSortedDictionary::<i64, &str>::with_capacity(100);
    assert!(reserved_dictionary.add(20, "twenty"));
    assert!(reserved_dictionary.add(10, "ten"));
    assert!(reserved_dictionary.add(30, "thirty"));

    assert_eq!(reserved_dictionary.count(), 3);
    assert_eq!(reserved_dictionary.first_key(), Some(&10));
    assert_eq!(reserved_dictionary.last_key(), Some(&30));
    assert!(reserved_dictionary.capacity() >= 100);

    // ======================================================================================
    //
    //                                5. 比較関数を指定して使う
    //
    // ======================================================================================

    // 通常は OrdRBTreeSortedDictionary で十分ですが、特殊な順序にしたい場合は比較関数を渡します。
    // これは大きいキーが先に来る降順の SortedDictionary です。
    let mut reverse_dictionary = RBTreeSortedDictionary::with_compare(|left: &i64, right: &i64| right.cmp(left));

    assert!(reverse_dictionary.add(10, "ten"));
    assert!(reverse_dictionary.add(30, "thirty"));
    assert!(reverse_dictionary.add(20, "twenty"));
    assert_eq!(reverse_dictionary.insert(20, "TWENTY"), Some("twenty"));

    assert_eq!(reverse_dictionary.count(), 3);
    assert_eq!(reverse_dictionary.get(&20), Some(&"TWENTY"));

    // 比較関数上の先頭なので、降順では first_key が最大値になります。
    assert_eq!(reverse_dictionary.first_key(), Some(&30));
    assert_eq!(reverse_dictionary.last_key(), Some(&10));

    let reverse_keys: Vec<i64> = reverse_dictionary.keys().into_iter().copied().collect();
    assert_eq!(reverse_keys, vec![30, 20, 10]);

    // ======================================================================================
    //
    //                                6. clear の使い方
    //
    // ======================================================================================

    dictionary.clear();
    assert!(dictionary.is_empty());
    assert_eq!(dictionary.count(), 0);
    assert_eq!(dictionary.count_by_tree(), 0);
    assert_eq!(dictionary.first_key(), None);
    assert_eq!(dictionary.last_key(), None);
    assert_eq!(dictionary.slot_count(), 0);
    assert_eq!(dictionary.free_slot_count(), 0);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
