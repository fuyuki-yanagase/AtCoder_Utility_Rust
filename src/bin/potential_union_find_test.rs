// 非可換ポテンシャルUnion-Findと制約グラフの独立比較。
#[cfg(test)]
use std::collections::VecDeque;
use utility_rust::monoid_group_abelian::{FnGroup, Group};
use utility_rust::union_find::PotentialUnionFind;

type Permutation = [usize; 3];
const IDENTITY: Permutation = [0, 1, 2];
const ELEMENTS: [Permutation; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

fn compose(a: &Permutation, b: &Permutation) -> Permutation {
    [a[b[0]], a[b[1]], a[b[2]]]
}

fn inverse(a: &Permutation) -> Permutation {
    std::array::from_fn(|i| a.iter().position(|&x| x == i).unwrap())
}

fn group() -> impl Group<S = Permutation> {
    FnGroup::new(|| IDENTITY, compose, inverse)
}

// 木の根・親・重みを使わず、受理した辺をBFSでたどる。
// 辺 v→u は a[u] = a[v] * x を表すので、道上の関係は右へ追加する。
#[cfg(test)]
fn relations(graph: &[Vec<(usize, Permutation)>], start: usize) -> Vec<Option<Permutation>> {
    let mut result = vec![None; graph.len()];
    let mut queue = VecDeque::from([start]);
    result[start] = Some(IDENTITY);
    while let Some(v) = queue.pop_front() {
        for &(u, relation) in &graph[v] {
            if result[u].is_none() {
                result[u] = Some(compose(&result[v].unwrap(), &relation));
                queue.push_back(u);
            }
        }
    }
    result
}

fn main() {
    let mut uf = PotentialUnionFind::new(2, group());
    assert!(uf.unite(0, 1, ELEMENTS[3]));
    assert_eq!(uf.diff(0, 1), Some(ELEMENTS[3]));
}

#[test]
fn permutation_constraints_match_bfs_after_accepted_and_rejected_edges() {
    let n = 11;
    let mut uf = PotentialUnionFind::new(n, group());
    let mut graph = vec![vec![]; n];
    let mut seed = 20261028_u64;
    let mut accepted = 0;
    let mut rejected = 0;
    // 単独成分、同サイズ、大小両方向の併合、閉路、自己制約を混ぜる。
    let initial = [(0, 1), (2, 3), (4, 5), (6, 7), (0, 2), (6, 8), (4, 6), (0, 4), (9, 0), (10, 10)];
    for step in 0..180 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let (u, v) =
            if step < initial.len() { initial[step] } else { ((seed >> 16) as usize % n, (seed >> 32) as usize % n) };
        let x = ELEMENTS[(seed >> 48) as usize % ELEMENTS.len()];
        let before = relations(&graph, v)[u];
        let expected = before.is_none_or(|known| known == x);
        assert_eq!(uf.unite(u, v, x), expected, "step={step}, u={u}, v={v}, x={x:?}");
        if expected {
            accepted += 1;
            graph[v].push((u, x));
            graph[u].push((v, inverse(&x)));
        } else {
            rejected += 1;
        }
        for source in 0..n {
            let expected = relations(&graph, source);
            assert_eq!(uf.size(source), expected.iter().flatten().count());
            let root = uf.root(source);
            assert_eq!(uf.potential(source), relations(&graph, root)[source].unwrap());
            for (target, &_) in expected.iter().enumerate().take(n) {
                // 矛盾拒否後も、全関係をそのまま維持することを検証する。
                assert_eq!(
                    uf.diff(target, source),
                    expected[target],
                    "step={step}, source={source}, target={target}"
                );
                assert_eq!(uf.connected(target, source), expected[target].is_some());
            }
        }
        let mut expected_groups = Vec::new();
        let mut seen = vec![false; n];
        for source in 0..n {
            if seen[source] {
                continue;
            }
            let component: Vec<_> = relations(&graph, source)
                .iter()
                .enumerate()
                .filter_map(|(v, value)| value.is_some().then_some(v))
                .collect();
            for &v in &component {
                seen[v] = true;
            }
            expected_groups.push(component);
        }
        assert_eq!(uf.groups(), expected_groups);
    }
    assert!(accepted > 10);
    assert!(rejected > 10);
}
