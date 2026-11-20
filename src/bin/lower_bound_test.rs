// lower_boundの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::lower_bound::*;

fn example() {
    let arr = vec![1, 3, 3, 5, 8, 10];

    // 3以上の最初の位置はindex 1
    assert_eq!(lower_bound(&arr, &3), Some(1));

    // 3より大きい最初の位置はindex 3
    assert_eq!(upper_bound(&arr, &3), Some(3));

    // 3未満の最大値は1なのでindex 0
    assert_eq!(less_than_max(&arr, &3), Some(0));

    // 3以下の最大値は右側の3なのでindex 2
    assert_eq!(less_than_or_equal_max(&arr, &3), Some(2));

    // 1未満の値は存在しない
    assert_eq!(less_than_max(&arr, &1), None);

    // 1以下の最大値は1なのでindex 0
    assert_eq!(less_than_or_equal_max(&arr, &1), Some(0));

    // 100未満の最大値は10なのでindex 5
    assert_eq!(less_than_max(&arr, &100), Some(5));

    // 100以下の最大値も10なのでindex 5
    assert_eq!(less_than_or_equal_max(&arr, &100), Some(5));

    // 空配列ではどちらも存在しない
    let empty = Vec::<i64>::new();
    assert_eq!(less_than_max(&empty, &10), None);
    assert_eq!(less_than_or_equal_max(&empty, &10), None);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
