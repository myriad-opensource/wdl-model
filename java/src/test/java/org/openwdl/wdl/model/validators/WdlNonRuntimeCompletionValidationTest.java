package org.openwdl.wdl.model.validators;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.validators.WdlStaticAnalysisValidator;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class WdlNonRuntimeCompletionValidationTest {

  private static final Path ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "non_runtime_completion");

  static Stream<Arguments> baselineFunctionArgFailures() {
    return Stream.of(
        Arguments.of("baseline_function_args/length_bad.wdl"),
        Arguments.of("baseline_function_args/contains_key_bad.wdl"));
  }

  static Stream<Arguments> memberAndIndexFailures() {
    return Stream.of(
        Arguments.of("member_index_checks/unknown_struct_field_fail.wdl"),
        Arguments.of("member_index_checks/unknown_call_output_fail.wdl"),
        Arguments.of("member_index_checks/index_out_of_bounds_fail.wdl"));
  }

  @Test
  void validatesNestedImportedTypeAliases() throws Exception {
    Path file = ROOT.resolve("import_alias_nested/root.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "baseline function rejects {0}")
  @MethodSource("baselineFunctionArgFailures")
  void rejectsInvalidBaselineFunctionArgs(String fixture) throws Exception {
    Path file = ROOT.resolve(fixture);
    assertThrows(WdlException.class, () -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @Test
  void acceptsValidBaselineFunctionArgs() throws Exception {
    Path file = ROOT.resolve("baseline_function_args/baseline_function_args_ok.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "member/index rejects {0}")
  @MethodSource("memberAndIndexFailures")
  void rejectsInvalidMemberAndIndexAccess(String fixture) throws Exception {
    Path file = ROOT.resolve(fixture);
    assertThrows(WdlException.class, () -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @Test
  void acceptsValidMemberAndIndexAccess() throws Exception {
    Path file = ROOT.resolve("member_index_checks/member_index_checks_ok.wdl");
    assertDoesNotThrow(() -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @Test
  void validatesPlaceholderInterpolationAndSectionSyntax() throws Exception {
    Path placeholders = ROOT.resolve("placeholder_interpolation_ok.wdl");
    Path sections = ROOT.resolve("requirements_hints_syntax_ok.wdl");

    WdlDocument placeholderDoc = WdlV1Loader.load(placeholders.toFile());
    WdlDocument sectionDoc = WdlV1Loader.load(sections.toFile());

    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(placeholderDoc));
    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(sectionDoc));
  }

  @Test
  void rejectsInvalidJsonTypeLevelStaticUsage() throws Exception {
    Path file = ROOT.resolve("json_type_level_static_fail.wdl");
    WdlDocument doc = WdlV1Loader.load(file.toFile());

    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertThrows(WdlException.class, () -> new WdlStaticAnalysisValidator().validate(doc));
  }
}
