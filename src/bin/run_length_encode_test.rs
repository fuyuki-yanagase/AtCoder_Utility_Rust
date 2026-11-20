// run_length_encodeの使用例・テスト。本体をコピーせず、公開モジュールを検証する。
use utility_rust::run_length_encode::*;

fn example() {
    // ============================================================
    // ASCII文字列版のテスト
    // ============================================================

    {
        let s = "aaabbcaaaa";
        let encoded = run_length_encode(s);

        // b'a'のようなu8のまま比較する
        let expected = vec![(b'a', 3), (b'b', 2), (b'c', 1), (b'a', 4)];

        assert_eq!(encoded, expected);

        println!("ASCII test 1: OK");
        println!("input   = {}", s);
        println!("encoded = {:?}", encoded);

        // 表示時だけu8をcharへ変換する
        for &(byte, count) in &encoded {
            println!("({}, {})", byte as char, count);
        }

        println!();
    }

    // 1文字だけの場合
    {
        let s = "x";
        let encoded = run_length_encode(s);
        let expected = vec![(b'x', 1)];

        assert_eq!(encoded, expected);

        println!("ASCII test 2: OK");
    }

    // すべて同じ文字の場合
    {
        let s = "aaaaaaaa";
        let encoded = run_length_encode(s);
        let expected = vec![(b'a', 8)];

        assert_eq!(encoded, expected);

        println!("ASCII test 3: OK");
    }

    // すべて異なる文字の場合
    {
        let s = "abcdef";
        let encoded = run_length_encode(s);

        let expected = vec![(b'a', 1), (b'b', 1), (b'c', 1), (b'd', 1), (b'e', 1), (b'f', 1)];

        assert_eq!(encoded, expected);

        println!("ASCII test 4: OK");
    }

    // 空文字列の場合
    {
        let s = "";
        let encoded = run_length_encode(s);
        let expected = Vec::<(u8, usize)>::new();

        assert_eq!(encoded, expected);

        println!("ASCII test 5: OK");
    }

    // 数字文字列の場合
    {
        let s = "111222333331";
        let encoded = run_length_encode(s);

        let expected = vec![(b'1', 3), (b'2', 3), (b'3', 5), (b'1', 1)];

        assert_eq!(encoded, expected);

        println!("ASCII test 6: OK");
    }

    // ============================================================
    // 任意のスライス版のテスト
    // ============================================================

    // i32の配列
    {
        let values = vec![1, 1, 1, 5, 5, 3, 3, 3, 3];
        let encoded = run_length_encode_slice(&values);

        let expected = vec![(1, 3), (5, 2), (3, 4)];

        assert_eq!(encoded, expected);

        println!("Slice<i32> test 1: OK");
        println!("input   = {:?}", values);
        println!("encoded = {:?}", encoded);
        println!();
    }

    // 負数を含むi64の配列
    {
        let values = vec![-1_i64, -1, 0, 0, 0, 100, 100];
        let encoded = run_length_encode_slice(&values);

        let expected = vec![(-1_i64, 2), (0_i64, 3), (100_i64, 2)];

        assert_eq!(encoded, expected);

        println!("Slice<i64> test 2: OK");
    }

    // charの配列
    {
        let values = vec!['a', 'a', 'b', 'c', 'c', 'c'];
        let encoded = run_length_encode_slice(&values);

        let expected = vec![('a', 2), ('b', 1), ('c', 3)];

        assert_eq!(encoded, expected);

        println!("Slice<char> test 3: OK");
    }

    // boolの配列
    {
        let values = vec![true, true, false, false, false, true];

        let encoded = run_length_encode_slice(&values);

        let expected = vec![(true, 2), (false, 3), (true, 1)];

        assert_eq!(encoded, expected);

        println!("Slice<bool> test 4: OK");
    }

    // 1要素だけの場合
    {
        let values = vec![100_usize];
        let encoded = run_length_encode_slice(&values);
        let expected = vec![(100_usize, 1)];

        assert_eq!(encoded, expected);

        println!("Slice test 5: OK");
    }

    // 空配列の場合
    {
        let values = Vec::<i32>::new();
        let encoded = run_length_encode_slice(&values);
        let expected = Vec::<(i32, usize)>::new();

        assert_eq!(encoded, expected);

        println!("Slice test 6: OK");
    }

    println!();
    println!("All tests passed.");
}

fn main() {
    example();
}

#[test]
fn usage_example() {
    example();
}
