// mod行列・行列群の独立oracle・境界テスト。
use utility_rust::matrix::*;
#[cfg(test)]
use utility_rust::monoid_group_abelian::{FnGroup, FnMonoid, Group, Monoid};

fn main() {
    let fibonacci: Matrix2 = Matrix2::new(1, 1, 1, 0);
    assert_eq!(fibonacci.pow(10).to_rows(), [[89, 55], [55, 34]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    // 実装の消去法・高速積・繰り返し二乗法を使わず、整数上の定義から計算する。
    type Rows = Vec<Vec<i128>>;

    fn product(a: &Rows, b: &Rows, cols: usize, modulo: i128) -> Rows {
        (0..a.len())
            .map(|i| {
                (0..cols).map(|j| (0..b.len()).map(|k| a[i][k] * b[k][j]).sum::<i128>().rem_euclid(modulo)).collect()
            })
            .collect()
    }

    fn identity(n: usize) -> Rows {
        (0..n).map(|i| (0..n).map(|j| i128::from(i == j)).collect()).collect()
    }

    // Leibnizの公式。行交換やpivotの選択とは独立した行列式。
    fn determinant(a: &Rows, modulo: i128) -> i128 {
        fn visit(a: &Rows, permutation: &mut Vec<usize>, used: &mut [bool], sum: &mut i128, modulo: i128) {
            if permutation.len() == a.len() {
                let inversions = (0..a.len())
                    .flat_map(|i| (i + 1..a.len()).map(move |j| (i, j)))
                    .filter(|&(i, j)| permutation[i] > permutation[j])
                    .count();
                let term: i128 = permutation.iter().enumerate().map(|(i, &j)| a[i][j]).product();
                *sum = (*sum + if inversions % 2 == 0 { term } else { -term }).rem_euclid(modulo);
                return;
            }
            for j in 0..a.len() {
                if !used[j] {
                    used[j] = true;
                    permutation.push(j);
                    visit(a, permutation, used, sum, modulo);
                    permutation.pop();
                    used[j] = false;
                }
            }
        }
        let mut sum = 0;
        visit(a, &mut Vec::new(), &mut vec![false; a.len()], &mut sum, modulo);
        sum.rem_euclid(modulo)
    }

    fn scalar_inverse(value: i128, modulo: i128) -> i128 {
        // 拡張Euclid。ライブラリのFermatによる逆元と異なる方法を使う。
        let (mut a, mut b, mut x, mut y) = (value, modulo, 1, 0);
        while b != 0 {
            let quotient = a / b;
            (a, b) = (b, a - quotient * b);
            (x, y) = (y, x - quotient * y);
        }
        assert_eq!(a, 1);
        x.rem_euclid(modulo)
    }

    fn inverse(a: &Rows, modulo: i128) -> Option<Rows> {
        let det = determinant(a, modulo);
        if det == 0 {
            return None;
        }
        let inv_det = scalar_inverse(det, modulo);
        Some(
            (0..a.len())
                .map(|i| {
                    (0..a.len())
                        .map(|j| {
                            let minor = a
                                .iter()
                                .enumerate()
                                .filter(|&(r, _)| r != j)
                                .map(|(_, row)| {
                                    row.iter().enumerate().filter(|&(c, _)| c != i).map(|(_, &x)| x).collect()
                                })
                                .collect();
                            let sign = if (i + j) % 2 == 0 { 1 } else { -1 };
                            (sign * determinant(&minor, modulo) * inv_det).rem_euclid(modulo)
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn rows<const MOD: u32>(a: &Matrix<MOD>) -> Rows {
        a.to_rows().into_iter().map(|r| r.into_iter().map(i128::from).collect()).collect()
    }

    fn rows2<const MOD: u32>(a: Matrix2<MOD>) -> Rows {
        a.to_rows().into_iter().map(|r| r.into_iter().map(i128::from).collect()).collect()
    }

    #[test]
    fn fixed_products_and_operators_preserve_order() {
        let a = Matrix2::<7>::new(1, 1, 0, 1);
        let b = Matrix2::<7>::new(1, 0, 1, 1);
        // 期待値をライブラリの別APIから生成しない。
        assert_eq!((a * b).to_rows(), [[2, 1], [1, 1]]);
        assert_eq!((b * a).to_rows(), [[1, 1], [1, 2]]);
        assert_eq!(a.try_inverse().unwrap().to_rows(), [[1, 6], [0, 1]]);
        assert_eq!((a + b).to_rows(), [[2, 1], [1, 2]]);
        assert_eq!((a - b).to_rows(), [[0, 1], [6, 0]]);
        assert_eq!((-a).to_rows(), [[6, 6], [0, 6]]);
        let mut value = a;
        value += b;
        value -= b;
        value *= b;
        assert_eq!(value.to_rows(), [[2, 1], [1, 1]]);
        assert_eq!(a.transpose().to_rows(), [[1, 0], [1, 1]]);
        assert_eq!(a.scale(9).to_rows(), [[2, 2], [0, 2]]);
        assert_eq!(a.apply([8, 9]), [3, 2]);
        assert_eq!(a.trace(), 2);
        assert_eq!(a.determinant(), 1);
    }

    #[test]
    fn exhaustive_matrix2_mod_three_against_integer_definitions() {
        let all: Vec<_> =
            (0..81).map(|code| Matrix2::<3>::new(code % 3, code / 3 % 3, code / 9 % 3, code / 27 % 3)).collect();
        for &a in &all {
            let expected = rows2(a);
            assert_eq!(a.determinant() as i128, determinant(&expected, 3));
            assert_eq!(a.try_inverse().map(rows2), inverse(&expected, 3));
            for &b in &all {
                assert_eq!(rows2(a * b), product(&expected, &rows2(b), 2, 3));
            }
        }
    }

    fn check_dynamic<const MOD: u32>() {
        let modulo = MOD as i128;
        let mut seed = 20261028_u64;
        for n in 0..=4 {
            for case in 0..24 {
                let values = (0..n)
                    .map(|_| {
                        (0..n)
                            .map(|_| {
                                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                                // 0成分、負数、法の倍数、正規化境界を混ぜる。
                                (seed % (MOD as u64 * 4 + 1)) as i64 - MOD as i64 * 2
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let expected: Rows =
                    values.iter().map(|r| r.iter().map(|&x| (x as i128).rem_euclid(modulo)).collect()).collect();
                let a = Matrix::<MOD>::from_signed_rows(values);
                assert_eq!(rows(&a), expected);
                assert_eq!(a.shape(), (n, n));
                assert_eq!(a.entries().len(), n * n);
                assert_eq!(a.trace() as i128, (0..n).map(|i| expected[i][i]).sum::<i128>() % modulo);
                assert_eq!(a.determinant() as i128, determinant(&expected, modulo), "n={n}, case={case}");
                let inv = inverse(&expected, modulo);
                assert_eq!(a.try_inverse().as_ref().map(rows), inv, "n={n}, case={case}");
                assert_eq!(InvertibleMatrix::try_new(a.clone()).is_some(), inv.is_some());
                let mut power = identity(n);
                for exponent in 0..=8 {
                    assert_eq!(rows(&a.pow(exponent)), power);
                    assert_eq!(a.pow_signed(exponent as i64).as_ref().map(rows), Some(power.clone()));
                    power = product(&power, &expected, n, modulo);
                }
                if let Some(inv) = inv {
                    let expected_negative = product(&product(&inv, &inv, n, modulo), &inv, n, modulo);
                    assert_eq!(rows(&a.pow_signed(-3).unwrap()), expected_negative);
                    let group = MatrixGroup::<MOD>::new(n);
                    let element = group.try_element(a.clone()).unwrap();
                    assert_eq!(rows(group.inverse(&element).as_matrix()), inv);
                    assert_eq!(rows(element.pow(-3).as_matrix()), expected_negative);
                    assert_eq!((&element * &element).into_matrix(), a.multiply(&a));
                } else {
                    assert_eq!(a.pow_signed(-1), None);
                }
                // 2x2特化実装と動的実装の両方を独立oracleに照らす。
                if n == 2 {
                    let fixed = Matrix2::<MOD>::new(
                        a.get(0, 0) as u64,
                        a.get(0, 1) as u64,
                        a.get(1, 0) as u64,
                        a.get(1, 1) as u64,
                    );
                    assert_eq!(fixed.try_inverse().map(rows2), inverse(&expected, modulo));
                    assert_eq!(rows2(fixed.pow(5)), rows(&a.pow(5)));
                    assert_eq!(rows2(fixed.pow_signed(0).unwrap()), identity(2));
                }
            }
        }
    }

    #[test]
    fn determinants_inverses_and_powers_against_cofactors() {
        check_dynamic::<2>();
        check_dynamic::<7>();
        check_dynamic::<998244353>();
        check_dynamic::<2147483647>();
    }

    #[test]
    fn pivot_swaps_singular_and_empty_matrices() {
        let a = Matrix::<7>::from_rows(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 2]]);
        assert_eq!(a.determinant(), 5);
        assert_eq!(
            a.try_inverse().unwrap().to_rows(),
            vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 4]]
        );
        let singular = Matrix::<7>::from_rows(vec![vec![1, 2, 3], vec![2, 4, 6], vec![0, 1, 2]]);
        assert_eq!(singular.determinant(), 0);
        assert_eq!(singular.try_inverse(), None);
        assert_eq!(singular.pow_signed(-2), None);
        assert_eq!(singular.pow_signed(0), Some(Matrix::identity(3)));
        let fixed_singular = Matrix2::<7>::new(1, 2, 2, 4);
        assert_eq!(fixed_singular.try_inverse(), None);
        assert_eq!(fixed_singular.pow_signed(-1), None);
        assert_eq!(fixed_singular.pow_signed(0), Some(Matrix2::identity()));
        assert_eq!(InvertibleMatrix2::try_new(fixed_singular), None);
        assert_eq!(Sl2::try_new(fixed_singular), None);
        assert_eq!(Matrix::<7>::identity(0).determinant(), 1);
        assert_eq!(Matrix::<7>::identity(0).try_inverse(), Some(Matrix::identity(0)));
        assert_eq!(
            Matrix::<7>::zero(3, 0).multiply(&Matrix::zero(0, 2)).to_rows(),
            vec![vec![0, 0]; 3]
        );
        assert_eq!(Matrix::<7>::zero(0, 3).transpose().shape(), (3, 0));
    }

    #[test]
    fn rectangular_signed_and_normalization_apis() {
        let mut a = Matrix::<7>::from_signed_rows(vec![vec![-1, 7, 8], vec![-8, 2, 3]]);
        assert_eq!(a.to_rows(), vec![vec![6, 0, 1], vec![6, 2, 3]]);
        assert_eq!((a.rows(), a.cols()), (2, 3));
        a.set(0, 1, 16);
        assert_eq!(a.get(0, 1), 2);
        let b = Matrix::<7>::from_flat(3, 2, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!((&a * &b).to_rows(), vec![vec![3, 5], vec![6, 3]]);
        assert_eq!(a.apply(&[8, 9, 10]), vec![6, 5]);
        assert_eq!(a.transpose().to_rows(), vec![vec![6, 6], vec![2, 2], vec![1, 3]]);
        assert_eq!((&a + &a).to_rows(), vec![vec![5, 4, 2], vec![5, 4, 6]]);
        assert_eq!((&a - &a).to_rows(), vec![vec![0; 3]; 2]);
        assert_eq!((-&a).to_rows(), vec![vec![1, 5, 6], vec![1, 5, 4]]);
        assert_eq!(a.scale(9).to_rows(), vec![vec![5, 4, 2], vec![5, 4, 6]]);
        assert_eq!(a.try_inverse(), None);
        assert_eq!(InvertibleMatrix::try_new(a.clone()), None);
        assert_eq!(MatrixGroup::<7>::new(2).try_element(a), None);
        let mut fixed = Matrix2::<7>::new(7, 8, 13, 14);
        fixed.set(1, 0, 15);
        assert_eq!(*fixed.entries(), [0, 1, 1, 0]);
        assert_eq!(fixed.get(1, 0), 1);
    }

    #[test]
    fn group_wrappers_and_signed_powers_have_fixed_expected_values() {
        let a = Matrix2::<7>::new(1, 2, 0, 1);
        assert_eq!(a.pow_signed(-3).unwrap().to_rows(), [[1, 1], [0, 1]]);
        let sl = Sl2::try_new(a).unwrap();
        assert_eq!(sl.as_matrix(), &a);
        assert_eq!(sl.inverse().into_matrix().to_rows(), [[1, 5], [0, 1]]);
        assert_eq!(sl.pow(-3).into_matrix().to_rows(), [[1, 1], [0, 1]]);
        assert_eq!(Sl2Group::<7>.combine(&sl, &sl).into_matrix().to_rows(), [[1, 4], [0, 1]]);
        assert_eq!(Sl2Group::<7>.inverse(&sl), sl.inverse());
        let b = Matrix2::<7>::new(2, 0, 0, 1);
        assert_eq!(Sl2::try_new(b), None);
        let invertible = InvertibleMatrix2::try_new(b).unwrap();
        assert_eq!(invertible.as_matrix(), &b);
        assert_eq!(invertible.inverse().into_matrix().to_rows(), [[4, 0], [0, 1]]);
        assert_eq!(invertible.pow(-2).into_matrix().to_rows(), [[2, 0], [0, 1]]);
        assert_eq!(
            Matrix2Group::<7>.combine(&invertible, &invertible).into_matrix().to_rows(),
            [[4, 0], [0, 1]]
        );
        assert_eq!(Matrix2Group::<7>.inverse(&invertible), invertible.inverse());
        let concat = FnMonoid::new(String::new, |a: &String, b: &String| format!("{a}{b}"));
        assert_eq!(monoid_pow(&concat, &"ab".to_string(), 3), "ababab");
        let addition = FnGroup::new(|| 0_i64, |a: &i64, b: &i64| a + b, |a: &i64| -a);
        assert_eq!(group_pow(&addition, &3, -5), -15);
        assert_eq!(group_pow(&addition, &3, 0), 0);
    }

    #[test]
    fn large_prime_boundary_arithmetic_stays_normalized() {
        const P: u32 = 2147483647;
        let p = P as u64;
        let a = Matrix2::<P>::new(p - 1, p - 2, p - 3, p - 4);
        let b = Matrix2::<P>::new(p - 4, p - 3, p - 2, p - 1);
        assert_eq!(rows2(a * b), product(&rows2(a), &rows2(b), 2, P as i128));
        assert_eq!((a + b).to_rows(), [[P - 5; 2]; 2]);
        assert_eq!((-a).to_rows(), [[1, 2], [3, 4]]);
        assert_eq!((a - b).to_rows(), [[3, 1], [P - 1, P - 3]]);
        let reduced = (u64::MAX % p) as u32;
        let mut large = Matrix2::<P>::new(u64::MAX, p, p + 1, 0);
        assert_eq!(large.to_rows(), [[reduced, 0], [1, 0]]);
        large.set(1, 1, u64::MAX);
        assert_eq!(large.get(1, 1), reduced);
        let scaled = a.scale(u64::MAX);
        for (actual, &value) in scaled.entries().iter().zip(a.entries()) {
            assert_eq!(*actual as u128, value as u128 * reduced as u128 % P as u128);
        }
        let dynamic = Matrix::<P>::from_flat(1, 3, vec![u64::MAX, p, p + 1]);
        assert_eq!(dynamic.to_rows(), vec![vec![reduced, 0, 1]]);
    }

    #[test]
    fn prime_and_modular_power_match_trial_division_and_repetition() {
        for n in 0..=10000_u32 {
            let expected = n >= 2 && !(2..).take_while(|&d| d * d <= n).any(|d| n % d == 0);
            assert_eq!(is_prime(n), expected, "n={n}");
        }
        for n in [561, 1105, 1729, 3215031751, u32::MAX] {
            assert!(!is_prime(n));
        }
        for n in [998244353, 2147483647, 4294967291] {
            assert!(is_prime(n));
        }
        for modulus in [1, 2, 7, 998244353, 2147483647, 4294967291] {
            for value in [0, 1, modulus - 1, modulus, u32::MAX] {
                let mut expected = 1 % modulus;
                for exponent in 0..=20 {
                    assert_eq!(mod_pow(value, exponent, modulus), expected);
                    expected = (expected as u64 * value as u64 % modulus as u64) as u32;
                }
            }
        }
    }
}
