package org.openwdl.wdl.model.benchmarks;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.resolvers.WdlImportResolverFilesystem;
import com.myriad.wdl.model.validators.WdlLintingValidator;
import com.myriad.wdl.model.validators.WdlValidator;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;

/**
 * Lightweight benchmark harness for Java loader/validator hotspot profiling.
 *
 * <p>This runner favors repeatable relative comparisons over statistical rigor. Use JMH for
 * publication-quality benchmark methodology.
 */
public final class WdlBenchmarkRunner {

  private static final class Scenario {
    final String name;
    final ThrowingRunnable action;

    Scenario(String name, ThrowingRunnable action) {
      this.name = name;
      this.action = action;
    }
  }

  @FunctionalInterface
  private interface ThrowingRunnable {
    void run() throws Exception;
  }

  private static final class Result {
    final String name;
    final int iterations;
    final long nanos;

    Result(String name, int iterations, long nanos) {
      this.name = name;
      this.iterations = iterations;
      this.nanos = nanos;
    }

    double opsPerSec() {
      return iterations / (nanos / 1_000_000_000.0);
    }

    double usPerOp() {
      return (nanos / 1_000.0) / iterations;
    }
  }

  public static void main(String[] args) throws Exception {
    int iterations = intArg(args, "--iterations", 2000);
    int warmup = intArg(args, "--warmup", 100);
    Path outputPath = Path.of(strArg(args, "--bench-output", ".profiles/bench.txt"));

    String simpleSource =
        Files.readString(
            Path.of("src", "test", "resources", "wdl_tests", "validator", "loader_valid_document.wdl"));
    Path importRoot =
        Path.of("src", "test", "resources", "wdl_tests", "loader_imports", "recursive", "root.wdl");

    WdlValidator semanticValidator = new WdlValidator();
    WdlLintingValidator lintingValidator = new WdlLintingValidator();
    lintingValidator.setThrowOnWarnings(false);
    WdlDocument parsedDocument = WdlV1Loader.load(simpleSource);

    List<Scenario> scenarios = new ArrayList<>();
    scenarios.add(new Scenario("load simple source string", () -> WdlV1Loader.load(simpleSource)));
    scenarios.add(
        new Scenario(
            "load recursive imports from file",
            () -> WdlV1Loader.load(importRoot.toFile(), null, new WdlImportResolverFilesystem())));
    scenarios.add(
        new Scenario("semantic validate parsed doc", () -> semanticValidator.validate(parsedDocument)));
    scenarios.add(
        new Scenario(
            "lint validate parsed doc", () -> lintingValidator.validate(parsedDocument)));

    List<Result> results = new ArrayList<>();
    for (Scenario scenario : scenarios) {
      for (int i = 0; i < warmup; i++) {
        scenario.action.run();
      }

      long start = System.nanoTime();
      for (int i = 0; i < iterations; i++) {
        scenario.action.run();
      }
      long elapsed = System.nanoTime() - start;
      results.add(new Result(scenario.name, iterations, elapsed));
    }

    String report = formatResults(results);
    Files.createDirectories(outputPath.getParent());
    Files.writeString(outputPath, report);
    System.out.print(report);
    System.out.println("Wrote Java benchmark summary to " + outputPath);
  }

  private static String formatResults(List<Result> results) {
    StringBuilder sb = new StringBuilder();
    sb.append("Java benchmark summary\n");
    sb.append("name                                   ops/s        us/op      iterations\n");
    sb.append("--------------------------------------------------------------------------\n");
    for (Result row : results) {
      sb.append(
          String.format(
              Locale.ROOT,
              "%-38s %10.2f %12.2f %12d\n",
              row.name,
              row.opsPerSec(),
              row.usPerOp(),
              row.iterations));
    }
    sb.append('\n');
    return sb.toString();
  }

  private static int intArg(String[] args, String name, int defaultValue) {
    String value = strArg(args, name, null);
    if (value == null) {
      return defaultValue;
    }
    try {
      return Integer.parseInt(value);
    } catch (NumberFormatException ex) {
      throw new IllegalArgumentException("Invalid integer for " + name + ": " + value, ex);
    }
  }

  private static String strArg(String[] args, String name, String defaultValue) {
    for (int i = 0; i < args.length; i++) {
      if (name.equals(args[i])) {
        if (i + 1 >= args.length) {
          throw new IllegalArgumentException("Missing value for argument " + name);
        }
        return args[i + 1];
      }
    }
    return defaultValue;
  }

  private WdlBenchmarkRunner() {}
}
