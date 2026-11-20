// in_outの使用例・テスト。

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

use std::process::{Command, Stdio};
use utility_rust::in_out::{FastScanner, FastWriter};

const CHILD_MODE: &str = "UTILITY_RUST_IO_TEST_MODE";

// Scannerの標準入力を子プロセスに渡す。本体へテスト用APIは追加しない。
fn perform_io(mode: &str) {
    println!("IO_RESULT");
    std::io::stdout().flush().unwrap();
    let mut scanner = if mode == "interactive" { FastScanner::new_interactive() } else { FastScanner::new() };
    let mut output = FastWriter::new();
    if mode == "interactive" {
        output.writeline(scanner.readint());
        output.writeline(scanner.read());
        output.writelist(&scanner.readints(2));
    } else {
        output.writeline(scanner.readint());
        output.writeline(scanner.readlong());
        output.writeline(scanner.readi128());
        output.writeline(scanner.readuint());
        output.writeline(scanner.readulong());
        output.writeline(scanner.readu128());
        output.writeline(scanner.readusize());
        output.writeline(scanner.readdouble());
        output.writeline(scanner.read());
        output.writelist(&scanner.readints(3));
        output.writedebugline(&scanner.readusizet2());
        output.writelist(&scanner.readbytes());
        output.writelist(&scanner.readintswith(2, -1, 1, 2, -9));
        output.writelist2_sep(&[vec![1, 2], vec![3]], ",");
        output.writeiter_sep([8, 9], ":");
    }
    output.flush();
}
fn run_child(mode: &str, input: &str) -> String {
    // testハーネス内では専用のignoredテストを実行し、通常実行ではmainが処理する。
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "io_child_process_entry", "--nocapture"])
        .env(CHILD_MODE, mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(
        result.status.success(),
        "child error: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8(result.stdout).unwrap().replace("\r\n", "\n");
    stdout.split_once("IO_RESULT\n").expect("子プロセスの結果がありません").1.to_owned()
}
fn example() {
    let output = run_child("interactive", "-2 word\n4 5\n");
    assert_eq!(output, "-2\nword\n4 5\n");
    println!("入力からの出力:\n{output}");
}
fn main() {
    if let Ok(mode) = std::env::var(CHILD_MODE) {
        perform_io(&mode);
    } else {
        example();
    }
}
#[test]
fn usage_example() {
    example();
}
#[test]
fn batch_scanner_and_writer() {
    let output = run_child(
        "batch",
        "\t-42 9223372036854775807 -170141183460469231731687303715884105727\n4294967295 18446744073709551615 340282366920938463463374607431768211455 3 1.25 rust\n5 7 9 10 20 abc 1 2",
    );
    assert_eq!(
        output,
        "-42\n9223372036854775807\n-170141183460469231731687303715884105727\n4294967295\n18446744073709551615\n340282366920938463463374607431768211455\n3\n1.25\nrust\n5 7 9\n(10, 20)\n97 98 99\n-9 0 1 -9 -9\n1,2\n3\n8:9\n"
    );
}
#[test]
#[ignore = "親テストが既知入力を渡す子プロセス専用"]
fn io_child_process_entry() {
    // --include-ignoredで単体実行された場合は、標準入力を読まず終了する。
    let Ok(mode) = std::env::var(CHILD_MODE) else {
        return;
    };
    perform_io(&mode);
    // ハーネスの終了メッセージをScanner/Writerの出力へ混ぜない。
    std::process::exit(0);
}
