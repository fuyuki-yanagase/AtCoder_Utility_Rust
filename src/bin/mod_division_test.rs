// mod_divisionの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::mod_division::*;

/// 動作確認用
fn example() {
    assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
    assert_eq!(mod_pow(-2, 3, 1_000_000_007), 999_999_999);

    // 3 の mod 11 における逆元は 4
    // 3 * 4 = 12 = 1 mod 11
    assert_eq!(mod_inv(3, 11), Some(4));

    // 10 / 3 mod 11
    // 3 の逆元は 4 なので、10 * 4 = 40 = 7 mod 11
    assert_eq!(mod_div(10, 3, 11), Some(7));

    // 2 と 4 は互いに素ではないので、2 の mod 4 逆元は存在しない
    assert_eq!(mod_div(10, 2, 4), None);

    // 素数 mod の場合
    assert_eq!(mod_div_prime(10, 2, 1_000_000_007), Some(5));
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
