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

class WdlStaticFunctionSignatureMatrixTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "static_function_signature_matrix");

  static Stream<Arguments> staticOnlyFailures() {
    return Stream.of(
        Arguments.of("keys_bad.wdl"),
        Arguments.of("range_bad.wdl"),
        Arguments.of("contains_bad.wdl"),
        Arguments.of("chunk_bad.wdl"),
        Arguments.of("cross_bad.wdl"),
        Arguments.of("join_paths_bad_first.wdl"),
        Arguments.of("join_paths_bad_tail.wdl"),
        Arguments.of("basename_bad_first.wdl"),
        Arguments.of("size_bad_second.wdl"));
  }

  @ParameterizedTest(name = "static rejects {0}")
  @MethodSource("staticOnlyFailures")
  void rejectsInvalidSignaturesUnderStaticValidator(String fixture) throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve(fixture).toFile());
    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertThrows(WdlException.class, () -> new WdlStaticAnalysisValidator().validate(doc));
  }

  @Test
  void acceptsValidSignaturesUnderStaticValidator() throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve("static_signatures_ok.wdl").toFile());
    assertDoesNotThrow(() -> new WdlValidator().validate(doc));
    assertDoesNotThrow(() -> new WdlStaticAnalysisValidator().validate(doc));
  }
}
