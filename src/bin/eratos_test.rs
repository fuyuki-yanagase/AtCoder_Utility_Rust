// eratosの使用例・テスト。
use utility_rust::eratos::eratos;
#[cfg(test)]
use utility_rust::eratos::eratosusize;

fn example() {
    let primes = eratos(20);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    println!("20以下の素数: {primes:?}");
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn boundaries_and_segment_transition() {
    assert!(eratos(-1).is_empty());
    for n in [0, 1, 2, 3, 4, 49, 1000, 524_291] {
        // 単純な全領域の篩を使い、区間篩のブロック境界も照合する。
        let mut prime = vec![true; n + 1];
        prime[0] = false;
        if n > 0 {
            prime[1] = false;
        }
        for p in 2..=n {
            if prime[p] {
                for multiple in (p * 2..=n).step_by(p) {
                    prime[multiple] = false;
                }
            }
        }
        let expected: Vec<_> = (0..=n).filter(|&i| prime[i]).collect();
        assert_eq!(eratosusize(n), expected);
        assert_eq!(eratos(n as i64), expected.iter().map(|&x| x as i64).collect::<Vec<_>>());
    }
}
