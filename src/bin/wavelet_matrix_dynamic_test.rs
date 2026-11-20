use utility_rust::in_out::{FastScanner, FastWriter};
use utility_rust::wavelet_matrix_dynamic::DynamicWaveletMatrix;

fn example() {
    let mut wm = DynamicWaveletMatrix::with_value_range(&[5, -1, 5, 2], -1, 10);
    assert_eq!(wm.set(1, 7), Some(-1));
    wm.insert(2, 3);
    assert_eq!(wm.remove(4), Some(2));
    // [5, 7, 3, 5]
    assert_eq!(wm.get(1), Some(7));
    assert_eq!(wm.kth_smallest(0, 4, 0), Some(3));
    assert_eq!(wm.kth_largest_with_index(0, 4, 2), Some((5, 3)));
    assert_eq!(wm.range_freq(0, 4, 3, 6), 3);
    assert_eq!(wm.occurrences(0, 4, 5), 2);
    assert_eq!(wm.select(0, 4, 5, 1), Some(3));

    let mut online = DynamicWaveletMatrix::new(&[]);
    online.insert(0, i64::MIN);
    assert_eq!(online.set(0, i64::MAX), Some(i64::MIN));
    let mut compressed = DynamicWaveletMatrix::with_universe(&[9, 9, 1], &[-100, 1000]);
    assert_eq!(compressed.set(0, -100), Some(9));
    assert_eq!(compressed.max(0, 3), Some(9));
    println!("動的ウェーブレット行列の使用例: 成功");
}

#[cfg(test)]
fn can_make_monochrome(wm: &DynamicWaveletMatrix, l: usize, r: usize) -> bool {
    let Some(maximum) = wm.max(l, r) else { return true };
    // 問題の値は -1 または正数。区間の正数が全て最大値と同じならよい。
    maximum == -1 || wm.occurrences(l, r, -1) + wm.occurrences(l, r, maximum) == r - l
}

/// ABC484 F 用。各ブロックを「全て -1」「正数が1種類」「正数が2種類以上」に要約する。
/// 入力に現れない 0 を、正数が2種類以上あったときの印に使う。
struct MonochromeBlocks {
    values: Vec<i64>,
    kinds: Vec<i64>,
    wavelet: DynamicWaveletMatrix,
}

impl MonochromeBlocks {
    const SIZE: usize = 64;

    fn combine(a: i64, b: i64) -> i64 {
        if a == -1 {
            b
        } else if b == -1 || a == b {
            a
        } else {
            0
        }
    }

    fn summarize(values: &[i64]) -> i64 {
        let mut kind = -1;
        for &value in values {
            kind = Self::combine(kind, value);
            if kind == 0 {
                break;
            }
        }
        kind
    }

    fn new(values: Vec<i64>) -> Self {
        let kinds: Vec<_> = values.chunks(Self::SIZE).map(Self::summarize).collect();
        let wavelet = DynamicWaveletMatrix::with_value_range(&kinds, -1, 1_000_000);
        Self {
            values,
            kinds,
            wavelet,
        }
    }

    fn set(&mut self, i: usize, x: i64) {
        self.values[i] = x;
        let block = i / Self::SIZE;
        let start = block * Self::SIZE;
        let end = (start + Self::SIZE).min(self.values.len());
        let kind = Self::summarize(&self.values[start..end]);
        if kind != self.kinds[block] {
            self.wavelet.set(block, kind);
            self.kinds[block] = kind;
        }
    }

    fn query(&self, l: usize, r: usize) -> bool {
        let first_full = l.div_ceil(Self::SIZE);
        let past_full = r / Self::SIZE;
        if first_full >= past_full {
            return Self::summarize(&self.values[l..r]) != 0;
        }
        let ends = Self::combine(
            Self::summarize(&self.values[l..first_full * Self::SIZE]),
            Self::summarize(&self.values[past_full * Self::SIZE..r]),
        );
        if ends == 0 {
            return false;
        }
        let maximum = self.wavelet.max(first_full, past_full).unwrap().max(ends);
        if maximum <= 0 {
            return maximum == -1;
        }
        if ends != -1 && ends != maximum {
            return false;
        }
        // 途中のブロックは、空白だけか、最大値1種類だけである必要がある。
        let blanks = self.wavelet.occurrences(first_full, past_full, -1);
        let matching = self.wavelet.occurrences(first_full, past_full, maximum);
        blanks + matching == past_full - first_full
    }
}

fn solve_abc484_f() {
    let mut sc = FastScanner::new();
    let mut out = FastWriter::new();
    let (n, q) = sc.readusizet2();
    let mut blocks = MonochromeBlocks::new(sc.readlongs(n));
    for _ in 0..q {
        let t = sc.readusize();
        let l = sc.readusize() - 1;
        if t == 1 {
            blocks.set(l, sc.readlong());
        } else {
            let r = sc.readusize(); // 1始まり閉区間の右端は0始まり半開区間でも同じ値
            out.writeline(if blocks.query(l, r) { "Yes" } else { "No" });
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--abc484-f") {
        solve_abc484_f();
    } else {
        example();
    }
}

#[test]
fn usage_example() {
    example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;

    fn random(seed: &mut u64) -> usize {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed as usize
    }

    fn verify(wm: &DynamicWaveletMatrix, a: &[i64], l: usize, r: usize, x: i64, k: usize) {
        let mut sorted: Vec<_> = (l..r).map(|i| (a[i], i)).collect();
        sorted.sort_unstable();
        assert_eq!(wm.kth_smallest_with_index(l, r, k), sorted.get(k).copied());
        assert_eq!(wm.kth_smallest(l, r, k), sorted.get(k).map(|e| e.0));
        sorted.sort_unstable_by_key(|&(v, i)| (Reverse(v), i));
        assert_eq!(wm.kth_largest_with_index(l, r, k), sorted.get(k).copied());
        assert_eq!(wm.kth_largest(l, r, k), sorted.get(k).map(|e| e.0));
        let slice = &a[l..r];
        let positions: Vec<_> = (l..r).filter(|&i| a[i] == x).collect();
        assert_eq!(wm.occurrences(l, r, x), positions.len());
        assert_eq!(wm.rank(r, x), a[..r].iter().filter(|&&v| v == x).count());
        for occurrence in 0..=positions.len() {
            assert_eq!(wm.select(l, r, x, occurrence), positions.get(occurrence).copied());
        }
        assert_eq!(wm.count_lt(l, r, x), slice.iter().filter(|&&v| v < x).count());
        assert_eq!(wm.count_le(l, r, x), slice.iter().filter(|&&v| v <= x).count());
        assert_eq!(
            wm.range_freq(l, r, x, i64::MAX),
            slice.iter().filter(|&&v| x <= v && v < i64::MAX).count()
        );
        assert_eq!(wm.prev(l, r, x), slice.iter().copied().filter(|&v| v < x).max());
        assert_eq!(wm.next(l, r, x), slice.iter().copied().filter(|&v| x <= v).min());
    }

    fn exercise(mut wm: DynamicWaveletMatrix, mut a: Vec<i64>, pool: &[i64]) {
        let mut seed = 0x39835c64a07d331f_u64;
        for step in 0..5_000 {
            let x = pool[random(&mut seed) % pool.len()];
            let idx = random(&mut seed) % (a.len() + 1);
            match random(&mut seed) % 4 {
                0 => {
                    wm.insert(idx, x);
                    a.insert(idx, x);
                }
                1 if idx < a.len() => assert_eq!(wm.remove(idx), Some(a.remove(idx))),
                2 if idx < a.len() => assert_eq!(wm.set(idx, x), Some(std::mem::replace(&mut a[idx], x))),
                _ => assert_eq!(wm.get(idx), a.get(idx).copied()),
            }
            assert_eq!(wm.len(), a.len());
            assert_eq!(wm.is_empty(), a.is_empty());
            if step % 37 == 0 || step < 64 {
                let (p, q) = (random(&mut seed) % (a.len() + 1), random(&mut seed) % (a.len() + 1));
                let (l, r) = (p.min(q), p.max(q));
                let k = random(&mut seed) % (r - l + 1);
                verify(&wm, &a, l, r, x, k);
            }
        }
        for (i, &value) in a.iter().enumerate() {
            assert_eq!(wm.get(i), Some(value));
        }
        while let Some(value) = a.pop() {
            assert_eq!(wm.remove(a.len()), Some(value));
        }
        assert_eq!(wm.remove(0), None);
        assert_eq!(wm.set(0, pool[0]), None);
        wm.insert(0, pool[0]);
        assert_eq!(wm.get(0), Some(pool[0]));
    }

    #[test]
    fn mixed_operations_match_vec() {
        let pool = [-12, -1, 0, 1, 3, 9, 11];
        let initial: Vec<_> = (0..130).map(|i| pool[i % pool.len()]).collect();
        exercise(
            DynamicWaveletMatrix::with_value_range(&initial, -12, 11),
            initial.clone(),
            &pool,
        );
        exercise(DynamicWaveletMatrix::with_universe(&initial, &pool), initial.clone(), &pool);
        let extremes = [i64::MIN, -1, 0, 1, i64::MAX];
        exercise(DynamicWaveletMatrix::new(&[]), Vec::new(), &extremes);
        exercise(DynamicWaveletMatrix::with_value_range(&[], 1, 1), Vec::new(), &[1]);
    }

    #[test]
    fn repeatedly_split_and_shrink_packed_blocks() {
        let mut wm = DynamicWaveletMatrix::with_value_range(&[], 0, 255);
        let mut a = Vec::new();
        for i in 0..1500 {
            let idx = if i % 3 == 0 { a.len() } else { 0 };
            let value = (i % 256) as i64;
            wm.insert(idx, value);
            a.insert(idx, value);
        }
        // 全体から1つおきに消し、空にならないまま葉が疎になる過程を通す。
        for _ in 0..3 {
            for idx in (1..a.len()).step_by(2).rev() {
                assert_eq!(wm.remove(idx), Some(a.remove(idx)));
            }
            verify(&wm, &a, 0, a.len(), 13, a.len() / 2);
        }
        while a.len() > 2 {
            let idx = a.len() / 3;
            assert_eq!(wm.remove(idx), Some(a.remove(idx)));
            if a.len() % 63 == 0 {
                verify(&wm, &a, 0, a.len(), 13, a.len() / 2);
            }
        }
        verify(&wm, &a, 0, a.len(), 13, 0);
    }

    #[test]
    fn rejected_updates_preserve_the_sequence() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let mut wm = DynamicWaveletMatrix::with_universe(&[5, 9], &[]);
        assert!(catch_unwind(AssertUnwindSafe(|| wm.set(0, 8))).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| wm.insert(1, 7))).is_err());
        assert_eq!((wm.get(0), wm.get(1), wm.len()), (Some(5), Some(9), 2));
    }

    #[test]
    fn abc484_f_matches_brute_force_during_updates() {
        let mut a = vec![-1; 273];
        let mut wm = DynamicWaveletMatrix::with_value_range(&a, -1, 1_000_000);
        let mut blocks = MonochromeBlocks::new(a.clone());
        assert!(blocks.query(1, 273));
        for (i, x) in [(64, 9), (128, 9), (129, 1), (129, -1)] {
            blocks.set(i, x);
            assert_eq!(blocks.query(1, 273), i != 129 || x == -1);
        }
        let mut blocks = MonochromeBlocks::new(a.clone());
        let mut seed = 91_u64;
        for _ in 0..3000 {
            let i = random(&mut seed) % a.len();
            let x = [-1, -1, 1, 356, 1_000_000][random(&mut seed) % 5];
            a[i] = x;
            wm.set(i, x);
            blocks.set(i, x);
            let l = random(&mut seed) % a.len();
            let r = l + 1 + random(&mut seed) % (a.len() - l);
            let mut positives = a[l..r].iter().copied().filter(|&v| v > 0);
            let expected = positives.next().is_none_or(|first| positives.all(|v| v == first));
            assert_eq!(can_make_monochrome(&wm, l, r), expected);
            assert_eq!(blocks.query(l, r), expected);
        }
        assert!(can_make_monochrome(&wm, 0, 0));
    }
}
