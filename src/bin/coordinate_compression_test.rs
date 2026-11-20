use utility_rust::coordinate_compression::CoordinateCompression;

fn example() {
    let (coordinates, indices) = CoordinateCompression::with_indices(&[30, -10, 30, 0]);
    assert_eq!(coordinates.as_slice(), &[-10, 0, 30]);
    assert_eq!(indices, vec![2, 0, 2, 1]);
    assert_eq!(coordinates.range(&-10, &30), 0..2);
    println!("座標圧縮の入力順対応・半開区間: 成功");
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
    use std::collections::BTreeSet;
    use std::fmt::Debug;

    #[test]
    fn half_open_range_excludes_registered_right_endpoint() {
        let coordinates = CoordinateCompression::new(&[-2, 1, 4]);
        assert_eq!(coordinates.range(&-2, &1), 0..1);
        assert_eq!(coordinates.range(&-3, &4), 0..2);
    }

    #[test]
    fn half_open_empty_range_at_registered_coordinate() {
        let coordinates = CoordinateCompression::new(&[-2, 1, 4]);
        assert_eq!(coordinates.range(&1, &1), 1..1);
        assert_eq!(coordinates.range(&4, &4), 2..2);
    }

    // ソート・dedupや二分探索を期待値に使わず、集合の列挙と線形countで照合する。
    fn check<T: Ord + Clone + Debug>(input: &[T], probes: &[T]) {
        let expected: Vec<_> = input.iter().cloned().collect::<BTreeSet<_>>().into_iter().collect();
        let coordinates = CoordinateCompression::new(input);
        let from_slice = CoordinateCompression::from_slice(input);
        let (with_indices, indices) = CoordinateCompression::with_indices(input);
        assert_eq!(coordinates.len(), expected.len());
        assert_eq!(coordinates.is_empty(), expected.is_empty());
        assert_eq!(coordinates.as_slice(), expected.as_slice());
        assert_eq!(from_slice.into_vec(), expected);
        assert_eq!(coordinates.to_vec(), expected);
        assert_eq!(coordinates.clone().into_vec(), expected);
        assert_eq!(with_indices.as_slice(), expected.as_slice());
        let expected_indices: Vec<_> = input.iter().map(|value| expected.iter().filter(|&item| item < value).count()).collect();
        assert_eq!(indices, expected_indices, "入力順: {input:?}");
        for (original, &rank) in input.iter().zip(&indices) {
            assert_eq!(&with_indices[rank], original);
        }
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(coordinates.get(index), Some(value));
            assert_eq!(&coordinates[index], value);
        }
        assert_eq!(coordinates.get(expected.len()), None);
        assert_eq!(coordinates.get(expected.len() + 7), None);
        for probe in probes {
            assert_eq!(coordinates.index(probe), expected.iter().position(|value| value == probe));
            assert_eq!(coordinates.lower_bound(probe), expected.iter().filter(|&value| value < probe).count());
            assert_eq!(coordinates.upper_bound(probe), expected.iter().filter(|&value| value <= probe).count());
            for right in probes.iter().filter(|&right| probe <= right) {
                let selected: Vec<_> = expected.iter().filter(|&value| probe <= value && value < right).cloned().collect();
                let start = expected.iter().filter(|&value| value < probe).count();
                let range = coordinates.range(probe, right);
                assert_eq!(range, start..start + selected.len(), "入力={input:?}, 区間=[{probe:?},{right:?})");
                assert_eq!(coordinates.as_slice()[range], selected);
                let other = with_indices.range(probe, right);
                assert_eq!(with_indices.as_slice()[other], selected);
            }
        }
    }

    #[test]
    fn exhaustive_small_integer_arrays_match_set_and_linear_oracles() {
        // 長さ0～5・5値の全3906配列。重複・負数・単一・順列をすべて含む。
        let alphabet = [-2, -1, 0, 1, 2];
        let probes: Vec<_> = (-3..=3).collect();
        let mut cases = 0;
        for len in 0..=5_u32 {
            for mut code in 0..5_usize.pow(len) {
                let input: Vec<_> = (0..len).map(|_| {
                    let value = alphabet[code % 5];
                    code /= 5;
                    value
                }).collect();
                check(&input, &probes);
                cases += 1;
            }
        }
        assert_eq!(cases, 3906);
    }

    #[test]
    fn non_copy_strings_preserve_ownership_and_order() {
        let mut input = ["zebra", "あ", "apple", "apple", "", "😀", "zebra"].map(String::from);
        let probes = ["", "a", "apple", "z", "zebra", "あ", "😀", "😁"].map(String::from);
        check(&input, &probes);
        check::<String>(&[], &probes);
        let coordinates = CoordinateCompression::new(&input);
        let cloned = coordinates.clone();
        input[0].push_str(" changed");
        let mut copied = coordinates.to_vec();
        copied[0].push_str(" independent");
        assert_eq!(coordinates.as_slice(), cloned.as_slice());
        assert_eq!(coordinates.index(&"zebra".to_string()), Some(2));
        assert_eq!(coordinates.index(&input[0]), None);
        assert_eq!(cloned.into_vec(), ["", "apple", "zebra", "あ", "😀"].map(String::from));
    }

    #[test]
    #[should_panic(expected = "区間の左端が右端を超えてる")]
    fn reversed_range_follows_documented_panic() {
        CoordinateCompression::new(&[1, 3]).range(&3, &1);
    }

    #[test]
    #[should_panic]
    fn index_operator_panics_outside_table() {
        let coordinates = CoordinateCompression::new(&[1]);
        let _ = coordinates[1];
    }
}
