// fenwick_treeの使用例・テスト。
use utility_rust::fenwick_tree::FenwickTree;

fn example() {
    let mut tree = FenwickTree::<i64>::new(5);
    tree.add(1, 3);
    tree.add(3, 7);
    assert_eq!(tree.range_sum(1, 4), 10);
    println!("[1,4)の和: {}", tree.range_sum(1, 4));
}
fn main() {
    example();
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn updates_and_all_ranges_match_array() {
    for n in [0, 1, 7, 32] {
        let mut tree = FenwickTree::<i64>::with_identity(n, 0);
        let mut values = vec![0; n];
        for step in 0..100 {
            if n > 0 {
                let i = step * 17 % n;
                let delta = (step % 11) as i64 - 5;
                values[i] += delta;
                tree.add(i, delta);
                assert_eq!(tree.get(i), values[i]);
            }
            for l in 0..=n {
                assert_eq!(tree.sum_prefix(l), values[..l].iter().sum());
                for r in l..=n {
                    assert_eq!(tree.range_sum(l, r), values[l..r].iter().sum());
                }
            }
        }
    }
}

#[test]
fn binary_construction_and_kth_match_all_small_bitstrings() {
    // 空・全0・全1・非2冪長を含む全ビット列を、配列の走査結果と照合する。
    for n in 0..=9 {
        for mask in 0..1_usize << n {
            let bytes: Vec<u8> = (0..n).map(|i| if mask >> i & 1 == 1 { b'1' } else { b'0' }).collect();
            let mut tree = FenwickTree::from_binary_bytes(&bytes);
            let mut counts: Vec<i32> = bytes.iter().map(|&b| i32::from(b == b'1')).collect();
            for step in 0..=n * 2 {
                let positions: Vec<usize> = counts.iter().enumerate()
                    .flat_map(|(i, &count)| std::iter::repeat_n(i, count as usize)).collect();
                for k in 0..=positions.len() + 1 {
                    assert_eq!(tree.kth(k), positions.get(k).copied(), "n={n}, mask={mask}, step={step}, k={k}");
                }
                for left in 0..=n {
                    assert_eq!(tree.sum_prefix(left), counts[..left].iter().sum());
                    for right in left..=n {
                        assert_eq!(tree.range_sum(left, right), counts[left..right].iter().sum());
                    }
                }
                if n > 0 {
                    let i = step * 5 % n;
                    // kthの前提（非負）を保ち、1より大きい度数と削除も検証する。
                    let delta = if step % 3 == 0 && counts[i] > 0 { -1 } else { 1 };
                    tree.add(i, delta);
                    counts[i] += delta;
                    assert_eq!(tree.get(i), counts[i]);
                }
            }
        }
    }
}
