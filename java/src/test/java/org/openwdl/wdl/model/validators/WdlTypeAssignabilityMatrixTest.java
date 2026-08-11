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

class WdlTypeAssignabilityMatrixTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "type_assignability_matrix");

  static Stream<Arguments> assignabilitySuccessCases() {
    return Stream.of(
        Arguments.of("optional_from_none_ok.wdl"),
        Arguments.of("array_nested_ok.wdl"),
        Arguments.of("map_value_type_ok.wdl"),
        Arguments.of("file_directory_from_string_ok.wdl"),
        Arguments.of("struct_to_struct_coercion_ok.wdl"));
  }

  static Stream<Arguments> assignabilityFailureCases() {
    return Stream.of(
        Arguments.of("required_from_none_fail.wdl"),
        Arguments.of("array_member_type_fail.wdl"),
        Arguments.of("required_string_to_int_fail.wdl"),
        Arguments.of("array_string_to_int_fail.wdl"),
        Arguments.of("map_value_type_fail.wdl"),
        Arguments.of("struct_to_struct_incompatible_fail.wdl"));
  }

  @ParameterizedTest(name = "assignability accepts {0}")
  @MethodSource("assignabilitySuccessCases")
  void acceptsCompatibleAssignments(String fixture) throws Exception {
    Path file = FIXTURES_ROOT.resolve(fixture);
    assertDoesNotThrow(() -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }

  @ParameterizedTest(name = "assignability rejects {0}")
  @MethodSource("assignabilityFailureCases")
  void rejectsIncompatibleAssignments(String fixture) throws Exception {
    Path file = FIXTURES_ROOT.resolve(fixture);
    assertThrows(WdlException.class, () -> WdlV1Loader.load(file.toFile(), new WdlValidator()));
  }
}
