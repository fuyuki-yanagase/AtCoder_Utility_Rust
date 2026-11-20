// vec2_angleの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
#[cfg(test)]
use std::cmp::Ordering;
#[cfg(test)]
use std::f64::consts::{PI, TAU};
use utility_rust::vec2_angle::*;

fn example() {
    // ========================================
    // 1. ベクトルの基本演算・内積・外積
    // ========================================
    let a = Vec2::new(3, 4);
    let b = Vec2::new(1, 2);

    assert_eq!(a + b, Vec2::new(4, 6));
    assert_eq!(a - b, Vec2::new(2, 2));
    assert_eq!(-a, Vec2::new(-3, -4));
    assert_eq!(a * 2, Vec2::new(6, 8));

    assert_eq!(a.dot(b), 11);
    assert_eq!(a.cross(b), 2);
    assert_eq!(a.norm_sq(), 25);
    assert!((a.norm() - 5.0).abs() < 1e-9);

    let mut v = a;
    v += b;
    v -= b;
    v *= 2;

    assert_eq!(v, a * 2);
    assert!(Vec2::default().is_zero());

    // ========================================
    // 2. 距離・方向・回転
    // ========================================
    let o = Vec2::new(0, 0);

    assert_eq!(o.distance_sq(a), 25);
    assert!((o.distance(a) - 5.0).abs() < 1e-9);
    assert_eq!(o.manhattan_distance(a), 7);
    assert_eq!(o.chebyshev_distance(a), 4);

    assert!(b.is_parallel(Vec2::new(2, 4)));
    assert!(b.is_perpendicular(Vec2::new(-2, 1)));
    assert!(b.same_direction(Vec2::new(2, 4)));
    assert!(b.opposite_direction(Vec2::new(-2, -4)));
    assert!(!o.same_direction(b));

    assert_eq!(a.rotate90(), Vec2::new(-4, 3));
    assert_eq!(a.rotate90_cw(), Vec2::new(4, -3));
    assert_eq!(a.rotate180(), Vec2::new(-3, -4));
    assert_eq!(a.rotate90().rotate90_cw(), a);

    // ========================================
    // 3. 向き・面積・線分上の点
    // ========================================
    let a = Vec2::new(0, 0);
    let b = Vec2::new(4, 0);
    let c = Vec2::new(0, 3);

    assert_eq!(Vec2::orient(a, b, c), 12);
    assert_eq!(Vec2::orient(a, c, b), -12);
    assert_eq!(Vec2::triangle_area2(a, b, c), 12);
    assert!((Vec2::triangle_area(a, b, c) - 6.0).abs() < 1e-9);
    assert!(Vec2::is_collinear(a, b, Vec2::new(2, 0)));

    assert!(Vec2::new(2, 0).is_on_segment(a, b));
    assert!(a.is_on_segment(a, b));
    assert!(!Vec2::new(5, 0).is_on_segment(a, b));

    // 両端点が同じ線分。
    assert!(a.is_on_segment(a, a));
    assert!(!b.is_on_segment(a, a));

    // ========================================
    // 4. 線分の交差
    // ========================================

    // 線分の内部で交差。
    assert!(Vec2::segments_intersect(
        Vec2::new(0, 0),
        Vec2::new(4, 4),
        Vec2::new(0, 4),
        Vec2::new(4, 0),
    ));

    // 端点で接する。
    assert!(Vec2::segments_intersect(a, b, b, Vec2::new(4, 3),));

    // 同一直線上で重なる。
    assert!(Vec2::segments_intersect(a, b, Vec2::new(2, 0), Vec2::new(6, 0),));

    // 同一直線上だが離れている。
    assert!(!Vec2::segments_intersect(a, b, Vec2::new(5, 0), Vec2::new(6, 0),));

    // 点に退化した線分。
    assert!(Vec2::segments_intersect(a, b, Vec2::new(2, 0), Vec2::new(2, 0),));

    // ========================================
    // 5. 点と直線・線分の距離
    // ========================================
    let p = Vec2::new(2, 3);

    // 垂線の足は線分の内部。
    assert!((p.distance_sq_to_line(a, b) - 9.0).abs() < 1e-9);
    assert!((p.distance_sq_to_segment(a, b) - 9.0).abs() < 1e-9);

    // 垂線の足が線分外にあるので、端点との距離になる。
    let p = Vec2::new(-2, 3);
    assert!((p.distance_sq_to_segment(a, b) - 13.0).abs() < 1e-9);

    let p = Vec2::new(6, 3);
    assert!((p.distance_sq_to_segment(a, b) - 13.0).abs() < 1e-9);

    // 両端点が同じなら、その点との距離。
    let p = Vec2::new(2, 3);
    assert!((p.distance_sq_to_segment(a, a) - 13.0).abs() < 1e-9);

    // ========================================
    // 6. 偏角順のソート
    // ========================================
    let mut dirs = [Vec2::new(0, -1), Vec2::new(-1, 0), Vec2::new(1, 1), Vec2::new(0, 1), Vec2::new(1, 0)];

    // Vec2の通常のsort()は座標順なので、cmp_argを指定する。
    dirs.sort_by(|a, b| a.cmp_arg(*b));

    // 0°、45°、90°、180°、270°。
    let expected = [Vec2::new(1, 0), Vec2::new(1, 1), Vec2::new(0, 1), Vec2::new(-1, 0), Vec2::new(0, -1)];

    assert_eq!(dirs, expected);

    let mut angles = [
        SpiralAngle::from_xy(0, -1),
        SpiralAngle::from_xy(-1, 0),
        SpiralAngle::from_xy(1, 1),
        SpiralAngle::from_xy(0, 1),
        SpiralAngle::from_xy(1, 0),
    ];

    // SpiralAngleは通常のsort()で角度順。
    angles.sort();

    assert_eq!(angles.map(|angle| angle.vector()), expected);

    // ========================================
    // 7. 等価性・回転数・角度の取得
    // ========================================
    let a = SpiralAngle::new(Vec2::new(1, 2), 0);
    let b = SpiralAngle::new(Vec2::new(2, 4), 0);

    // ベクトルは異なるが、角度としては等しい。
    assert_ne!(a.vector(), b.vector());
    assert_eq!(a, b);

    // 反対方向は異なる角度。
    assert_ne!(a, SpiralAngle::from_xy(-1, -2));

    // 回転数を優先して比較する。
    assert!(a.with_turn(-1) < a);
    assert!(a < a.with_turn(1));

    // 回転数を無視すれば同じ方向。
    assert!(a.same_direction(&a.with_turn(1)));
    assert_eq!(a.checked_add_turns(2).unwrap().turn(), 2);

    let a = SpiralAngle::from_xy(1, 1);

    assert!((a.degrees() - 45.0).abs() < 1e-9);
    assert!((a.with_turn(1).degrees() - 405.0).abs() < 1e-9);

    // ========================================
    // 8. 回転と周回補正
    // ========================================

    // 270° → 360°。
    let a = SpiralAngle::from_xy(0, -1).rotate90();

    assert_eq!(a.vector(), Vec2::new(1, 0));
    assert_eq!(a.turn(), 1);
    assert!((a.degrees() - 360.0).abs() < 1e-9);

    // 0° → -90°。
    let a = SpiralAngle::from_xy(1, 0).rotate90_cw();

    assert_eq!(a.vector(), Vec2::new(0, -1));
    assert_eq!(a.turn(), -1);
    assert!((a.degrees() + 90.0).abs() < 1e-9);

    let a = SpiralAngle::new(Vec2::new(2, -7), 3);

    // 反時計回りと時計回りは逆操作。
    assert_eq!(a.rotate90().rotate90_cw(), a);
    assert_eq!(a.rotate90_cw().rotate90(), a);

    // 180°回転は90°回転2回と等しい。
    assert_eq!(a.rotate180(), a.rotate90().rotate90());

    // 180°回転2回で1周増える。
    assert_eq!(a.rotate180().rotate180(), a.with_turn(4));

    // ========================================
    // 9. 角度差180°未満の判定
    // ========================================
    let x = SpiralAngle::from_xy(1, 0);

    // 同じ角度。
    assert!(x.within_half_turn(&x));

    // 90°差。
    assert!(x.within_half_turn(&SpiralAngle::from_xy(0, 1)));

    // ちょうど180°差。
    assert!(!x.within_half_turn(&SpiralAngle::from_xy(-1, 0)));

    // 270°差。
    assert!(!x.within_half_turn(&SpiralAngle::from_xy(0, -1)));

    // 同じ方向でも回転数が1違うので360°差。
    assert!(!x.within_half_turn(&x.with_turn(1)));

    // 315°と405°は90°差。
    let a = SpiralAngle::new(Vec2::new(1, -1), 0);
    let b = SpiralAngle::new(Vec2::new(1, 1), 1);

    assert!(a.within_half_turn(&b));
    assert!(b.within_half_turn(&a));

    println!("すべてのテストに成功しました！");
}

#[cfg(test)]
mod tests {
    use super::*;

    // 比較実装の半平面・外積を再利用せず、atan2を独立した照合基準とする。
    // 同一直線上の整数点には同じ偏角があるため、浮動小数点の誤差だけ許容する。
    fn oracle_angle(v: Vec2, turn: i64) -> f64 {
        (v.y as f64).atan2(v.x as f64).rem_euclid(TAU) + turn as f64 * TAU
    }

    #[test]
    fn all_240_angles_match_independent_comparison_and_half_turn() {
        let mut angles = Vec::new();
        for turn in -1..=1 {
            for x in -4..=4 {
                for y in -4..=4 {
                    if x != 0 || y != 0 {
                        angles.push(SpiralAngle::new(Vec2::new(x, y), turn));
                    }
                }
            }
        }
        assert_eq!(angles.len(), 240);
        for a in &angles {
            for b in &angles {
                let difference = oracle_angle(a.vector(), a.turn()) - oracle_angle(b.vector(), b.turn());
                let order = if difference.abs() < 1e-12 { Ordering::Equal } else { difference.partial_cmp(&0.0).unwrap() };
                assert_eq!(a.cmp(b), order, "a={a:?}, b={b:?}");
                assert_eq!(a == b, order == Ordering::Equal);
                assert_eq!(a.partial_cmp(b), Some(order));
                assert_eq!(a.within_half_turn(b), difference.abs() < PI - 1e-12, "a={a:?}, b={b:?}");
                if a.turn() == b.turn() {
                    assert_eq!(a.vector().cmp_arg(b.vector()), order);
                }
            }
        }
    }

    #[test]
    fn direction_ignores_length_and_coordinate_order_is_distinct() {
        for x in -4..=4 {
            for y in -4..=4 {
                if x == 0 && y == 0 {
                    continue;
                }
                let v = Vec2::new(x, y);
                let a = SpiralAngle::new(v, -2);
                let b = SpiralAngle::new(v * 7, -2);
                assert_ne!(a.vector(), b.vector());
                assert_eq!(a, b);
                assert_eq!(v.cmp_arg(v * 7), Ordering::Equal);
                assert!(a.same_direction(&b.with_turn(8)));
                assert_ne!(a, b.with_turn(8));
                assert!(!a.same_direction(&SpiralAngle::new(-v, -2)));
            }
        }
        // Vec2::Ordは座標順。SpiralAngle::Ordは偏角順。
        let a = Vec2::new(0, 1);
        let b = Vec2::new(1, 0);
        assert!(a < b);
        assert_eq!(a.cmp_arg(b), Ordering::Greater);
        assert!(!Vec2::default().same_direction(a));
        assert!(!Vec2::default().opposite_direction(a));
    }

    #[test]
    fn rotations_carry_turns_in_both_directions() {
        for x in -4..=4 {
            for y in -4..=4 {
                if x == 0 && y == 0 {
                    continue;
                }
                for turn in -2..=2 {
                    let a = SpiralAngle::new(Vec2::new(x, y), turn);
                    for (rotated, delta) in [(a.rotate90(), PI / 2.0), (a.rotate90_cw(), -PI / 2.0), (a.rotate180(), PI)] {
                        let actual = oracle_angle(rotated.vector(), rotated.turn());
                        assert!((actual - oracle_angle(a.vector(), turn) - delta).abs() < 1e-12);
                    }
                    assert_eq!(a.rotate90().rotate90_cw(), a);
                    assert_eq!(a.rotate90_cw().rotate90(), a);
                    assert_eq!(a.rotate180(), a.rotate90().rotate90());
                    assert_eq!(a.rotate180().rotate180(), a.with_turn(turn + 1));
                    let mut ccw = a;
                    let mut cw = a;
                    for _ in 0..4 {
                        ccw = ccw.rotate90();
                        cw = cw.rotate90_cw();
                    }
                    assert_eq!(ccw, a.with_turn(turn + 1));
                    assert_eq!(cw, a.with_turn(turn - 1));
                }
            }
        }
    }

    #[test]
    fn checked_angles_preserve_overflow_contract() {
        assert!(SpiralAngle::from_xy(1, i64::MIN).checked_rotate90().is_none());
        assert!(SpiralAngle::from_xy(i64::MIN, 1).checked_rotate90_cw().is_none());
        assert!(SpiralAngle::from_xy(i64::MIN, 0).checked_rotate180().is_none());
        assert!(SpiralAngle::new(Vec2::new(0, -1), i64::MAX).checked_rotate90().is_none());
        assert!(SpiralAngle::new(Vec2::new(1, 0), i64::MIN).checked_rotate90_cw().is_none());
        assert!(SpiralAngle::new(Vec2::new(-1, 0), i64::MAX).checked_rotate180().is_none());
        let x = SpiralAngle::from_xy(1, 0);
        assert!(x.with_turn(i64::MAX).checked_add_turns(1).is_none());
        assert!(x.with_turn(i64::MIN).checked_add_turns(-1).is_none());
        assert_eq!(x.with_turn(i64::MAX).checked_add_turns(-1).unwrap().turn(), i64::MAX - 1);
        assert_eq!(x.with_turn(i64::MIN).checked_add_turns(1).unwrap().turn(), i64::MIN + 1);
        assert!(!x.with_turn(i64::MIN).within_half_turn(&x.with_turn(i64::MAX)));
        assert!(!x.with_turn(i64::MAX).within_half_turn(&x.with_turn(i64::MIN)));
    }

    #[test]
    fn wide_geometry_promotes_before_subtraction_and_products() {
        let large = 5_000_000_000_000_000_000_i64;
        let width = 2 * large as i128; // i64を越える差。二乗はi128内。
        let a = Vec2::new(-large, -1);
        let b = Vec2::new(large, -1);
        let c = Vec2::new(large, 1);
        assert_eq!(a.distance_sq(b), width * width);
        assert_eq!(a.manhattan_distance(c), width + 2);
        assert_eq!(a.chebyshev_distance(c), width);
        assert_eq!(Vec2::orient(a, b, c), width * 2);
        assert_eq!(Vec2::triangle_area2(a, c, b), width * 2);
        assert!(Vec2::new(0, -1).is_on_segment(a, b));
        assert!(!Vec2::new(0, 0).is_on_segment(a, b));
        assert!(Vec2::segments_intersect(a, b, Vec2::new(0, -2), Vec2::new(0, 2)));
        assert!(!Vec2::segments_intersect(a, b, Vec2::new(0, 0), Vec2::new(0, 2)));
        let p = Vec2::new(0, 2);
        assert!((p.distance_sq_to_line(a, b) - 9.0).abs() < 1e-12);
        assert!((p.distance_sq_to_segment(a, b) - 9.0).abs() < 1e-12);
        assert!((c.distance_sq_to_segment(a, b) - 4.0).abs() < 1e-12);
        assert!((p.distance_sq_to_segment(p, p)).abs() < 1e-12);
        let v = Vec2::new(4_000_000_000, 3_000_000_000);
        assert_eq!(v.norm_sq(), 25_000_000_000_000_000_000);
        assert_eq!(v.cross(Vec2::new(-3_000_000_000, 4_000_000_000)), v.norm_sq());
        let low = Vec2::new(i64::MIN, 0);
        let high = Vec2::new(i64::MAX, 0);
        assert_eq!(low.manhattan_distance(high), u64::MAX as i128);
        assert_eq!(low.chebyshev_distance(high), u64::MAX as i128);
    }

    #[test]
    fn segment_contact_overlap_and_point_degeneracy() {
        let a = Vec2::new(0, 0);
        let b = Vec2::new(4, 0);
        for (c, d, expected) in [
            (b, Vec2::new(4, 3), true),
            (Vec2::new(2, 0), Vec2::new(2, 3), true),
            (Vec2::new(2, 0), Vec2::new(6, 0), true),
            (Vec2::new(5, 0), Vec2::new(6, 0), false),
            (Vec2::new(2, 0), Vec2::new(2, 0), true),
            (Vec2::new(2, 1), Vec2::new(2, 1), false),
        ] {
            assert_eq!(Vec2::segments_intersect(a, b, c, d), expected);
            assert_eq!(Vec2::segments_intersect(d, c, b, a), expected);
        }
        assert!(Vec2::segments_intersect(a, a, a, a));
        assert!(!Vec2::segments_intersect(a, a, b, b));
    }

    #[test]
    fn near_full_turn_remains_near_full_turn() {
        let a = SpiralAngle::from_xy(i64::MAX, -1);
        assert!(a.radians() > PI);
        assert!((a.radians() - TAU).abs() < 1e-12);
        assert!(a.with_turn(-1).radians().abs() < 1e-12);
    }

    #[test]
    #[should_panic(expected = "ゼロベクトル")]
    fn zero_angle_is_rejected() {
        SpiralAngle::new(Vec2::default(), 0);
    }
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
