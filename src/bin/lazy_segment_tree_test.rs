// lazy_segment_treeの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::lazy_segment_tree::*;

fn example() {
    test();
}

fn test() {
    // ======================================================================================
    //
    //                                  RangeAddMin
    //
    // ======================================================================================

    {
        // 区間最小 + 区間加算
        //
        // update(left, right, x)
        //  - 半開区間 [left, right) に x を加算
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最小値を取得
        let values = vec![5_i64, 2, 8, 1, 7];

        let mut seg = LazySegmentTree::<RangeAddMin<i64>>::new(&values);

        println!("RangeAddMin");
        println!("{}", seg.query(0, 5)); // 1

        seg.update(1, 4, 10);

        println!("{}", seg.query(0, 5)); // 5
        println!("{}", seg.query(1, 4)); // 11
        println!("{}", seg.all()); // 5
    }

    // ======================================================================================
    //
    //                                  RangeAssignMin
    //
    // ======================================================================================

    {
        // 区間最小 + 区間代入
        //
        // update(left, right, Some(x))
        //  - 半開区間 [left, right) を x に代入
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最小値を取得
        let values = vec![100_i64, 100, 100, 100, 100];

        let mut seg = LazySegmentTree::<RangeAssignMin<i64>>::new(&values);

        println!("RangeAssignMin");
        println!("{}", seg.query(0, 5)); // 100

        seg.update(1, 4, Some(10));
        seg.update(2, 5, Some(3));

        println!("{}", seg.query(0, 5)); // 3
        println!("{}", seg.query(1, 2)); // 10
        println!("{}", seg.query(2, 5)); // 3
        println!("{}", seg.all()); // 3
    }

    // ======================================================================================
    //
    //                                  RangeAddMax
    //
    // ======================================================================================

    {
        // 区間最大 + 区間加算
        //
        // update(left, right, x)
        //  - 半開区間 [left, right) に x を加算
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最大値を取得
        let values = vec![5_i64, 2, 8, 1, 7];

        let mut seg = LazySegmentTree::<RangeAddMax<i64>>::new(&values);

        println!("RangeAddMax");
        println!("{}", seg.query(0, 5)); // 8

        seg.update(1, 4, 10);

        println!("{}", seg.query(0, 5)); // 18
        println!("{}", seg.query(1, 4)); // 18
        println!("{}", seg.all()); // 18
    }

    // ======================================================================================
    //
    //                                  RangeAssignMax
    //
    // ======================================================================================

    {
        // 区間最大 + 区間代入
        //
        // update(left, right, Some(x))
        //  - 半開区間 [left, right) を x に代入
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最大値を取得
        let values = vec![0_i64, 0, 0, 0, 0];

        let mut seg = LazySegmentTree::<RangeAssignMax<i64>>::new(&values);

        println!("RangeAssignMax");
        println!("{}", seg.query(0, 5)); // 0

        seg.update(1, 4, Some(10));
        seg.update(2, 5, Some(3));

        println!("{}", seg.query(0, 5)); // 10
        println!("{}", seg.query(1, 2)); // 10
        println!("{}", seg.query(2, 5)); // 3
        println!("{}", seg.all()); // 10
    }

    // ======================================================================================
    //
    //                                  RangeAddSum
    //
    // ======================================================================================

    {
        // 区間和 + 区間加算
        //
        // Value は SumCount<T>
        //
        // update(left, right, x)
        //  - 半開区間 [left, right) に x を加算
        //
        // query(left, right).sum
        //  - 半開区間 [left, right) の区間和を取得
        //
        // 注意:
        // SumCount の count は区間長の管理に使う
        // 実際の要素は count = 1 で初期化する
        let values = vec![
            SumCount::new(1_i64, 1),
            SumCount::new(2_i64, 1),
            SumCount::new(3_i64, 1),
            SumCount::new(4_i64, 1),
            SumCount::new(5_i64, 1),
        ];

        let mut seg = LazySegmentTree::<RangeAddSum<i64>>::new(&values);

        println!("RangeAddSum");
        println!("{}", seg.query(0, 5).sum); // 15

        seg.update(0, 3, 10);
        seg.update(2, 5, 5);

        println!("{}", seg.query(0, 5).sum); // 60
        println!("{}", seg.query(2, 4).sum); // 27
        println!("{}", seg.all().sum); // 60
    }

    // ======================================================================================
    //
    //                                  RangeAssignSum
    //
    // ======================================================================================

    {
        // 区間和 + 区間代入
        //
        // Value は SumCount<T>
        //
        // update(left, right, Some(x))
        //  - 半開区間 [left, right) を x に代入
        //
        // query(left, right).sum
        //  - 半開区間 [left, right) の区間和を取得
        //
        // 注意:
        // SumCount の count は区間長の管理に使う
        // 実際の要素は count = 1 で初期化する
        let values = vec![
            SumCount::new(1_i64, 1),
            SumCount::new(2_i64, 1),
            SumCount::new(3_i64, 1),
            SumCount::new(4_i64, 1),
            SumCount::new(5_i64, 1),
        ];

        let mut seg = LazySegmentTree::<RangeAssignSum<i64>>::new(&values);

        println!("RangeAssignSum");
        println!("{}", seg.query(0, 5).sum); // 15

        seg.update(0, 3, Some(10));
        seg.update(2, 5, Some(4));

        println!("{}", seg.query(0, 5).sum); // 32
        println!("{}", seg.query(0, 2).sum); // 20
        println!("{}", seg.query(2, 5).sum); // 12
        println!("{}", seg.all().sum); // 32
    }

    // ======================================================================================
    //
    //                                  RangeAssignMinIndex
    //
    // ======================================================================================

    {
        // 区間最小(index付き) + 区間代入
        //
        // update(left, right, Some(x))
        //  - 半開区間 [left, right) を x に代入
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最小値と index (同値なら最も左)
        //
        // 注意:
        // 葉は MinIndex::new(値, 自分のindex) で初期化する
        // new_empty は使えない
        let a = [5_i64, 2, 8, 1, 7];
        let values: Vec<MinIndex<i64>> = (0..a.len()).map(|i| MinIndex::new(a[i], i)).collect();

        let mut seg = LazySegmentTree::<RangeAssignMinIndex<i64>>::new(&values);

        println!("RangeAssignMinIndex");
        let r = seg.query(0, 5);
        println!("{} {}", r.value, r.index); // 1 3

        seg.update(1, 4, Some(10)); // [5, 10, 10, 10, 7]
        let r = seg.query(1, 5);
        println!("{} {}", r.value, r.index); // 7 4

        seg.update(2, 5, Some(3)); // [5, 10, 3, 3, 3]
        let r = seg.query(0, 5);
        println!("{} {}", r.value, r.index); // 3 2
        let r = seg.all();
        println!("{} {}", r.value, r.index); // 3 2
    }

    // ======================================================================================
    //
    //                                  RangeAssignMaxIndex
    //
    // ======================================================================================

    {
        // 区間最大(index付き) + 区間代入
        //
        // update(left, right, Some(x))
        //  - 半開区間 [left, right) を x に代入
        //
        // query(left, right)
        //  - 半開区間 [left, right) の最大値と index (同値なら最も左)
        //
        // 注意:
        // 葉は MaxIndex::new(値, 自分のindex) で初期化する
        // new_empty は使えない
        let a = [5_i64, 2, 8, 1, 7];
        let values: Vec<MaxIndex<i64>> = (0..a.len()).map(|i| MaxIndex::new(a[i], i)).collect();

        let mut seg = LazySegmentTree::<RangeAssignMaxIndex<i64>>::new(&values);

        println!("RangeAssignMaxIndex");
        let r = seg.query(0, 5);
        println!("{} {}", r.value, r.index); // 8 2

        seg.update(1, 4, Some(10)); // [5, 10, 10, 10, 7]
        let r = seg.query(0, 5);
        println!("{} {}", r.value, r.index); // 10 1

        seg.update(2, 5, Some(3)); // [5, 10, 3, 3, 3]
        let r = seg.query(2, 5);
        println!("{} {}", r.value, r.index); // 3 2
        let r = seg.all();
        println!("{} {}", r.value, r.index); // 10 1
    }
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}

#[cfg(test)]
fn check_numeric<Op>(assign: bool, lazy: fn(i64) -> Op::Lazy, aggregate: fn(&[i64]) -> i64)
where
    Op: LazySegmentTreeOperator<Value = i64>,
{
    for n in [0, 1, 7] {
        let mut values = vec![0; n];
        let mut tree = LazySegmentTree::<Op>::new(&values);
        for step in 0..30 {
            let a = step * 7 % (n + 1);
            let b = (step * 11 + 1) % (n + 1);
            let (l, r) = (a.min(b), a.max(b));
            let value = step as i64 % 9 - 4;
            tree.update(l, r, lazy(value));
            for v in &mut values[l..r] {
                if assign {
                    *v = value;
                } else {
                    *v += value;
                }
            }
            for l in 0..=n {
                for r in l..=n {
                    assert_eq!(tree.query(l, r), if l == r { Op::identity() } else { aggregate(&values[l..r]) });
                }
            }
            assert_eq!(tree.to_vec(), values);
        }
    }
}

#[test]
fn add_assign_min_max_match_array() {
    let min: fn(&[i64]) -> i64 = |a| *a.iter().min().unwrap();
    let max: fn(&[i64]) -> i64 = |a| *a.iter().max().unwrap();
    check_numeric::<RangeAddMin<i64>>(false, |v| v, min);
    check_numeric::<RangeAddMax<i64>>(false, |v| v, max);
    check_numeric::<RangeAssignMin<i64>>(true, Some, min);
    check_numeric::<RangeAssignMax<i64>>(true, Some, max);
}

#[test]
fn overlapping_sum_updates_and_leftmost_ties() {
    let values: Vec<_> = (0..7).map(|i| SumCount::new(i as i64, 1)).collect();
    let mut add = LazySegmentTree::<RangeAddSum<i64>>::new(&values);
    let mut assign = LazySegmentTree::<RangeAssignSum<i64>>::new(&values);
    let mut a: Vec<_> = (0..7_i64).collect();
    let mut b = a.clone();
    for (l, r, x) in [(0, 7, 3), (1, 6, -2), (2, 5, 9), (3, 3, 7), (0, 7, 0)] {
        add.update(l, r, x);
        assign.update(l, r, Some(x));
        for v in &mut a[l..r] {
            *v += x;
        }
        for v in &mut b[l..r] {
            *v = x;
        }
        for l in 0..=7 {
            for r in l..=7 {
                let x = add.query(l, r);
                let y = assign.query(l, r);
                assert_eq!((x.sum, x.count), (a[l..r].iter().sum(), r - l));
                assert_eq!((y.sum, y.count), (b[l..r].iter().sum(), r - l));
            }
        }
    }
    assign.update(0, 7, None);
    assert_eq!(assign.all().sum, b.iter().sum());
    let mut min = LazySegmentTree::<RangeAssignMinIndex<i64>>::new(&(0..7).map(|i| MinIndex::new(10, i)).collect::<Vec<_>>());
    let mut max = LazySegmentTree::<RangeAssignMaxIndex<i64>>::new(&(0..7).map(|i| MaxIndex::new(10, i)).collect::<Vec<_>>());
    min.update(1, 6, Some(-1));
    max.update(1, 6, Some(20));
    assert_eq!((min.all().value, min.all().index), (-1, 1));
    assert_eq!((max.all().value, max.all().index), (20, 1));
    assert_eq!(min.query(3, 7).index, 3);
    assert_eq!(max.query(3, 7).index, 3);
    let mut empty = LazySegmentTree::<RangeAddSum<i64>>::new_empty(0);
    empty.update(0, 0, 3);
    assert_eq!((empty.query(0, 0).sum, empty.all().count), (0, 0));
}

// 非可換な写像を使い、新しい写像が外側になる合成順を検証する。
#[cfg(test)]
struct AffineSum;
#[cfg(test)]
impl LazySegmentTreeOperator for AffineSum {
    type Value = SumCount<i64>;
    type Lazy = (i64, i64);
    fn identity() -> Self::Value {
        SumCount::new(0, 0)
    }
    fn lazy_identity() -> Self::Lazy {
        (1, 0)
    }
    fn operate(a: Self::Value, b: Self::Value) -> Self::Value {
        SumCount::new(a.sum + b.sum, a.count + b.count)
    }
    fn mapping((a, b): Self::Lazy, x: Self::Value) -> Self::Value {
        SumCount::new(a * x.sum + b * x.count as i64, x.count)
    }
    fn composition((a, b): Self::Lazy, (c, d): Self::Lazy) -> Self::Lazy {
        (a * c, a * d + b)
    }
}
#[test]
fn affine_composition_order() {
    let mut tree = LazySegmentTree::<AffineSum>::new(&(1..=8).map(|v| SumCount::new(v, 1)).collect::<Vec<_>>());
    let mut values: Vec<i64> = (1..=8).collect();
    for (l, r, (a, b)) in [(0, 8, (2, 3)), (0, 8, (3, 5)), (1, 7, (0, -4)), (2, 8, (-1, 2))] {
        tree.update(l, r, (a, b));
        for v in &mut values[l..r] {
            *v = a * (*v) + b;
        }
    }
    for l in 0..=8 {
        for r in l..=8 {
            assert_eq!(tree.query(l, r).sum, values[l..r].iter().sum());
        }
    }
}
