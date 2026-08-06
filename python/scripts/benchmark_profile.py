from __future__ import annotations

import argparse
import cProfile
import gc
import pstats
from pathlib import Path
from time import perf_counter

from wdl_model.model import (
    WdlLintingSemanticValidator,
    WdlSemanticValidator,
    WdlV1Loader,
)
from wdl_model.model.resolvers import WdlImportResolverFilesystem


def run_scenario(name: str, fn, iterations: int, warmup: int) -> dict[str, float]:
    for _ in range(warmup):
        fn()

    gc.collect()
    start = perf_counter()
    for _ in range(iterations):
        fn()
    elapsed = perf_counter() - start

    return {
        "name": name,
        "iterations": float(iterations),
        "seconds": elapsed,
        "ops_per_sec": iterations / elapsed if elapsed > 0 else 0.0,
        "us_per_op": (elapsed * 1_000_000.0) / iterations if iterations > 0 else 0.0,
    }


def format_results(results: list[dict[str, float]]) -> str:
    lines = []
    lines.append("Python benchmark summary")
    lines.append("name                                   ops/s        us/op      iterations")
    lines.append("--------------------------------------------------------------------------")
    for row in results:
        lines.append(
            f"{row['name']:<38} {row['ops_per_sec']:>10.2f} {row['us_per_op']:>12.2f} {int(row['iterations']):>12}"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description="Run Python loader/validator microbenchmarks with cProfile output.")
    parser.add_argument("--profile-dir", default=".profiles", help="Output directory for profile artifacts")
    parser.add_argument("--iterations", type=int, default=3000, help="Iterations per benchmark scenario")
    parser.add_argument("--warmup", type=int, default=100, help="Warmup iterations per benchmark scenario")
    args = parser.parse_args()

    profile_dir = Path(args.profile_dir)
    profile_dir.mkdir(parents=True, exist_ok=True)

    simple_source = (Path("wdl_tests") / "validator" / "loader_valid_document.wdl").read_text(
        encoding="utf-8"
    )
    import_root = Path("wdl_tests") / "loader_imports" / "recursive" / "root.wdl"

    semantic_validator = WdlSemanticValidator()
    lint_validator = WdlLintingSemanticValidator().setThrowOnWarnings(False)
    resolver = WdlImportResolverFilesystem()
    parsed_document = WdlV1Loader.load_from_string(simple_source)

    scenarios = [
        ("load simple source string", lambda: WdlV1Loader.load_from_string(simple_source)),
        (
            "load recursive imports from file",
            lambda: WdlV1Loader.load_from_file(import_root, import_resolver=resolver),
        ),
        ("semantic validate parsed doc", lambda: semantic_validator.validateDocument(parsed_document)),
        ("lint validate parsed doc", lambda: lint_validator.validateDocument(parsed_document)),
    ]

    profiler = cProfile.Profile()
    profiler.enable()
    results = [
        run_scenario(name, fn, iterations=args.iterations, warmup=args.warmup)
        for name, fn in scenarios
    ]
    profiler.disable()

    bench_summary = format_results(results)
    bench_path = profile_dir / "bench.txt"
    bench_path.write_text(bench_summary, encoding="utf-8")
    print(bench_summary, end="")

    prof_path = profile_dir / "cpu.prof"
    profiler.dump_stats(str(prof_path))

    cpu_top_path = profile_dir / "cpu-top.txt"
    with cpu_top_path.open("w", encoding="utf-8") as out:
        stats = pstats.Stats(profiler, stream=out)
        stats.sort_stats("cumtime")
        stats.print_stats(50)

    print(f"Wrote benchmark and profile artifacts to {profile_dir}/")
    print(f"  - {bench_path}")
    print(f"  - {prof_path}")
    print(f"  - {cpu_top_path}")


if __name__ == "__main__":
    main()
