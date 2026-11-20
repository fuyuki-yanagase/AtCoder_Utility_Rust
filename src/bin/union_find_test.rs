// union_findの使用例・テスト。
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::zero_repeat_side_effects)]

use num_rational::Ratio;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::f64::consts::{PI, TAU};
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

// 1行でのbreak, continue, return
macro_rules! breakif {
    ($c:expr) => {
        if $c {
            break;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            break $l;
        }
    };
}
macro_rules! continueif {
    ($c:expr) => {
        if $c {
            continue;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            continue $l;
        }
    };
}
macro_rules! returnif {
    ($c:expr) => {
        if $c {
            return;
        }
    };
    ($c:expr, $v:expr) => {
        if $c {
            return $v;
        }
    };
}

const M107: i64 = 1000000007_i64;
const M998: i64 = 998244353_i64;
const L108: i64 = 100_000_000_i64;
const I108: i32 = 100_000_000_i32;
const L1018: i64 = 1_000_000_000_000_000_000_i64;
const IINF: i32 = (1_i32 << 29) - 1;
const LINF: i64 = (1_i64 << 61) - 1;
const UINF: usize = (1_usize << 61) - 1;

use utility_rust::union_find::*;

fn example() {
    let mut uf = UnionFind::new(6);
    uf.unite(0, 2);
    uf.unite(2, 3);
    uf.unite(1, 4);
    assert_eq!(uf.groups(), vec![vec![0, 2, 3], vec![1, 4], vec![5]]);
    assert_eq!(uf.size(2), 3);
    println!("連結成分: {:?}", uf.groups());
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn empty_isolated_and_repeated_merges() {
    assert!(UnionFind::new(0).groups().is_empty());
    let mut uf = UnionFind::new(5);
    assert_eq!(uf.groups(), vec![vec![0], vec![1], vec![2], vec![3], vec![4]]);
    for (u, v) in [(3, 1), (1, 3), (0, 4), (1, 4), (0, 0)] {
        uf.unite(u, v);
    }
    assert_eq!(uf.groups(), vec![vec![0, 1, 3, 4], vec![2]]);
    for v in [0, 1, 3, 4] {
        assert_eq!(uf.size(v), 4);
        assert!(uf.connected(v, 3));
    }
    assert!(!uf.connected(2, 3));
    assert_eq!(uf.size(2), 1);
}

#[cfg(test)]
mod range_parallel_tests {
    use super::*;

    fn naive_unite(labels: &mut [usize], a: usize, b: usize, k: usize) -> usize {
        let mut count = 0;
        for i in 0..k {
            let from = labels[a + i];
            let to = labels[b + i];
            if from != to {
                count += 1;
                for label in labels.iter_mut() {
                    if *label == from {
                        *label = to;
                    }
                }
            }
        }
        return count;
    }

    fn check_partition(uf: &mut RangeParallelUnionFind, labels: &[usize]) {
        for a in 0..labels.len() {
            let expected_size = labels.iter().filter(|&&x| x == labels[a]).count();
            assert_eq!(uf.size(a), expected_size);
            let root = uf.root(a);
            assert_eq!(uf.root(root), root);
            for b in 0..labels.len() {
                assert_eq!(uf.connected(a, b), labels[a] == labels[b]);
            }
        }
        let mut expected_groups: Vec<Vec<usize>> = Vec::new();
        for a in 0..labels.len() {
            if (0..a).all(|b| labels[b] != labels[a]) {
                expected_groups.push((a..labels.len()).filter(|&b| labels[a] == labels[b]).collect());
            }
        }
        assert_eq!(uf.groups(), expected_groups);
    }

    #[test]
    fn empty_singleton_and_invalid_ranges() {
        let mut empty = RangeParallelUnionFind::new(0);
        assert!(empty.groups().is_empty());
        assert_eq!(empty.unite_range_with(0, 0, 0, |_, _| panic!("空区間でコールバック")), 0);

        let mut singleton = RangeParallelUnionFind::new(1);
        assert!(!singleton.unite(0, 0));
        assert_eq!(singleton.unite_range(1, 1, 0), 0);
        assert_eq!(singleton.size(0), 1);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            empty.unite_range(0, 0, 1);
        }));
        assert!(result.is_err());
        assert!(empty.groups().is_empty());
        for (n, a, b) in [(0, 0, 0), (5, 5, 0), (5, 5, 5)] {
            let mut uf = RangeParallelUnionFind::new(n);
            let before = uf.groups();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                uf.unite(a, b);
            }));
            assert!(result.is_err());
            assert_eq!(uf.groups(), before);
        }

        for (a, b, k) in [(6, 0, 0), (0, 6, 0), (4, 0, 2), (0, 4, 2), (0, 0, usize::MAX)] {
            let mut uf = RangeParallelUnionFind::new(5);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                uf.unite_range_with(a, b, k, |_, _| panic!("入力検証より先に併合"));
            }));
            assert!(result.is_err());
            assert_eq!(uf.groups(), vec![vec![0], vec![1], vec![2], vec![3], vec![4]]);
        }
    }

    #[test]
    fn all_small_ranges_and_mixed_single_merges() {
        for n in 1..=9 {
            for k in 0..=n {
                for a in 0..=n - k {
                    for b in 0..=n - k {
                        let mut uf = RangeParallelUnionFind::new(n);
                        let mut labels: Vec<_> = (0..n).collect();
                        assert_eq!(uf.unite_range(a, b, k), naive_unite(&mut labels, a, b, k));
                        check_partition(&mut uf, &labels);
                        assert_eq!(uf.unite_range_with(b, a, k, |_, _| panic!("重複した結合")), 0);
                        assert_eq!(uf.unite(0, n - 1), naive_unite(&mut labels, 0, n - 1, 1) != 0);
                        check_partition(&mut uf, &labels);
                    }
                }
            }
        }
    }

    #[test]
    fn samples_with_component_sums() {
        let queries = [(0, 0, 0), (1, 0, 0), (1, 0, 2), (2, 2, 1), (2, 0, 1), (4, 0, 0), (3, 2, 1)];
        for (mut sums, expected) in [
            (vec![1_u64; 5], vec![0, 0, 1, 6, 6, 6, 10]),
            (vec![12_u64, 34, 56, 78, 90], vec![0, 0, 672, 10940, 10940, 10940, 27140]),
        ] {
            let mut uf = RangeParallelUnionFind::new(5);
            let mut answer = 0;
            for ((k, a, b), expected) in queries.into_iter().zip(expected) {
                uf.unite_range_with(a, b, k, |root, absorbed| {
                    answer = (answer + sums[root] * sums[absorbed]) % 998244353;
                    sums[root] = (sums[root] + sums[absorbed]) % 998244353;
                });
                assert_eq!(answer, expected);
            }
        }
    }

    #[test]
    fn random_callbacks_against_independent_labels() {
        const MOD: u64 = 998244353;
        let mut state = 918273645_u64;
        let mut random = || {
            state ^= state << 7;
            state ^= state >> 9;
            return state;
        };
        for _ in 0..100 {
            let n = random() as usize % 40 + 1;
            let values: Vec<_> = (0..n).map(|_| random() % MOD).collect();
            let mut sums = values.clone();
            let mut uf = RangeParallelUnionFind::new(n);
            let mut labels: Vec<_> = (0..n).collect();
            // コールバックの根の向き・多重通知を独立に検証するラベル。
            let mut callback_labels = labels.clone();
            let mut answer = 0;
            for turn in 0..100 {
                let max_k = if turn < 40 { n.min(2) } else { n };
                let k = random() as usize % (max_k + 1);
                let a = random() as usize % (n - k + 1);
                let b = random() as usize % (n - k + 1);
                let expected_count = naive_unite(&mut labels, a, b, k);
                let mut calls = 0;
                let actual_count = uf.unite_range_with(a, b, k, |root, absorbed| {
                    assert_ne!(root, absorbed);
                    assert_eq!(callback_labels[root], root);
                    assert_eq!(callback_labels[absorbed], absorbed);
                    for label in &mut callback_labels {
                        if *label == absorbed {
                            *label = root;
                        }
                    }
                    calls += 1;
                    answer = (answer + sums[root] * sums[absorbed]) % MOD;
                    sums[root] = (sums[root] + sums[absorbed]) % MOD;
                });
                assert_eq!(actual_count, expected_count);
                assert_eq!(calls, expected_count);
                check_partition(&mut uf, &labels);

                let mut expected_answer = 0;
                for i in 0..n {
                    let root = uf.root(i);
                    assert_eq!(callback_labels[i], root);
                    let expected_sum = (0..n).filter(|&j| labels[j] == labels[i])
                        .fold(0, |sum, j| (sum + values[j]) % MOD);
                    assert_eq!(sums[root], expected_sum);
                    for j in 0..i {
                        if labels[i] == labels[j] {
                            expected_answer = (expected_answer + values[i] * values[j]) % MOD;
                        }
                    }
                }
                assert_eq!(answer, expected_answer);
            }
        }
    }

    #[test]
    fn callback_panic_does_not_mark_unfinished_blocks() {
        for stop_after in 1..=6 {
            let mut uf = RangeParallelUnionFind::new(10);
            let mut calls_before_panic = 0;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                uf.unite_range_with(0, 4, 6, |_, _| {
                    calls_before_panic += 1;
                    if calls_before_panic == stop_after {
                        panic!("コールバックの中断");
                    }
                });
            }));
            assert!(result.is_err());
            let mut calls_on_retry = 0;
            let count = uf.unite_range_with(0, 4, 6, |root, absorbed| {
                assert_ne!((root, absorbed), (4, 0));
                assert_ne!((root, absorbed), (0, 4));
                calls_on_retry += 1;
            });
            assert_eq!(count, 6 - stop_after);
            assert_eq!(calls_on_retry, count);
            let mut labels: Vec<_> = (0..10).collect();
            naive_unite(&mut labels, 0, 4, 6);
            check_partition(&mut uf, &labels);
        }
    }

    #[test]
    fn mixed_single_and_range_sequences() {
        let mut state = 135792468_u64;
        let mut random = || {
            state ^= state << 7;
            state ^= state >> 9;
            return state as usize;
        };
        for _ in 0..50 {
            let n = random() % 32 + 1;
            let mut uf = RangeParallelUnionFind::new(n);
            let mut labels: Vec<_> = (0..n).collect();
            for turn in 0..100 {
                let a = random() % n;
                let b = random() % n;
                if turn % 3 == 0 {
                    assert_eq!(uf.unite(a, b), naive_unite(&mut labels, a, b, 1) != 0);
                } else {
                    let k = random() % ((n - a).min(n - b) + 1);
                    assert_eq!(uf.unite_range(a, b, k), naive_unite(&mut labels, a, b, k));
                }
                check_partition(&mut uf, &labels);
            }
        }
    }

    #[test]
    #[ignore = "最大制約の性能確認用。releaseで実行する"]
    fn maximum_constraints() {
        let n = 500_000;
        let mut uf = RangeParallelUnionFind::new(n);
        let mut count = 0;
        for b in 1..=n {
            count += uf.unite_range(0, b, n - b);
        }
        assert_eq!(count, n - 1);
        assert_eq!(uf.size(0), n);
        drop(uf);

        // 成分が1つずつ増大するケースでは、毎回の集計も検証する。
        let mut uf = RangeParallelUnionFind::new(n);
        let mut sizes = vec![1_usize; n];
        let mut calls = 0;
        for b in 1..n {
            assert_eq!(uf.unite_range_with(b - 1, b, 1, |root, absorbed| {
                sizes[root] += sizes[absorbed];
                calls += 1;
            }), 1);
        }
        assert_eq!(calls, n - 1);
        let root = uf.root(0);
        assert_eq!(sizes[root], n);
        assert_eq!(uf.size(0), n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_rust::monoid_group_abelian::{FnGroup, Group};

    const MOD: i64 = 998244353;

    fn additive_group() -> impl Group<S = i64> {
        FnGroup::new(
            || 0_i64,
            |a: &i64, b: &i64| (*a + *b).rem_euclid(MOD),
            |a: &i64| (-*a).rem_euclid(MOD),
        )
    }

    #[test]
    fn potential_empty_isolated_and_self_constraints() {
        for n in [0, 1, 2, 7] {
            let mut uf = PotentialUnionFind::new(n, additive_group());
            let expected: Vec<Vec<usize>> = (0..n).map(|v| vec![v]).collect();
            assert_eq!(uf.groups(), expected);
            for v in 0..n {
                assert_eq!(uf.size(v), 1);
                assert_eq!(uf.potential(v), 0);
                assert_eq!(uf.diff(v, v), Some(0));
                assert!(uf.unite(v, v, 0));
                assert!(!uf.unite(v, v, 1));
            }
            assert_eq!(uf.groups(), expected);
        }
    }

    /// mod MOD の加法群で差分制約を検証する。
    #[test]
    fn additive_group_constraints() {
        let group = FnGroup::new(
            || 0_i64,
            |a: &i64, b: &i64| (*a + *b).rem_euclid(MOD),
            |a: &i64| (-*a).rem_euclid(MOD),
        );
        let mut uf = PotentialUnionFind::new(4, group);

        assert!(uf.unite(0, 1, 3));
        assert!(uf.unite(1, 2, 5));
        assert_eq!(uf.diff(0, 2), Some(8));
        assert_eq!(uf.diff(2, 0), Some(MOD - 8));
        assert!(uf.unite(0, 2, 8));

        // 矛盾する制約は受理されず、既存の差を維持する。
        assert!(!uf.unite(0, 2, 9));
        assert_eq!(uf.diff(0, 2), Some(8));
        assert_eq!(uf.diff(0, 3), None);
        assert_eq!(uf.size(1), 3);
        assert_eq!(uf.groups().len(), 2);
    }

    /// 行列式1の2x2行列。行列積は非可換。
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Matrix2([[i64; 2]; 2]);

    impl Matrix2 {
        fn identity() -> Self {
            return Self([[1, 0], [0, 1]]);
        }

        fn multiply(&self, rhs: &Self) -> Self {
            let a = &self.0;
            let b = &rhs.0;

            return Self([
                [(a[0][0] * b[0][0] + a[0][1] * b[1][0]) % MOD, (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % MOD],
                [(a[1][0] * b[0][0] + a[1][1] * b[1][0]) % MOD, (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % MOD],
            ]);
        }

        /// 行列式が1である行列の逆行列。
        fn inverse(&self) -> Self {
            let a = &self.0;
            return Self([[a[1][1], (MOD - a[0][1]) % MOD], [(MOD - a[1][0]) % MOD, a[0][0]]]);
        }
    }

    /// 演算順序の違いが結果に現れる非可換群で検証する。
    #[test]
    fn non_commutative_matrix_constraints() {
        let a = Matrix2([[1, 1], [0, 1]]);
        let b = Matrix2([[1, 0], [1, 1]]);
        assert_ne!(a.multiply(&b), b.multiply(&a));

        let group = FnGroup::new(
            Matrix2::identity,
            |left: &Matrix2, right: &Matrix2| left.multiply(right),
            |value: &Matrix2| value.inverse(),
        );
        let mut uf = PotentialUnionFind::new(3, group);

        // 未知の値を a[0]=a, a[1]=b, a[2]=I と仮定する。
        // 0,1間の差は inverse(b)*a、1,2間の差は b。
        let x01 = b.inverse().multiply(&a);
        assert!(uf.unite(0, 1, x01.clone()));
        assert!(uf.unite(1, 2, b.clone()));

        // inverse(a[2])*a[0] = a、逆方向は inverse(a)。
        assert_eq!(uf.diff(0, 2), Some(a.clone()));
        assert_eq!(uf.diff(2, 0), Some(a.inverse()));
        assert_eq!(uf.diff(0, 1), Some(x01));

        // 同じ頂点同士の差は単位行列。
        assert!(uf.unite(2, 2, Matrix2::identity()));
        assert!(!uf.unite(2, 2, a));
        let root2 = uf.root(2);
        assert_eq!(uf.potential(root2), Matrix2::identity());
    }

    #[test]
    fn balanced_non_commutative_merges_and_path_compression() {
        let a = Matrix2([[1, 1], [0, 1]]);
        let b = Matrix2([[1, 0], [1, 1]]);
        let potentials = [
            Matrix2::identity(),
            a.clone(),
            b.clone(),
            a.multiply(&b),
            b.multiply(&a),
            a.inverse(),
            b.inverse(),
            a.multiply(&b).multiply(&a),
        ];
        let group = FnGroup::new(
            Matrix2::identity,
            |left: &Matrix2, right: &Matrix2| left.multiply(right),
            |value: &Matrix2| value.inverse(),
        );
        let mut uf = PotentialUnionFind::new(potentials.len(), group);

        // 同サイズの成分を段階的に結合し、経路圧縮前の深さを作る。
        for (u, v) in [(0, 1), (2, 3), (4, 5), (6, 7), (0, 2), (4, 6), (0, 4)] {
            let relation = potentials[v].inverse().multiply(&potentials[u]);
            assert!(uf.unite(u, v, relation));
        }

        // 固定した頂点値から差を直接計算し、圧縮後も全頂点対で一致するか確認。
        for _ in 0..2 {
            for u in 0..potentials.len() {
                assert_eq!(uf.size(u), potentials.len());
                let root = uf.root(u);
                assert_eq!(uf.potential(u), potentials[root].inverse().multiply(&potentials[u]));
                for v in 0..potentials.len() {
                    let expected = potentials[v].inverse().multiply(&potentials[u]);
                    assert_eq!(uf.diff(u, v), Some(expected));
                }
            }
        }
        assert_eq!(uf.groups(), vec![(0..potentials.len()).collect::<Vec<usize>>()]);

        let relation = potentials[7].inverse().multiply(&potentials[0]);
        assert!(uf.unite(0, 7, relation.clone()));
        // 単位元でないaを掛けて矛盾を作る。拒否後も既存の関係は保持される。
        assert!(!uf.unite(0, 7, relation.multiply(&a)));
        assert_eq!(uf.diff(0, 7), Some(relation));
        assert_eq!(uf.size(0), potentials.len());
    }

    #[test]
    fn weighted_and_group_versions_match_fixed_modular_potentials() {
        // WeightedUnionFindへ渡す重みは、ここでは[0, MOD)に正規化する。
        let potentials = [0, 1, MOD - 1, 30, 4, MOD - 5, 6, 7];
        let n = potentials.len();
        let mut weighted = WeightedUnionFind::new(n);
        let mut generic = PotentialUnionFind::new(n, additive_group());
        let mut labels: Vec<usize> = (0..n).collect();

        for (u, v) in [(0, 1), (2, 3), (0, 2), (4, 5), (6, 7), (4, 6), (0, 4)] {
            let relation = (potentials[u] - potentials[v]).rem_euclid(MOD);
            assert!(weighted.unite(u, v, relation));
            assert!(generic.unite(u, v, relation));

            // 木構造を使わない成分ラベルで、連結性と成分サイズを照合する。
            let from = labels[u];
            let to = labels[v];
            for label in &mut labels {
                if *label == from {
                    *label = to;
                }
            }
            for a in 0..n {
                let size = labels.iter().filter(|&&label| label == labels[a]).count();
                assert_eq!(weighted.size(a), size);
                assert_eq!(generic.size(a), size);
                for b in 0..n {
                    let connected = labels[a] == labels[b];
                    let expected = connected.then(|| (potentials[a] - potentials[b]).rem_euclid(MOD));
                    assert_eq!(weighted.connected(a, b), connected);
                    assert_eq!(generic.connected(a, b), connected);
                    assert_eq!(weighted.diff(a, b), expected);
                    assert_eq!(generic.diff(a, b), expected);
                }
            }
        }

        let relation = (potentials[0] - potentials[7]).rem_euclid(MOD);
        assert!(weighted.unite(0, 7, relation));
        assert!(generic.unite(0, 7, relation));
        assert!(!weighted.unite(0, 7, (relation + 1) % MOD));
        assert!(!generic.unite(0, 7, (relation + 1) % MOD));
        assert_eq!(weighted.diff(0, 7), Some(relation));
        assert_eq!(generic.diff(0, 7), Some(relation));
    }
}
