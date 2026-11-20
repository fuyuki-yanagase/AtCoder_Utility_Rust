// gcd_lcmの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::gcd_lcm::*;

/// 動作確認用
fn example() {
    assert_eq!(gcd_i32(12, 18), 6);
    assert_eq!(gcd_i32(17, 13), 1);
    assert_eq!(gcd_i32(0, 10), 0);
    assert_eq!(gcd_i32(-12, 18), 0);

    assert_eq!(lcm_i32(12, 18), 36);
    assert_eq!(lcm_i32(4, 6), 12);
    assert_eq!(lcm_i32(0, 6), 0);
    assert_eq!(lcm_i32(-4, 6), 0);

    assert_eq!(gcd_i64(1_000_000_000_000, 500_000_000_000), 500_000_000_000);
    assert_eq!(lcm_i64(1_000_000_000_000, 500_000_000_000), 1_000_000_000_000);

    assert_eq!(gcd_i128(36_i128, 48_i128), 12_i128);
    assert_eq!(lcm_i128(36_i128, 48_i128), 144_i128);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
