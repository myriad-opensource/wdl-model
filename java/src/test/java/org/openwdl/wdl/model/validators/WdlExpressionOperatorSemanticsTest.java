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

class WdlExpressionOperatorSemanticsTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "expression_operator_semantics");

  static Stream<Arguments> staticOnlyOperatorFailures() {
    return Stream.of(
        Arguments.of("logical_operand_type_fail.wdl"),
        Arguments.of("numeric_operand_type_fail.wdl"),
        Arguments.of("order_comparison_type_fail.wdl"),
        Arguments.of("ternary_condition_type_fail.wdl"));
  }

  @ParameterizedTest(name = "static rejects {0}")
  @MethodSource("staticOnlyOperatorFailures")
  void rejectsOperatorTypeMismatchesUnderStaticValidator(String fixture) throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve(fixture).toFile());
    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertThrows(WdlException.class, () -> new WdlStaticAnalysisValidator().validate(doc));
  }

  @Test
  void acceptsValidOperatorExpressions() throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve("operators_ok.wdl").toFile());
    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(doc));
  }

  @Test
  void acceptsOperatorPrecedenceAndCompoundEquality() throws Exception {
    WdlDocument precedenceDoc =
        WdlV1Loader.load(FIXTURES_ROOT.resolve("operator_precedence_ok.wdl").toFile());
    WdlDocument equalityDoc =
        WdlV1Loader.load(FIXTURES_ROOT.resolve("compound_equality_ok.wdl").toFile());

    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(precedenceDoc));
    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(equalityDoc));
  }

  @Test
  void rejectsIncompatibleCompoundEquality() throws Exception {
    WdlDocument doc =
        WdlV1Loader.load(FIXTURES_ROOT.resolve("compound_equality_incompatible_fail.wdl").toFile());

    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertThrows(WdlException.class, () -> new WdlStaticAnalysisValidator().validate(doc));
  }
}
