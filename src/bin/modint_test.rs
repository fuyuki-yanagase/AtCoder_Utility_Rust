use utility_rust::modint::{Combinatorics, ModInt998};

fn example() {
    type Mint = ModInt998;
    let value = (Mint::new(-2_i64) + 5) * 7 / 3;
    assert_eq!(value.val(), 7);
    let combinatorics = Combinatorics::<998244353>::new(5);
    assert_eq!(combinatorics.permutation(5, 2), Mint::new(20));
    assert_eq!(combinatorics.combination(5, 2), Mint::new(10));
    println!("ModIntの四則演算・順列・組合せ: 成功");
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
    use std::collections::HashSet;
    use std::panic::catch_unwind;
    use utility_rust::lazy_segment_tree_monoid::{Affine, AffineComposite};
    use utility_rust::modint::{ModInt, ModInt107};
    use utility_rust::monoid_group_abelian::Monoid;

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    #[test]
    fn normalizes_every_integer_conversion() {
        type Mint = ModInt<97>;
        macro_rules! signed {
            ($($kind:ty),+) => { $(
                for value in [-123_i128, -97, -1, 0, 1, 97, 123] {
                    let input = value as $kind;
                    assert_eq!(Mint::from(input).val(), (input as i128).rem_euclid(97) as u32);
                    assert_eq!(Mint::new(input).val(), (input as i128).rem_euclid(97) as u32);
                }
            )+ };
        }
        macro_rules! unsigned {
            ($($kind:ty),+) => { $(
                for value in [0_u128, 1, 96, 97, 123] {
                    let input = value as $kind;
                    assert_eq!(Mint::from(input).val(), (input as u128 % 97) as u32);
                }
            )+ };
        }
        signed!(i8, i16, i32, i64, isize, i128);
        unsigned!(u8, u16, u32, u64, usize, u128);
        for value in [-(1_i128 << 100), -1_000_000_000_001, 1_i128 << 100] {
            assert_eq!(Mint::new(value).val(), value.rem_euclid(97) as u32);
        }
        for value in [u64::MAX as u128, (1_u128 << 100) + 17, u128::MAX] {
            assert_eq!(Mint::new(value).val(), (value % 97) as u32);
        }
        let original = Mint::new(123);
        assert_eq!(Mint::from(&original), original);
        assert_eq!(ModInt998::modulus(), 998244353);
        assert_eq!(ModInt107::modulus(), 1000000007);
    }

    #[test]
    fn combinatorics_matches_integer_counts_up_to_the_last_residue() {
        const PRIME: u32 = 13;
        let table = Combinatorics::<PRIME>::new(PRIME as usize - 1);
        let mut row = vec![1_u64];
        let mut factorial = 1_u64;
        for n in 0..PRIME as usize {
            if n > 0 {
                factorial *= n as u64;
                // パスカルの三角形を整数で作り、逆元を使わずに組合せを照合する。
                row.push(0);
                for r in (1..=n).rev() {
                    row[r] += row[r - 1];
                }
            }
            assert_eq!(table.factorial(n).val() as u64, factorial % u64::from(PRIME));
            let mut permutation = 1_u64;
            for (r, &combination) in row.iter().enumerate() {
                if r > 0 {
                    permutation *= (n - r + 1) as u64;
                }
                assert_eq!(table.permutation(n, r).val() as u64, permutation % u64::from(PRIME));
                assert_eq!(table.combination(n, r).val() as u64, combination % u64::from(PRIME));
            }
            assert_eq!(table.permutation(n, n + 1), ModInt::ZERO);
            assert_eq!(table.combination(n, n + 1), ModInt::ZERO);
        }
    }

    #[test]
    fn combinatorics_checks_precomputation_bounds() {
        let table = Combinatorics::<2>::new(0);
        assert_eq!(table.factorial(0), ModInt::ONE);
        assert_eq!(table.permutation(0, 0), ModInt::ONE);
        assert_eq!(table.combination(0, 0), ModInt::ONE);
        assert!(catch_unwind(|| Combinatorics::<2>::new(2)).is_err());
        assert!(catch_unwind(|| table.factorial(1)).is_err());
        assert!(catch_unwind(|| table.permutation(1, 0)).is_err());
        assert!(catch_unwind(|| table.combination(1, 0)).is_err());
    }

    // 剰余の四則は広い整数から直接求める。逆元は積が1になることとgcdで独立に判定する。
    fn check_arithmetic<const MOD: u32>() {
        let mut state = 20261030_u64;
        let mut random = || {
            state ^= state << 7;
            state ^= state >> 9;
            (state % MOD as u64) as u32
        };
        let mut pairs: Vec<_> = [0, 1, MOD - 2, MOD - 1].into_iter()
            .flat_map(|a| [0, 1, MOD - 2, MOD - 1].map(|b| (a, b))).collect();
        pairs.extend((0..512).map(|_| (random(), random())));
        for (a, b) in pairs {
            let x = ModInt::<MOD>::new(a);
            let y = ModInt::<MOD>::new(b);
            assert_eq!((x + y).val(), ((a as u128 + b as u128) % MOD as u128) as u32);
            assert_eq!((x - y).val(), (a as i128 - b as i128).rem_euclid(MOD as i128) as u32);
            assert_eq!((x * y).val(), (a as u128 * b as u128 % MOD as u128) as u32);
            assert_eq!((-x).val(), (-(a as i128)).rem_euclid(MOD as i128) as u32);
            let inverse = y.checked_inv();
            assert_eq!(inverse.is_some(), gcd(b, MOD) == 1);
            if let Some(inverse) = inverse {
                assert_eq!(b as u128 * inverse.val() as u128 % MOD as u128, 1);
                assert_eq!(y.inv(), inverse);
                let quotient = x / y;
                assert_eq!(quotient.val() as u128 * b as u128 % MOD as u128, a as u128);
            }
        }
    }

    #[test]
    fn arithmetic_matches_integer_oracle_across_moduli() {
        check_arithmetic::<2>();
        check_arithmetic::<6>();
        check_arithmetic::<97>();
        check_arithmetic::<998244353>();
        check_arithmetic::<1000000007>();
        check_arithmetic::<2147483646>();
        check_arithmetic::<2147483647>();
    }

    fn exhaustive_inverse<const MOD: u32>() {
        for denominator in 0..MOD {
            let expected = (0..MOD).find(|&candidate| denominator as u128 * candidate as u128 % MOD as u128 == 1);
            let value = ModInt::<MOD>::new(denominator);
            assert_eq!(value.checked_inv().map(|v| v.val()), expected);
            if let Some(inverse) = expected {
                for numerator in 0..MOD {
                    assert_eq!((ModInt::<MOD>::new(numerator) / value).val(), (numerator as u128 * inverse as u128 % MOD as u128) as u32);
                }
            }
        }
    }

    #[test]
    fn small_moduli_inverse_and_division_exhaustive() {
        exhaustive_inverse::<2>();
        exhaustive_inverse::<7>();
        exhaustive_inverse::<12>();
        exhaustive_inverse::<25>();
    }

    #[test]
    fn powers_match_naive_and_const_extremes() {
        type Mint = ModInt<2147483647>;
        const RAW: Mint = Mint::raw(2147483646);
        const ODD: Mint = RAW.pow(u64::MAX);
        const EVEN: Mint = RAW.pow(u64::MAX - 1);
        const INVERSE: Option<Mint> = Mint::raw(2).checked_inv();
        assert_eq!(ODD.val(), 2147483646);
        assert_eq!(EVEN, Mint::ONE);
        assert_eq!(INVERSE.unwrap().val(), 1073741824);
        assert_eq!(Mint::ZERO.pow(0), Mint::ONE);
        for value in [0_u32, 1, 2, 123456789, 2147483646] {
            let mut expected = 1_u128;
            for exponent in 0..64 {
                assert_eq!(Mint::new(value).pow(exponent).val() as u128, expected);
                expected = expected * value as u128 % 2147483647;
            }
        }
        assert_eq!(Mint::new(2).pow(63).val(), ((1_u128 << 63) % 2147483647) as u32);
    }

    #[test]
    fn reference_assignment_and_conversion_operators() {
        type Mint = ModInt<97>;
        let a = Mint::new(83);
        let b = Mint::new(19);
        assert_eq!(&a + &b, a + b);
        assert_eq!(&a - &b, a - b);
        assert_eq!(&a * &b, a * b);
        assert_eq!(&a / &b, a / b);
        assert_eq!(-&a, -a);
        let mut value = a;
        value += &b;
        assert_eq!(value, a + b);
        value -= 19_i64;
        assert_eq!(value, a);
        value *= 19_u128;
        assert_eq!(value, a * b);
        value /= &b;
        assert_eq!(value, a);
        assert_eq!(&a + -19_i128, a - b);
        assert_eq!(&a / 19_u8, a / b);
    }

    #[test]
    fn iterators_and_value_traits() {
        type Mint = ModInt<97>;
        let raw = [96_u128, 2, 55, 12];
        let values = raw.map(Mint::new);
        let sum = (raw.iter().sum::<u128>() % 97) as u32;
        let product = (raw.iter().product::<u128>() % 97) as u32;
        assert_eq!(values.iter().sum::<Mint>().val(), sum);
        assert_eq!(values.into_iter().sum::<Mint>().val(), sum);
        assert_eq!(values.iter().product::<Mint>().val(), product);
        assert_eq!(values.into_iter().product::<Mint>().val(), product);
        let empty: [Mint; 0] = [];
        assert_eq!(empty.into_iter().sum::<Mint>(), Mint::ZERO);
        assert_eq!(empty.iter().sum::<Mint>(), Mint::ZERO);
        assert_eq!(empty.into_iter().product::<Mint>(), Mint::ONE);
        assert_eq!(empty.iter().product::<Mint>(), Mint::ONE);
        assert_eq!(Mint::default(), Mint::ZERO);
        assert_eq!(format!("{}", Mint::new(-1)), "96");
        assert_eq!(format!("{:?}", Mint::new(-1)), "96");
        assert_eq!([Mint::new(-1), Mint::new(96), Mint::new(193), Mint::ZERO].into_iter().collect::<HashSet<_>>().len(), 2);
    }

    #[test]
    fn decimal_bytes_and_strings_match_integer_oracle() {
        type Mint = ModInt<998244353>;
        for text in ["0", "-0", "+0", "00097", "-12345678901234567890", "+12345678901234567890"] {
            let expected = text.parse::<i128>().unwrap().rem_euclid(998244353) as u32;
            assert_eq!(Mint::from_bytes(text).val(), expected);
            assert_eq!(Mint::try_from_bytes(text.as_bytes()).unwrap().val(), expected);
            assert_eq!(text.parse::<Mint>().unwrap().val(), expected);
        }
        // u128より長い10進数は、9桁ずつのブロックを広い整数で集約して照合する。
        let digits = "123456789".repeat(30);
        let expected = digits.as_bytes().chunks(9).fold(0_u128, |value, chunk| {
            (value * 1_000_000_000 + std::str::from_utf8(chunk).unwrap().parse::<u128>().unwrap()) % 998244353
        }) as u32;
        assert_eq!(Mint::from_bytes(digits.clone()).val(), expected);
        assert_eq!(Mint::from_bytes(digits.as_bytes().to_vec()).val(), expected);
        assert_eq!(Mint::from_bytes(format!("-{digits}")).val(), (-(expected as i128)).rem_euclid(998244353) as u32);
        // 法が10より小さい場合の桁処理も独立な整数値と比較する。
        assert_eq!(ModInt::<2>::from_bytes("123456789").val(), 1);
        assert_eq!(ModInt::<6>::from_bytes("-123456789").val(), 3);
    }

    #[test]
    fn rejects_invalid_decimal_and_noninvertible_values() {
        type Mint = ModInt<12>;
        for text in ["", "+", "-", " 1", "1 ", "1_0", "1.0", "--1", "１２"] {
            assert!(Mint::try_from_bytes(text).is_err(), "{text}");
            assert!(text.parse::<Mint>().is_err(), "{text}");
        }
        assert!(Mint::try_from_bytes([0xff_u8]).is_err());
        assert!(catch_unwind(|| Mint::from_bytes("x")).is_err());
        for value in [0, 2, 3, 4, 6, 8, 9, 10] {
            let value = Mint::new(value);
            assert!(value.checked_inv().is_none());
            assert!(catch_unwind(|| value.inv()).is_err());
            assert!(catch_unwind(|| Mint::ONE / value).is_err());
        }
    }

    #[test]
    fn affine_composition_matches_application_order() {
        const MOD: u128 = 2147483647;
        let monoid = AffineComposite::<2147483647>;
        let left = Affine::new(2, 3);
        let right = Affine::new(5, 7);
        assert_eq!(monoid.combine(&left, &right).eval(11).val(), 132);
        assert_eq!(monoid.combine(&right, &left).eval(11).val(), 127);
        let maps = [(2_u128, 3_u128), (MOD - 1, MOD - 2), (7, 0), (1, 19)];
        let combined = maps.iter().fold(monoid.identity(), |acc, &(a, b)| {
            monoid.combine(&acc, &Affine::new(a, b))
        });
        for x in [0_u128, 1, 13, MOD - 1] {
            let expected = maps.iter().fold(x, |value, &(a, b)| (a * value + b) % MOD);
            assert_eq!(combined.eval(x).val() as u128, expected);
            assert_eq!(monoid.combine(&combined, &monoid.identity()).eval(x), combined.eval(x));
        }
    }
}
