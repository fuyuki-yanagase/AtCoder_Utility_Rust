// 原本のモジュールを参照せず、依存を展開したファイルだけでAPIを利用する。

#[test]
fn coordinate_compression_preserves_indices_and_half_open_ranges() {
    use super::coordinate_compression_import::CoordinateCompression;
    let (table, indices) = CoordinateCompression::with_indices(&[30, -10, 30, 0]);
    assert_eq!(table.as_slice(), &[-10, 0, 30]);
    assert_eq!(indices, vec![2, 0, 2, 1]);
    assert_eq!(table.range(&-10, &30), 0..2);
    assert_eq!(table.range(&0, &0), 1..1);
    assert_eq!(table.get(indices[0]), Some(&30));
}

#[test]
fn dijkstra_uses_embedded_priority_queue() {
    use super::dijkstra_import::dijkstra;
    let graph = vec![vec![(1, 8), (2, 2)], vec![], vec![(1, 3)]];
    assert_eq!(dijkstra(&graph, 0), vec![0, 5, 2]);
}

#[test]
fn matrix_uses_embedded_monoid() {
    use super::matrix_import::{Matrix2, Matrix2Monoid, monoid_pow};
    let matrix = Matrix2::<101>::new(1, 1, 0, 1);
    assert_eq!(monoid_pow(&Matrix2Monoid::<101>, &matrix, 5), Matrix2::new(1, 5, 0, 1));
}

#[test]
fn potential_union_find_uses_embedded_group() {
    use super::union_find_import::{FnGroup, PotentialUnionFind};
    let group = FnGroup::new(|| 0_i64, |a, b| a + b, |a| -a);
    let mut uf = PotentialUnionFind::new(3, group);
    assert!(uf.unite(0, 1, 7));
    assert!(uf.unite(1, 2, 4));
    assert_eq!(uf.diff(0, 2), Some(11));
    assert!(!uf.unite(0, 2, 10));
}

#[test]
fn rerooting_uses_embedded_monoid() {
    use super::rerooting_zenhouigi_dp_import::{FnMonoid, Rerooting};
    let mut tree = Rerooting::new(3);
    tree.add_edge(0, 1, ());
    tree.add_edge(1, 2, ());
    let monoid = FnMonoid::new(|| 0_usize, |a, b| a + b);
    let counts = tree.solve(&monoid, |_, total| total + 1, |_, _, _, count| *count);
    assert_eq!(counts, vec![3; 3]);
}

#[test]
fn lazy_monoid_includes_reexported_operators_and_modint() {
    use super::lazy_segment_tree_monoid_import::{Affine, ModInt, RangeAssignSum, SumCount, lazy_from_operator};
    let values = [SumCount::new(1_i64, 1), SumCount::new(2, 1), SumCount::new(3, 1)];
    let mut tree = lazy_from_operator::<RangeAssignSum<i64>>(&values);
    tree.update(1, 3, Some(10));
    assert_eq!(tree.query(0, 3), SumCount::new(21, 3));
    assert_eq!(
        tree.into_vec(),
        vec![SumCount::new(1, 1), SumCount::new(10, 1), SumCount::new(10, 1)]
    );
    assert_eq!(Affine::<998244353>::new(2_u32, 3_u32).eval(ModInt::new(4_u32)).val(), 11);
}

#[test]
fn lazy_monoid_import_supports_rightmost_index() {
    use super::lazy_segment_tree_monoid_import::indexed::{MaxIndex, RangeAssignMaxIndex, Right};
    use super::lazy_segment_tree_monoid_import::lazy_from_operator;

    let values = [MaxIndex::new(9_i64, 0), MaxIndex::new(1, 1), MaxIndex::new(5, 2)];
    let mut tree = lazy_from_operator::<RangeAssignMaxIndex<i64, Right>>(&values);
    tree.update(0, 3, Some(4));
    let answer = tree.query(0, 3);
    assert_eq!((answer.value, answer.index), (4, 2));
}

#[test]
fn lazy_monoid_import_supports_indexed_top_bottom_with_independent_ties() {
    use super::lazy_segment_tree_monoid_import::indexed::{Left, Right};
    use super::lazy_segment_tree_monoid_import::{IndexedTopBottomMonoid, TopBottomUpdate};

    let mut tree = IndexedTopBottomMonoid::<i64, 2, 2, true, Right, Left>::build(&[9, 9, 1, 1]);
    tree.update(0, 4, TopBottomUpdate::Assign(3));
    tree.update(0, 4, TopBottomUpdate::Add(2));
    let result = tree.query(1, 4);
    assert_eq!(result.top().map(|e| (e.value, e.index)).collect::<Vec<_>>(), [(5, 3)]);
    assert_eq!(result.bottom().map(|e| (e.value, e.index)).collect::<Vec<_>>(), [(5, 1)]);
}

#[test]
fn modint_import_exposes_combinatorics_and_new_aliases() {
    use super::modint_import::{Combinatorics, ModInt107, ModInt998};

    let table = Combinatorics::<998244353>::new(6);
    assert_eq!(table.factorial(6), ModInt998::new(720));
    assert_eq!(table.combination(6, 3), ModInt998::new(20));
    let table = Combinatorics::<1000000007>::new(6);
    assert_eq!(table.permutation(6, 3), ModInt107::new(120));
}

#[test]
fn lazy_monoid_import_supports_modular_affine_sum() {
    use super::lazy_segment_tree_monoid_import::{
        Affine, Combinatorics, ModInt998, RangeAffineSum, SumCount, lazy_from_operator,
    };

    let table = Combinatorics::<998244353>::new(4);
    let leaves = [1, 2, 3, 4].map(|n| SumCount::new(table.combination(n, 1), 1));
    let mut tree = lazy_from_operator::<RangeAffineSum<998244353>>(&leaves);
    tree.update(0, 4, Affine::new(2, 3));
    tree.update(0, 4, Affine::new(4, -1));
    assert_eq!(tree.query(1, 3), SumCount::new(ModInt998::new(62), 2));
    assert_eq!(tree.query(2, 2), SumCount::new(ModInt998::ZERO, 0));
}

#[test]
fn wavelet_matrix_import_supports_both_duplicate_modes() {
    use super::wavelet_matrix_import::WaveletMatrix;
    let values = [9, 9, 1, -2, 1];
    let ordinary = WaveletMatrix::<false>::with_sums(&values);
    let distinct = WaveletMatrix::<true>::new(&values);
    assert_eq!(ordinary.top_k(0, 3, 2), [9, 9]);
    assert_eq!(distinct.top_k(0, 3, 2), [9, 1]);
    assert_eq!(distinct.kth_smallest_with_index(3, 5, 1), Some((1, 4)));
    assert_eq!(ordinary.sum_largest(0, 3, 2), Some(18));
}

#[test]
fn wavelet_matrix_dynamic_import_updates_and_queries() {
    use super::wavelet_matrix_dynamic_import::DynamicWaveletMatrix;
    let mut wm = DynamicWaveletMatrix::with_universe(&[9, 1, 9], &[-1]);
    assert_eq!(wm.set(0, -1), Some(9));
    wm.insert(1, 9);
    assert_eq!(wm.remove(2), Some(1));
    assert_eq!(wm.kth_largest_with_index(0, 3, 1), Some((9, 2)));
    assert_eq!(wm.range_freq(0, 3, -1, 9), 1);
}
