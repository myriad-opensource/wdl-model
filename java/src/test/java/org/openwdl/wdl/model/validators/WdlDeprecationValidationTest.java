package org.openwdl.wdl.model.validators;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.validators.WdlLintingValidator;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class WdlDeprecationValidationTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "deprecations");

  static Stream<Arguments> deprecatedFeatureFixtures() {
    return Stream.of(
        Arguments.of("runtime_section_deprecated.wdl"),
        Arguments.of("object_type_deprecated.wdl"),
        Arguments.of("placeholder_options_deprecated.wdl"),
        Arguments.of("file_scheme_import_deprecated.wdl"));
  }

  @ParameterizedTest(name = "deprecated feature warns: {0}")
  @MethodSource("deprecatedFeatureFixtures")
  void warnsOnDeprecatedFeatures(String fixture) throws Exception {
    Path file = FIXTURES_ROOT.resolve(fixture);
    WdlDocument doc;
    if ("file_scheme_import_deprecated.wdl".equals(fixture)) {
      doc = WdlV1Loader.load(Files.readString(file));
    } else {
      doc = WdlV1Loader.load(file.toFile());
    }

    WdlException ex =
        assertThrows(WdlException.class, () -> new WdlLintingValidator().validate(doc));

    assertTrue(
        ex.getErrors().stream()
            .filter(WdlSemanticError.class::isInstance)
            .map(WdlSemanticError.class::cast)
            .map(WdlSemanticError::code)
            .anyMatch(code -> code == WdlSemanticError.Code.LINT_DEPRECATED_FEATURE),
        "Expected at least one deprecation warning code");
  }

  @ParameterizedTest(name = "non-deprecated fixture has no deprecation warning: {0}")
  @MethodSource("nonDeprecatedFixtures")
  void doesNotReportDeprecationWarnings(String fixture) throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve(fixture).toFile());
    WdlLintingValidator lint = new WdlLintingValidator();
    lint.setThrowOnWarnings(false);

    assertDoesNotThrow(() -> lint.validate(doc));
  }

  static Stream<Arguments> nonDeprecatedFixtures() {
    return Stream.of(Arguments.of("no_deprecations.wdl"));
  }
}
