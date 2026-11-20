// rolling_hashの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::zero_repeat_side_effects)]
#![allow(clippy::bool_assert_comparison)]

use num_rational::Ratio;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::f64::consts::{PI, TAU};
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

// 1行でのbreak, continue, return
macro_rules! breakif {
    ($c:expr) => {
        if $c {
            break;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            break $l;
        }
    };
}
macro_rules! continueif {
    ($c:expr) => {
        if $c {
            continue;
        }
    };
    ($c:expr, $l:lifetime) => {
        if $c {
            continue $l;
        }
    };
}
macro_rules! returnif {
    ($c:expr) => {
        if $c {
            return;
        }
    };
    ($c:expr, $v:expr) => {
        if $c {
            return $v;
        }
    };
}

const M107: i64 = 1000000007_i64;
const M998: i64 = 998244353_i64;
const L108: i64 = 100_000_000_i64;
const I108: i32 = 100_000_000_i32;
const L1018: i64 = 1_000_000_000_000_000_000_i64;
const IINF: i32 = (1_i32 << 29) - 1;
const LINF: i64 = (1_i64 << 61) - 1;
const UINF: usize = (1_usize << 61) - 1;

use utility_rust::rolling_hash::*;

fn example() {
    // 基本的な文字列でRollingHashを作ります。
    let rh = RollingHash::new("abcabc", 2);

    // 同じ部分文字列の比較です。
    // s[0..3] = "abc"
    // s[3..6] = "abc"
    assert_eq!(rh.compare_hash(0, 3, 3, 6), true);

    // 違う部分文字列の比較です。
    // s[0..2] = "ab"
    // s[1..3] = "bc"
    assert_eq!(rh.compare_hash(0, 2, 1, 3), false);

    // 長さが違う場合は必ずfalseです。
    // s[0..2] = "ab"
    // s[0..3] = "abc"
    assert_eq!(rh.compare_hash(0, 2, 0, 3), false);

    // 同じ場所同士なら当然trueです。
    assert_eq!(rh.compare_hash(1, 5, 1, 5), true);

    // 空区間同士の比較です。
    // どちらも空文字列なのでtrueになります。
    assert_eq!(rh.compare_hash(0, 0, 3, 3), true);

    // 空区間と非空区間の比較です。
    // 長さが違うのでfalseになります。
    assert_eq!(rh.compare_hash(0, 0, 0, 1), false);

    // 空区間のハッシュは0になります。
    assert_eq!(rh.calc_hashes(2, 2), vec![0, 0]);

    // byte列から作るテストです。
    let bytes = vec![10_u8, 20_u8, 30_u8, 10_u8, 20_u8, 30_u8];
    let rh_bytes = RollingHash::new_from_bytes(&bytes, 3);

    // bytes[0..3] と bytes[3..6] は同じです。
    assert_eq!(rh_bytes.compare_hash(0, 3, 3, 6), true);

    // bytes[0..2] と bytes[1..3] は違います。
    assert_eq!(rh_bytes.compare_hash(0, 2, 1, 3), false);

    // 全区間について, 実際のスライス比較とRollingHashの結果が一致するか確認します。
    let text = "abracadabra";
    let rh_all = RollingHash::new(text, 2);
    let data = text.as_bytes();
    let n = data.len();

    for l1 in 0..=n {
        for r1 in l1..=n {
            for l2 in 0..=n {
                for r2 in l2..=n {
                    // 実際のスライス比較結果です。
                    let expected = data[l1..r1] == data[l2..r2];

                    // RollingHashによる比較結果です。
                    let actual = rh_all.compare_hash(l1, r1, l2, r2);

                    // 両方が一致することを確認します。
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    // random_numを1にした場合も最低限の確認をします。
    let rh_single = RollingHash::new("aaaaa", 1);
    assert_eq!(rh_single.compare_hash(0, 1, 1, 2), true);
    assert_eq!(rh_single.compare_hash(0, 2, 2, 4), true);
    assert_eq!(rh_single.compare_hash(0, 3, 1, 4), true);
    assert_eq!(rh_single.compare_hash(0, 4, 1, 5), true);

    // ここまでpanicしなければ成功です。
    println!("OK");
} // end of function

#[cfg(test)]
mod search_tests {
    use super::RollingHash;

    #[test]
    fn finds_shorter_pattern_and_overlapping_matches() {
        let text = RollingHash::new("abracadabra", 2);
        let pattern = RollingHash::new("abra", 2);
        assert_eq!(text.find(&pattern), Some(0));
        assert_eq!(text.find_all(&pattern), vec![0, 7]);
        assert!(text.contains_hash(&pattern));
        assert_eq!(RollingHash::new("aaaa", 2).find_all(&RollingHash::new("aa", 2)), vec![0, 1, 2]);
    }

    #[test]
    fn search_boundaries_and_bytes_match_naive_search() {
        // 全長・部分列・不一致・空列・長すぎる検索語をスライス検索と照合する。
        for bytes in [&b""[..], &b"a"[..], &b"ababa"[..], &[0, 255, 0, 255, 1][..]] {
            let mut patterns = vec![vec![], vec![42], vec![0; bytes.len() + 1]];
            for l in 0..bytes.len() {
                for r in l + 1..=bytes.len() {
                    patterns.push(bytes[l..r].to_vec());
                }
            }
            for seed in [0, 123456789] {
                let text = RollingHash::with_seed_from_bytes(bytes, 3, seed);
                for pattern in &patterns {
                    let target = RollingHash::with_seed_from_bytes(pattern, 3, seed);
                    let expected: Vec<_> = (0..=bytes.len()).filter(|&i| bytes[i..].starts_with(pattern)).collect();
                    assert_eq!(text.find_all(&target), expected);
                    assert_eq!(text.find(&target), expected.first().copied());
                    assert_eq!(text.contains_hash(&target), !expected.is_empty());
                }
            }
        }
    }
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
