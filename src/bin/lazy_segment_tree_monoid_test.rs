// 既存の再帰版・Monoid版・独立した配列を同じ操作列で比較する。
// ライブラリ本体はコピーせず、公開モジュールの最新コードを直接検証する。
#[cfg(test)]
use utility_rust::lazy_segment_tree::LazySegmentTree;
use utility_rust::lazy_segment_tree_monoid::*;
use utility_rust::modint::ModInt998;

fn example() {
    // use indexed::{Left, Right};
    // use TopBottomUpdate::{Add, Assign};

    let values = [5_i64, 2, 8, 2, 8];

    // 区間最小・最大。葉には数値をそのまま渡す。
    let _add_min = lazy_from_operator::<RangeAddMin<i64>>(&values);
    let _assign_min = lazy_from_operator::<RangeAssignMin<i64>>(&values);
    let _add_max = lazy_from_operator::<RangeAddMax<i64>>(&values);
    let _assign_max = lazy_from_operator::<RangeAssignMax<i64>>(&values);

    // 区間和。各葉は値と要素数1を持つ。
    let sums = values.map(|value| SumCount::new(value, 1));
    let _add_sum = lazy_from_operator::<RangeAddSum<i64>>(&sums);
    let _assign_sum = lazy_from_operator::<RangeAssignSum<i64>>(&sums);

    // 添字付き最小・最大。第2型引数を省略すると同値の左端、Rightなら右端を選ぶ。
    let leaves: Vec<_> = values.iter().enumerate().map(|(i, &value)| indexed::IndexedValue::new(value, i)).collect();
    let _add_min_index_left = lazy_from_operator::<indexed::RangeAddMinIndex<i64>>(&leaves);
    let _add_min_index_right = lazy_from_operator::<indexed::RangeAddMinIndex<i64, indexed::Right>>(&leaves);
    let _assign_min_index_left = lazy_from_operator::<indexed::RangeAssignMinIndex<i64>>(&leaves);
    let _assign_min_index_right = lazy_from_operator::<indexed::RangeAssignMinIndex<i64, indexed::Right>>(&leaves);
    let _add_max_index_left = lazy_from_operator::<indexed::RangeAddMaxIndex<i64>>(&leaves);
    let _add_max_index_right = lazy_from_operator::<indexed::RangeAddMaxIndex<i64, indexed::Right>>(&leaves);
    let _assign_max_index_left = lazy_from_operator::<indexed::RangeAssignMaxIndex<i64>>(&leaves);
    let _assign_max_index_right = lazy_from_operator::<indexed::RangeAssignMaxIndex<i64, indexed::Right>>(&leaves);

    // 従来の添字付き区間代入。左端固定で、葉の型は indexed 配下と異なる。
    let min_leaves: Vec<_> = values.iter().enumerate().map(|(i, &value)| MinIndex::new(value, i)).collect();
    let max_leaves: Vec<_> = values.iter().enumerate().map(|(i, &value)| MaxIndex::new(value, i)).collect();
    let _assign_min_index_legacy = lazy_from_operator::<RangeAssignMinIndex<i64>>(&min_leaves);
    let _assign_max_index_legacy = lazy_from_operator::<RangeAssignMaxIndex<i64>>(&max_leaves);

    // 添字なしで上位3件・下位2件。falseで重複あり、trueで重複なし。
    let mut top_bottom = lazy_top_bottom::<i64, 3, 2, false>(&values);
    top_bottom.update(0, 2, TopBottomUpdate::Add(3)); // 半開区間 [0, 2) の各要素に3を加える。
    top_bottom.update(3, 5, TopBottomUpdate::Assign(5)); // 半開区間 [3, 5) の各要素を5にする。
    let result = top_bottom.query(0, values.len()); // 更新後は [8, 5, 8, 5, 5]。
    let top: Vec<_> = result.top().copied().collect();
    let bottom: Vec<_> = result.bottom().copied().collect();
    assert_eq!(top, [8, 8, 5]);
    assert_eq!(bottom, [5, 5]);

    let _distinct_top_bottom = lazy_top_bottom::<i64, 3, 2, true>(&values);

    // 片側だけ取得する場合は、もう片側の件数を0にする。
    let _top_only = lazy_top_bottom::<i64, 3, 0, false>(&values);
    let _bottom_only = lazy_top_bottom::<i64, 0, 2, false>(&values);

    // 添字付きで最大・最小を1件ずつ取得。最大値の同値は右優先、最小値は左優先。
    let mut indexed_extrema = IndexedTopBottomMonoid::<i64, 1, 1, false, indexed::Right, indexed::Left>::build(&values);
    indexed_extrema.update(0, 2, TopBottomUpdate::Add(3));
    indexed_extrema.update(3, 5, TopBottomUpdate::Assign(5));
    let result = indexed_extrema.query(0, values.len());
    let max = result.top().next().map(|entry| (entry.value, entry.index));
    let min = result.bottom().next().map(|entry| (entry.value, entry.index));
    assert_eq!(max, Some((8, 2)));
    assert_eq!(min, Some((5, 1)));

    // 優先方向の型引数は上位、下位の順。省略すると両方Left。
    // DISTINCT = false → 重複あり、 true → 重複なし
    let _indexed_top_bottom = IndexedTopBottomMonoid::<i64, 3, 2>::build(&values);
    let _indexed_top_right = IndexedTopBottomMonoid::<i64, 3, 2, false, indexed::Right, indexed::Left>::build(&values);
    let _indexed_bottom_right =
        IndexedTopBottomMonoid::<i64, 3, 2, false, indexed::Left, indexed::Right>::build(&values);
    let _indexed_both_right =
        IndexedTopBottomMonoid::<i64, 3, 2, false, indexed::Right, indexed::Right>::build(&values);
    let _distinct_indexed_top_bottom =
        IndexedTopBottomMonoid::<i64, 3, 2, true, indexed::Left, indexed::Right>::build(&values);

    // 法998244353の区間アフィン変換と区間和。葉の和には ModInt998 を使う。
    let modular_sums = values.map(|value| SumCount::new(ModInt998::new(value), 1));
    let _affine_sum = lazy_from_operator::<RangeAffineSum<998244353>>(&modular_sums);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();

    let values = [1_i64, 2, 3, 4, 5].map(|value| SumCount::new(value, 1));
    let mut original = LazySegmentTree::<RangeAssignSum<i64>>::new(&values);
    let mut monoid = lazy_from_operator::<RangeAssignSum<i64>>(&values);
    original.update(1, 4, Some(10));
    monoid.update(1, 4, Some(10));
    assert_eq!(original.query(0, 5), SumCount::new(36, 5));
    assert_eq!(monoid.query(0, 5), SumCount::new(36, 5));
    assert_eq!(original.to_vec(), monoid.into_vec());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;
    use std::cell::Cell;
    use std::fmt::Debug;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::rc::Rc;
    use utility_rust::monoid_group_abelian::FnMonoid;

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> usize {
            self.0 ^= self.0 << 7;
            self.0 ^= self.0 >> 9;
            self.0 as usize
        }

        fn range(&mut self, n: usize) -> (usize, usize) {
            let a = self.next() % (n + 1);
            let b = self.next() % (n + 1);
            (a.min(b), a.max(b))
        }
    }

    // 集約の正解はOp::operate/identityから作らず、配列を直接走査する。
    // 共有する演算定義の誤りを、両実装の単なる一致で見逃さないため。
    fn check_operator<Op>(
        assign: bool,
        leaf: impl Fn(i64, usize) -> Op::Value,
        make_update: impl Fn(Option<i64>) -> Op::Lazy,
        expected: impl Fn(&[i64], usize, usize) -> Op::Value,
    ) where
        Op: LazySegmentTreeOperator,
        Op::Value: Debug + PartialEq,
    {
        let boundary_lengths = [0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65];
        let mut rng = Rng(123456789);
        for case in 0..64 {
            let n = if case < boundary_lengths.len() { boundary_lengths[case] } else { rng.next() % 66 };
            let mut values: Vec<_> = (0..n).map(|_| (rng.next() % 21) as i64 - 10).collect();
            let input: Vec<_> = values.iter().enumerate().map(|(i, &v)| leaf(v, i)).collect();
            let mut original = LazySegmentTree::<Op>::new(&input);
            let mut monoid = lazy_from_operator::<Op>(&input);
            assert_eq!(original.len(), n);
            assert_eq!(monoid.len(), n);
            assert_eq!(original.is_empty(), n == 0);
            assert_eq!(monoid.is_empty(), n == 0);

            for step in 0..256 {
                // 全区間・空区間・接頭/接尾・一点を明示的に混ぜる。
                let (left, right) = match step % 12 {
                    0 | 1 => (0, n),
                    2 => (n, n),
                    3 => (0, 0),
                    4 => (n / 2, n),
                    5 => (0, n / 2),
                    6 if n > 0 => {
                        let i = rng.next() % n;
                        (i, i + 1)
                    }
                    _ => rng.range(n),
                };
                let action = if step % 11 == 0 { None } else { Some((rng.next() % 7) as i64 - 3) };
                original.update(left, right, make_update(action));
                monoid.update(left, right, make_update(action));
                if let Some(action) = action {
                    for value in &mut values[left..right] {
                        if assign {
                            *value = action;
                        } else {
                            *value += action;
                        }
                    }
                }

                let all = expected(&values, 0, n);
                assert_eq!(original.all(), all, "既存版: {} case={case} step={step}", type_name::<Op>());
                assert_eq!(*monoid.all(), all, "Monoid版: {} case={case} step={step}", type_name::<Op>());
                let (l, r) = rng.range(n);
                let answer = expected(&values, l, r);
                assert_eq!(original.query(l, r), answer, "既存版: {} [{l},{r})", type_name::<Op>());
                assert_eq!(monoid.query(l, r), answer, "Monoid版: {} [{l},{r})", type_name::<Op>());
                if n > 0 {
                    let i = rng.next() % n;
                    assert_eq!(*monoid.get(i), leaf(values[i], i));
                }
                if step % 31 == 0 {
                    let leaves: Vec<_> = values.iter().enumerate().map(|(i, &v)| leaf(v, i)).collect();
                    assert_eq!(original.to_vec(), leaves);
                    assert_eq!(monoid.as_slice(), leaves.as_slice());
                    assert_eq!(monoid.to_vec(), leaves);
                }
            }

            // 最後にすべての半開区間を照合する。
            for l in 0..=n {
                for r in l..=n {
                    let answer = expected(&values, l, r);
                    assert_eq!(original.query(l, r), answer);
                    assert_eq!(monoid.query(l, r), answer);
                }
            }
            let leaves: Vec<_> = values.iter().enumerate().map(|(i, &v)| leaf(v, i)).collect();
            assert_eq!(original.to_vec(), leaves);
            assert_eq!(monoid.into_vec(), leaves);
        }
    }

    macro_rules! scalar_test {
        ($name:ident, $op:ty, $assign:expr, $make:expr, $aggregate:expr) => {
            #[test]
            fn $name() {
                check_operator::<$op>($assign, |v, _| v, $make, |values, l, r| $aggregate(&values[l..r]));
            }
        };
    }

    scalar_test!(both_add_min, RangeAddMin<i64>, false, |v| v.unwrap_or(0), |v: &[i64]| v
        .iter()
        .copied()
        .min()
        .unwrap_or(i64::MAX));
    scalar_test!(both_assign_min, RangeAssignMin<i64>, true, |v| v, |v: &[i64]| v
        .iter()
        .copied()
        .min()
        .unwrap_or(i64::MAX));
    scalar_test!(both_add_max, RangeAddMax<i64>, false, |v| v.unwrap_or(0), |v: &[i64]| v
        .iter()
        .copied()
        .max()
        .unwrap_or(i64::MIN));
    scalar_test!(both_assign_max, RangeAssignMax<i64>, true, |v| v, |v: &[i64]| v
        .iter()
        .copied()
        .max()
        .unwrap_or(i64::MIN));

    #[test]
    fn both_add_sum() {
        check_operator::<RangeAddSum<i64>>(
            false,
            |v, _| SumCount::new(v, 1),
            |v| v.unwrap_or(0),
            |a, l, r| SumCount::new(a[l..r].iter().sum(), r - l),
        );
    }

    #[test]
    fn both_assign_sum() {
        check_operator::<RangeAssignSum<i64>>(
            true,
            |v, _| SumCount::new(v, 1),
            |v| v,
            |a, l, r| SumCount::new(a[l..r].iter().sum(), r - l),
        );
    }

    fn expected_min_index(values: &[i64], l: usize, r: usize) -> MinIndex<i64> {
        if l == r {
            return MinIndex {
                value: i64::MAX,
                index: usize::MAX,
                left: usize::MAX,
            };
        }
        let value = *values[l..r].iter().min().unwrap();
        let index = l + values[l..r].iter().position(|&v| v == value).unwrap();
        MinIndex {
            value,
            index,
            left: l,
        }
    }

    fn expected_max_index(values: &[i64], l: usize, r: usize) -> MaxIndex<i64> {
        if l == r {
            return MaxIndex {
                value: i64::MIN,
                index: usize::MAX,
                left: usize::MAX,
            };
        }
        let value = *values[l..r].iter().max().unwrap();
        let index = l + values[l..r].iter().position(|&v| v == value).unwrap();
        MaxIndex {
            value,
            index,
            left: l,
        }
    }

    #[test]
    fn both_assign_min_index() {
        check_operator::<RangeAssignMinIndex<i64>>(true, MinIndex::new, |v| v, expected_min_index);
    }

    #[test]
    fn both_assign_max_index() {
        check_operator::<RangeAssignMaxIndex<i64>>(true, MaxIndex::new, |v| v, expected_max_index);
    }

    fn check_indexed<Op>(maximum: bool, rightmost: bool, assign: bool, update: impl Fn(i64) -> Op::Lazy)
    where
        Op: LazySegmentTreeOperator<Value = indexed::IndexedValue<i64>>,
    {
        use indexed::IndexedValue;

        let mut rng = Rng(123456789);
        for n in [0, 1, 2, 3, 5, 7, 8, 9, 17, 33] {
            let mut values: Vec<_> = (0..n).map(|_| (rng.next() % 5) as i64 - 2).collect();
            let leaves: Vec<_> = values.iter().enumerate().map(|(i, &v)| IndexedValue::new(v, i)).collect();
            let mut tree = lazy_from_operator::<Op>(&leaves);

            let check = |answer: IndexedValue<i64>, values: &[i64], l: usize, r: usize| {
                let slice = &values[l..r];
                let extreme = if maximum { slice.iter().max() } else { slice.iter().min() };
                if let Some(&value) = extreme {
                    let offset = if rightmost {
                        slice.iter().rposition(|&v| v == value)
                    } else {
                        slice.iter().position(|&v| v == value)
                    };
                    assert!(!answer.is_empty());
                    assert_eq!(
                        (answer.value, answer.index),
                        (value, l + offset.unwrap()),
                        "{} [{l},{r})",
                        type_name::<Op>()
                    );
                } else {
                    assert!(answer.is_empty());
                    assert_eq!(answer.index, usize::MAX);
                }
            };

            for step in 0..96 {
                // 取得せず2回更新し、遅延中の加算と代入の合成も確かめる。
                for (l, r) in [if step % 5 == 0 { (0, n) } else { rng.range(n) }, rng.range(n)] {
                    let value = (rng.next() % 5) as i64 - 2;
                    tree.update(l, r, update(value));
                    for element in &mut values[l..r] {
                        if assign {
                            *element = value;
                        } else {
                            *element += value;
                        }
                    }
                }
                if n > 0 && step % 7 == 0 {
                    let i = rng.next() % n;
                    values[i] = (rng.next() % 5) as i64 - 2;
                    tree.set(i, IndexedValue::new(values[i], i));
                    assert_eq!(*tree.get(i), IndexedValue::new(values[i], i));
                }
                check(*tree.all(), &values, 0, n);
                let (l, r) = rng.range(n);
                check(tree.query(l, r), &values, l, r);
            }

            for l in 0..=n {
                for r in l..=n {
                    check(tree.query(l, r), &values, l, r);
                }
            }
            let leaves: Vec<_> = values.iter().enumerate().map(|(i, &v)| IndexedValue::new(v, i)).collect();
            assert_eq!(tree.into_vec(), leaves);
        }
    }

    #[test]
    fn indexed_add_matches_array_for_both_tie_directions() {
        use indexed::{RangeAddMaxIndex, RangeAddMinIndex, Right};
        check_indexed::<RangeAddMaxIndex<i64>>(true, false, false, |v| v);
        check_indexed::<RangeAddMaxIndex<i64, Right>>(true, true, false, |v| v);
        check_indexed::<RangeAddMinIndex<i64>>(false, false, false, |v| v);
        check_indexed::<RangeAddMinIndex<i64, Right>>(false, true, false, |v| v);
    }

    #[test]
    fn indexed_assign_matches_array_for_both_tie_directions() {
        use indexed::{RangeAssignMaxIndex, RangeAssignMinIndex, Right};
        check_indexed::<RangeAssignMaxIndex<i64>>(true, false, true, Some);
        check_indexed::<RangeAssignMaxIndex<i64, Right>>(true, true, true, Some);
        check_indexed::<RangeAssignMinIndex<i64>>(false, false, true, Some);
        check_indexed::<RangeAssignMinIndex<i64, Right>>(false, true, true, Some);
    }

    #[test]
    fn indexed_assign_prefers_real_endpoints_even_at_identity_values() {
        use indexed::{Left, MaxIndex, MinIndex, RangeAssignMaxIndex, RangeAssignMinIndex, Right};
        let values = [MaxIndex::new(5_i64, 0), MaxIndex::new(9, 1), MaxIndex::new(3, 2)];
        let mut max = lazy_from_operator::<RangeAssignMaxIndex<i64, Right>>(&values);
        assert_eq!(max.all().index, 1);
        max.update(0, 3, Some(i64::MIN));
        max.update(0, 3, None);
        assert_eq!((max.all().value, max.all().index), (i64::MIN, 2));
        assert_eq!(max.query(0, 2).index, 1);
        assert!(max.query(3, 3).is_empty());

        let mut min = lazy_from_operator::<RangeAssignMinIndex<i64, Left>>(&[MinIndex::new(i64::MAX, 0)]);
        assert_eq!((min.all().value, min.all().index), (i64::MAX, 0));
        min.update(0, 1, Some(i64::MAX));
        assert_eq!(min.query(0, 1).index, 0);
    }

    #[test]
    fn top_bottom_assignment_preserves_the_requested_multiplicity() {
        use TopBottomUpdate::{Add, Assign};

        let values = [9_i64, 9, 1, -2, 0];
        let mut multiple = lazy_top_bottom::<i64, 2, 2, false>(&values);
        let mut distinct = lazy_top_bottom::<i64, 2, 2, true>(&values);

        assert_eq!(multiple.query(0, 3).top().copied().collect::<Vec<_>>(), [9, 9]);
        assert_eq!(distinct.query(0, 3).top().copied().collect::<Vec<_>>(), [9, 1]);

        // 先頭4要素を同じ内部ノードに重ねて更新し、部分区間の取得で子に伝播させる。
        for update in [Add(10), Assign(7), Add(-3), Add(-2)] {
            multiple.update(0, 4, update);
            distinct.update(0, 4, update);
        }
        let repeated = multiple.query(1, 4);
        let unique = distinct.query(1, 4);
        assert_eq!(repeated.top().copied().collect::<Vec<_>>(), [2, 2]);
        assert_eq!(repeated.bottom().copied().collect::<Vec<_>>(), [2, 2]);
        assert_eq!(unique.top().copied().collect::<Vec<_>>(), [2]);
        assert_eq!(unique.bottom().copied().collect::<Vec<_>>(), [2]);
        assert_eq!(repeated.len(), 3);
        assert_eq!(unique.len(), 3);
    }

    fn check_top_bottom<const TOP: usize, const BOTTOM: usize, const DISTINCT: bool>() {
        use TopBottomUpdate::{Add, Assign, Identity};

        let mut rng = Rng(813_705_214);
        for n in [0, 1, 2, 3, 4, 5, 8, 9, 17] {
            let mut values: Vec<_> = (0..n).map(|_| (rng.next() % 7) as i64 - 3).collect();
            let mut tree = lazy_top_bottom::<i64, TOP, BOTTOM, DISTINCT>(&values);
            for step in 0..64 {
                // クエリを挟まず、同じ区間に加算や代入を合成する。
                for (left, right) in [(0, n), (0, n), rng.range(n), rng.range(n)] {
                    let value = (rng.next() % 11) as i64 - 5;
                    let update = match rng.next() % 7 {
                        0 => Identity,
                        1..=3 => Add(value),
                        _ => Assign(value),
                    };
                    tree.update(left, right, update);
                    match update {
                        Identity => {}
                        Add(delta) => {
                            for entry in &mut values[left..right] {
                                *entry += delta;
                            }
                        }
                        Assign(value) => values[left..right].fill(value),
                    }
                }

                // 木の結合処理とは独立に、全要素のソートから期待値を求める。
                for left in 0..=n {
                    for right in left..=n {
                        let mut sorted = values[left..right].to_vec();
                        sorted.sort_unstable();
                        if DISTINCT {
                            sorted.dedup();
                        }
                        let expected_top: Vec<_> = sorted.iter().rev().take(TOP).copied().collect();
                        let expected_bottom: Vec<_> = sorted.iter().take(BOTTOM).copied().collect();
                        let node = tree.query(left, right);
                        let context = (TOP, BOTTOM, DISTINCT, n, step, left, right);
                        assert_eq!(node.len(), right - left, "{context:?}");
                        assert_eq!(node.is_empty(), left == right, "{context:?}");
                        assert_eq!(node.top().copied().collect::<Vec<_>>(), expected_top, "{context:?}");
                        assert_eq!(node.bottom().copied().collect::<Vec<_>>(), expected_bottom, "{context:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn top_bottom_mixed_updates_match_array_with_duplicates() {
        check_top_bottom::<3, 2, false>();
        check_top_bottom::<2, 0, false>();
        check_top_bottom::<0, 2, false>();
    }

    #[test]
    fn top_bottom_mixed_updates_match_array_without_duplicates() {
        check_top_bottom::<3, 2, true>();
        check_top_bottom::<2, 0, true>();
        check_top_bottom::<0, 2, true>();
    }

    #[test]
    fn indexed_top_bottom_assigns_indices_in_each_tie_order() {
        use TopBottomUpdate::{Add, Assign};
        use indexed::{Left, Right};

        let values = [0_i64, 9, 8, 7, 6, 1, 2, 10, 99];
        let mut default_left = IndexedTopBottomMonoid::<i64, 3, 2>::build(&values);
        let mut opposite = IndexedTopBottomMonoid::<i64, 3, 2, false, Right, Left>::build(&values);
        let mut distinct = IndexedTopBottomMonoid::<i64, 3, 2, true, Left, Right>::build(&values);

        // 先頭8要素を内部ノードで代入し、加算を合成した後に部分区間へ伝播させる。
        for update in [Add(12), Assign(5), Add(-2)] {
            default_left.update(0, 8, update);
            opposite.update(0, 8, update);
            distinct.update(0, 8, update);
        }
        let fallback = default_left.query(1, 7);
        assert_eq!(fallback.top().map(|entry| entry.index).collect::<Vec<_>>(), [1, 2, 3]);
        assert_eq!(fallback.bottom().map(|entry| entry.index).collect::<Vec<_>>(), [1, 2]);
        let assigned = opposite.query(1, 7);
        assert_eq!(
            assigned.top().map(|e| (e.value, e.index)).collect::<Vec<_>>(),
            [(3, 6), (3, 5), (3, 4)]
        );
        assert_eq!(
            assigned.bottom().map(|e| (e.value, e.index)).collect::<Vec<_>>(),
            [(3, 1), (3, 2)]
        );
        let unique = distinct.query(1, 7);
        assert_eq!(unique.top().map(|e| (e.value, e.index)).collect::<Vec<_>>(), [(3, 1)]);
        assert_eq!(unique.bottom().map(|e| (e.value, e.index)).collect::<Vec<_>>(), [(3, 6)]);
        assert_eq!(unique.len(), 6);

        assert!(opposite.query(9, 9).is_empty());
        let tail = opposite.query(7, 9);
        assert_eq!(tail.top().map(|e| (e.value, e.index)).collect::<Vec<_>>(), [(99, 8), (3, 7)]);
    }

    fn check_indexed_top_bottom<
        const TOP: usize,
        const BOTTOM: usize,
        const DISTINCT: bool,
        TopTie: indexed::TieBreak,
        BottomTie: indexed::TieBreak,
    >(
        top_right: bool,
        bottom_right: bool,
    ) {
        use TopBottomUpdate::{Add, Assign, Identity};
        use std::cmp::Reverse;

        let mut rng = Rng(295_238_109);
        for n in [0, 1, 3, 4, 8, 9, 17] {
            let mut values: Vec<_> = (0..n).map(|_| (rng.next() % 5) as i64 - 2).collect();
            let mut tree = IndexedTopBottomMonoid::<i64, TOP, BOTTOM, DISTINCT, TopTie, BottomTie>::build(&values);
            for step in 0..32 {
                for (l, r) in [(0, n), (0, n), rng.range(n)] {
                    let value = (rng.next() % 7) as i64 - 3;
                    let update = match rng.next() % 5 {
                        0 => Identity,
                        1 | 2 => Add(value),
                        _ => Assign(value),
                    };
                    tree.update(l, r, update);
                    match update {
                        Identity => {}
                        Add(delta) => values[l..r].iter_mut().for_each(|entry| *entry += delta),
                        Assign(value) => values[l..r].fill(value),
                    }
                }
                if n > 0 && step % 5 == 0 {
                    let index = rng.next() % n;
                    values[index] = (rng.next() % 9) as i64 - 4;
                    tree.set(index, IndexedTopBottom::leaf(values[index], index));
                }

                // 期待値は区間の全要素を値と元の添字でソートし、値だけで重複を除いて求める。
                for l in 0..=n {
                    for r in l..=n {
                        let index_key = |index, right| if right { usize::MAX - index } else { index };
                        let mut top: Vec<_> = (l..r).map(|i| (values[i], i)).collect();
                        let mut bottom = top.clone();
                        top.sort_unstable_by_key(|&(value, index)| (Reverse(value), index_key(index, top_right)));
                        bottom.sort_unstable_by_key(|&(value, index)| (value, index_key(index, bottom_right)));
                        if DISTINCT {
                            top.dedup_by_key(|entry| entry.0);
                            bottom.dedup_by_key(|entry| entry.0);
                        }
                        top.truncate(TOP);
                        bottom.truncate(BOTTOM);
                        let result = tree.query(l, r);
                        let context = (DISTINCT, top_right, bottom_right, n, step, l, r);
                        assert_eq!(result.len(), r - l, "{context:?}");
                        assert_eq!(result.top().map(|e| (e.value, e.index)).collect::<Vec<_>>(), top, "{context:?}");
                        assert_eq!(
                            result.bottom().map(|e| (e.value, e.index)).collect::<Vec<_>>(),
                            bottom,
                            "{context:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn indexed_top_bottom_mixed_updates_match_array_in_all_tie_directions() {
        use indexed::{Left, Right};
        check_indexed_top_bottom::<3, 2, false, Left, Left>(false, false);
        check_indexed_top_bottom::<3, 2, false, Left, Right>(false, true);
        check_indexed_top_bottom::<3, 2, false, Right, Left>(true, false);
        check_indexed_top_bottom::<3, 2, false, Right, Right>(true, true);
        check_indexed_top_bottom::<3, 2, true, Left, Left>(false, false);
        check_indexed_top_bottom::<3, 2, true, Left, Right>(false, true);
        check_indexed_top_bottom::<3, 2, true, Right, Left>(true, false);
        check_indexed_top_bottom::<3, 2, true, Right, Right>(true, true);
        check_indexed_top_bottom::<0, 2, false, Left, Right>(false, true);
        check_indexed_top_bottom::<2, 0, true, Right, Left>(true, false);
    }

    const MOD: i64 = 998244353;

    // 加算だけでは交換順を取り違えても検出できないため、非可換の写像を使う。
    struct AffineSum;
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
            SumCount::new((a.sum + b.sum) % MOD, a.count + b.count)
        }
        fn mapping((a, b): Self::Lazy, x: Self::Value) -> Self::Value {
            SumCount::new((a * x.sum + b * x.count as i64) % MOD, x.count)
        }
        fn composition((a, b): Self::Lazy, (c, d): Self::Lazy) -> Self::Lazy {
            (a * c % MOD, (a * d + b) % MOD)
        }
    }

    #[test]
    fn affine_updates_match_integer_oracle() {
        let mut rng = Rng(987654321);
        for n in [0, 1, 7, 8, 9, 31, 32, 33, 63, 64, 65] {
            let mut values: Vec<_> = (0..n).map(|_| (rng.next() as i64).rem_euclid(MOD)).collect();
            let input: Vec<_> = values.iter().map(|&v| SumCount::new(v, 1)).collect();
            let mut original = LazySegmentTree::<AffineSum>::new(&input);
            let mut adapter = lazy_from_operator::<AffineSum>(&input);
            let modular_input: Vec<_> = values.iter().map(|&v| SumCount::new(ModInt998::new(v), 1)).collect();
            let mut modular = lazy_from_operator::<RangeAffineSum<998244353>>(&modular_input);
            let as_integer = |node: SumCount<ModInt998>| SumCount::new(i64::from(node.sum.val()), node.count);
            for step in 0..256 {
                let (l, r) = if step % 8 < 2 { (0, n) } else { rng.range(n) };
                let (a, b) = match step % 8 {
                    0 => (2, 3),
                    1 => (4, 1),
                    _ => ((rng.next() % 5) as i64, (rng.next() % 101) as i64),
                };
                original.update(l, r, (a, b));
                adapter.update(l, r, (a, b));
                modular.update(l, r, Affine::new(a, b));
                for value in &mut values[l..r] {
                    *value = ((a as u128 * *value as u128 + b as u128) % MOD as u128) as i64;
                }
                let expected = SumCount::new(values.iter().sum::<i64>() % MOD, n);
                assert_eq!(original.all(), expected);
                assert_eq!(*adapter.all(), expected);
                assert_eq!(as_integer(*modular.all()), expected);
                let (ql, qr) = rng.range(n);
                let expected = SumCount::new(values[ql..qr].iter().sum::<i64>() % MOD, qr - ql);
                assert_eq!(original.query(ql, qr), expected);
                assert_eq!(adapter.query(ql, qr), expected);
                assert_eq!(as_integer(modular.query(ql, qr)), expected);
            }
            let leaves: Vec<_> = values.iter().map(|&v| SumCount::new(v, 1)).collect();
            assert_eq!(original.to_vec(), leaves);
            assert_eq!(adapter.into_vec(), leaves);
            assert_eq!(modular.into_vec().into_iter().map(as_integer).collect::<Vec<_>>(), leaves);
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Bits {
        bits: u64,
        len: usize,
    }

    struct FlipBits;
    impl LazySegmentTreeOperator for FlipBits {
        type Value = Bits;
        type Lazy = bool;
        fn identity() -> Bits {
            Bits {
                bits: 0,
                len: 0,
            }
        }
        fn lazy_identity() -> bool {
            false
        }
        fn operate(a: Bits, b: Bits) -> Bits {
            Bits {
                bits: (a.bits << b.len) | b.bits,
                len: a.len + b.len,
            }
        }
        fn mapping(f: bool, x: Bits) -> Bits {
            Bits {
                bits: if f { x.bits ^ ((1_u64 << x.len) - 1) } else { x.bits },
                len: x.len,
            }
        }
        fn composition(new: bool, old: bool) -> bool {
            new ^ old
        }
    }

    // 値の集約も非可換にして、queryの左右の順番を独立に検証する。
    #[test]
    fn both_non_commutative_value_order() {
        let mut rng = Rng(246813579);
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 47] {
            let mut values: Vec<_> = (0..n).map(|_| !rng.next().is_multiple_of(2)).collect();
            let input: Vec<_> = values
                .iter()
                .map(|&v| Bits {
                    bits: v as u64,
                    len: 1,
                })
                .collect();
            let mut original = LazySegmentTree::<FlipBits>::new(&input);
            let mut monoid = lazy_from_operator::<FlipBits>(&input);
            for _ in 0..256 {
                let (l, r) = rng.range(n);
                original.update(l, r, true);
                monoid.update(l, r, true);
                for v in &mut values[l..r] {
                    *v = !*v;
                }
                let (l, r) = rng.range(n);
                let bits = values[l..r].iter().fold(0, |a, &b| (a << 1) | b as u64);
                let expected = Bits {
                    bits,
                    len: r - l,
                };
                assert_eq!(original.query(l, r), expected);
                assert_eq!(monoid.query(l, r), expected);
            }
        }
    }

    #[derive(Clone)]
    struct Shift(u8); // Copy/PartialEqなし。新しい木の汎用APIを検証する。

    #[test]
    fn native_non_copy_values_updates_and_point_set() {
        let input: Vec<_> = ["ab", "c", "def", "g", "hi"].map(String::from).to_vec();
        let mut seg = LazySegmentTreeMonoid::new(
            &input,
            FnMonoid::new(String::new, |a: &String, b: &String| {
                let mut s = a.clone();
                s.push_str(b);
                s
            }),
            FnMonoid::new(|| Shift(0), |new: &Shift, old: &Shift| Shift((new.0 + old.0) % 26)),
            |f: &Shift, x: &mut String| {
                *x = x.bytes().map(|b| (b'a' + (b - b'a' + f.0) % 26) as char).collect();
            },
        );
        seg.update(0, 5, Shift(1));
        seg.update(1, 4, Shift(2));
        seg.set(2, "xy".to_string());
        let expected = ["bc", "f", "xy", "j", "ij"].map(String::from).to_vec();
        assert_eq!(seg.get(2), "xy");
        assert_eq!(seg.all(), "bcfxyjij");
        assert_eq!(seg.query(1, 4), "fxyj");
        assert_eq!(seg.as_slice(), expected.as_slice());
        assert_eq!(seg.to_vec(), expected);
        seg.update(0, 1, Shift(1));
        assert_eq!(seg.get(0), "cd");
        let out = seg.into_vec();
        assert_eq!(out, ["cd", "f", "xy", "j", "ij"].map(String::from).to_vec());
    }

    struct Tracked {
        sum: i64,
        count: usize,
        clones: Rc<Cell<usize>>,
    }
    impl Clone for Tracked {
        fn clone(&self) -> Self {
            self.clones.set(self.clones.get() + 1);
            Self {
                sum: self.sum,
                count: self.count,
                clones: self.clones.clone(),
            }
        }
    }

    #[test]
    fn native_extraction_clone_counts() {
        let clones = Rc::new(Cell::new(0));
        let input: Vec<_> = (1..=5)
            .map(|sum| Tracked {
                sum,
                count: 1,
                clones: clones.clone(),
            })
            .collect();
        let identity_counter = clones.clone();
        let mut seg = LazySegmentTreeMonoid::new(
            &input,
            FnMonoid::new(
                move || Tracked {
                    sum: 0,
                    count: 0,
                    clones: identity_counter.clone(),
                },
                |a: &Tracked, b: &Tracked| Tracked {
                    sum: a.sum + b.sum,
                    count: a.count + b.count,
                    clones: a.clones.clone(),
                },
            ),
            FnMonoid::new(|| 0_i64, |a: &i64, b: &i64| *a + *b),
            |f: &i64, x: &mut Tracked| {
                x.sum += *f * x.count as i64;
            },
        );
        clones.set(0);
        seg.update(0, 5, 10);
        assert_eq!(seg.query(1, 4).sum, 39);
        assert_eq!(seg.get(2).sum, 13);
        assert_eq!(seg.as_slice().len(), 5);
        assert_eq!(clones.get(), 0);
        assert_eq!(seg.to_vec().len(), 5);
        assert_eq!(clones.get(), 5);
        clones.set(0);
        seg.update(1, 5, 1);
        assert_eq!(
            seg.into_vec().iter().map(|v| v.sum).collect::<Vec<_>>(),
            vec![11, 13, 14, 15, 16]
        );
        assert_eq!(clones.get(), 0);
    }

    #[test]
    fn native_point_set_after_pending_updates() {
        let mut rng = Rng(918273645);
        for n in [1, 2, 3, 7, 8, 9, 31, 32, 33, 65] {
            let mut values = vec![1_i64; n];
            let input: Vec<_> = values.iter().map(|&v| SumCount::new(v, 1)).collect();
            let mut seg = lazy_from_operator::<AffineSum>(&input);
            for _ in 0..128 {
                seg.update(0, n, (2, 3));
                for v in &mut values {
                    *v = (2 * *v + 3) % MOD;
                }
                let index = rng.next() % n;
                let value = (rng.next() as i64).rem_euclid(MOD);
                seg.set(index, SumCount::new(value, 1));
                values[index] = value;
                assert_eq!(*seg.all(), SumCount::new(values.iter().sum::<i64>() % MOD, n));
                assert_eq!(*seg.get(index), SumCount::new(value, 1));
            }
            assert_eq!(seg.into_vec().iter().map(|v| v.sum).collect::<Vec<_>>(), values);
        }
    }

    #[test]
    fn invalid_ranges_and_indices() {
        for n in [0, 1, 7] {
            let input = vec![SumCount::new(1_i64, 1); n];
            let mut original = LazySegmentTree::<RangeAssignSum<i64>>::new(&input);
            let mut monoid = lazy_from_operator::<RangeAssignSum<i64>>(&input);
            for (l, r) in [(1, 0), (0, n + 1), (usize::MAX, usize::MAX)] {
                // 既存版の範囲検証はdebug_assertなのでreleaseの前提外入力は実行しない。
                if cfg!(debug_assertions) {
                    assert!(catch_unwind(AssertUnwindSafe(|| original.update(l, r, Some(3)))).is_err());
                    assert!(catch_unwind(AssertUnwindSafe(|| original.query(l, r))).is_err());
                    assert_eq!(original.to_vec(), input);
                }
                assert!(catch_unwind(AssertUnwindSafe(|| monoid.update(l, r, Some(3)))).is_err());
                assert!(catch_unwind(AssertUnwindSafe(|| monoid.query(l, r))).is_err());
                assert_eq!(monoid.to_vec(), input);
            }
            for i in [n, usize::MAX] {
                assert!(
                    catch_unwind(AssertUnwindSafe(|| {
                        monoid.get(i);
                    }))
                    .is_err()
                );
                assert!(catch_unwind(AssertUnwindSafe(|| monoid.set(i, SumCount::new(3, 1)))).is_err());
                assert_eq!(monoid.to_vec(), input);
            }
        }
    }

    #[test]
    fn builtin_numeric_types_and_extremes() {
        let mut old = LazySegmentTree::<RangeAddMin<i32>>::new(&[i32::MAX]);
        let mut new = lazy_from_operator::<RangeAddMin<i32>>(&[i32::MAX]);
        old.update(0, 1, -1);
        new.update(0, 1, -1);
        assert_eq!(old.all(), i32::MAX - 1);
        assert_eq!(*new.all(), i32::MAX - 1);
        let mut old = LazySegmentTree::<RangeAddMax<i64>>::new(&[i64::MIN]);
        let mut new = lazy_from_operator::<RangeAddMax<i64>>(&[i64::MIN]);
        old.update(0, 1, 1);
        new.update(0, 1, 1);
        assert_eq!(old.all(), i64::MIN + 1);
        assert_eq!(*new.all(), i64::MIN + 1);
        let input = [SumCount::new(1_f64, 1), SumCount::new(2_f64, 1), SumCount::new(3_f64, 1)];
        let mut old = LazySegmentTree::<RangeAddSum<f64>>::new(&input);
        let mut new = lazy_from_operator::<RangeAddSum<f64>>(&input);
        old.update(0, 3, 1.5);
        new.update(0, 3, 1.5);
        assert_eq!(old.all().sum, 10.5);
        assert_eq!(new.all().sum, 10.5);
        let input = [SumCount::new(0_u32, 1); 3];
        let mut old = LazySegmentTree::<RangeAssignSum<u32>>::new(&input);
        let mut new = lazy_from_operator::<RangeAssignSum<u32>>(&input);
        old.update(0, 3, Some(2));
        new.update(0, 3, Some(2));
        assert_eq!(old.all().sum, 6);
        assert_eq!(new.all().sum, 6);
        let input = [MinIndex::new(i64::MAX, 0), MinIndex::new(i64::MAX, 1)];
        let mut old = LazySegmentTree::<RangeAssignMinIndex<i64>>::new(&input);
        let mut new = lazy_from_operator::<RangeAssignMinIndex<i64>>(&input);
        assert_eq!(old.all().index, 0);
        assert_eq!(new.all().index, 0);
        old.update(0, 2, Some(5));
        new.update(0, 2, Some(5));
        assert_eq!(old.query(1, 2).index, 1);
        assert_eq!(new.query(1, 2).index, 1);
    }

    #[test]
    #[ignore = "大規模検証用。releaseで実行する"]
    fn large_uniform_affine_updates() {
        let n = 500_000;
        let input = vec![SumCount::new(1_i64, 1); n];
        let mut old = LazySegmentTree::<AffineSum>::new(&input);
        let mut new = lazy_from_operator::<AffineSum>(&input);
        let mut value = 1_i64;
        for step in 0..500_000 {
            let (a, b) = ((step % 4) as i64, (step % 101) as i64);
            old.update(0, n, (a, b));
            new.update(0, n, (a, b));
            value = ((a as u128 * value as u128 + b as u128) % MOD as u128) as i64;
            if step % 4096 == 0 {
                let l = step % n;
                let answer = SumCount::new(value * (n - l) as i64 % MOD, n - l);
                assert_eq!(old.query(l, n), answer);
                assert_eq!(new.query(l, n), answer);
            }
        }
        let answer = SumCount::new(value * n as i64 % MOD, n);
        assert_eq!(old.all(), answer);
        assert_eq!(*new.all(), answer);
        assert!(old.to_vec().iter().all(|&v| v == SumCount::new(value, 1)));
        assert!(new.into_vec().iter().all(|&v| v == SumCount::new(value, 1)));
    }
}
