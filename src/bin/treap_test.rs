// treapの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::treap::*;

fn example() {
    sample_usage();
    // cargo run で雑に動作確認したいので、普通の関数としてテストを呼びます。
    run_all_tests();

    // ここまでpanicしなければ成功です。
    println!("OK");
} // end of function

fn run_all_tests() {
    // ソート済みMultiSet Treapのテストを実行します。
    run_multiset_treap_test();

    // 配列として使うImplicit Treapの基本操作テストを実行します。
    run_implicit_treap_basic_test();

    // splitとmergeのテストを実行します。
    run_implicit_split_merge_test();
} // end of function

fn run_multiset_treap_test() {
    let mut set = OrdTreapMultiSet::<i64>::new();

    set.add(5);
    set.add(1);
    set.add(10);
    set.add(3);
    set.add(3);
    set.add_count(3, 2);

    assert_eq!(set.count(), 7);
    assert_eq!(set.unique_count(), 4);
    assert_eq!(set.count_of(&3), 4);
    assert!(set.contains(&5));
    assert!(!set.contains(&100));

    assert_eq!(set.first(), Some(&1));
    assert_eq!(set.last(), Some(&10));

    assert_eq!(set.lower_bound(&0), Some(&1));
    assert_eq!(set.lower_bound(&3), Some(&3));
    assert_eq!(set.lower_bound(&4), Some(&5));
    assert_eq!(set.lower_bound(&11), None);

    assert_eq!(set.upper_bound(&3), Some(&5));
    assert_eq!(set.upper_bound(&10), None);

    assert_eq!(set.less_than_max(&3), Some(&1));
    assert_eq!(set.less_than_or_equal_max(&3), Some(&3));

    assert_eq!(set.get_at(0), Some(&1));
    assert_eq!(set.get_at(1), Some(&3));
    assert_eq!(set.get_at(4), Some(&3));
    assert_eq!(set.get_at(5), Some(&5));
    assert_eq!(set.get_at(6), Some(&10));
    assert_eq!(set.get_at(7), None);

    assert_eq!(set.get_unique_at(0), Some(&1));
    assert_eq!(set.get_unique_at(1), Some(&3));
    assert_eq!(set.get_unique_at(2), Some(&5));
    assert_eq!(set.get_unique_at(3), Some(&10));
    assert_eq!(set.get_unique_at(4), None);

    assert_eq!(set.count_less_than(&3), 1);
    assert_eq!(set.count_less_than_or_equal(&3), 5);
    assert_eq!(set.count_unique_less_than(&3), 1);
    assert_eq!(set.count_unique_less_than_or_equal(&3), 2);

    assert_eq!(set.index_of(&3), Some(1));
    assert_eq!(set.index_range_of(&3), Some((1, 5)));
    assert_eq!(set.unique_index_of(&3), Some(1));

    let keys: Vec<i64> = set.keys().into_iter().copied().collect();
    assert_eq!(keys, vec![1, 3, 3, 3, 3, 5, 10]);

    let unique_keys: Vec<i64> = set.unique_keys().into_iter().copied().collect();
    assert_eq!(unique_keys, vec![1, 3, 5, 10]);

    assert!(set.remove_count(&3, 2));
    assert_eq!(set.count(), 5);
    assert_eq!(set.count_of(&3), 2);
    assert_eq!(set.index_range_of(&3), Some((1, 3)));

    assert!(set.remove(&3));
    assert_eq!(set.count_of(&3), 1);

    assert!(set.remove_all(&3));
    assert!(!set.contains(&3));
    assert_eq!(set.count(), 3);
    assert_eq!(set.unique_count(), 3);

    assert!(!set.remove(&3));
    assert!(!set.remove_all(&3));

    let range_keys_empty: Vec<i64> = set.range_keys(&100, &200).into_iter().copied().collect();
    assert_eq!(range_keys_empty, Vec::<i64>::new());

    let range_unique_keys_empty: Vec<i64> = set.range_unique_keys(&100, &200).into_iter().copied().collect();
    assert_eq!(range_unique_keys_empty, Vec::<i64>::new());
} // end of function

fn run_implicit_treap_basic_test() {
    let mut tr = ImplicitTreap::from_slice(&[1, 2, 3, 4, 5]);

    assert_eq!(tr.len(), 5);
    assert_eq!(tr.to_vec(), vec![1, 2, 3, 4, 5]);
    assert_eq!(tr.range_sum(0, 5), 15);
    assert_eq!(tr.range_max(0, 5), Some(5));
    assert_eq!(tr.range_min(0, 5), Some(1));

    tr.range_add(1, 4, 10);
    assert_eq!(tr.to_vec(), vec![1, 12, 13, 14, 5]);
    assert_eq!(tr.range_sum(1, 4), 39);
    assert_eq!(tr.range_max(0, 5), Some(14));

    tr.range_reverse(1, 5);
    assert_eq!(tr.to_vec(), vec![1, 5, 14, 13, 12]);

    tr.insert(2, 100);
    assert_eq!(tr.to_vec(), vec![1, 5, 100, 14, 13, 12]);
    assert_eq!(tr.get(2), Some(100));

    assert!(tr.set(2, 7));
    assert_eq!(tr.to_vec(), vec![1, 5, 7, 14, 13, 12]);

    assert_eq!(tr.remove_at(3), Some(14));
    assert_eq!(tr.to_vec(), vec![1, 5, 7, 13, 12]);

    tr.rotate_right_range(1, 5, 1);
    assert_eq!(tr.to_vec(), vec![1, 12, 5, 7, 13]);

    tr.rotate_left_range(1, 5, 2);
    assert_eq!(tr.to_vec(), vec![1, 7, 13, 12, 5]);

    tr.move_range(1, 3, 3);
    assert_eq!(tr.to_vec(), vec![1, 12, 5, 7, 13]);

    let removed = tr.remove_range(1, 4);
    assert_eq!(removed, vec![12, 5, 7]);
    assert_eq!(tr.to_vec(), vec![1, 13]);
} // end of function

fn run_implicit_split_merge_test() {
    let tr = ImplicitTreap::from_slice(&[10, 20, 30, 40, 50]);
    let (mut left, mut right) = tr.split(2);

    assert_eq!(left.to_vec(), vec![10, 20]);
    assert_eq!(right.to_vec(), vec![30, 40, 50]);

    let mut merged = ImplicitTreap::merge(left, right);
    assert_eq!(merged.to_vec(), vec![10, 20, 30, 40, 50]);
} // end of function

fn sample_usage() {
    // ソート済み MultiSet として使う例です。
    let mut set = OrdTreapMultiSet::<i64>::new();
    set.add(10);
    set.add(3);
    set.add(3);
    set.add(20);

    assert_eq!(set.lower_bound(&4), Some(&10));
    assert_eq!(set.count_less_than(&10), 2);
    assert_eq!(set.get_at(1), Some(&3));

    // 配列として使う例です。
    let mut seq = ImplicitTreap::from_slice(&[1, 2, 3, 4, 5]);
    seq.range_add(1, 4, 100);
    seq.range_reverse(0, 5);
    assert_eq!(seq.range_max(0, 5), Some(104));
    assert_eq!(seq.to_vec(), vec![5, 104, 103, 102, 1]);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
