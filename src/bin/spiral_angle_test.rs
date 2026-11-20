// spiral_angleの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
#[cfg(test)]
use std::f64::consts::{PI, TAU};
use utility_rust::spiral_angle::*;

fn example() {
    // ========================================
    // 1. 偏角のソート
    // ========================================
    let mut angles = [
        SpiralAngle::from_xy(0, -1),
        SpiralAngle::from_xy(-1, 0),
        SpiralAngle::from_xy(0, 1),
        SpiralAngle::from_xy(1, 0),
        SpiralAngle::from_xy(1, 1),
    ];

    angles.sort();

    // 0°, 45°, 90°, 180°, 270° の順になる。
    let result: Vec<_> = angles.iter().map(|a| a.vector()).collect();

    assert_eq!(result, vec![(1, 0), (1, 1), (0, 1), (-1, 0), (0, -1)]);

    // ========================================
    // 2. 同じ方向の判定
    // ========================================
    let a = SpiralAngle::new(1, 2, 0);
    let b = SpiralAngle::new(2, 4, 0);
    let c = SpiralAngle::new(-1, -2, 0);

    // 長さが異なっても同じ方向なら等しい。
    assert_eq!(a, b);

    // 反対方向は異なる角度。
    assert_ne!(a, c);

    // ========================================
    // 3. 回転数の比較
    // ========================================
    let a = SpiralAngle::new(1, 0, 0);
    let b = SpiralAngle::new(1, 0, 1);
    let c = SpiralAngle::new(1, 0, -1);

    assert!(c < a);
    assert!(a < b);
    assert!(c < b);

    // 回転数を無視すれば同じ方向。
    assert!(a.same_direction(&b));

    // ========================================
    // 4. 角度の取得
    // ========================================
    let a = SpiralAngle::new(1, 1, 0);
    let b = SpiralAngle::new(1, 1, 1);

    assert!((a.degrees() - 45.0).abs() < 1e-9);
    assert!((b.degrees() - 405.0).abs() < 1e-9);

    // ========================================
    // 5. 回転数の変更
    // ========================================
    let a = SpiralAngle::from_xy(1, 0);

    let b = a.with_turn(2);
    assert_eq!(b.turn(), 2);

    let c = b.checked_add_turns(-3).unwrap();
    assert_eq!(c.turn(), -1);

    // オーバーフローした場合はNoneを返す。
    let d = a.with_turn(i64::MAX);
    assert!(d.checked_add_turns(1).is_none());

    // ========================================
    // 6. 外積
    // ========================================
    let a = SpiralAngle::from_xy(1, 0);
    let b = SpiralAngle::from_xy(0, 1);
    let c = SpiralAngle::from_xy(0, -1);

    assert!(a.cross(&b) > 0);
    assert!(a.cross(&c) < 0);
    assert_eq!(a.cross(&a), 0);

    // ========================================
    // 7. ゼロベクトルの禁止
    // ========================================

    // 現在のパニックフックを保存
    let original_hook = std::panic::take_hook();

    // パニック時のメッセージ出力を抑制
    std::panic::set_hook(Box::new(|_| {}));

    // ゼロベクトルでパニックが発生することを確認
    let result = std::panic::catch_unwind(|| {
        SpiralAngle::new(0, 0, 0);
    });

    // 元のパニックフックに戻す
    std::panic::set_hook(original_hook);

    assert!(result.is_err());

    // ========================================
    // 8. 境界条件の比較
    // ========================================

    // 正のX軸の上下では、上側の方が角度が小さい
    let a = SpiralAngle::from_xy(100, 1);
    let b = SpiralAngle::from_xy(100, -1);
    assert!(a < b);

    // 負のX軸の上下では、上側の方が角度が小さい
    let a = SpiralAngle::from_xy(-100, 1);
    let b = SpiralAngle::from_xy(-100, -1);
    assert!(a < b);

    // 180度反対の方向は等しくならない
    let a = SpiralAngle::from_xy(1, 1);
    let b = SpiralAngle::from_xy(-1, -1);
    assert_ne!(a, b);

    // ベクトルの長さが異なっても同じ偏角
    let a = SpiralAngle::from_xy(-3, 6);
    let b = SpiralAngle::from_xy(-1, 2);
    assert_eq!(a, b);

    // turnが異なる場合は、偏角より回転数を優先
    let a = SpiralAngle::new(100, -1, 0);
    let b = SpiralAngle::new(100, 1, 1);
    assert!(a < b);

    // 大きな座標でもi128によって安全に外積を計算
    let a = SpiralAngle::from_xy(i64::MAX, i64::MAX);
    let b = SpiralAngle::from_xy(i64::MIN, i64::MAX);
    assert!(a < b);

    // ========================================
    // 9. 回転
    // ========================================

    // 90°回転
    let a = SpiralAngle::from_xy(1, 0).rotate90();
    assert_eq!(a.vector(), (0, 1));
    assert_eq!(a.turn(), 0);

    // 270° → 360° で回転数が繰り上がる
    let a = SpiralAngle::from_xy(0, -1).rotate90();
    assert_eq!(a.vector(), (1, 0));
    assert_eq!(a.turn(), 1);
    assert!((a.degrees() - 360.0).abs() < 1e-9);

    // 4回90°回転すると1周
    let a = SpiralAngle::from_xy(3, 5);
    let b = a.rotate90().rotate90().rotate90().rotate90();
    assert_eq!(b, a.with_turn(1));

    // 時計回りは0°をまたぐと回転数が減る
    let a = SpiralAngle::from_xy(1, 0).rotate90_cw();
    assert_eq!(a.vector(), (0, -1));
    assert_eq!(a.turn(), -1);

    // rotate90とrotate90_cwは逆操作
    let a = SpiralAngle::new(2, -7, 3);
    assert_eq!(a.rotate90().rotate90_cw(), a);
    assert_eq!(a.rotate90_cw().rotate90(), a);

    // 180°回転
    let a = SpiralAngle::from_xy(1, 1);
    let b = a.rotate180();
    assert_eq!(b.vector(), (-1, -1));
    assert_eq!(b.turn(), 0);
    assert_eq!(b.rotate180(), a.with_turn(1));

    // 180°回転は90°回転2回と等しい
    let a = SpiralAngle::new(-4, 9, -2);
    assert_eq!(a.rotate180(), a.rotate90().rotate90());

    // i64::MINの符号反転はNone
    assert!(SpiralAngle::from_xy(1, i64::MIN).checked_rotate90().is_none());
    assert!(SpiralAngle::from_xy(i64::MIN, 1).checked_rotate90_cw().is_none());
    assert!(SpiralAngle::from_xy(i64::MIN, 0).checked_rotate180().is_none());

    // 回転数がオーバーフローする場合もNone
    let a = SpiralAngle::new(0, -1, i64::MAX);
    assert!(a.checked_rotate90().is_none());

    // ========================================
    // 10. 角度差180°未満の判定
    // ========================================
    let x = SpiralAngle::from_xy(1, 0);

    // 同じ角度
    assert!(x.within_half_turn(&x));
    assert!(x.within_half_turn(&SpiralAngle::from_xy(5, 0)));

    // 90°差
    assert!(x.within_half_turn(&SpiralAngle::from_xy(0, 1)));

    // ちょうど180°差はfalse
    assert!(!x.within_half_turn(&SpiralAngle::from_xy(-1, 0)));
    assert!(!x.within_half_turn(&x.rotate180()));

    // 270°差はfalse
    assert!(!x.within_half_turn(&SpiralAngle::from_xy(0, -1)));

    // 0°をまたぐ: 315°(turn 0) と 45°(turn 1) は90°差
    let a = SpiralAngle::new(1, -1, 0);
    let b = SpiralAngle::new(1, 1, 1);
    assert!(a.within_half_turn(&b));
    assert!(b.within_half_turn(&a));

    // 回転数だけ異なる同じ方向は360°差
    assert!(!x.within_half_turn(&x.with_turn(1)));

    // 引数の順序に依存しない
    let a = SpiralAngle::new(-1, 3, 2);
    let b = SpiralAngle::new(2, -5, 1);
    assert_eq!(a.within_half_turn(&b), b.within_half_turn(&a));

    // 極端な回転数でもオーバーフローしない
    let a = SpiralAngle::new(1, 0, i64::MIN);
    let b = SpiralAngle::new(1, 0, i64::MAX);
    assert!(!a.within_half_turn(&b));

    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_full_turn_is_not_reset_to_zero() {
        // TAU加算時に端数が丸め落ちても、ほぼ360°を0°へ飛ばさない。
        let angle = SpiralAngle::from_xy(i64::MAX, -1);
        assert!(angle.radians() > PI);
        assert!((angle.radians() - TAU).abs() < 1e-12);
        assert!(angle.with_turn(-1).radians().abs() < 1e-12);
    }
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
