// AtCoder Library Practice Contest K - Range Affine Range Sum
// https://atcoder.jp/contests/practice2/tasks/practice2_k
// 反復型LazySegmentTreeMonoid版。標準ライブラリのみで動作する提出用1ファイル。
// 比較用の入出力処理・剰余計算・テスト入力はもう一方の版と同じ。
use std::fmt::Write as FmtWrite;
use std::io::{self, Read, Write as IoWrite};

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
pub trait Monoid {
    type S;
    fn identity(&self) -> Self::S;
    fn combine(&self, left: &Self::S, right: &Self::S) -> Self::S;
} // end of trait Monoid
  // 木本体: Utility_Rust/src/lazy_segment_tree_monoid.rsから抽出。
pub struct LazySegmentTreeMonoid<M, F, Map>
where
    M: Monoid,
    F: Monoid,
{
    original_len: usize,
    leaf_len: usize,
    log: u32,
    node: Vec<M::S>,
    lazy: Vec<Option<F::S>>,
    value_monoid: M,
    update_monoid: F,
    mapping: Map,
} // end of struct LazySegmentTreeMonoid
impl<M, F, Map> LazySegmentTreeMonoid<M, F, Map>
where
    M: Monoid,
    F: Monoid,
    M::S: Clone,
    F::S: Clone,
    Map: Fn(&F::S, &mut M::S),
{
    pub fn new(values: &[M::S], value_monoid: M, update_monoid: F, mapping: Map) -> Self {
        let n = values.len();
        let size = n.max(1).checked_next_power_of_two().expect("配列が大きすぎるよー(´・ω・｀)");
        let capacity = size.checked_mul(2).expect("配列が大きすぎるよー(´・ω・｀)");
        let mut node = Vec::with_capacity(capacity);
        node.extend((0..capacity).map(|_| value_monoid.identity()));
        node[size..size + n].clone_from_slice(values);
        for k in (1..size).rev() {
            node[k] = value_monoid.combine(&node[k << 1], &node[k << 1 | 1]);
        }
        let lazy = (0..size).map(|_| None).collect();
        return Self {
            original_len: n,
            leaf_len: size,
            log: size.trailing_zeros(),
            node,
            lazy,
            value_monoid,
            update_monoid,
            mapping,
        };
    } // end of constructor
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.original_len;
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        return self.original_len == 0;
    }
    #[inline(always)]
    pub fn all(&self) -> &M::S {
        return &self.node[1];
    }
    #[inline(always)]
    pub fn get(&mut self, index: usize) -> &M::S {
        assert!(index < self.original_len, "添字が範囲外だよー(´・ω・｀)");
        let k = self.leaf_len + index;
        for h in (1..=self.log).rev() {
            self.push(k >> h);
        }
        return &self.node[k];
    }
    pub fn set(&mut self, index: usize, value: M::S) {
        assert!(index < self.original_len, "添字が範囲外だよー(´・ω・｀)");
        let k = self.leaf_len + index;
        for h in (1..=self.log).rev() {
            self.push(k >> h);
        }
        self.node[k] = value;
        for h in 1..=self.log {
            self.pull(k >> h);
        }
    }
    #[inline(always)]
    pub fn update(&mut self, left: usize, right: usize, f: F::S) {
        assert!(left <= right && right <= self.original_len, "区間範囲外だよー(´・ω・｀)");
        returnif!(left >= right);
        let mut l = left + self.leaf_len;
        let mut r = right + self.leaf_len;
        self.push_boundaries(l, r);
        let (l0, r0) = (l, r);
        while l < r {
            if (l & 1) != 0 {
                self.apply(l, &f);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                self.apply(r, &f);
            }
            l >>= 1;
            r >>= 1;
        }
        for h in l0.trailing_zeros() + 1..=self.log {
            self.pull(l0 >> h);
        }
        for h in r0.trailing_zeros() + 1..=self.log {
            self.pull((r0 - 1) >> h);
        }
    }
    #[inline(always)]
    pub fn query(&mut self, left: usize, right: usize) -> M::S {
        assert!(left <= right && right <= self.original_len, "区間範囲外だよー(´・ω・｀)");
        returnif!(left >= right, self.value_monoid.identity());
        let mut l = left + self.leaf_len;
        let mut r = right + self.leaf_len;
        self.push_boundaries(l, r);
        let mut a = self.value_monoid.identity();
        let mut b = self.value_monoid.identity();
        while l < r {
            if (l & 1) != 0 {
                a = self.value_monoid.combine(&a, &self.node[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                b = self.value_monoid.combine(&self.node[r], &b);
            }
            l >>= 1;
            r >>= 1;
        }
        return self.value_monoid.combine(&a, &b);
    }
    #[inline(always)]
    pub fn as_slice(&mut self) -> &[M::S] {
        self.push_all();
        return &self.node[self.leaf_len..self.leaf_len + self.original_len];
    }
    #[inline(always)]
    pub fn to_vec(&mut self) -> Vec<M::S>
    where
        M::S: Clone,
    {
        return self.as_slice().to_vec();
    }
    pub fn into_vec(mut self) -> Vec<M::S> {
        self.push_all();
        drop(self.node.drain(..self.leaf_len));
        self.node.truncate(self.original_len);
        return self.node;
    }
    #[inline(always)]
    fn pull(&mut self, k: usize) {
        self.node[k] = self.value_monoid.combine(&self.node[k << 1], &self.node[k << 1 | 1]);
    }
    #[inline(always)]
    fn apply(&mut self, k: usize, f: &F::S) {
        (self.mapping)(f, &mut self.node[k]);
        if k < self.leaf_len {
            let next = match &self.lazy[k] {
                Some(old) => self.update_monoid.combine(f, old),
                None => self.update_monoid.combine(f, &self.update_monoid.identity()),
            };
            self.lazy[k] = Some(next);
        }
    }
    #[inline(always)]
    fn apply_owned(&mut self, k: usize, f: F::S) {
        (self.mapping)(&f, &mut self.node[k]);
        if k < self.leaf_len {
            self.lazy[k] = Some(match self.lazy[k].take() {
                Some(old) => self.update_monoid.combine(&f, &old),
                None => f,
            });
        }
    }
    #[inline(always)]
    fn push(&mut self, k: usize) {
        if let Some(f) = self.lazy[k].take() {
            self.apply(k << 1, &f);
            self.apply_owned(k << 1 | 1, f); // 片方の子どには更新の所有権を移動できる
        }
    }
    #[inline(always)]
    fn push_boundaries(&mut self, l: usize, r: usize) {
        for h in (l.trailing_zeros() + 1..=self.log).rev() {
            self.push(l >> h);
        }
        for h in (r.trailing_zeros() + 1..=self.log).rev() {
            self.push((r - 1) >> h);
        }
    }
    #[inline(always)]
    fn push_all(&mut self) {
        for k in 1..self.leaf_len {
            self.push(k);
        }
    }
}

struct SumMonoid;
impl Monoid for SumMonoid {
    type S = Value;

    #[inline(always)]
    fn identity(&self) -> Value {
        Value {
            sum: 0,
            count: 0,
        }
    }

    #[inline(always)]
    fn combine(&self, left: &Value, right: &Value) -> Value {
        join(left, right)
    }
}

struct AffineMonoid;
impl Monoid for AffineMonoid {
    type S = Affine;

    #[inline(always)]
    fn identity(&self) -> Affine {
        Affine {
            mul: 1,
            add: 0,
        }
    }

    #[inline(always)]
    fn combine(&self, new: &Affine, old: &Affine) -> Affine {
        compose(new, old)
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
    let mut seg = LazySegmentTreeMonoid::new(&values, SumMonoid, AffineMonoid, apply);
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
