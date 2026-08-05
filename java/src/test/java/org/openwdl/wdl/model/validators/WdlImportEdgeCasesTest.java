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

class WdlImportEdgeCasesTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "import_edge_cases");

  static Stream<Arguments> importFailureCases() {
    return Stream.of(
        Arguments.of("duplicate_namespace"),
        Arguments.of("namespace_conflicts_local"),
        Arguments.of("member_alias_conflicts_local"),
        Arguments.of("member_alias_duplicate"));
  }

  static Stream<Arguments> importSuccessCases() {
    return Stream.of(Arguments.of("mixed_forms_ok"));
  }

  @ParameterizedTest(name = "import rejects {0}")
  @MethodSource("importFailureCases")
  void rejectsImportEdgeCase(String fixtureDir) throws Exception {
    Path root = FIXTURES_ROOT.resolve(fixtureDir).resolve("root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "import accepts {0}")
  @MethodSource("importSuccessCases")
  void acceptsImportEdgeCase(String fixtureDir) throws Exception {
    Path root = FIXTURES_ROOT.resolve(fixtureDir).resolve("root.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }
}
