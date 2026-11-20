// モノイド・群・アーベル群の使用例・テスト。
use utility_rust::monoid_group_abelian::*;

fn example() {
    let sum = FnMonoid::new(|| 0_i64, |a: &i64, b: &i64| a + b);
    assert_eq!(sum.combine(&3, &4), 7);
    println!("3と4の結合: {}", sum.combine(&3, &4));

    // mod MODの加法群
    const MOD: i64 = 998244353;
    let group = FnGroup::new(
        || 0_i64,
        // 加法
        |a: &i64, b: &i64| (*a + *b).rem_euclid(MOD),
        // 加法の逆元
        |a: &i64| (-*a).rem_euclid(MOD),
    );

    // 単位元
    assert_eq!(group.identity(), 0);

    // 結合
    assert_eq!(group.combine(&3, &5), 8);

    // 逆元
    assert_eq!(group.inverse(&3), MOD - 3);

    // a + (-a) = 0
    let result = group.combine(&3, &group.inverse(&3));
    assert_eq!(result, group.identity());
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn identity_order_and_captured_configuration() {
    let concat = FnMonoid::new(String::new, |a: &String, b: &String| format!("{a}{b}"));
    let a = String::from("left");
    let b = String::from("right");
    assert_eq!(concat.combine(&a, &b), "leftright");
    assert_eq!(concat.combine(&concat.identity(), &a), a);
    assert_eq!(concat.combine(&a, &concat.identity()), a);
    assert_eq!(
        concat.combine(&concat.combine(&a, &b), &a),
        concat.combine(&a, &concat.combine(&b, &a))
    );
    let modulus = 7;
    let modular_sum = FnMonoid::new(|| 0, |a: &i32, b: &i32| (a + b) % modulus);
    assert_eq!(modular_sum.combine(&5, &6), 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 有限個の代表値で、結合律と左右の単位元を確認する。
    /// 型を実装しただけでは、これらの数学的性質は保証されない。
    fn assert_monoid_laws<M>(monoid: &M, values: &[M::S])
    where
        M: Monoid,
        M::S: PartialEq + std::fmt::Debug,
    {
        let identity = monoid.identity();
        for a in values {
            assert_eq!(&monoid.combine(&identity, a), a);
            assert_eq!(&monoid.combine(a, &identity), a);
            for b in values {
                for c in values {
                    assert_eq!(
                        monoid.combine(&monoid.combine(a, b), c),
                        monoid.combine(a, &monoid.combine(b, c))
                    );
                }
            }
        }
    }

    fn assert_group_laws<G>(group: &G, values: &[G::S])
    where
        G: Group,
        G::S: PartialEq + std::fmt::Debug,
    {
        assert_monoid_laws(group, values);
        let identity = group.identity();
        for a in values {
            let inverse = group.inverse(a);
            assert_eq!(group.combine(a, &inverse), identity);
            assert_eq!(group.combine(&inverse, a), identity);
            assert_eq!(&group.inverse(&inverse), a);
            for b in values {
                // 非可換群では、積の逆元を取ると演算順序が逆になる。
                assert_eq!(group.inverse(&group.combine(a, b)), group.combine(&group.inverse(b), &inverse));
            }
        }
    }

    #[test]
    fn test_additive_group() {
        const MOD: i64 = 998244353;

        // mod MODの加法群
        let group = FnGroup::new(
            || 0_i64,
            |a: &i64, b: &i64| (a + b).rem_euclid(MOD),
            |a: &i64| (-a).rem_euclid(MOD),
        );

        let a = 123456789_i64;
        let b = 987654321_i64;
        let e = group.identity();

        // 単位元
        assert_eq!(group.combine(&a, &e), a);
        assert_eq!(group.combine(&e, &a), a);

        // 左逆元・右逆元
        let inv = group.inverse(&a);
        assert_eq!(group.combine(&a, &inv), e);
        assert_eq!(group.combine(&inv, &a), e);

        // 結合律
        let c = 123456_i64;
        let ab = group.combine(&a, &b);
        let bc = group.combine(&b, &c);

        assert_eq!(group.combine(&ab, &c), group.combine(&a, &bc));
    }

    #[test]
    fn non_commutative_permutation_group_laws() {
        let group = FnGroup::new(
            || [0_usize, 1, 2],
            // 左の置換を、右の置換の後に適用する。
            |left: &[usize; 3], right: &[usize; 3]| [left[right[0]], left[right[1]], left[right[2]]],
            |value: &[usize; 3]| {
                let mut inverse = [0; 3];
                for i in 0..3 {
                    inverse[value[i]] = i;
                }
                inverse
            },
        );
        let permutations = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

        // 3要素の置換群の全要素で検証する。交換律は成立しない。
        assert_group_laws(&group, &permutations);
        let a = [1, 0, 2];
        let b = [0, 2, 1];
        assert_eq!(group.combine(&a, &b), [1, 2, 0]);
        assert_eq!(group.combine(&b, &a), [2, 0, 1]);
        assert_ne!(group.combine(&a, &b), group.combine(&b, &a));
    }

    #[test]
    fn abelian_group_laws_with_captured_modulus() {
        fn assert_abelian_laws<G>(group: &G, values: &[G::S])
        where
            G: AbelianGroup,
            G::S: PartialEq + std::fmt::Debug,
        {
            assert_group_laws(group, values);
            for a in values {
                for b in values {
                    assert_eq!(group.combine(a, b), group.combine(b, a));
                }
            }
        }

        let modulus = 7_i64;
        let group = FnAbelianGroup::new(
            || 0_i64,
            |a: &i64, b: &i64| (a + b).rem_euclid(modulus),
            |a: &i64| (-a).rem_euclid(modulus),
        );
        let values: Vec<i64> = (0..modulus).collect();
        assert_abelian_laws(&group, &values);
        assert_eq!(group.combine(&5, &6), 4);
    }
}
