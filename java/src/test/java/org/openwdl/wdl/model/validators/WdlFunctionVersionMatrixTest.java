package org.openwdl.wdl.model.validators;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class WdlFunctionVersionMatrixTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "function_version_matrix");

  static Stream<Arguments> acceptsVersionCompatibleFunctions() {
    return Stream.of(
        Arguments.of("v11_keys_ok.wdl"),
        Arguments.of("v12_contains_ok.wdl"),
        Arguments.of("v13_value_ok.wdl"));
  }

  static Stream<Arguments> rejectsVersionIncompatibleFunctions() {
    return Stream.of(
        Arguments.of("v11_contains_key_fail.wdl"),
        Arguments.of("v11_join_paths_fail.wdl"),
        Arguments.of("v12_value_fail.wdl"));
  }

  @ParameterizedTest(name = "accepts {0}")
  @MethodSource("acceptsVersionCompatibleFunctions")
  void acceptsVersionCompatibleFunctions(String fixture) throws Exception {
    Path file = FIXTURES_ROOT.resolve(fixture);
    assertDoesNotThrow(() -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "rejects {0}")
  @MethodSource("rejectsVersionIncompatibleFunctions")
  void rejectsVersionIncompatibleFunctions(String fixture) throws Exception {
    Path file = FIXTURES_ROOT.resolve(fixture);
    assertThrows(WdlException.class, () -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }
}
