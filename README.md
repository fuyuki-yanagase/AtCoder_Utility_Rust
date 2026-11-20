# AtCoder_Utility_Rust

競技プログラミング用のRustライブラリ。公開先は [Tick Tack TechのRustライブラリ](https://fuyuki-connect.net/libraries/rust) です。

## 構成

- `src/*.rs`：ブログの公開対象はこの階層だけです。
- `src/lib.rs`：モジュールの公開設定。
- `src/main.rs`：実行用コード・テンプレート。
- `src/import/<module>_import.rs`：依存ライブラリを展開したコピペ用の生成ファイル。ブログの公開対象外です。
- `src/bin/<module>_test.rs`：ライブラリの使用例・テスト。ライブラリ本体を `utility_rust::<module>` から読み込みます。`main` で使用例を実行し、`#[test]` で自動検証します。公開対象外です。
- `src/bin/_submit.rs`・`src/bin/_rust_minify.rs`・`src/bin/_rust_import.rs`：提出コード、圧縮ツール、コピペ用ファイルの生成器。ライブラリ別テストの整理対象から除外します。

## コピペ用ファイル

`src`直下のライブラリから、`src/import/<元の名前>_import.rs`を生成してコンパイル確認します。`lib.rs`（モジュール一覧）と`main.rs`（実行テンプレート）は対象外です。

たとえばダイクストラ法は`src/import/dijkstra_import.rs`を使います。共通ヘッダーを1回置き、優先度付きキューの本文、ダイクストラ法の本文を順に収録します。ファイル全体を提出ソースの先頭に貼り付け、必要な`main`などを続けてください。`use crate::...`や原本のモジュール構成は不要です。`num-complex`・`num-rational`は共通ヘッダーと同じ依存関係を利用します。

依存先の依存も再帰的に展開し、共有する依存は1回だけ含めます。`pub use crate::...`の再公開元も収録します。複数の`_import.rs`を同時に貼ると共通ヘッダーや定義が重複するため、1ファイル単位で利用してください。

このリポジトリのルートで実行します。

```powershell
cargo run --bin _rust_import --locked
cargo check --test imports --locked
```

原本や`common_header.rs`を編集した後は再生成します。生成物と`tests/imports.rs`は自動生成のため直接編集しません。生成済みファイルの更新と、原本がなくなった古い生成物の整理を行います。同名の手書きファイルがある場合は上書き・削除を中止します。

更新漏れだけを確認する場合は、次のコマンドを使います。書き込みは行いません。

```powershell
cargo run --bin _rust_import --locked -- --check
```

`tests/imports.rs`は全生成ファイルを独立したモジュールとしてCargoへ登録します。既存のrust-analyzer設定で解析でき、通常の`cargo test --all-targets --locked`でも全件をコンパイルします。依存展開が必要なAPIの動作は`tests/support/import_usage.rs`で検証します。

## ModInt

`ModInt998` と `ModInt107` は、それぞれ法998244353と1000000007の型別名です。素数の法で繰り返し順列・組合せを求める場合は `Combinatorics::<MOD>::new(max_n)` で前計算し、`factorial`・`permutation`・`combination` を使います。前計算の上限は法未満です。

モノイド版遅延セグメント木では、`SumCount::new(ModInt998::new(value), 1)` で葉を作り、`lazy_from_operator::<RangeAffineSum<998244353>>` に渡すと区間和を法998244353で管理できます。区間更新には `Affine::new(a, b)` を渡します。

## 畳み込み

- `src/convolution.rs`：愚直法 `convolution_naive_i64`、FFT/逆FFT、NTT/逆NTT、FFT/NTTによる畳み込み、高速な反復型 `convolution_ntt_fast`。愚直法は積・加算途中の値を含めてi64内に収まる入力が前提です。FFTは近似値、NTTの法は998244353です。
- `cargo test --bin convolution_test --locked` で変換・逆変換・空入力・負数・零係数と、定義どおりの二重ループとの一致を検証します。高速版は独立u128愚直mod oracleによる4テスト・175ケースで、16／17の切替・変換長・法以上の入力も確認します。

## インデックス付き遅延セグメント木

モノイド版の `lazy_segment_tree_monoid::indexed` には `RangeAddMaxIndex`、`RangeAssignMaxIndex`、`RangeAddMinIndex`、`RangeAssignMinIndex` があります。`MaxIndex::new(value, index)` または `MinIndex::new(value, index)` で元の添字を付けた葉を作り、`lazy_from_operator` に渡します。例えば `lazy_from_operator::<RangeAssignMaxIndex<i64, Right>>(&leaves)` は最大値が同じ場合に右端を選びます。第2型引数を省略すると `Left` になります。

`query` の結果は `value` と `index` から取得でき、空区間は `is_empty()` で判定できます。値には `Ord` を満たす数値型を使います。加算では中間値が型の範囲内に収まることが前提です。コピペ用の `lazy_segment_tree_monoid_import.rs` でも同じAPIを使えます。

## Wavelet Matrix

静的な `i64` 配列について、第k要素、元の添字、値域の個数、上位・下位の列挙や合計を求めます。位置は `[l, r)`、順位は0始まりです。

```rust
use utility_rust::wavelet_matrix::WaveletMatrix;

let a = [9, 9, 1];
let ordinary = WaveletMatrix::<false>::new(&a);
let distinct = WaveletMatrix::<true>::new(&a);
assert_eq!(ordinary.top_k(0, 3, 2), [9, 9]);
assert_eq!(distinct.top_k(0, 3, 2), [9, 1]);
assert_eq!(distinct.kth_largest_with_index(0, 3, 0), Some((9, 0)));
```

両モードとも `occurrences`、`rank`、`select`、`histogram` は出現回数を基準にします。`range_freq_bounds` と `range_sum_bounds` は `x..` や `..=x` も受け取ります。通常モードで合計を多く求める場合は、`WaveletMatrix::<false>::with_sums` で構築すると値域・上位・下位の合計を値のビット数に比例する時間で求められます。

詳しい使い方は `src/bin/wavelet_matrix_test.rs` の `example()` にあります。提出コードには `src/import/wavelet_matrix_import.rs` を使えます。

## 動的 Wavelet Matrix

`DynamicWaveletMatrix` は `i64` の列への点代入・挿入・削除と、区間の第k要素・出現回数・値域件数に対応します。添字と順位は0始まり、区間は半開です。`new` なら i64 全域、`with_value_range` なら指定した値域、`with_universe` なら初期値と更新候補を座標圧縮して扱います。

```rust
use utility_rust::wavelet_matrix_dynamic::DynamicWaveletMatrix;

let mut wm = DynamicWaveletMatrix::with_value_range(&[4, -1, 4], -1, 1_000_000);
assert_eq!(wm.set(1, 7), Some(-1));
assert_eq!(wm.kth_largest_with_index(0, 3, 0), Some((7, 1)));
```

詳しい使用例と ABC484 F の解法は `src/bin/wavelet_matrix_dynamic_test.rs` にあります。`cargo run --release --locked --bin wavelet_matrix_dynamic_test -- --abc484-f` を実行すると、標準入力から同問題を解きます。コピペ用は `src/import/wavelet_matrix_dynamic_import.rs` です。

## Union-Find

`src/union_find.rs` の `RangeParallelUnionFind` は、既存の `UnionFind` を元の頂点の管理に使い、2冪長ごとの区間情報を追加します。

- `new(n)`：頂点数nで構築。空集合にも対応。
- `unite(u, v)`：1組を結合し、実際に併合した場合にtrue。
- `unite_range(a, b, k)`：区間 `[a, a+k)` と `[b, b+k)` の対応位置を結合し、元の連結成分の併合回数を返す。
- `unite_range_with(a, b, k, |root, absorbed| ...)`：元の成分の併合ごとに、新しい根と吸収された根を通知。通知時には併合済み。集計を維持する場合は、すべての結合で同じ集計処理を渡す。
- `root`・`connected`・`size`・`groups`：通常のUnion-Findと同様。

構築はO(N log N)時間・空間、Q回の区間結合は合計O((N log N + Q) α(N))時間（コールバックの処理時間を除く）。上位段は負のサイズと親をisizeの1配列へ格納します。64ビット環境・N=500000では、下位の既存UnionFindを含む内部要素は約76MiBです。入力範囲は結合開始前に検証します。

`cargo test --bin union_find_test --locked` で使用例・サンプル・独立した愚直解との比較を検証し、`cargo test --release --bin union_find_test --locked maximum_constraints -- --ignored` で最大制約を検証できます。

## Treap

`ImplicitTreap::to_vec(&mut self)` は遅延更新を反映し、現在の配列を `Vec<i64>` で返します。原本・コピペ用ファイル・使用例は同じAPI名に統一しています。

## 幾何

- `src/vec2_angle.rs`：新実装。`Vec2` と、方向ベクトル `dir: Vec2`・周回数 `turn: i64` を持つ `SpiralAngle`。`SpiralAngle::new(Vec2, turn)` を使用します。
- `src/spiral_angle.rs`：既存API互換用。`SpiralAngle::new(x, y, turn)` を維持します。新実装と型名が共通なので、必要なモジュールを選んで利用してください。

`Vec2` の通常比較は座標順、`cmp_arg` は偏角順です。`SpiralAngle` の比較は周回数と偏角によるため、同方向のベクトル長は無視します。i64演算は全中間値がi64内、広域演算は差・積・和・絶対値を含む全中間値がi128内に収まる入力を前提とします。checked角度APIは成分の符号反転・周回数のオーバーフロー時に `None` を返します。浮動小数点で返す角度・距離は近似値です。

## ソースの書式

`src` 直下の全Rustファイルを、次の順序に統一します。

1. ブログで使う表示名の1行コメント。
2. `common_header.rs` の内容をそのまま展開した共通ヘッダー（属性、標準ライブラリ等のimport、マクロ、定数）。
3. 必要な依存ライブラリの `use crate::<module>::*;`。依存がない場合は省略。既存APIの再公開に必要な `pub use` は、この直後に明示指定で置きます。
4. タイトルの上下に区切り線を4本ずつ並べた見出しと `Library URL`。
5. ライブラリ本体。

```rust
// 遅延セグメント木
// ここに common_header.rs の内容を展開

// 依存がある場合のみ
use crate::monoid_group_abelian::*;

// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
//
//                                     遅延セグメント木
//
// ======================================================================================
// ======================================================================================
// ======================================================================================
// ======================================================================================
// Library URL : https://fuyuki-connect.net/libraries/rust/<版ID>/src/lazy_segment_tree.rs
```

共通ヘッダーを変更する場合は `common_header.rs` と各ファイルへの展開内容を揃えます。本文中に複数のライブラリセクションがある場合は、その見出しと参照URLも維持します。最初のセクション名がファイル全体の掲載名と異なる場合は、URLの後に小見出しとして残します。未公開の新規ファイルに公開済みの版IDを推測して指定しません。取り込み時は版なしの別名URLを使い、公開確認後に固定版URLへ揃えます。

`main`・テスト・使用例は対応する `src/bin/<module>_test.rs` にまとめます。`main.rs` は提出用テンプレートとして `main` / `solve` とScanner／Writerを維持し、既存の `Template URL` も参照コメントとして残します。

`<版ID>` は公開版の小文字16桁の16進数です。提出コードには使用した版の固定URLを残し、その版のコードと公開確認日時を閲覧できるようにします。ブログの「コードをコピー」「参照コメントをコピー」も固定URLを付けます。版なしの `/libraries/rust/src/...` は最新版へ案内する別名URLで、未公開の作業中ソースに使えますが、使用版の記録には使いません。

圧縮ツールは `// Template URL` と `// Library URL` の参照行を先頭に残します。コロンあり・なしに対応し、同一の参照行は最初の1件だけ残します。詳細は [提出用圧縮コマンド](提出用圧縮コマンド.md) を参照してください。

## 入力前提

競技プログラミングの制約を満たす入力を前提とします。高速Scannerへの符号付き整数型の最小値（`i32::MIN` など）の入力は対象外です。入力の追加検証やオーバーフロー検査は導入せず、Scanner本体の速度を維持します。各アルゴリズムも、木・非負辺・正の法・演算結果が型の範囲内など、ソースに記載した前提で利用してください。

## 検証

このディレクトリで実行します。

```powershell
cargo test --all-targets --locked
cargo test --doc --locked
cargo run --bin union_find_test --locked
```

`--all-targets` は各binの `#[test]` と使用例を呼ぶ `usage_example` を実行し、`src/main.rs` もコンパイルします。`--doc` はドキュメント内のRustコード例を検証します。特定のライブラリだけなら `cargo test --bin union_find_test --locked`、使用例なら `cargo run --bin union_find_test --locked` のように実行します。テスト内の検証には、releaseでも有効な `assert!` / `assert_eq!` を使います。

各 `*_test` の通常実行は入力不要です。`in_out_test` は既知入力を子プロセスに渡してScanner／Writerを検証します。子プロセス専用の `io_child_process_entry` は通常のテスト一覧ではignoredですが、親テストから実際に実行します。`_submit`・`_rust_minify`・`_rust_import` の `main` は自動起動しません。
