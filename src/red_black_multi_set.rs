// 赤黒木 MultiSet
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::zero_repeat_side_effects)]

use num_complex::*;
use num_rational::*;
use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::f64::consts::*;
use std::fmt;
use std::fmt::{Debug, Display};
use std::io::{self, BufWriter, Read, Write};
use std::iter::*;
use std::marker::*;
use std::mem::*;
use std::ops::*;
use std::str::FromStr;

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

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                   赤黒木 MultiSet
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/324c72d2a705af1b/src/red_black_multi_set.rs

/// ノードが存在しないことを表す添字
/// Vecの通常の添字として usize::MAX は使われないはず
/// 赤黒木内部の null 相当として利用
const RBNIL: usize = usize::MAX;

/// ノードの色
/// nullの葉は黒として扱う
#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeColor {
    Red,
    Black,
} // end of enum

/// 赤黒木のノード
/// 親ポインタの参照はBorrow Checker的に扱いづらい
///  → Vecの添字を疑似ポインタとして利用
struct RBTreeNode<T> {
    /// キー
    key: T,

    /// このキーの個数
    node_count: usize,

    /// このノードを根とする部分木の重複込み要素数
    subtree_size: usize,

    /// このノードを根とする部分木のユニークキー数
    unique_size: usize,

    /// ノードの色
    color: NodeColor,

    /// 左の子, 存在しない場合はRBNIL
    left: usize,

    /// 右の子、存在しない場合はRBNIL
    right: usize,

    /// 親, 根の場合はRBNIL
    parent: usize,
} // end of struct

impl<T> RBTreeNode<T> {
    /// 新しいノードを作成
    #[inline(always)]
    fn new(key: T, color: NodeColor, parent: usize, count: usize) -> Self {
        return Self {
            key,
            node_count: count,
            subtree_size: count,
            unique_size: 1,
            color,
            left: RBNIL,
            right: RBNIL,
            parent,
        };
    } // end of method
} // end of impl RBTreeNode

/// ノード置き場の1スロット
///  - Used : 使用中
///  - Free : 削除済み
///  - FreeのNext : free_listの次の空きスロット
enum RBTreeSlot<T> {
    Used(RBTreeNode<T>),
    Free { next: usize },
} // end of enum

/// 標準の Ord 比較を使う赤黒木 MultiSet。
/// これを使うと比較関数を明示せずに `OrdRBTreeMultiSet::<T>::new()` で作れる。
pub type OrdRBTreeMultiSet<T> = RBTreeMultiSet<T, fn(&T, &T) -> Ordering>;

/// 重複可の赤黒木
///
/// - 追加/削除/検索 : O(logN)
/// - LowerBound / UpperBound : O(logN)
/// - 0-indexedでK番目取得 : O(logN)
/// - 同じキーは 1ノードにまとめて node_count で管理
/// - 比較関数がEqualで返したものを等価なキーとして扱う
pub struct RBTreeMultiSet<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    /// 木の根
    root: usize,

    /// 最小ノード, 空木ならRBNIL
    first: usize,

    /// 最大ノード, 空木ならRBNIL
    last: usize,

    /// 異なるキーの個数
    unique_count: usize,

    /// 使用中スロットと空きスロットをまとめて持つ配列
    /// Vec::removeは使用しない
    /// 添字を安定させるため、削除時はfreeスロットに置き換え
    nodes: Vec<RBTreeSlot<T>>,

    /// 空きスロットの先頭, 空きがなければRBNIL
    free_head: usize,

    /// 空きスロット数
    /// 統計用, 探索や更新の本体には使用しない
    free_count: usize,

    /// 比較関数
    /// compare(a, b) == Ordering::Less → a は bより左側
    /// compare(a, b) == Ordering::Equal → MultiSet上は同じキーとして扱う
    compare: F,
} // end of struct

impl<T, F> RBTreeMultiSet<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    // ======================================================================================
    //
    //                                        構築・基本参照
    //
    // ======================================================================================

    /// 比較関数を指定して空の木を作る
    #[inline(always)]
    pub fn with_compare(compare: F) -> Self {
        return Self {
            root: RBNIL,
            first: RBNIL,
            last: RBNIL,
            unique_count: 0,
            nodes: Vec::new(),
            free_head: RBNIL,
            free_count: 0,
            compare,
        };
    } // end of constructor

    /// 比較関数を指定して、容量を予約した空の木を作る
    #[inline(always)]
    pub fn with_capacity_and_compare(capacity: usize, compare: F) -> Self {
        return Self {
            root: RBNIL,
            first: RBNIL,
            last: RBNIL,
            unique_count: 0,
            nodes: Vec::with_capacity(capacity),
            free_head: RBNIL,
            free_count: 0,
            compare,
        };
    } // end of method

    /// 重複を含む総要素数を返す
    #[inline(always)]
    pub fn count(&self) -> usize {
        return self.subtree_size_of(self.root);
    } // end of method

    /// 異なるキーの個数を返す
    #[inline(always)]
    pub fn unique_count(&self) -> usize {
        return self.unique_count;
    } // end of method

    /// rootの unique_size から異なるキーの個数を返す
    /// 基本的には unique_count() と同じ値になる
    #[inline(always)]
    pub fn unique_count_by_tree(&self) -> usize {
        return self.unique_size_of(self.root);
    } // end of method

    /// 木が空かどうかを返す
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.root == RBNIL;
    } // end of method

    /// 内部Vecのスロット数を返す
    ///  → 使用中スロットと空きスロットの合計
    #[inline(always)]
    pub fn slot_count(&self) -> usize {
        return self.nodes.len();
    } // end of method

    /// 使用中スロット数を返す
    #[inline(always)]
    pub fn used_slot_count(&self) -> usize {
        return self.nodes.len() - self.free_count;
    } // end of method

    ///空きスロット数を返す
    #[inline(always)]
    pub fn free_slot_count(&self) -> usize {
        return self.free_count;
    } // end of method

    /// 最小キーを返す
    #[inline(always)]
    pub fn first(&self) -> Option<&T> {
        if self.first == RBNIL {
            return None;
        }
        return Some(&self.node(self.first).key);
    } // end of method

    /// 最大キーを返す
    pub fn last(&self) -> Option<&T> {
        if self.last == RBNIL {
            return None;
        }
        return Some(&self.node(self.last).key);
    } // end of method

    /// 最小キーの個数を返す
    #[inline(always)]
    pub fn first_count(&self) -> usize {
        if self.first == RBNIL {
            return 0;
        }
        return self.node(self.first).node_count;
    } // end of method

    /// 最大キーの個数を返す
    pub fn last_count(&self) -> usize {
        if self.last == RBNIL {
            return 0;
        }
        return self.node(self.last).node_count;
    } // end of method

    /// キーを1個追加
    #[inline(always)]
    pub fn add(&mut self, key: T) {
        self.add_count(key, 1);
    } // end of method

    /// キーを複数個追加する
    #[inline(always)]
    pub fn add_count(&mut self, key: T, count: usize) {
        if count == 0 {
            return;
        }

        self.insert(key, count);
    } // end of method

    /// キーを1個削除
    /// 1個以上削除できたらtrue, 存在しなければfalse
    #[inline(always)]
    pub fn remove(&mut self, key: &T) -> bool {
        return self.remove_count(key, 1);
    } // end of method

    /// キーを複数個削除
    /// countが現在個数以上なら, そのキーのノードを丸ごと削除
    #[inline(always)]
    pub fn remove_count(&mut self, key: &T, count: usize) -> bool {
        if count == 0 {
            return false;
        }

        let z: usize = self.find_node_index(key);
        if z == RBNIL {
            return false;
        }

        let current_count: usize = self.node(z).node_count;
        if count < current_count {
            self.node_mut(z).node_count -= count;
            self.update_size_to_root(z);
            return true;
        }

        self.remove_node(z);
        return true;
    } // end of method

    /// キーをの要素を全て削除
    #[inline(always)]
    pub fn remove_all(&mut self, key: &T) -> bool {
        let z = self.find_node_index(key);
        if z == RBNIL {
            return false;
        }

        self.remove_node(z);
        return true;
    } // end of method

    /// キーが存在するかを返す
    #[inline(always)]
    pub fn contains(&self, key: &T) -> bool {
        return self.find_node_index(key) != RBNIL;
    } // end of method

    /// キーの個数を返す
    #[inline(always)]
    pub fn count_of(&self, key: &T) -> usize {
        let index: usize = self.find_node_index(key);
        if index == RBNIL {
            return 0;
        }

        return self.node(index).node_count;
    } // end of method

    /// キーが存在するなら, 木の中に保存されているキーへの参照を返す
    /// 比較関数では等価でも, 値として完全一致しない型を使う場合に便利かも
    #[inline(always)]
    pub fn get(&self, key: &T) -> Option<&T> {
        let index: usize = self.find_node_index(key);
        if index == RBNIL {
            return None;
        }
        return Some(&self.node(index).key);
    } // end of method

    /// 0-indexedでindex番目のキーを返す
    /// 重複要素も1個ずつ数える
    #[inline(always)]
    pub fn get_at(&self, mut index: usize) -> Option<&T> {
        if self.count() <= index {
            return None;
        }

        let mut current: usize = self.root;
        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            let left_size: usize = self.subtree_size_of(node.left);

            if index < left_size {
                current = node.left;
                continue;
            }

            index -= left_size;

            if index < node.node_count {
                return Some(&node.key);
            }

            index -= node.node_count;
            current = node.right;
        }

        return None;
    } // end of method

    /// 0-indexedで, index番目のユニークキーを返す
    /// 重複個数は無視, 同じキーは1つとしてカウント
    #[inline(always)]
    pub fn get_unique_at(&self, mut index: usize) -> Option<&T> {
        if self.unique_count <= index {
            return None;
        }

        let mut current: usize = self.root;
        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            let left_unique_size: usize = self.unique_size_of(node.left);

            if index < left_unique_size {
                current = node.left;
                continue;
            }

            index -= left_unique_size;
            if index == 0 {
                return Some(&node.key);
            }

            index -= 1;
            current = node.right;
        }
        return None;
    } // end of method

    /// key以上の最小キーを返す
    #[inline(always)]
    pub fn lower_bound(&self, key: &T) -> Option<&T> {
        let index: usize = self.lower_bound_index(key);
        if index == RBNIL {
            return None;
        }

        return Some(&self.node(index).key);
    } // end of method

    /// keyより大きい最小のキーを返す
    #[inline(always)]
    pub fn upper_bound(&self, key: &T) -> Option<&T> {
        let index: usize = self.upper_bound_index(key);
        if index == RBNIL {
            return None;
        }

        return Some(&self.node(index).key);
    } // end of method

    /// key未満の最大のキーを返す
    #[inline(always)]
    pub fn less_than_max(&self, key: &T) -> Option<&T> {
        let index: usize = self.less_than_max_index(key);
        if index == RBNIL {
            return None;
        }

        return Some(&self.node(index).key);
    } // end of method

    /// key以下の最大キーを返す
    #[inline(always)]
    pub fn less_than_or_equal_max(&self, key: &T) -> Option<&T> {
        let index: usize = self.less_than_or_equal_max_index(key);
        if index == RBNIL {
            return None;
        }

        return Some(&self.node(index).key);
    } // end of method

    /// key未満の要素数を返す
    /// 重複もすべて数える
    #[inline(always)]
    pub fn count_less_than(&self, key: &T) -> usize {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less | Ordering::Equal => {
                    current = node.left;
                }
                Ordering::Greater => {
                    answer += self.subtree_size_of(node.left);
                    answer += node.node_count;
                    current = node.right;
                }
            }
        }
        return answer;
    } // end of method

    /// key以下の要素数を返す
    /// 重複も全て数える
    #[inline(always)]
    pub fn count_less_than_or_equal(&self, key: &T) -> usize {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Equal | Ordering::Greater => {
                    answer += self.subtree_size_of(node.left);
                    answer += node.node_count;
                    current = node.right;
                }
            }
        }
        return answer;
    } // end of method

    /// key未満のユニークキー数を返す
    /// keyを挿入するとしたら,ユニークキー列の何番目に入るか
    #[inline(always)]
    pub fn count_unique_less_than(&self, key: &T) -> usize {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less | Ordering::Equal => {
                    current = node.left;
                }
                Ordering::Greater => {
                    answer += self.unique_size_of(node.left);
                    answer += 1;
                    current = node.right;
                }
            }
        }
        return answer;
    } // end of method

    /// key以下のユニークキー数を返す
    #[inline(always)]
    pub fn count_unique_less_than_or_equal(&self, key: &T) -> usize {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Equal | Ordering::Greater => {
                    answer += self.unique_size_of(node.left);
                    answer += 1;
                    current = node.right;
                }
            }
        }

        return answer;
    } // end of method

    /// keyがユニークキー列の何番目かを返す
    /// 0-indexed
    /// keyが存在しなければNone
    #[inline(always)]
    pub fn unique_index_of(&self, key: &T) -> Option<usize> {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Greater => {
                    answer += self.unique_size_of(node.left);
                    answer += 1;
                    current = node.right;
                }
                Ordering::Equal => {
                    answer += self.unique_size_of(node.left);
                    return Some(answer);
                }
            }
        }
        return None;
    } // end of method

    /// keyが重複込みの要素列の何番目から始まるかを返す
    /// 0-indexed
    /// keyが存在しなければNone
    #[inline(always)]
    pub fn index_of(&self, key: &T) -> Option<usize> {
        let mut answer: usize = 0;
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Greater => {
                    answer += self.subtree_size_of(node.left);
                    answer += node.node_count;
                    current = node.right;
                }
                Ordering::Equal => {
                    answer += self.subtree_size_of(node.left);
                    return Some(answer);
                }
            }
        }
        return None;
    } // end of method

    /// keyが重複込みの要素列で占める半開区間を返す
    /// 0-indexedの [left, right) として返す
    /// keyが存在しなければNone
    #[inline(always)]
    pub fn index_range_of(&self, key: &T) -> Option<(usize, usize)> {
        let left: usize = self.index_of(key)?;
        let right: usize = left + self.count_of(key);
        return Some((left, right));
    } // end of method

    /// 昇順に並んだキーを返す
    /// 重複キーは重複個数分だけ入る
    /// 大量データで毎回呼ぶとVecを作るので必要な場面だけで
    #[inline(always)]
    pub fn keys(&self) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::with_capacity(self.count());
        let mut current: usize = self.first;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            for _ in 0..node.node_count {
                result.push(&node.key);
            }
            current = self.next_index(current);
        }
        return result;
    } // end of method

    /// 昇順に並んだユニークキーを返す
    #[inline(always)]
    pub fn unique_keys(&self) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::with_capacity(self.unique_count);
        let mut current: usize = self.first;

        while current != RBNIL {
            result.push(&self.node(current).key);
            current = self.next_index(current);
        }
        return result;
    } // end of method

    /// 木を空にする
    /// 全てのkeyをdrop, 空きスロットも消す
    #[inline(always)]
    pub fn clear(&mut self) {
        self.root = RBNIL;
        self.first = RBNIL;
        self.last = RBNIL;
        self.unique_count = 0;
        self.nodes.clear();
        self.free_head = RBNIL;
        self.free_count = 0;
    } // end of method

    /// Vecのcapacityを返す
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        return self.nodes.capacity();
    } // end of method

    /// 追加される可能性があるユニークキー数だけ容量を予約
    #[inline(always)]
    pub fn reserve_slots(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    } // end of method

    /// [left_key, right_key] に含まれるキーを昇順で返す
    /// MultiSetなので, 重複キーは重複個数分だけ返す
    #[inline(always)]
    pub fn range_keys(&self, left_key: &T, right_key: &T) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::new();

        // left_key以上の最初のノードから
        let mut current: usize = self.lower_bound_index(left_key);

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);

            // node.keyがright_keyを超えたら区間外
            if (self.compare)(&node.key, right_key) == Ordering::Greater {
                break;
            }

            // MutilSetなので, 同じキーをnoe_count個だけ出力対称に
            for _ in 0..node.node_count {
                result.push(&node.key);
            }
            // 中間順で次のノードへ
            current = self.next_index(current);
        }
        return result;
    } // end of method

    /// [left_key, right_key] に含まれるユニークキーを昇順で返す
    #[inline(always)]
    pub fn range_unique_keys(&self, left_key: &T, right_key: &T) -> Vec<&T> {
        let mut result: Vec<&T> = Vec::new();

        // left_key以上の最初のノードから
        let mut current: usize = self.lower_bound_index(left_key);

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);

            // node.keyがright_keyを超えたら区間外
            if (self.compare)(&node.key, right_key) == Ordering::Greater {
                break;
            }

            // ユニークキー列挙なので1回のみ
            result.push(&node.key);

            // 中間順で次のノードへ
            current = self.next_index(current);
        }
        return result;
    } // end of method

    // ======================================================================================
    //
    //                                    赤黒木内部メソッド
    //
    // ======================================================================================

    /// 使用中ノードを不変参照で取得
    /// 赤黒木のリンクが正しければ、Freeスロットには到達しないはず
    fn node(&self, index: usize) -> &RBTreeNode<T> {
        match &self.nodes[index] {
            RBTreeSlot::Used(node) => {
                return node;
            }
            RBTreeSlot::Free {
                ..
            } => {
                panic!("空きスロットを使用中ノードとして参照しました。");
            }
        }
    } // end of method

    /// 使用中ノードを可変参照で取得する。
    #[inline(always)]
    fn node_mut(&mut self, index: usize) -> &mut RBTreeNode<T> {
        match &mut self.nodes[index] {
            RBTreeSlot::Used(node) => {
                return node;
            }
            RBTreeSlot::Free {
                ..
            } => {
                panic!("空きスロットを使用中ノードとして可変参照しました。");
            }
        }
    } // end of method

    /// 新しいノードスロットを確保
    /// 空きスロットがあれば再利用, なければ末尾にpush
    #[inline(always)]
    fn allocate_node(&mut self, key: T, color: NodeColor, parent: usize, count: usize) -> usize {
        let new_node = RBTreeNode::new(key, color, parent, count);

        if self.free_head != RBNIL {
            let index: usize = self.free_head;
            let next_free = match &self.nodes[index] {
                RBTreeSlot::Free {
                    next,
                } => *next,
                RBTreeSlot::Used(_) => {
                    panic!("free_head が使用中ノードを指しています。");
                }
            };

            self.free_head = next_free;
            self.free_count -= 1;
            self.nodes[index] = RBTreeSlot::Used(new_node);
            return index;
        }

        self.nodes.push(RBTreeSlot::Used(new_node));
        return self.nodes.len() - 1;
    } // end of method

    /// ノードスロットを開放
    ///  **この関数を呼ぶ前に、必ず木から node_index へのリンクを外しておく**
    ///  UsedをFreeに置き換えることで、古いkeyはここでdrop
    #[inline(always)]
    fn free_node(&mut self, node_index: usize) {
        let old_slot = replace(
            &mut self.nodes[node_index],
            RBTreeSlot::Free {
                next: self.free_head,
            },
        );

        match old_slot {
            RBTreeSlot::Used(_) => {
                // 正常動作
            }
            RBTreeSlot::Free {
                ..
            } => {
                panic!("解放済みのノードを再度解放しようとしました。");
            }
        }

        self.free_head = node_index;
        self.free_count += 1;
    } // end of method

    /// indexがRBNILかチェック
    #[inline(always)]
    fn is_nil(index: usize) -> bool {
        return index == RBNIL;
    } // end of method

    /// ノードの色を返す
    /// RBNILは葉なので黒
    #[inline(always)]
    fn color_of(&self, index: usize) -> NodeColor {
        if index == RBNIL {
            return NodeColor::Black;
        }

        return self.node(index).color;
    } // end of method

    /// ノードが赤？
    #[inline(always)]
    fn is_red(&self, index: usize) -> bool {
        return self.color_of(index) == NodeColor::Red;
    } // end of method

    /// ノードが黒？
    #[inline(always)]
    fn is_black(&self, index: usize) -> bool {
        return self.color_of(index) == NodeColor::Black;
    } // end of method

    /// 指定ノードが左の子？
    #[inline(always)]
    fn is_left_child(&self, child: usize, parent: usize) -> bool {
        return self.node(parent).left == child;
    } // end of method

    /// 指定ノードが右の子？
    #[inline(always)]
    fn is_right_child(&self, child: usize, parent: usize) -> bool {
        return self.node(parent).right == child;
    } // end of method

    /// 部分木サイズを返す
    /// RBNILは空部分木なので0
    #[inline(always)]
    fn subtree_size_of(&self, index: usize) -> usize {
        if index == RBNIL {
            return 0;
        }

        return self.node(index).subtree_size;
    } // end of method

    /// 部分木のユニークキーの数を返す
    /// RBNILは空部分木なので0
    #[inline(always)]
    fn unique_size_of(&self, index: usize) -> usize {
        if index == RBNIL {
            return 0;
        }
        return self.node(index).unique_size;
    } // end of method

    /// ノードの部分木サイズを再計算
    ///  - subtree_size : 重複込みの要素数
    ///  - unique_size : ユニークキーの数
    #[inline(always)]
    fn update_size(&mut self, index: usize) {
        if index == RBNIL {
            return;
        }

        let left: usize = self.node(index).left;
        let right: usize = self.node(index).right;
        let node_count: usize = self.node(index).node_count;

        let left_size: usize = self.subtree_size_of(left);
        let right_size: usize = self.subtree_size_of(right);
        let left_unique_size: usize = self.unique_size_of(left);
        let right_unique_size: usize = self.unique_size_of(right);

        let node: &mut RBTreeNode<T> = self.node_mut(index);
        node.subtree_size = node_count + left_size + right_size;
        node.unique_size = 1 + left_unique_size + right_unique_size;
    } // end of method

    /// 指定ノードから根まで部分木サイズを再計算
    #[inline(always)]
    fn update_size_to_root(&mut self, mut index: usize) {
        while index != RBNIL {
            self.update_size(index);
            index = self.node(index).parent;
        }
    } // end of method

    /// 左回転
    /// x の右の子 y を上に持ち上げる
    #[inline(always)]
    fn rotate_left(&mut self, x: usize) {
        let y: usize = self.node(x).right;
        if y == RBNIL {
            return;
        }

        let y_left: usize = self.node(y).left;
        self.node_mut(x).right = y_left;
        if y_left != RBNIL {
            self.node_mut(y_left).parent = x;
        }

        let x_parent: usize = self.node(x).parent;
        self.node_mut(y).parent = x_parent;

        if x_parent == RBNIL {
            self.root = y;
        } else if self.is_left_child(x, x_parent) {
            self.node_mut(x_parent).left = y;
        } else {
            self.node_mut(x_parent).right = y;
        }

        self.node_mut(y).left = x;
        self.node_mut(x).parent = y;

        self.update_size(x);
        self.update_size(y);

        let y_parent: usize = self.node(y).parent;
        self.update_size_to_root(y_parent);
    } // end of method

    /// 右回転
    /// x の左の子 y を上に持ち上げる
    #[inline(always)]
    fn rotate_right(&mut self, x: usize) {
        let y: usize = self.node(x).left;
        if y == RBNIL {
            return;
        }

        let y_right: usize = self.node(y).right;
        self.node_mut(x).left = y_right;
        if y_right != RBNIL {
            self.node_mut(y_right).parent = x;
        }

        let x_parent: usize = self.node(x).parent;
        self.node_mut(y).parent = x_parent;

        if x_parent == RBNIL {
            self.root = y;
        } else if self.is_left_child(x, x_parent) {
            self.node_mut(x_parent).left = y;
        } else {
            self.node_mut(x_parent).right = y;
        }

        self.node_mut(y).right = x;
        self.node_mut(x).parent = y;

        self.update_size(x);
        self.update_size(y);

        let y_parent: usize = self.node(y).parent;
        self.update_size_to_root(y_parent);
    } // end of method

    /// キーに対応するノード添字を探す
    /// 見つからなければRBNIL
    #[inline(always)]
    fn find_node_index(&self, key: &T) -> usize {
        let mut current: usize = self.root;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Greater => {
                    current = node.right;
                }
                Ordering::Equal => {
                    return current;
                }
            }
        }
        return RBNIL;
    } // end of method

    /// 挿入本体
    #[inline(always)]
    fn insert(&mut self, key: T, count: usize) {
        // 空木の場合は追加するノードが根, 根は常に黒ノード
        if self.root == RBNIL {
            let root: usize = self.allocate_node(key, NodeColor::Black, RBNIL, count);
            self.root = root;
            self.first = root;
            self.last = root;
            self.unique_count = 1;
            return;
        }

        let mut current: usize = self.root;
        let mut parent: usize = RBNIL;
        let mut last_ordering: Ordering = Ordering::Equal;

        // 左右判別して子に追加
        while current != RBNIL {
            parent = current;
            last_ordering = (self.compare)(&key, &self.node(current).key);

            match last_ordering {
                Ordering::Less => {
                    current = self.node(current).left;
                }
                Ordering::Greater => {
                    current = self.node(current).right;
                }
                Ordering::Equal => {
                    self.node_mut(current).node_count += count;
                    self.update_size_to_root(current);
                    return;
                }
            }
        }

        // ここに到達したら、同じkeyは存在していない
        // 新規ノードは赤で追加
        let new_node: usize = self.allocate_node(key, NodeColor::Red, parent, count);

        if last_ordering == Ordering::Less {
            self.node_mut(parent).left = new_node;
        } else {
            self.node_mut(parent).right = new_node;
        }

        self.unique_count += 1;

        // first, lastの更新
        if self.first == RBNIL || (self.compare)(&self.node(new_node).key, &self.node(self.first).key) == Ordering::Less
        {
            self.first = new_node;
        }
        if self.last == RBNIL || (self.compare)(&self.node(self.last).key, &self.node(new_node).key) == Ordering::Less {
            self.last = new_node;
        }

        self.update_size_to_root(parent);
        self.fix_after_insert(new_node);
        self.update_size_to_root(new_node);
    } // end of method

    /// 挿入後の赤黒木修正
    #[inline(always)]
    fn fix_after_insert(&mut self, mut z: usize) {
        // 親が黒なら赤黒木の条件はこわれていないはず
        // 親が赤の間だけ、祖父と叔父を見ながら更新
        while self.is_red(self.node(z).parent) {
            let parent: usize = self.node(z).parent;
            let grand_parent: usize = self.node(parent).parent;
            if grand_parent == RBNIL {
                break;
            }

            // 親が祖父の左の子
            if self.node(grand_parent).left == parent {
                // 叔父は祖父の右の子
                let uncle: usize = self.node(grand_parent).right;

                // Case 1:
                // - 親と叔父がどちらも赤
                // - 親と叔父を黒, 祖父を赤に
                // - 赤黒木の違反を祖父に押し上げる
                if self.is_red(uncle) {
                    self.node_mut(parent).color = NodeColor::Black;
                    if uncle != RBNIL {
                        self.node_mut(uncle).color = NodeColor::Black;
                    }

                    self.node_mut(grand_parent).color = NodeColor::Red;

                    // 祖父が赤になったので、更に上で赤赤違反の可能性
                    z = grand_parent;
                } else {
                    // Case 2:
                    // 親は赤, 叔父は黒, zが親の子
                    // 折れ曲がり形なので、親を左回転して一直線に
                    if self.node(parent).right == z {
                        z = parent;
                        self.rotate_left(z);
                    }

                    // Case 3:
                    // 親は赤, 叔父は黒, zが親の左の子
                    // 親を黒, 祖父を赤にして, 祖父を右回転
                    let parent: usize = self.node(z).parent;
                    let grand_parent: usize = self.node(parent).parent;
                    self.node_mut(parent).color = NodeColor::Black;
                    self.node_mut(grand_parent).color = NodeColor::Red;
                    self.rotate_right(grand_parent);
                }
            } else {
                // 親が祖父の右の子である場合
                // 左右を反転した対象ケースを処理
                let uncle: usize = self.node(grand_parent).left;

                // Case 1の対称:
                // 親と祖父がどちらも赤
                if self.is_red(uncle) {
                    self.node_mut(parent).color = NodeColor::Black;

                    if uncle != RBNIL {
                        self.node_mut(uncle).color = NodeColor::Black;
                    }

                    self.node_mut(grand_parent).color = NodeColor::Red;
                    z = grand_parent;
                } else {
                    // Case 2の対称:
                    // zが親の左の子なら, 親を右回転して一直線形へ
                    if self.node(parent).left == z {
                        z = parent;
                        self.rotate_right(z);
                    }

                    // Case 3の対称:
                    // 親を黒, 祖父を赤, 祖父を左回転
                    let parent: usize = self.node(z).parent;
                    let grand_parent: usize = self.node(parent).parent;
                    self.node_mut(parent).color = NodeColor::Black;
                    self.node_mut(grand_parent).color = NodeColor::Red;
                    self.rotate_left(grand_parent);
                }
            }
        }

        // 赤黒木では根は必ず黒
        // 色反転で根が赤になる可能性も → 最後に必ず黒へ戻す
        if self.root != RBNIL {
            self.node_mut(self.root).color = NodeColor::Black;
        }
    } // end of method

    /// ノードを削除
    /// zは使用中ノードでなければならない
    /// この関数の最後でzのスロットをFreeにしてkeyをdrop
    #[inline(always)]
    fn remove_node(&mut self, z: usize) {
        // zが first または last だった場合に備えて、削除前に次ノードと前ノードを保存
        // 削除後に z は Free, その後に next_index や prev_index は呼べない
        let next_of_z = if self.first == z { self.next_index(z) } else { RBNIL };
        let prev_of_z = if self.last == z { self.prev_index(z) } else { RBNIL };

        // 実際に木から取り外されるノードをy
        // z の子が 0 or 1 個なら y == z
        // z の子が2個なら, 後継ノードyをzの位置へ移動
        let mut y: usize = z;

        // yが黒 → 削除によって黒の個数が不足する可能性
        // 削除前の色を保存
        let mut y_original_color: NodeColor = self.node(y).color;

        // x : yが元いた位置に入ってくる子
        // xがRBNILの場合でも, 黒不足がRBNIL葉に乗るものとして扱う
        let x: usize;

        // xがRBNILの場合, xから親をたどれない
        // xの親を別変数として保存
        let x_parent: usize;

        // XがRBNIL → 親の左, 親の右 どちらにいたのか判定できない
        // 削除修正で兄弟判定用に
        let mut x_was_left_child = false;

        // subtree_size, unique_size の更新を開始するノード
        let count_start_node: usize;

        let z_left: usize = self.node(z).left;
        let z_right: usize = self.node(z).right;

        if z_left == RBNIL {
            // Case A:
            // zに左部分木がない
            // zを右の子に置き換える
            x = z_right;
            x_parent = self.node(z).parent;
            x_was_left_child = x_parent != RBNIL && self.node(x_parent).left == z;

            // zの位置にz_rightを接続
            self.transplant(z, z_right);

            // zの親から上の部分木サイズが変わる
            count_start_node = x_parent;
        } else if z_right == RBNIL {
            // Case B:
            // zに右部分木がない
            // zを左の子で置き換える
            x = z_left;
            x_parent = self.node(z).parent;
            x_was_left_child = x_parent != RBNIL && self.node(x_parent).left == z;

            // zの位置にz_leftを接続
            self.transplant(z, z_left);

            // zの親から上の部分木サイズが変わる
            count_start_node = x_parent;
        } else {
            // Case C:
            // zに左右両方の子がある
            // zの右部分木の最小ノードを後継ノードyとして使う
            y = self.first_index(z_right);
            y_original_color = self.node(y).color;

            // yは右部分木の最小, 左の子は持たない
            // yを外した後, yの右の子 x が y の元位置へ入る
            x = self.node(y).right;

            // yを元の位置からはず前の親
            let old_y_parent: usize = self.node(y).parent;

            if old_y_parent == z {
                // yがzの直下の右の子である場合
                // xの親はzの位置に移動するyになる
                x_parent = y;
                x_was_left_child = false;

                if x != RBNIL {
                    self.node_mut(x).parent = y;
                }
            } else {
                // yがzの右部分木のさらに下にある場合
                // yはold_y_parentの左の子として存在している
                x_parent = old_y_parent;
                x_was_left_child = true;

                // yの元位置をyの右の子で置き換える
                let y_right: usize = self.node(y).right;
                self.transplant(y, y_right);

                // yをzの位置へ移す準備として, zの右部分木をyの右へ接続
                self.node_mut(y).right = z_right;
                if z_right != RBNIL {
                    self.node_mut(z_right).parent = y;
                }
            }

            // zの位置にyを接続
            self.transplant(z, y);

            // zの左部分木をyの左へ接続
            self.node_mut(y).left = z_left;
            if z_left != RBNIL {
                self.node_mut(z_left).parent = y;
            }

            // yはzの位置を引き継ぐので, 色もzの色を引き継ぐ
            let z_color: NodeColor = self.node(z).color;
            self.node_mut(y).color = z_color;

            // yの子が変わったので, yの部分木サイズを再計算する
            self.update_size(y);

            // yを元の位置から外した場合, old_y_parent 側のサイズも縮んでいる
            if old_y_parent != RBNIL && old_y_parent != z {
                self.update_size_to_root(old_y_parent);
            }

            // 最終的にはyの位置から根までのサイズを更新
            count_start_node = y;
        }

        // ノード自体が1つ消えるので, ユニークキー数を減らす
        self.unique_count -= 1;

        // 構造変化によるsubtree_size, unique_sizeの変化を反映
        self.update_size_to_root(count_start_node);

        // 黒ノードを実際に取り外した場合, 黒の高さが崩れる可能性
        // xに黒不足が乗っているとみなして, 削除後修正
        if y_original_color == NodeColor::Black {
            self.fix_after_remove(x, x_parent, x_was_left_child);
        }

        // 回転を伴う修正の後に, 影響がありそうな場所から再度サイズを更新
        if x != RBNIL {
            self.update_size_to_root(x);
        } else {
            self.update_size_to_root(x_parent);
        }

        // first / last を削除した場合、削除前に保存した隣接ノードへ更新
        if self.first == z {
            self.first = next_of_z;
        }
        if self.last == z {
            self.last = prev_of_z;
        }

        // 木が空になった場合, ショートカット類も空に
        if self.root == RBNIL {
            self.first = RBNIL;
            self.last = RBNIL;
            self.unique_count = 0;
        }

        // 最後にzのスロットをFreeに
        // ここでzが持っていたkeyはdrop
        self.free_node(z);
    } // end of method

    /// u の位置を v で置き換える
    /// v が RBNIL の場合, u の位置を空にする
    #[inline(always)]
    fn transplant(&mut self, u: usize, v: usize) {
        let parent: usize = self.node(u).parent;

        if parent == RBNIL {
            self.root = v;
        } else if self.node(parent).left == u {
            self.node_mut(parent).left = v;
        } else {
            self.node_mut(parent).right = v;
        }

        if v != RBNIL {
            self.node_mut(v).parent = parent;
        }
    } // end of method

    /// 削除後の赤黒木修正
    /// x が RBNILL の場合でも, その仮想null葉の親と左右位置を受け取る
    /// 黒ノードの削除で不足した黒を, 回転と色変更で解消
    #[inline(always)]
    fn fix_after_remove(&mut self, mut x: usize, mut parent: usize, mut x_was_left_child: bool) {
        // xが根に到達する or xが赤になるまで黒を上に押し上げる
        // RBNILは黒ノード
        while x != self.root && self.is_black(x) {
            // xがRBNILLなら, parentがないと兄弟を決められない
            // parentがRBNILなら修正を続けられないので終了
            if parent == RBNIL {
                break;
            }

            // xが親の左側?
            // xがRBNILLなら, 削除前に保存した x_was_left_child
            let is_left: bool = if x != RBNIL { self.node(parent).left == x } else { x_was_left_child };

            if is_left {
                // xが左側になる場合, 兄弟はparentの右の子
                let mut sibling: usize = self.node(parent).right;

                // Case 1:
                // 兄弟が赤
                // 兄弟を黒, 親を赤に左回転すると, 兄弟が黒のケースに変換できる
                if self.is_red(sibling) {
                    self.node_mut(sibling).color = NodeColor::Black;
                    self.node_mut(parent).color = NodeColor::Red;
                    self.rotate_left(parent);

                    // 回転後, xの新しい兄弟を取り直す
                    sibling = self.node(parent).right;
                }

                // 兄弟の左右の子を取得
                // 兄弟がRBNILの場合, その子もRBNIL
                let sibling_left = if sibling == RBNIL { RBNIL } else { self.node(sibling).left };
                let sibling_right = if sibling == RBNIL { RBNIL } else { self.node(sibling).right };

                if self.is_black(sibling_left) && self.is_black(sibling_right) {
                    // Case 2:
                    // 兄弟も兄弟の子も黒
                    // 兄弟を赤にして, 黒不足を parent へ押し上げる
                    if sibling != RBNIL {
                        self.node_mut(sibling).color = NodeColor::Red;
                    }

                    x = parent;
                    parent = self.node(parent).parent;
                    x_was_left_child = parent != RBNIL && self.node(parent).left == x;
                } else {
                    // Case 3:
                    // 兄弟は黒, 近い子が赤, 遠い子が黒
                    // 兄弟を右回転して Case4: に変換
                    if self.is_black(sibling_right) {
                        if sibling != RBNIL {
                            let left: usize = self.node(sibling).left;
                            if left != RBNIL {
                                self.node_mut(left).color = NodeColor::Black;
                            }

                            self.node_mut(sibling).color = NodeColor::Red;
                            self.rotate_right(sibling);
                        }
                        sibling = self.node(parent).right;
                    }

                    // Case 4:
                    // 兄弟は黒, 遠い子が赤
                    // 親の色を兄弟へ渡し, 親と遠い子を黒にして左回転
                    if sibling != RBNIL {
                        let parent_color: NodeColor = self.node(parent).color;
                        self.node_mut(sibling).color = parent_color;
                    }

                    self.node_mut(parent).color = NodeColor::Black;

                    if sibling != RBNIL {
                        let right: usize = self.node(sibling).right;
                        if right != RBNIL {
                            self.node_mut(right).color = NodeColor::Black;
                        }
                    }

                    self.rotate_left(parent);

                    // 黒不足は解消, ループを終えるための root を入れる
                    x = self.root;
                    parent = RBNIL;
                }
            } else {
                // xが右側にある場合
                // 左右を反転した対称ケースを処理
                let mut sibling = self.node(parent).left;

                // Case 1の対称:
                // 兄弟が赤なら, 兄弟を黒, 親を赤にして右回転
                if self.is_red(sibling) {
                    self.node_mut(sibling).color = NodeColor::Black;
                    self.node_mut(parent).color = NodeColor::Red;
                    self.rotate_right(parent);
                    sibling = self.node(parent).left;
                }

                // 兄弟の左右の子を取得
                let sibling_left: usize = if sibling == RBNIL { RBNIL } else { self.node(sibling).left };
                let sibling_right: usize = if sibling == RBNIL { RBNIL } else { self.node(sibling).right };

                if self.is_black(sibling_left) && self.is_black(sibling_right) {
                    // Case 2の対称:
                    // 兄弟と兄弟の子が黒なら, 兄弟を赤にして黒不足を parent へ押し上げる
                    if sibling != RBNIL {
                        self.node_mut(sibling).color = NodeColor::Red;
                    }

                    x = parent;
                    parent = self.node(parent).parent;
                    x_was_left_child = parent != RBNIL && self.node(parent).left == x;
                } else {
                    // Case 3の対称:
                    // 近い子が赤, 遠い子が黒なら, 兄弟を左回転して Case4 へ変換
                    if self.is_black(sibling_left) {
                        if sibling != RBNIL {
                            let right: usize = self.node(sibling).right;
                            if right != RBNIL {
                                self.node_mut(right).color = NodeColor::Black;
                            }

                            self.node_mut(sibling).color = NodeColor::Red;
                            self.rotate_left(sibling);
                        }

                        sibling = self.node(parent).left;
                    }

                    // Case 4の対称:
                    // 親の色を兄弟へ渡し, 親と遠い子を黒にして右回転
                    if sibling != RBNIL {
                        let parent_color: NodeColor = self.node(parent).color;
                        self.node_mut(sibling).color = parent_color;
                    }

                    self.node_mut(parent).color = NodeColor::Black;

                    if sibling != RBNIL {
                        let left: usize = self.node(sibling).left;
                        if left != RBNIL {
                            self.node_mut(left).color = NodeColor::Black;
                        }
                    }

                    self.rotate_right(parent);

                    // 黒不足は解消したので, ループを終えるための root を入れる
                    x = self.root;
                    parent = RBNIL;
                }
            }
        }

        // xが赤だった場合は, 黒にすることで不足分を吸収
        if x != RBNIL {
            self.node_mut(x).color = NodeColor::Black;
        }

        // 根は常に黒
        if self.root != RBNIL {
            self.node_mut(self.root).color = NodeColor::Black;
        }
    } // end of method

    /// 指定部分木の最小ノード添字を返す
    /// indexがRBNILの場合はRBNIL
    #[inline(always)]
    fn first_index(&self, mut index: usize) -> usize {
        if index == RBNIL {
            return RBNIL;
        }

        while self.node(index).left != RBNIL {
            index = self.node(index).left;
        }
        return index;
    } // end of method

    /// 指定部分木の最大ノード添字を返す
    /// indexがRBNILの場合はRBNIL
    #[inline(always)]
    fn last_index(&self, mut index: usize) -> usize {
        if index == RBNIL {
            return RBNIL;
        }

        while self.node(index).right != RBNIL {
            index = self.node(index).right;
        }
        return index;
    } // end of method

    /// 中間順で次のノード添字を返す
    /// 存在しなければ RBNIL
    #[inline(always)]
    fn next_index(&self, index: usize) -> usize {
        let right: usize = self.node(index).right;
        if right != RBNIL {
            return self.first_index(right);
        }

        let mut current: usize = index;
        let mut parent: usize = self.node(current).parent;

        while parent != RBNIL {
            if self.node(parent).left == current {
                return parent;
            }

            current = parent;
            parent = self.node(current).parent;
        }

        return RBNIL;
    } // end of method

    /// 中間順で前のノード添字を返す
    /// 存在しなければ RBNIL
    #[inline(always)]
    fn prev_index(&self, index: usize) -> usize {
        let left: usize = self.node(index).left;
        if left != RBNIL {
            return self.last_index(left);
        }

        let mut current: usize = index;
        let mut parent: usize = self.node(current).parent;

        while parent != RBNIL {
            if self.node(parent).right == current {
                return parent;
            }

            current = parent;
            parent = self.node(current).parent;
        }

        return RBNIL;
    } // end of method

    /// lower_boundの添字版
    /// 見つからなければRBNIL
    #[inline(always)]
    fn lower_bound_index(&self, key: &T) -> usize {
        let mut current: usize = self.root;
        let mut candidate: usize = RBNIL;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less | Ordering::Equal => {
                    candidate = current;
                    current = node.left;
                }
                Ordering::Greater => {
                    current = node.right;
                }
            }
        }

        return candidate;
    } // end of method

    /// upper_boundの添字版
    /// 見つからなければRBNILL
    #[inline(always)]
    fn upper_bound_index(&self, key: &T) -> usize {
        let mut current: usize = self.root;
        let mut candidate: usize = RBNIL;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    candidate = current;
                    current = node.left;
                }
                Ordering::Equal | Ordering::Greater => {
                    current = node.right;
                }
            }
        }

        return candidate;
    } // end of method

    /// key未満の最大ノード添字を返す
    /// 見つからなければRBNIL
    #[inline(always)]
    fn less_than_max_index(&self, key: &T) -> usize {
        let mut current: usize = self.root;
        let mut candidate = RBNIL;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less | Ordering::Equal => {
                    current = node.left;
                }
                Ordering::Greater => {
                    candidate = current;
                    current = node.right;
                }
            }
        }
        return candidate;
    } // end of method

    /// key以下の最大の添字を返す
    /// 見つからなければRBNIL
    #[inline(always)]
    fn less_than_or_equal_max_index(&self, key: &T) -> usize {
        let mut current: usize = self.root;
        let mut candidate: usize = RBNIL;

        while current != RBNIL {
            let node: &RBTreeNode<T> = self.node(current);
            match (self.compare)(key, &node.key) {
                Ordering::Less => {
                    current = node.left;
                }
                Ordering::Equal | Ordering::Greater => {
                    candidate = current;
                    current = node.right;
                }
            }
        }
        return candidate;
    } // end of method
} // end of impl RBTreeMultiSet<T,F>

impl<T> RBTreeMultiSet<T, fn(&T, &T) -> Ordering>
where
    T: Ord,
{
    /// Ordのcmpを使って空の木を作る
    #[inline(always)]
    pub fn new() -> Self {
        fn compare_ord<T>(left: &T, right: &T) -> Ordering
        where
            T: Ord,
        {
            return left.cmp(right);
        }

        return Self::with_compare(compare_ord::<T>);
    } // end of method

    /// Ordのcmpを使って,容量を予約した空の木を作る
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        fn compare_ord<T>(left: &T, right: &T) -> Ordering
        where
            T: Ord,
        {
            return left.cmp(right);
        }

        return Self::with_capacity_and_compare(capacity, compare_ord::<T>);
    } // end of method
} // end of impl RBTreeMultiSet

impl<T> Default for RBTreeMultiSet<T, fn(&T, &T) -> Ordering>
where
    T: Ord,
{
    /// DefaultではOrdのcmp
    #[inline(always)]
    fn default() -> Self {
        return Self::new();
    } // end of method
} // end of impl Default for RBTreeMultiSet
