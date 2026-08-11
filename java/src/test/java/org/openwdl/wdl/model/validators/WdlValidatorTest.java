package org.openwdl.wdl.model.validators;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.validators.WdlLintingValidator;
import com.myriad.wdl.model.validators.WdlStaticAnalysisValidator;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.openwdl.wdl.model.v1.WdlTestResources;

class WdlValidatorTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "validator");

  @Test
  void rejectsKnownParseOkFailExamplesV13() throws Exception {
    List<String> parseOkFails =
        List.of(
            "empty_array_fail.wdl",
            "illegal_access_fail.wdl",
            "non_empty_optional_fail.wdl",
            "private_declaration_fail.wdl",
            "select_first_empty_fail.wdl",
            "select_first_only_none_fail.wdl",
            "test_as_map_fail.wdl",
            "test_map_fail.wdl",
            "test_zip_fail.wdl",
            "write_json_fail.wdl");

    WdlValidator validator = new WdlValidator();

    for (String filename : parseOkFails) {
      WdlDocument doc = WdlV1Loader.load(WdlTestResources.loadWdlFile("v1_3", filename));
      assertThrows(WdlException.class, () -> validator.validate(doc), filename);
    }
  }

  @Test
  void acceptsSimpleValidWorkflow() throws Exception {
    String source = loadFixture("accepts_simple_valid_workflow.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlValidator validator = new WdlValidator();
    assertDoesNotThrow(() -> validator.validate(doc));
  }

  @Test
  void loaderRunsValidatorWhenProvidedAndThrowsSemanticErrors() throws Exception {
    String source = WdlTestResources.loadWdlFile("v1_3", "select_first_empty_fail.wdl");
    WdlValidator validator = new WdlValidator();

    assertThrows(WdlException.class, () -> WdlV1Loader.load(source, validator));
  }

  @Test
  void loaderRunsValidatorWhenProvidedAndReturnsValidDocument() throws Exception {
    String source = loadFixture("loader_valid_document.wdl");

    WdlValidator validator = new WdlValidator();
    assertDoesNotThrow(() -> WdlV1Loader.load(source, validator));
  }

  @Test
  void normalValidatorRejectsFunctionNotAvailableInDocumentVersion() throws Exception {
    String source = loadFixture("function_version_invalid.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlValidator validator = new WdlValidator();
    assertThrows(WdlException.class, () -> validator.validate(doc));
  }

  @Test
  void fullValidatorCatchesAdditionalStaticFunctionSignatureErrors() throws Exception {
    String source = loadFixture("static_function_signature_bad.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlValidator baseValidator = new WdlValidator();
    WdlStaticAnalysisValidator fullValidator = new WdlStaticAnalysisValidator();

    assertDoesNotThrow(() -> baseValidator.validate(doc));
    assertThrows(WdlException.class, () -> fullValidator.validate(doc));
  }

  @Test
  void fullValidatorCatchesAdditionalStaticWorkflowStructureErrors() throws Exception {
    String source = loadFixture("static_workflow_structure_bad.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlValidator baseValidator = new WdlValidator();
    WdlStaticAnalysisValidator fullValidator = new WdlStaticAnalysisValidator();

    assertDoesNotThrow(() -> baseValidator.validate(doc));
    assertThrows(WdlException.class, () -> fullValidator.validate(doc));
  }

  @Test
  void staticAnalysisValidatorCatchesNestedWorkflowStructureErrors() throws Exception {
    String source = loadFixture("nested_workflow_structure_bad.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlValidator baseValidator = new WdlValidator();
    WdlStaticAnalysisValidator staticValidator = new WdlStaticAnalysisValidator();

    assertDoesNotThrow(() -> baseValidator.validate(doc));
    assertThrows(WdlException.class, () -> staticValidator.validate(doc));
  }

  @Test
  void lintingValidatorCatchesUnusedSymbols() throws Exception {
    String source = loadFixture("lint_unused_symbols_bad.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlStaticAnalysisValidator staticValidator = new WdlStaticAnalysisValidator();
    WdlLintingValidator lintingValidator = new WdlLintingValidator();

    assertDoesNotThrow(() -> staticValidator.validate(doc));
    WdlException lintEx = assertThrows(WdlException.class, () -> lintingValidator.validate(doc));
    assertTrue(
        lintEx.getErrors().stream().anyMatch(e -> e instanceof WdlSemanticError),
        "Expected at least one semantic lint diagnostic");
    WdlSemanticError lintError =
        (WdlSemanticError)
            lintEx.getErrors().stream()
                .filter(WdlSemanticError.class::isInstance)
                .findFirst()
                .orElseThrow();
    assertEquals(WdlSemanticError.Severity.WARNING, lintError.severity());
  }

  @Test
  void lintingValidatorCanSkipThrowOnWarnings() throws Exception {
    String source = loadFixture("lint_unused_symbols_bad.wdl");

    WdlDocument doc = WdlV1Loader.load(source);
    WdlLintingValidator lintingValidator = new WdlLintingValidator();
    lintingValidator.setThrowOnWarnings(false);

    assertDoesNotThrow(() -> lintingValidator.validate(doc));
  }

  private static String loadFixture(String filename) throws Exception {
    return Files.readString(FIXTURES_ROOT.resolve(filename));
  }
}
