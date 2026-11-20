// AtCoder Library Practice Contest K - Range Affine Range Sum
// https://atcoder.jp/contests/practice2/tasks/practice2_k
// 既存の再帰型LazySegmentTree版。標準ライブラリのみで動作する提出用1ファイル。
// 比較用の入出力処理・剰余計算・テスト入力はもう一方の版と同じ。
use std::fmt::Write as FmtWrite;
use std::io::{self, Read, Write as IoWrite};
use std::marker::PhantomData;

const MOD: u64 = 998244353;

#[derive(Clone, Copy)]
struct Value {
    sum: u64,
    count: u64,
}

#[derive(Clone, Copy, PartialEq)]
struct Affine {
    mul: u64,
    add: u64,
}

#[inline(always)]
fn join(left: &Value, right: &Value) -> Value {
    let sum = left.sum + right.sum;
    Value {
        sum: if sum >= MOD { sum - MOD } else { sum },
        count: left.count + right.count,
    }
}

#[inline(always)]
fn compose(new: &Affine, old: &Affine) -> Affine {
    Affine {
        mul: new.mul * old.mul % MOD,
        add: (new.mul * old.add + new.add) % MOD,
    }
}

#[inline(always)]
fn apply(f: &Affine, value: &mut Value) {
    // 制約内では2項の積和もu64に収まる。
    value.sum = (f.mul * value.sum + f.add * value.count) % MOD;
}

struct Scanner<'a> {
    input: &'a [u8],
    index: usize,
}

impl Scanner<'_> {
    #[inline(always)]
    fn next(&mut self) -> u64 {
        while self.index < self.input.len() && self.input[self.index] <= b' ' {
            self.index += 1;
        }
        let mut value = 0;
        while self.index < self.input.len() && self.input[self.index].is_ascii_digit() {
            value = value * 10 + (self.input[self.index] - b'0') as u64;
            self.index += 1;
        }
        value
    }
}
// 木本体: Utility_Rust/src/lazy_segment_tree.rsから抽出。
pub trait LazySegmentTreeOperator {
    type Value: Copy;
    type Lazy: Copy + PartialEq;
    fn identity() -> Self::Value;
    fn lazy_identity() -> Self::Lazy;
    fn operate(left: Self::Value, right: Self::Value) -> Self::Value;
    fn mapping(f: Self::Lazy, x: Self::Value) -> Self::Value;
    fn composition(new_f: Self::Lazy, current_f: Self::Lazy) -> Self::Lazy;
} // end of trait
pub struct LazySegmentTree<Op>
where
    Op: LazySegmentTreeOperator,
{
    original_len: usize,
    leaf_len: usize,
    node: Vec<Op::Value>,
    lazy: Vec<Op::Lazy>,
    _marker: PhantomData<Op>,
} // end of struct
impl<Op> LazySegmentTree<Op>
where
    Op: LazySegmentTreeOperator,
{
    pub fn new(values: &[Op::Value]) -> Self {
        let original_len: usize = values.len();
        let mut leaf_len: usize = 1_usize;
        while leaf_len < original_len {
            leaf_len <<= 1;
        }
        let mut node: Vec<Op::Value> = vec![Op::identity(); leaf_len << 1];
        let lazy: Vec<Op::Lazy> = vec![Op::lazy_identity(); leaf_len << 1];
        node[leaf_len..(leaf_len + original_len)].copy_from_slice(values);
        for i in (1..leaf_len).rev() {
            node[i] = Op::operate(node[i << 1], node[(i << 1) | 1]);
        }
        return Self {
            original_len,
            leaf_len,
            node,
            lazy,
            _marker: PhantomData,
        };
    } // end of constructor
    pub fn new_empty(size: usize) -> Self {
        let values: Vec<Op::Value> = vec![Op::identity(); size];
        return Self::new(&values);
    } // end of constructor
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.original_len;
    } // end of method
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.original_len == 0;
    } // end of method
    #[inline(always)]
    pub fn all(&self) -> Op::Value {
        return self.node[1];
    } // end of method
    #[inline(always)]
    fn apply_to_node(&mut self, k: usize, node_left: usize, node_right: usize, f: Op::Lazy) {
        self.node[k] = Op::mapping(f, self.node[k]);
        if node_right - node_left > 1 {
            self.lazy[k] = Op::composition(f, self.lazy[k]);
        }
    } // end of method
    #[inline(always)]
    fn push(&mut self, k: usize, node_left: usize, node_right: usize) {
        if self.lazy[k] == Op::lazy_identity() {
            return;
        }
        if node_right - node_left == 1 {
            self.lazy[k] = Op::lazy_identity();
            return;
        }
        let f: Op::Lazy = self.lazy[k];
        let mid: usize = (node_left + node_right) >> 1;
        self.apply_to_node(k << 1, node_left, mid, f);
        self.apply_to_node((k << 1) | 1, mid, node_right, f);
        self.lazy[k] = Op::lazy_identity();
    } // end of method
    #[inline(always)]
    fn pull(&mut self, k: usize) {
        self.node[k] = Op::operate(self.node[k << 1], self.node[(k << 1) | 1]);
    } // end of method
    #[inline(always)]
    pub fn update(&mut self, left: usize, right: usize, f: Op::Lazy) {
        debug_assert!(left <= right, "left must be less than or equal to right");
        debug_assert!(right <= self.original_len, "right is out of range");
        if left >= right {
            return;
        }
        self.update_internal(left, right, 1, 0, self.leaf_len, f);
    } // end of method
    #[inline(always)]
    fn update_internal(
        &mut self,
        update_left: usize,
        update_right: usize,
        k: usize,
        node_left: usize,
        node_right: usize,
        f: Op::Lazy,
    ) {
        if update_right <= node_left || node_right <= update_left {
            return;
        }
        if update_left <= node_left && node_right <= update_right {
            self.apply_to_node(k, node_left, node_right, f);
            return;
        }
        self.push(k, node_left, node_right);
        let mid: usize = (node_left + node_right) >> 1;
        self.update_internal(update_left, update_right, k << 1, node_left, mid, f);
        self.update_internal(update_left, update_right, (k << 1) | 1, mid, node_right, f);
        self.pull(k);
    } // end of method
    #[inline(always)]
    pub fn query(&mut self, left: usize, right: usize) -> Op::Value {
        debug_assert!(left <= right, "left must be less than or equal to right");
        debug_assert!(right <= self.original_len, "right is out of range");
        if left >= right {
            return Op::identity();
        }
        return self.query_internal(left, right, 1, 0, self.leaf_len);
    } // end of method
    #[inline(always)]
    fn query_internal(
        &mut self,
        query_left: usize,
        query_right: usize,
        k: usize,
        node_left: usize,
        node_right: usize,
    ) -> Op::Value {
        if query_right <= node_left || node_right <= query_left {
            return Op::identity();
        }
        if query_left <= node_left && node_right <= query_right {
            return self.node[k];
        }
        self.push(k, node_left, node_right);
        let mid: usize = (node_left + node_right) >> 1;
        let left_value: Op::Value = self.query_internal(query_left, query_right, k << 1, node_left, mid);
        let right_value: Op::Value = self.query_internal(query_left, query_right, (k << 1) | 1, mid, node_right);
        return Op::operate(left_value, right_value);
    } // end of method
    pub fn to_vec(&mut self) -> Vec<Op::Value> {
        self.push_all(1, 0, self.leaf_len);
        return self.node[self.leaf_len..(self.leaf_len + self.original_len)].to_vec();
    } // end of method
    fn push_all(&mut self, k: usize, node_left: usize, node_right: usize) {
        if node_right - node_left == 1 {
            return;
        }
        if self.original_len <= node_left {
            return;
        }
        self.push(k, node_left, node_right);
        let mid: usize = (node_left + node_right) >> 1;
        self.push_all(k << 1, node_left, mid);
        self.push_all((k << 1) | 1, mid, node_right);
    } // end of method
} // end of impl LazySegmentTree

struct AffineSum;

impl LazySegmentTreeOperator for AffineSum {
    type Value = Value;
    type Lazy = Affine;

    #[inline(always)]
    fn identity() -> Value {
        Value {
            sum: 0,
            count: 0,
        }
    }

    #[inline(always)]
    fn lazy_identity() -> Affine {
        Affine {
            mul: 1,
            add: 0,
        }
    }

    #[inline(always)]
    fn operate(left: Value, right: Value) -> Value {
        join(&left, &right)
    }

    #[inline(always)]
    fn mapping(f: Affine, mut value: Value) -> Value {
        apply(&f, &mut value);
        value
    }

    #[inline(always)]
    fn composition(new: Affine, old: Affine) -> Affine {
        compose(&new, &old)
    }
}

fn solve(input: &[u8]) -> String {
    let mut scanner = Scanner {
        input,
        index: 0,
    };
    let n = scanner.next() as usize;
    let q = scanner.next() as usize;
    let values: Vec<_> = (0..n)
        .map(|_| Value {
            sum: scanner.next(),
            count: 1,
        })
        .collect();
    let mut seg = LazySegmentTree::<AffineSum>::new(&values);
    let mut output = String::with_capacity(q * 10);
    for _ in 0..q {
        let kind = scanner.next();
        let l = scanner.next() as usize;
        let r = scanner.next() as usize;
        if kind == 0 {
            let f = Affine {
                mul: scanner.next(),
                add: scanner.next(),
            };
            seg.update(l, r, f);
        } else {
            writeln!(output, "{}", seg.query(l, r).sum).unwrap();
        }
    }
    output
}

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let output = solve(&input);
    io::stdout().lock().write_all(output.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_sample() {
        let input = b"5 7\n1 2 3 4 5\n1 0 5\n0 2 4 100 101\n1 0 3\n0 1 3 102 103\n1 2 5\n0 2 5 104 105\n1 0 5\n";
        assert_eq!(solve(input), "15\n404\n41511\n4317767\n");
    }

    #[test]
    fn largest_coefficients_and_singleton() {
        let input = b"1 5\n998244352\n1 0 1\n0 0 1 998244352 998244352\n1 0 1\n0 0 1 998244352 998244352\n1 0 1\n";
        assert_eq!(solve(input), "998244352\n0\n998244352\n");
    }

    #[test]
    fn randomized_input_against_u128_naive() {
        let mut state = 246813579_u64;
        let mut random = || {
            state ^= state << 7;
            state ^= state >> 9;
            state
        };
        for case in 0..100 {
            let n = 1 + random() as usize % 40;
            let q = 200;
            let mut values: Vec<_> = (0..n).map(|_| random() % MOD).collect();
            let mut input = format!("{n} {q}\n");
            for value in &values {
                write!(input, "{value} ").unwrap();
            }
            input.push('\n');
            let mut expected = String::new();
            for step in 0..q {
                let l = random() as usize % n;
                let r = l + 1 + random() as usize % (n - l);
                if step % 2 == 0 {
                    // b >= 1というAtCoderの制約を満たす入力だけを生成。
                    let b = 1 + random() % (MOD - 1);
                    let c = random() % MOD;
                    writeln!(input, "0 {l} {r} {b} {c}").unwrap();
                    for value in &mut values[l..r] {
                        *value = ((b as u128 * *value as u128 + c as u128) % MOD as u128) as u64;
                    }
                } else {
                    writeln!(input, "1 {l} {r}").unwrap();
                    let answer = values[l..r].iter().fold(0_u128, |sum, &v| (sum + v as u128) % MOD as u128);
                    writeln!(expected, "{answer}").unwrap();
                }
            }
            assert_eq!(solve(input.as_bytes()), expected, "case={case}");
        }
    }
}
