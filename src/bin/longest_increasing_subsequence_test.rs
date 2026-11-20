// longest_increasing_subsequenceの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use std::cmp::Reverse;
use utility_rust::longest_increasing_subsequence::*;

fn example() {
    // 空配列
    {
        let a: Vec<i32> = vec![];
        assert_eq!(increasing_subsequence(&a), vec![]);
        assert_eq!(non_decreasing_subsequence(&a), vec![]);
        assert_eq!(monotonic_subsequence(&a, |x, y| x < y), vec![]);
    }

    // 1要素
    {
        let a = vec![10];
        assert_eq!(increasing_subsequence(&a), vec![0]);
        assert_eq!(non_decreasing_subsequence(&a), vec![0]);
    }

    // 同じ値を含む
    //
    // 狭義: [1, 2, 3]    （長さ3）
    // 広義: [1, 2, 2, 3] （長さ4）
    {
        let a = vec![1, 2, 2, 3];
        assert_eq!(increasing_subsequence(&a).len(), 3);
        assert_eq!(non_decreasing_subsequence(&a), vec![0, 1, 2, 3]);
    }

    // 全て同じ値
    {
        let a = vec![5, 5, 5, 5];
        assert_eq!(increasing_subsequence(&a).len(), 1);
        assert_eq!(non_decreasing_subsequence(&a), vec![0, 1, 2, 3]);
    }

    // 全て減少
    {
        let a = vec![5, 4, 3, 2, 1];
        assert_eq!(increasing_subsequence(&a).len(), 1);
        assert_eq!(non_decreasing_subsequence(&a).len(), 1);

        // 減少列は Reverse で包めば同じ関数で求まる
        let b: Vec<_> = a.iter().map(Reverse).collect();
        assert_eq!(increasing_subsequence(&b), vec![0, 1, 2, 3, 4]);
    }

    // 典型例
    //
    // [3, 1, 4, 1, 5, 9, 2, 6]
    // 狭義LIS: 例えば [1, 4, 5, 9]（長さ4）
    // dp[i] = a[i] で終わる狭義LISの長さ
    {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let idx = increasing_subsequence(&a);
        assert_eq!(idx.len(), 4);
        assert!(is_valid(&a, &idx, true));

        assert_eq!(increasing_subsequence_dp(&a), vec![1, 1, 2, 1, 3, 4, 2, 4]);
        assert_eq!(non_decreasing_subsequence_dp(&a), vec![1, 1, 2, 2, 3, 4, 3, 4]);
    }

    // ランダムに O(N^2) の愚直DPと突き合わせる
    {
        let mut seed: u64 = 88172645463325252;
        let mut rng = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        for _ in 0..10000 {
            let n = (rng() % 11) as usize;
            let a: Vec<u64> = (0..n).map(|_| rng() % 6).collect();

            for &strict in &[true, false] {
                let (idx, dp) = lis_core(&a, strict);

                // 愚直DP
                let mut naive = vec![1usize; n];
                for i in 0..n {
                    for j in 0..i {
                        let ok = if strict { a[j] < a[i] } else { a[j] <= a[i] };
                        if ok {
                            naive[i] = naive[i].max(naive[j] + 1);
                        }
                    }
                }

                assert_eq!(dp, naive);
                assert_eq!(idx.len(), naive.iter().copied().max().unwrap_or(0));
                assert!(is_valid(&a, &idx, strict));

                let generic = if strict { monotonic_subsequence(&a, |x, y| x < y) } else { monotonic_subsequence(&a, |x, y| x <= y) };
                assert_eq!(generic.len(), idx.len());
                assert!(is_valid(&a, &generic, strict));
            }
        }
    }

    println!("All tests passed!");
}

/// テスト用: idx が a の単調部分列になっているか
fn is_valid<T: Ord>(a: &[T], idx: &[usize], strict: bool) -> bool {
    idx.windows(2).all(|w| w[0] < w[1] && if strict { a[w[0]] < a[w[1]] } else { a[w[0]] <= a[w[1]] })
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
