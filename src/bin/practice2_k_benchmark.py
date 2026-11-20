"""ACL Practice Kの提出用2プログラムを、同じ最大制約入力で比較する。

実行例（リポジトリ直下）:
    python Utility_Rust/src/bin/practice2_k_benchmark.py
コンパイルと入力生成を除き、起動・標準入出力を含む7回の中央値を表示する。
"""
import argparse
import hashlib
import os
from pathlib import Path
import random
import statistics
import subprocess
import sys
import time

MOD = 998244353


def make_input(n, q, narrow):
    rng = random.Random(20261029 + int(narrow))
    lines = [f"{n} {q}", " ".join(str(rng.randrange(MOD)) for _ in range(n))]
    for step in range(q):
        left = rng.randrange(n)
        if narrow:
            right = min(n, left + rng.randrange(1, 65))
        else:
            right = rng.randrange(left + 1, n + 1)
        if step % 2 == 0:
            b = rng.randrange(1, MOD)
            c = rng.randrange(MOD)
            lines.append(f"0 {left} {right} {b} {c}")
        else:
            lines.append(f"1 {left} {right}")
    return ("\n".join(lines) + "\n").encode("ascii")


def run(executable, data):
    start = time.perf_counter()
    result = subprocess.run(
        [str(executable)], input=data,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        check=True, timeout=60,
    )
    return time.perf_counter() - start, result.stdout


def main():
    # Windowsでパイプ経由でも日本語の測定結果を文字化けさせない。
    sys.stdout.reconfigure(encoding="utf-8")
    parser = argparse.ArgumentParser(description="ACL Practice Kの既存版・Monoid版の性能比較")
    parser.add_argument("--n", type=int, default=500000)
    parser.add_argument("--q", type=int, default=500000)
    parser.add_argument("--runs", type=int, default=7)
    args = parser.parse_args()
    if not (1 <= args.n <= 500000 and 1 <= args.q <= 500000 and args.runs >= 1):
        parser.error("n/qは1〜500000、runsは1以上を指定してください")

    bin_dir = Path(__file__).resolve().parent
    target = bin_dir.parent.parent / "target" / "practice2_k_compare"
    target.mkdir(parents=True, exist_ok=True)
    executables = {}
    for name in ("practice2_k_lazy", "practice2_k_lazy_monoid"):
        executable = target / (name + (".exe" if os.name == "nt" else ""))
        subprocess.run(
            ["rustc", "--edition=2021", "-O", str(bin_dir / (name + ".rs")), "-o", str(executable)],
            check=True,
        )
        executables[name] = executable

    old = "practice2_k_lazy"
    new = "practice2_k_lazy_monoid"
    for narrow in (False, True):
        data = make_input(args.n, args.q, narrow)
        _, expected = run(executables[old], data)
        _, actual = run(executables[new], data)
        if actual != expected:
            raise AssertionError("既存版とMonoid版の出力が一致しません")

        samples = {old: [], new: []}
        for turn in range(args.runs):
            order = (old, new) if turn % 2 == 0 else (new, old)
            for name in order:
                elapsed, output = run(executables[name], data)
                if output != expected:
                    raise AssertionError(f"{name}の繰り返し実行で出力が変化しました")
                samples[name].append(elapsed)

        old_time = statistics.median(samples[old])
        new_time = statistics.median(samples[new])
        pattern = "長さ64以下" if narrow else "乱数区間"
        print(f"{pattern}: N={args.n} Q={args.q} {args.runs}回中央値", flush=True)
        print(f"  既存版: {old_time * 1000:.3f} ms", flush=True)
        print(f"  Monoid版: {new_time * 1000:.3f} ms", flush=True)
        print(f"  速度比: {old_time / new_time:.3f}倍", flush=True)
        print(f"  出力一致: SHA-256 {hashlib.sha256(expected).hexdigest()}", flush=True)


if __name__ == "__main__":
    main()

