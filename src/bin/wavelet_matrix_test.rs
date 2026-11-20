use utility_rust::wavelet_matrix::WaveletMatrix;

fn example() {
    let a = [9, 9, 1, -3, 9, 5];
    let ordinary = WaveletMatrix::<false>::new(&a); // 出現回数を数える
    let distinct = WaveletMatrix::<true>::new(&a); // 区間内で値を1度だけ数える
    let with_sums = WaveletMatrix::<false>::with_sums(&a); // 合計クエリ向けの構築

    // 位置区間は [l, r)、順位 k は0始まり。0なら最大・最小。
    assert_eq!(ordinary.kth_largest(0, 3, 1), Some(9));
    assert_eq!(distinct.kth_largest(0, 3, 1), Some(1));
    assert_eq!(ordinary.kth_smallest(1, 5, 0), Some(-3));
    assert_eq!(ordinary.top_k(0, 3, 2), [9, 9]);
    assert_eq!(distinct.top_k(0, 3, 2), [9, 1]);
    assert_eq!(distinct.bottom_k(1, 6, 3), [-3, 1, 5]);

    // 同値の index は左から。重複を除くと、その区間での初出の index を返す。
    assert_eq!(ordinary.kth_largest_with_index(0, 3, 1), Some((9, 1)));
    assert_eq!(distinct.kth_largest_with_index(1, 5, 0), Some((9, 1)));
    assert_eq!(ordinary.get(3), Some(-3));

    // 位置が [1, 6)、値が [0, 10) に入る個数。
    assert_eq!(ordinary.range_freq(1, 6, 0, 10), 4);
    assert_eq!(distinct.range_freq(1, 6, 0, 10), 3);
    assert_eq!(distinct.count(1, 6), 4);
    assert_eq!(distinct.freq(1, 6, 9), 1);
    assert_eq!(distinct.occurrences(1, 6, 9), 2);
    assert_eq!(distinct.rank(5, 9), 3); // [0, 5) にある実際の出現回数
    assert_eq!(distinct.select(1, 6, 9, 1), Some(4)); // [1, 6) で2回目の 9
    assert_eq!(distinct.prev(0, 6, 5), Some(1)); // 5未満の最大
    assert_eq!(distinct.next(0, 6, 5), Some(5)); // 5以上の最小
    assert_eq!(distinct.histogram(0, 3), [(1, 1), (9, 2)]);

    assert_eq!(with_sums.range_sum(1, 6, 0, 10), 24);
    assert_eq!(with_sums.range_sum_bounds(1, 6, 0..), 24);
    assert_eq!(distinct.range_sum(1, 6, 0, 10), 15);
    assert_eq!(with_sums.sum_largest(0, 3, 2), Some(18));
    assert_eq!(distinct.sum_largest(0, 3, 2), Some(10));
    assert_eq!(distinct.sum_smallest(0, 3, 3), None); // 異なる値は2個
    println!("ウェーブレット行列の使用例: 成功");
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BTreeMap;

    fn verify_range<const DISTINCT: bool>(wm: &WaveletMatrix<DISTINCT>, a: &[i64], l: usize, r: usize, probes: &[i64]) {
        let mut asc: Vec<_> = a[l..r].iter().copied().zip(l..r).collect();
        asc.sort_unstable();
        if DISTINCT {
            asc.dedup_by_key(|entry| entry.0);
        }
        let mut desc = asc.clone();
        desc.sort_unstable_by_key(|&(value, i)| (Reverse(value), i));

        assert_eq!(wm.count(l, r), asc.len());
        assert_eq!(wm.min(l, r), asc.first().map(|entry| entry.0));
        assert_eq!(wm.max(l, r), desc.first().map(|entry| entry.0));
        assert_eq!(wm.sum(l, r), asc.iter().map(|&(x, _)| i128::from(x)).sum::<i128>());
        let mut histogram = BTreeMap::new();
        for &x in &a[l..r] {
            *histogram.entry(x).or_insert(0) += 1;
        }
        assert_eq!(wm.histogram(l, r), histogram.into_iter().collect::<Vec<_>>());

        for k in 0..=asc.len() + 1 {
            assert_eq!(wm.kth_smallest(l, r, k), asc.get(k).map(|entry| entry.0));
            assert_eq!(wm.kth_largest(l, r, k), desc.get(k).map(|entry| entry.0));
            assert_eq!(wm.kth_smallest_with_index(l, r, k), asc.get(k).copied());
            assert_eq!(wm.kth_largest_with_index(l, r, k), desc.get(k).copied());
            assert_eq!(
                wm.bottom_k(l, r, k),
                asc.iter().take(k).map(|entry| entry.0).collect::<Vec<_>>()
            );
            assert_eq!(wm.top_k(l, r, k), desc.iter().take(k).map(|entry| entry.0).collect::<Vec<_>>());
            let sum = |items: &[(i64, usize)]| items.get(..k).map(|s| s.iter().map(|&(x, _)| i128::from(x)).sum());
            assert_eq!(wm.sum_smallest(l, r, k), sum(&asc));
            assert_eq!(wm.sum_largest(l, r, k), sum(&desc));
        }

        for &x in probes {
            let raw: Vec<_> = (l..r).filter(|&i| a[i] == x).collect();
            assert_eq!(wm.occurrences(l, r, x), raw.len());
            assert_eq!(wm.rank(r, x), a[..r].iter().filter(|&&v| v == x).count());
            assert_eq!(wm.freq(l, r, x), asc.iter().filter(|entry| entry.0 == x).count());
            assert_eq!(wm.count_lt(l, r, x), asc.iter().filter(|entry| entry.0 < x).count());
            assert_eq!(wm.count_le(l, r, x), asc.iter().filter(|entry| entry.0 <= x).count());
            assert_eq!(
                wm.prev(l, r, x),
                asc.iter().rev().find(|entry| entry.0 < x).map(|entry| entry.0)
            );
            assert_eq!(wm.next(l, r, x), asc.iter().find(|entry| entry.0 >= x).map(|entry| entry.0));
            for k in 0..=raw.len() {
                assert_eq!(wm.select(l, r, x, k), raw.get(k).copied());
            }
            for &y in probes.iter().filter(|&&y| x <= y) {
                let band = asc.iter().filter(|entry| x <= entry.0 && entry.0 < y);
                assert_eq!(wm.range_freq(l, r, x, y), band.clone().count());
                assert_eq!(wm.range_sum(l, r, x, y), band.map(|entry| i128::from(entry.0)).sum::<i128>());
            }
        }
    }

    fn verify_all(a: &[i64], probes: &[i64]) {
        let ordinary = WaveletMatrix::<false>::new(a);
        let distinct = WaveletMatrix::<true>::new(a);
        let with_sums = WaveletMatrix::<false>::with_sums(a);
        assert_eq!(ordinary.len(), a.len());
        assert_eq!(distinct.is_empty(), a.is_empty());
        for i in 0..=a.len() {
            assert_eq!(ordinary.get(i), a.get(i).copied());
            assert_eq!(distinct.get(i), a.get(i).copied());
            for r in i..=a.len() {
                verify_range(&ordinary, a, i, r, probes);
                verify_range(&distinct, a, i, r, probes);
                verify_range(&with_sums, a, i, r, probes);
            }
        }
    }

    #[test]
    fn exhaustive_small_arrays_match_sorted_and_linear_oracles() {
        let alphabet = [-1, 0, 2];
        let probes = [-2, -1, 0, 1, 2, 3];
        for n in 0..=5 {
            for mut code in 0..3_usize.pow(n) {
                let a: Vec<_> = (0..n)
                    .map(|_| {
                        let x = alphabet[code % alphabet.len()];
                        code /= alphabet.len();
                        x
                    })
                    .collect();
                verify_all(&a, &probes);
            }
        }
    }

    #[test]
    fn both_i64_endpoints_and_all_equal_ranges() {
        verify_all(&[i64::MAX, -1, i64::MIN, i64::MAX, i64::MIN], &[i64::MIN, -1, 0, i64::MAX]);
        verify_all(&[i64::MIN; 5], &[i64::MIN, i64::MIN + 1, i64::MAX]);

        let a = [i64::MIN, i64::MAX, 0, i64::MAX];
        let ordinary = WaveletMatrix::<false>::with_sums(&a);
        let distinct = WaveletMatrix::<true>::new(&a);
        assert_eq!(ordinary.range_freq_bounds(0, 4, i64::MAX..), 2);
        assert_eq!(ordinary.range_sum_bounds(0, 4, i64::MAX..=i64::MAX), i128::from(i64::MAX) * 2);
        assert_eq!(distinct.range_freq_bounds(0, 4, ..), 3);
        assert_eq!(distinct.range_sum_bounds(0, 4, ..=i64::MIN), i128::from(i64::MIN));
        use std::ops::Bound::{Excluded, Included};
        assert_eq!(distinct.range_freq_bounds(0, 4, (Excluded(i64::MIN), Included(i64::MAX))), 2);
        assert_eq!(ordinary.range_sum_bounds(0, 4, (Excluded(i64::MAX), Included(i64::MAX))), 0);
    }

    #[test]
    fn packed_words_and_previous_occurrences_cross_boundaries() {
        let probes = [-4, -3, -1, 0, 1, 3, 4];
        // 長い同値列と、ワードをまたぐ出現を混在させて rank/select を検証する。
        for n in [63, 64, 65, 128, 129, 193] {
            let a: Vec<_> =
                (0..n).map(|i| if i < 70 || 100 < i && i < 168 { -1 } else { ((i * 11) % 31) as i64 - 15 }).collect();
            let ordinary = WaveletMatrix::<false>::with_sums(&a);
            let distinct = WaveletMatrix::<true>::new(&a);
            for (l, r) in [(0, n), (n / 3, n), (n / 2, n - 1), (n - 2, n)] {
                verify_range(&ordinary, &a, l, r, &probes);
                verify_range(&distinct, &a, l, r, &probes);
            }
            for i in 0..n {
                assert_eq!(distinct.get(i), Some(a[i]));
            }
        }
    }

    #[test]
    #[should_panic(expected = "位置区間")]
    fn invalid_positions_panic_even_if_k_is_zero() {
        WaveletMatrix::<true>::new(&[1, 2]).top_k(2, 1, 0);
    }

    #[test]
    #[should_panic(expected = "位置区間")]
    fn position_end_must_not_exceed_length() {
        WaveletMatrix::<false>::new(&[1, 2]).kth_smallest(0, 3, 0);
    }

    #[test]
    #[should_panic(expected = "値域")]
    fn half_open_count_rejects_reversed_value_bounds() {
        WaveletMatrix::<true>::new(&[1, 2]).range_freq(0, 2, 5, 3);
    }

    #[test]
    #[should_panic(expected = "値域")]
    fn half_open_sum_rejects_reversed_value_bounds() {
        WaveletMatrix::<false>::with_sums(&[1, 2]).range_sum(0, 2, 5, 3);
    }
}
