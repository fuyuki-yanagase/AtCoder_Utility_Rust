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
use rustc_lexer::{Token, TokenKind, first_token};
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::f64::consts::{PI, TAU};
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{self, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::mem::{replace, swap};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

/// コメントや空白トークンかどうかを判定する
fn is_skip_token(kind: TokenKind) -> bool {
    match kind {
        TokenKind::Whitespace => {
            return true;
        }
        TokenKind::LineComment => {
            return true;
        }
        TokenKind::BlockComment {
            ..
        } => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

/// 参照コメントとして残すラベル
const REFERENCE_LABELS: [&str; 2] = ["Template URL", "Library URL"];

/// 行コメントが出典を示す参照コメントなら、末尾の改行を除いた全文を返す
/// ラベルの後ろはコロンあり・なしのどちらも許可する
/// ///, //! のドキュメントコメントは対象外
fn reference_comment(comment: &str) -> Option<&str> {
    let body = comment.strip_prefix("//")?;
    if body.starts_with('/') || body.starts_with('!') {
        return None;
    }

    let body = body.trim_start();
    for label in REFERENCE_LABELS {
        if let Some(rest) = body.strip_prefix(label) {
            if rest.is_empty() || rest.starts_with(':') || rest.starts_with(char::is_whitespace) {
                return Some(comment.trim_end_matches('\r'));
            }
        }
    }
    return None;
}

/// 隣り合うと1つの複合演算子になる2文字かどうかを判定する
/// rustc_lexer は演算子を1文字ずつのトークンにするため、
/// 空白を消すと && や -> などの別のトークンに変わる組み合わせを拾う
fn is_compound_operator(left: char, right: char) -> bool {
    return matches!(
        (left, right),
        ('&', '&')
            | ('|', '|')
            | ('-', '>')
            | ('=', '>')
            | (':', ':')
            | ('<', '<')
            | ('>', '>')
            | ('.', '.')
            | ('=', '=')
            | ('!', '=')
            | ('<', '=')
            | ('>', '=')
            | ('.', '=')
            | ('+', '=')
            | ('-', '=')
            | ('*', '=')
            | ('/', '=')
            | ('%', '=')
            | ('^', '=')
            | ('&', '=')
            | ('|', '=')
    );
}

/// 空白を完全に消すと別のトークンに結合されるかどうかを判定する
fn need_space(left: &str, right: &str) -> bool {
    if left.is_empty() || right.is_empty() {
        return false;
    }

    // 区切られていた記号同士が複合演算子にならないようにする
    let left_last = left.chars().last().unwrap();
    let right_first = right.chars().next().unwrap();
    // 複合演算子以外も、手続きマクロに渡る記号の接続状態を変えない。
    const PUNCTUATION: &str = "=<>!~+-*/%^&|@.,;:#$?'";
    if left.len() == 1 && right.len() == 1 && PUNCTUATION.contains(left_last) && PUNCTUATION.contains(right_first) {
        return true;
    }
    if is_compound_operator(left_last, right_first) {
        return true;
    }

    // rustc_lexer 0.1にはRust 2021以降の予約prefix判定がない。
    // ident "文字列" / ident # / ident 'lifetime の元の区切りを保持する。
    if (matches!(first_token(left).kind, TokenKind::Ident | TokenKind::RawIdent) || left == "#")
        && matches!(right_first, '"' | '\'' | '#')
    {
        return true;
    }

    let joined = format!("{}{}", left, right);
    let first = first_token(&joined);

    return first.len != left.len();
}

/// ドキュメントコメントは属性としてマクロから参照されるので、その場に残す。
fn is_doc_comment(text: &str) -> bool {
    return text.starts_with("//!")
        || (text.starts_with("///") && !text.starts_with("////"))
        || text.starts_with("/*!")
        || (text.starts_with("/**") && !text.starts_with("/***") && text != "/**/");
}

/// 古いlexerにないC文字列も、同じ長さのバイト文字列として字句解析する。
/// 元ソースの文字列自体には触れず、raw C文字列内のコメントらしき文字も保全する。
fn next_token(source: &str) -> Token {
    if source.starts_with("c\"") || source.starts_with("cr\"") || source.starts_with("cr#") {
        let compatible = format!("b{}", &source[1..]);
        return first_token(&compatible);
    }
    return first_token(source);
}

/// Rustコードを提出用に圧縮する
/// Template URL, Library URL の参照コメントは先頭にまとめて残す
fn minify(source: &str) -> String {
    // shebangはファイル先頭に必要。参照コメントをそれより前へ移動しない。
    let (bom, source) =
        if let Some(rest) = source.strip_prefix('\u{feff}') { ("\u{feff}", rest) } else { ("", source) };
    let mut header = bom.to_string();
    let mut index = 0usize;
    if let Some(mut rest) = source.strip_prefix("#!") {
        while !rest.is_empty() {
            let token = next_token(rest);
            if !is_skip_token(token.kind) || is_doc_comment(&rest[..token.len]) {
                break;
            }
            rest = &rest[token.len..];
        }
        // #! [allow(...)] などの内側属性は通常のコードとして圧縮する。
        if !rest.starts_with('[') {
            index = source.find('\n').map_or(source.len(), |i| i + 1);
            header.push_str(&source[..index]);
            if !header.ends_with('\n') {
                header.push('\n');
            }
        }
    }
    let mut references: Vec<String> = Vec::new();
    let mut code = String::new();

    let mut previous_token_text = String::new();
    let mut has_skipped_space = false;

    while index < source.len() {
        let token = next_token(&source[index..]);
        let start = index;
        let end = index + token.len;
        let text = &source[start..end];

        index = end;

        if matches!(token.kind, TokenKind::LineComment | TokenKind::BlockComment { .. }) && is_doc_comment(text) {
            // 単独の / とdocが // に結合すると、記号も属性も通常コメントに消える。
            if previous_token_text == "/" {
                code.push(' ');
            }
            code.push_str(text);
            if matches!(token.kind, TokenKind::LineComment) {
                code.push('\n');
            }
            previous_token_text.clear();
            has_skipped_space = true;
            continue;
        }

        // 文字列などの中身はLineCommentにならないので、本物のコメントだけが対象になる
        if matches!(token.kind, TokenKind::LineComment) {
            if let Some(reference) = reference_comment(text) {
                if !references.iter().any(|saved| saved == reference) {
                    references.push(reference.to_string());
                }
            }
        }

        if is_skip_token(token.kind) {
            has_skipped_space = true;
            continue;
        }

        if has_skipped_space && need_space(&previous_token_text, text) {
            code.push(' ');
        }

        code.push_str(text);

        previous_token_text.clear();
        previous_token_text.push_str(text);

        has_skipped_space = false;
    }

    let mut result = header;
    for reference in &references {
        result.push_str(reference);
        result.push('\n');
    }
    result.push_str(&code);

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: cargo run --bin _rust_minify -- <input.rs> <output.rs>");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let source = fs::read_to_string(input_path).unwrap();
    let result = minify(&source);

    fs::write(output_path, result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_references_once_in_first_appearance_order() {
        let source = "// Template URL : https://example.com/a.rs\n\
                      fn main() {}\n\
                      // Library URL https://example.com/b.rs\n\
                      // Template URL : https://example.com/a.rs\n\
                      // Library URL: https://example.com/c.rs\n";

        assert_eq!(
            minify(source),
            "// Template URL : https://example.com/a.rs\n\
             // Library URL https://example.com/b.rs\n\
             // Library URL: https://example.com/c.rs\n\
             fn main(){}"
        );
    }

    #[test]
    fn keeps_pinned_url_and_strips_carriage_return() {
        let source = "// Template URL : https://example.com/libraries/rust/0123456789abcdef/src/a.rs\r\nfn a() {}\r\n";

        assert_eq!(
            minify(source),
            "// Template URL : https://example.com/libraries/rust/0123456789abcdef/src/a.rs\nfn a(){}"
        );
    }

    #[test]
    fn keeps_library_reference_label_and_version() {
        let source = "// Library URL : https://fuyuki-connect.net/libraries/rust/06ee2507fd93dbee/src/a.rs\r\n\
                      fn a() {}\r\n\
                      // Template URL https://fuyuki-connect.net/posts/template\r\n\
                      // Library URL : https://fuyuki-connect.net/libraries/rust/06ee2507fd93dbee/src/a.rs\r\n";
        assert_eq!(
            minify(source),
            "// Library URL : https://fuyuki-connect.net/libraries/rust/06ee2507fd93dbee/src/a.rs\n\
             // Template URL https://fuyuki-connect.net/posts/template\n\
             fn a(){}"
        );
    }

    #[test]
    fn does_not_hoist_lookalikes_inside_literals_and_docs() {
        let source = "const A: &str = r#\"// Template URL : https://example.com/raw\"#;\n\
                      const B: &str = \"// Library URL : https://example.com/string\";\n\
                      /// Template URL : https://example.com/doc\n\
                      //! Library URL : https://example.com/inner\n\
                      fn f() {}\n";

        assert_eq!(
            minify(source),
            "const A: &str=r#\"// Template URL : https://example.com/raw\"#;\
             const B: &str=\"// Library URL : https://example.com/string\";\
             /// Template URL : https://example.com/doc\n\
             //! Library URL : https://example.com/inner\n\
             fn f(){}"
        );
    }

    #[test]
    fn removes_ordinary_comments() {
        let source = "// plain note\n\
                      // Template URLs are plain text\n\
                      let a = 1; /* block Template URL : https://example.com/block */ // trailing\n\
                      let b = 2;\n";

        assert_eq!(minify(source), "let a=1;let b=2;");
    }

    #[test]
    fn keeps_space_between_separated_punctuation() {
        assert_eq!(minify("x & &y"), "x& &y");
        assert_eq!(minify("let f = | | 1;"), "let f= | |1;");
        assert_eq!(minify("let s: ::std::string::String;"), "let s: ::std::string::String;");
        assert_eq!(minify("Vec<Vec<i32> >"), "Vec<Vec<i32> >");
        assert_eq!(minify("a < <T as U>::V"), "a< <T as U>::V");
    }

    #[test]
    fn keeps_space_before_assignment_forms() {
        assert_eq!(minify("a < = b"), "a< =b");
        assert_eq!(minify("a ! = b"), "a! =b");
        assert_eq!(minify("a - = b"), "a- =b");
        assert_eq!(minify("a = > b"), "a= >b");
        assert_eq!(minify("a - > b"), "a- >b");
        assert_eq!(minify("a . . b"), "a. .b");
    }

    #[test]
    fn keeps_already_adjacent_operators() {
        assert_eq!(minify("a && b || c"), "a&&b||c");
        assert_eq!(minify("fn g() -> i32 { 0 }"), "fn g()->i32{0}");
        assert_eq!(minify("a <<= 1; b >>= 2; c ..= d"), "a<<=1;b>>=2;c..=d");
        assert_eq!(minify("match x { 1 => y, _ => z }"), "match x{1=>y,_=>z}");
        assert_eq!(minify("std::mem::swap(&mut a, &mut b)"), "std::mem::swap(&mut a, &mut b)");
    }

    #[test]
    fn keeps_strings_lifetimes_and_numerics() {
        let source = "fn f<'a>(x: &'a str) -> &'a str {\n    let n = 1 as u8;\n    let r = 1 ..= 5;\n    x\n}\n";
        assert_eq!(minify(source), "fn f<'a>(x: &'a str)-> &'a str{let n=1 as u8;let r=1 ..=5;x}");

        let source = "let s = \"a  b // c\"; let c = ' '; let b = b'x'; let f = 1.5e3 + 0x1f_u8 as f64;";
        assert_eq!(
            minify(source),
            "let s=\"a  b // c\";let c=' ';let b=b'x';let f=1.5e3+0x1f_u8 as f64;"
        );
    }

    #[test]
    fn keeps_space_between_words_after_comment_removal() {
        assert_eq!(minify("a /* x */ b"), "a b");
        assert_eq!(minify("a // x\nb"), "a b");
        assert_eq!(minify("1 /* x */ as u8"), "1 as u8");
        assert_eq!(minify("a / /* x */ * b"), "a/ *b");
    }

    #[test]
    fn keeps_reserved_literal_and_identifier_prefixes_separate() {
        assert_eq!(minify("m!(foo \"ok\")"), "m!(foo \"ok\")");
        assert_eq!(minify("m!(ident # token)"), "m!(ident #token)");
        assert_eq!(minify("m!(foo 'a)"), "m!(foo 'a)");
        assert_eq!(minify("c\"hello\""), "c\"hello\"");
    }

    #[test]
    fn keeps_doc_attributes_in_place() {
        assert_eq!(minify("m!{\n/// hello\nfn f() {}\n}"), "m!{/// hello\nfn f(){}}");
        assert_eq!(minify("/*! inner */ fn f(){}"), "/*! inner */fn f(){}");
        assert_eq!(minify("/** outer */ fn f(){}"), "/** outer */fn f(){}");
        assert_eq!(minify("//// ordinary\n/***/ /**/ fn f(){}"), "fn f(){}");
    }

    #[test]
    fn keeps_slash_separate_from_line_docs() {
        for gap in [" ", "\n", " /* gap */", " // gap\n"] {
            for doc in ["/// hello", "//! hello"] {
                let source = format!("m!{{/{gap}{doc}\nfn f(){{}}}}");
                assert_eq!(minify(&source), format!("m!{{/ {doc}\nfn f(){{}}}}"));
            }
        }
    }

    #[test]
    fn keeps_slash_separate_from_block_docs() {
        for gap in [" ", "\n", " /* gap */", " // gap\n"] {
            for doc in ["/** hello */", "/*! hello */"] {
                let source = format!("m!(/{gap}{doc} x)");
                assert_eq!(minify(&source), format!("m!(/ {doc}x)"));
            }
        }
        // doc末尾の / を単独トークンと誤認して余分な空白を加えない。
        assert_eq!(minify("m!(/** a */ /** b */ x)"), "m!(/** a *//** b */x)");
    }

    #[test]
    fn keeps_separated_macro_punctuation() {
        assert_eq!(minify("m!(+ - # #)"), "m!(+ - # #)");
        assert_eq!(minify("m!(+/* gap */-)"), "m!(+ -)");
    }

    #[test]
    fn keeps_c_raw_string_contents_and_guarded_string_spacing() {
        let source = "let s = cr##\"quote \" // Library URL: literal\n/* kept */ \"##;";
        assert_eq!(minify(source), "let s=cr##\"quote \" // Library URL: literal\n/* kept */ \"##;");
        assert_eq!(minify("m!(# \"text\")"), "m!(# \"text\")");
    }

    #[test]
    fn keeps_shebang_before_references_and_accepts_bom() {
        let source = "\u{feff}#!/usr/bin/env rust-script\n// Template URL : https://example.com/a\nfn main() {}";
        assert_eq!(
            minify(source),
            "\u{feff}#!/usr/bin/env rust-script\n// Template URL : https://example.com/a\nfn main(){}"
        );
        assert_eq!(
            minify("#! /* gap */ [allow(unused)]\nfn main() {}"),
            "#![allow(unused)]fn main(){}"
        );
        assert_eq!(minify(""), "");
    }
}
