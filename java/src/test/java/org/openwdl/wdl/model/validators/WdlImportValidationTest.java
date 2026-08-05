package org.openwdl.wdl.model.validators;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class WdlImportValidationTest {

  private static final Path WDL_TESTS_ROOT = Paths.get("src", "test", "resources", "wdl_tests");

  private static final List<String> POSITIVE_IMPORT_EXAMPLES =
      List.of("call_example.wdl", "call_imported.wdl");

  private static final List<String> NEGATIVE_IMPORT_EXAMPLES =
      List.of("call_subworkflow_fail.wdl", "incomplete_struct_fail.wdl", "illegal_access_fail.wdl");

  static Stream<Arguments> positiveImportExamples() {
    return specExamples(POSITIVE_IMPORT_EXAMPLES);
  }

  static Stream<Arguments> negativeImportExamples() {
    return specExamples(NEGATIVE_IMPORT_EXAMPLES);
  }

  private static Stream<Arguments> specExamples(List<String> filenames) {
    List<Arguments> out = new ArrayList<>();
    for (String version : List.of("v1_1", "v1_2", "v1_3")) {
      for (String filename : filenames) {
        Path filePath = Paths.get("src", "test", "resources", "spec_examples", version, filename);
        if (Files.exists(filePath)) {
          out.add(Arguments.of(version, filename, filePath));
        }
      }
    }
    return out.stream();
  }

  @ParameterizedTest(name = "{0}/{1}")
  @MethodSource("positiveImportExamples")
  void validatesPositiveImportSpecExamples(String version, String filename, Path filePath)
      throws Exception {
    assertDoesNotThrow(() -> WdlV1Loader.load(filePath.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "{0}/{1}")
  @MethodSource("negativeImportExamples")
  void rejectsNegativeImportSpecExamples(String version, String filename, Path filePath)
      throws Exception {
    assertThrows(WdlException.class, () -> WdlV1Loader.load(filePath.toFile(), new WdlValidator()));
  }

  @Test
  void validatesStarAndMembersImportForms() throws Exception {
    Path root = fixturePath("import_validation", "star_members", "root.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void validatesStandardImportStructAliases() throws Exception {
    Path root = fixturePath("import_validation", "standard_alias", "root.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void rejectsUnknownMemberImport() throws Exception {
    Path root = fixturePath("import_validation", "unknown_member", "root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void rejectsDuplicateImportNamespaces() throws Exception {
    Path root = fixturePath("import_validation", "duplicate_namespace", "root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void rejectsImportAliasTargetThatDoesNotExist() throws Exception {
    Path root = fixturePath("import_validation", "bad_alias", "root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void rejectsIncompatibleImportedStructsWithoutAlias() throws Exception {
    Path root = fixturePath("import_validation", "struct_conflict", "root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  @Test
  void rejectsImportFromHigherMinorVersion() throws Exception {
    Path root = fixturePath("import_validation", "version_mismatch", "root.wdl");
    assertThrows(WdlException.class, () -> WdlV1Loader.load(root.toFile(), new WdlValidator()));
  }

  private static Path fixturePath(String... parts) {
    Path path = WDL_TESTS_ROOT;
    for (String part : parts) {
      path = path.resolve(part);
    }
    return path;
  }
}
