// sqrt_bucketの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::sqrt_bucket::*;

/// SqrtBucket の動作確認
pub fn test_sqrt_bucket() {
    let bucket = SqrtBucket::with_block_size(10, 3);

    assert_eq!(bucket.len(), 10);
    assert_eq!(bucket.block_size(), 3);
    assert_eq!(bucket.block_count(), 4);

    assert_eq!(bucket.block_range(0), 0..3);
    assert_eq!(bucket.block_range(1), 3..6);
    assert_eq!(bucket.block_range(2), 6..9);
    assert_eq!(bucket.block_range(3), 9..10);

    assert_eq!(bucket.block_id(0), 0);
    assert_eq!(bucket.block_id(2), 0);
    assert_eq!(bucket.block_id(3), 1);
    assert_eq!(bucket.block_id(8), 2);
    assert_eq!(bucket.block_id(9), 3);

    let sqrt_range = bucket.split_range(2, 9);

    assert_eq!(sqrt_range.head_indexes, 2..3);
    assert_eq!(sqrt_range.body_blocks, 1..2);
    assert_eq!(sqrt_range.tail_indexes, 6..9);

    let same_block_range = bucket.split_range(3, 5);

    assert_eq!(same_block_range.head_indexes, 3..5);
    assert_eq!(same_block_range.body_blocks, 0..0);
    assert_eq!(same_block_range.tail_indexes, 5..5);

    let empty_range = bucket.split_range(4, 4);

    assert!(empty_range.is_empty());

    let mut visited = Vec::<(usize, usize)>::new();

    bucket.for_each_range_part(2, 9, |part| match part {
        SqrtRangePart::Index(index) => {
            visited.push((0, index));
        }
        SqrtRangePart::Block(block_id) => {
            visited.push((1, block_id));
        }
    });

    assert_eq!(visited, vec![(0, 2), (1, 1), (0, 6), (0, 7), (0, 8)]);
} // end of function

/// SqrtArray の動作確認
pub fn test_sqrt_array() {
    let mut array = SqrtArray::with_block_size(
        vec![10_i64, 20_i64, 30_i64, 40_i64, 50_i64, 60_i64, 70_i64, 80_i64, 90_i64, 100_i64],
        3,
    );

    assert_eq!(array.len(), 10);
    assert_eq!(array.block_size(), 3);
    assert_eq!(array.block_count(), 4);

    assert_eq!(*array.get(0), 10);
    assert_eq!(*array.get(9), 100);

    array.set(4, 555);

    assert_eq!(*array.get(4), 555);

    assert_eq!(array.block_slice(0), &[10_i64, 20_i64, 30_i64]);
    assert_eq!(array.block_slice(1), &[40_i64, 555_i64, 60_i64]);
    assert_eq!(array.block_slice(2), &[70_i64, 80_i64, 90_i64]);
    assert_eq!(array.block_slice(3), &[100_i64]);

    let mut sum = 0_i64;

    array.for_each_range_values(2, 9, |_part, values| {
        // 今回は Index か Block かを区別せず、渡されたスライスをすべて足す
        for value in values {
            sum += *value;
        }
    });

    assert_eq!(sum, 30 + 40 + 555 + 60 + 70 + 80 + 90);
    let mut block_lengths = Vec::<usize>::new();

    array.for_each_block_values(|_block_id, block_values| {
        block_lengths.push(block_values.len());
    });

    assert_eq!(block_lengths, vec![3, 3, 3, 1]);
} // end of function

/// SqrtRangeSum の動作確認
pub fn test_sqrt_range_sum() {
    let mut sqrt = SqrtRangeSum::with_block_size(
        vec![10_i64, 20_i64, 30_i64, 40_i64, 50_i64, 60_i64, 70_i64, 80_i64, 90_i64, 100_i64],
        3,
    );

    assert_eq!(sqrt.range_sum(0, 10), 550);
    assert_eq!(sqrt.range_sum(2, 9), 30 + 40 + 50 + 60 + 70 + 80 + 90);
    assert_eq!(sqrt.range_sum(4, 4), 0);

    sqrt.set(4, 555);

    assert_eq!(sqrt.get(4), 555);
    assert_eq!(sqrt.range_sum(2, 9), 30 + 40 + 555 + 60 + 70 + 80 + 90);
    assert_eq!(sqrt.range_sum(0, 10), 10 + 20 + 30 + 40 + 555 + 60 + 70 + 80 + 90 + 100);
} // end of function

/// SqrtRangeMin の動作確認
pub fn test_sqrt_range_min() {
    let mut sqrt = SqrtRangeMin::with_block_size(vec![5_i64, 2_i64, 7_i64, 9_i64, 1_i64, 4_i64, 8_i64], 3);

    assert_eq!(sqrt.range_min(0, 7), 1);
    assert_eq!(sqrt.range_min(0, 3), 2);
    assert_eq!(sqrt.range_min(2, 4), 7);

    sqrt.set(4, 10);

    assert_eq!(sqrt.get(4), 10);
    assert_eq!(sqrt.range_min(0, 7), 2);

    sqrt.set(1, 100);

    assert_eq!(sqrt.range_min(0, 7), 4);
} // end of function

/// LazyAddSqrtRangeSum の動作確認
pub fn test_lazy_add_sqrt_range_sum() {
    let mut sqrt = LazyAddSqrtRangeSum::with_block_size(vec![1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64, 7_i64, 8_i64, 9_i64, 10_i64], 3);

    assert_eq!(sqrt.range_sum(0, 10), 55);

    sqrt.range_add(2, 9, 10);

    assert_eq!(sqrt.get(2), 13);
    assert_eq!(sqrt.get(8), 19);
    assert_eq!(sqrt.range_sum(2, 9), 13 + 14 + 15 + 16 + 17 + 18 + 19);
    assert_eq!(sqrt.range_sum(0, 10), 125);

    sqrt.range_add(0, 10, -1);

    assert_eq!(sqrt.get(0), 0);
    assert_eq!(sqrt.get(2), 12);
    assert_eq!(sqrt.range_sum(0, 10), 115);

    sqrt.set(5, 100);

    assert_eq!(sqrt.get(5), 100);
    assert_eq!(sqrt.range_sum(0, 10), 200);
} // end of function

/// LazySortedSqrt の動作確認
pub fn test_lazy_sorted_sqrt() {
    let mut sqrt = LazySortedSqrt::with_block_size(vec![-3_i64, 5_i64, 0_i64, 2_i64, -1_i64, 7_i64, -4_i64, 10_i64], 3);

    assert_eq!(sqrt.count_less_than(0, 8, 0), 3);
    assert_eq!(sqrt.count_greater_equal(0, 8, 2), 4);
    assert_eq!(sqrt.sum_greater_equal(0, 8, 2), 5 + 2 + 7 + 10);
    assert_eq!(sqrt.sum_max_zero(0, 8), 5 + 2 + 7 + 10);

    sqrt.range_add(1, 7, 3);

    assert_eq!(sqrt.get(0), -3);
    assert_eq!(sqrt.get(1), 8);
    assert_eq!(sqrt.get(2), 3);
    assert_eq!(sqrt.get(6), -1);
    assert_eq!(sqrt.get(7), 10);

    assert_eq!(sqrt.count_less_than(0, 8, 0), 2);
    assert_eq!(sqrt.sum_greater_equal(2, 7, 3), 3 + 5 + 10);
    assert_eq!(sqrt.sum_max_zero(0, 8), 8 + 3 + 5 + 2 + 10 + 10);

    sqrt.set(0, 100);

    assert_eq!(sqrt.get(0), 100);
    assert_eq!(sqrt.count_less_than(0, 8, 0), 1);
    assert_eq!(sqrt.sum_max_zero(0, 8), 100 + 8 + 3 + 5 + 2 + 10 + 10);
} // end of function

/// 動作確認用
fn example() {
    test_sqrt_bucket();
    test_sqrt_array();

    test_sqrt_range_sum();
    test_sqrt_range_min();
    test_lazy_add_sqrt_range_sum();
    test_lazy_sorted_sqrt();
} // end of function

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
