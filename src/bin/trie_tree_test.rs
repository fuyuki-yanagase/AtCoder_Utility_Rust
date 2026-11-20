// trie_treeの使用例・テスト。本体をコピーせず、公開モジュールを検証する。

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

use utility_rust::trie_tree::*;

fn example() {
    let mut trie = ByteTrie::new();

    // 同じ単語を2回追加する
    trie.insert("abc");
    trie.insert("abc");

    // 重複して数えられることを確認する
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.count_word("abc"), 2);
    assert_eq!(trie.count_prefix("a"), 2);

    // 1回だけ削除しても、まだ1個残る
    assert_eq!(trie.remove("abc"), true);
    assert_eq!(trie.contains("abc"), true);
    assert_eq!(trie.count_word("abc"), 1);

    // 2回目の削除で消える
    assert_eq!(trie.remove("abc"), true);
    assert_eq!(trie.contains("abc"), false);
    assert_eq!(trie.count_word("abc"), 0);

    // 存在しない単語は削除できない
    assert_eq!(trie.remove("abc"), false);

    // prefixだけ削除しても、長い単語は残る
    trie.insert("a");
    trie.insert("ab");
    trie.insert("abc");

    assert_eq!(trie.remove("a"), true);
    assert_eq!(trie.contains("a"), false);
    assert_eq!(trie.contains("ab"), true);
    assert_eq!(trie.contains("abc"), true);

    // 空byte列も扱える
    trie.insert_bytes(&[]);
    assert_eq!(trie.contains_bytes(&[]), true);
    assert_eq!(trie.remove_bytes(&[]), true);
    assert_eq!(trie.contains_bytes(&[]), false);
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
