// prime_factorizeの使用例・テスト。
#[cfg(test)]
use utility_rust::prime_factorize::is_prime;
use utility_rust::prime_factorize::prime_factorize;

fn example() {
    let factors = prime_factorize(84);
    assert_eq!(factors, vec![2, 2, 3, 7]);
    println!("84の素因数: {factors:?}");
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn trial_division_and_large_composites() {
    for n in -2..=2000 {
        let expected_prime = n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0);
        assert_eq!(is_prime(n), expected_prime, "n={n}");
        let mut x = n;
        let mut expected = Vec::new();
        let mut d = 2;
        while d * d <= x {
            while x % d == 0 {
                expected.push(d);
                x /= d;
            }
            d += 1;
        }
        if x > 1 {
            expected.push(x);
        }
        assert_eq!(prime_factorize(n), expected, "n={n}");
    }
    for n in [3_215_031_751, 1_000_000_007 * 1_000_000_009, i64::MAX] {
        let factors = prime_factorize(n);
        assert!(!is_prime(n));
        assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        assert!(factors.iter().all(|&p| is_prime(p)));
        assert_eq!(factors.iter().map(|&p| p as i128).product::<i128>(), n as i128);
    }
    assert!(is_prime(2_305_843_009_213_693_951));
}
