// segment_treeの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::segment_tree::*;

fn test() {
    let values_i64 = vec![5_i64, 2, 8, 1, 7];

    // i64の区間最小値
    let mut seg_min = SegmentTree::<i64, MinNum<i64>>::new(&values_i64);

    println!("{}", seg_min.query(0, 5));
    println!("{}", seg_min.query(1, 4));

    seg_min.set(3, 10);
    println!("{}", seg_min.query(0, 5));

    seg_min.add(1, 20);
    println!("{}", seg_min.query(0, 5));

    // i64の区間最大値
    let mut seg_max = SegmentTree::<i64, MaxNum<i64>>::new(&values_i64);

    println!("{}", seg_max.query(0, 5));

    seg_max.set(2, 100);
    println!("{}", seg_max.query(0, 5));

    // i64の区間合計
    let mut seg_sum = SegmentTree::<i64, SumNum<i64>>::new(&values_i64);

    println!("{}", seg_sum.query(0, 5));
    println!("{}", seg_sum.query(1, 4));

    seg_sum.add(0, 10);
    println!("{}", seg_sum.query(0, 5));

    // usizeでも使える
    let values_usize = vec![5_usize, 2, 8, 1, 7];

    let mut seg_sum_usize = SegmentTree::<usize, SumNum<usize>>::new(&values_usize);

    println!("{}", seg_sum_usize.query(0, 5));

    seg_sum_usize.add(2, 100);
    println!("{}", seg_sum_usize.query(0, 5));

    // f64でも使える
    let values_f64 = vec![1.5_f64, -2.0, 3.25, 10.0];

    let seg_min_f64 = SegmentTree::<f64, MinNum<f64>>::new(&values_f64);
    let seg_max_f64 = SegmentTree::<f64, MaxNum<f64>>::new(&values_f64);
    let seg_sum_f64 = SegmentTree::<f64, SumNum<f64>>::new(&values_f64);

    println!("{}", seg_min_f64.query(0, 4));
    println!("{}", seg_max_f64.query(0, 4));
    println!("{}", seg_sum_f64.query(0, 4));
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

#[test]
fn empty_updates_and_ranges_match_array() {
    for n in [0, 1, 7, 32] {
        let mut a = vec![0_i64; n];
        let mut sum = SegmentTree::<i64, SumNum<i64>>::new(&a);
        let mut min = SegmentTree::<i64, MinNum<i64>>::new(&a);
        let mut max = SegmentTree::<i64, MaxNum<i64>>::new(&a);
        assert_eq!(sum.len(), n);
        assert_eq!(sum.is_empty(), n == 0);
        for step in 0..50 {
            if n > 0 {
                let i = step * 17 % n;
                a[i] = step as i64 - 25;
                sum.set(i, a[i]);
                min.set(i, a[i]);
                max.set(i, a[i]);
                sum.add(i, 2);
                min.add(i, 2);
                max.add(i, 2);
                a[i] += 2;
                assert_eq!(sum.get(i), a[i]);
            }
            for l in 0..=n {
                for r in l..=n {
                    assert_eq!(sum.query(l, r), a[l..r].iter().sum());
                    assert_eq!(min.query(l, r), a[l..r].iter().copied().min().unwrap_or(i64::MAX));
                    assert_eq!(max.query(l, r), a[l..r].iter().copied().max().unwrap_or(i64::MIN));
                }
            }
        }
    }
}

#[test]
fn identities_and_numeric_types() {
    let mut zeros = SegmentTree::<i64, SumNum<i64>>::new_empty(3);
    assert_eq!(zeros.all(), 0);
    zeros.set(1, 7);
    assert_eq!(zeros.all(), 7);
    let mut usize_sum = SegmentTree::<usize, SumNum<usize>>::new(&[5, 2, 8, 1, 7]);
    assert_eq!(usize_sum.all(), 23);
    usize_sum.add(2, 100);
    assert_eq!(usize_sum.all(), 123);
    let values = [1.5_f64, -2.0, 3.25, 10.0];
    assert_eq!(SegmentTree::<f64, MinNum<f64>>::new(&values).all(), -2.0);
    assert_eq!(SegmentTree::<f64, MaxNum<f64>>::new(&values).all(), 10.0);
    assert_eq!(SegmentTree::<f64, SumNum<f64>>::new(&values).all(), 12.75);
}
