package org.openwdl.wdl.model.v1;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.stream.Stream;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;

/**
 * Tests for WDL v1.2 spec examples parsing. Runs through all .wdl files in spec_examples/v1_2 and
 * attempts to parse them. Some files have "_fail" suffix but may still parse successfully (they
 * have semantic failures we don't catch yet).
 */
@DisplayName("WDL v1.2 Spec Examples")
public class WdlV12SpecExamplesTest {

  private static final Pattern IMPORT_PATTERN = Pattern.compile("(?m)^\\s*import\\s+\"");
  private static final Pattern REMOTE_IMPORT_PATTERN =
      Pattern.compile("(?m)^\\s*import\\s+\"https?://");

  private static Set<String> failsThatShouldParseOkV1_2 =
      new HashSet<>(
          Arrays.asList(
              "select_first_only_none_fail.wdl",
              "empty_array_fail.wdl",
              "test_as_map_fail.wdl",
              "write_json_fail.wdl",
              "test_map_fail.wdl",
              "select_first_empty_fail.wdl",
              "private_declaration_fail.wdl",
              "non_empty_optional_fail.wdl",
              "test_zip_fail.wdl",
              "illegal_access_fail.wdl"));

  private static Set<String> parseFailuresExpectedWithReservedKeywordsV1_2 =
      new HashSet<>(
          Arrays.asList(
              "test_find_task.wdl", "test_meta_values.wdl", "test_runtime_info_task.wdl"));

  static Stream<org.junit.jupiter.params.provider.Arguments> v12Examples() throws Exception {
    return WdlTestResources.loadWdlExamples("v1_2");
  }

  static Stream<org.junit.jupiter.params.provider.Arguments> v12FailExamples() throws Exception {
    return v12Examples().filter(a -> ((String) a.get()[0]).endsWith("_fail.wdl"));
  }

  @ParameterizedTest(name = "{0}")
  @MethodSource("v12Examples")
  @DisplayName("Parse WDL v1.2 spec example")
  void testParseSpecExample(String filename, Path filePath) throws Exception {
    String wdlContent = java.nio.file.Files.readString(filePath);
    boolean hasImports = IMPORT_PATTERN.matcher(wdlContent).find();
    boolean hasRemoteImports = REMOTE_IMPORT_PATTERN.matcher(wdlContent).find();

    try {
      WdlDocument doc =
          (hasImports && !hasRemoteImports)
              ? WdlV1Loader.load(wdlContent, filePath.toUri())
              : WdlV1Loader.load(wdlContent);
      org.junit.jupiter.api.Assertions.assertNotNull(doc);
      org.junit.jupiter.api.Assertions.assertNotNull(doc.elements());
      if (hasImports && !hasRemoteImports) {
        assertImportMapsPopulated(doc, filename);
      }

      // Files ending in _fail may or may not parse successfully (semantic failures aren't caught)
      if (filename.endsWith("_fail.wdl") && !failsThatShouldParseOkV1_2.contains(filename)) {
        throw new AssertionError("Parsed but failure expected: " + filename);
      }
    } catch (WdlException e) {
      // Files ending in _fail are expected to potentially fail
      if (!filename.endsWith("_fail.wdl")
          && !parseFailuresExpectedWithReservedKeywordsV1_2.contains(filename)) {
        throw new AssertionError("Failed to parse " + filename + ": " + e.toDebugMessage(), e);
      }
    }
  }

  @ParameterizedTest(name = "{0}")
  @MethodSource("v12FailExamples")
  @DisplayName("Parse + validate WDL v1.2 fail examples")
  void testParseAndValidateFailSpecExample(String filename, Path filePath) throws Exception {
    String wdlContent = java.nio.file.Files.readString(filePath);

    org.junit.jupiter.api.Assertions.assertThrows(
        WdlException.class, () -> WdlV1Loader.load(wdlContent, new WdlValidator()), filename);
  }

  private static void assertImportMapsPopulated(WdlDocument doc, String filename) {
    if (doc.importStatements().isEmpty()) {
      return;
    }

    org.junit.jupiter.api.Assertions.assertFalse(
        doc.importedDocuments().isEmpty(),
        "Expected importedDocuments map to be populated for " + filename);
    org.junit.jupiter.api.Assertions.assertEquals(
        doc.importStatements().size(),
        doc.importedDocuments().size(),
        "Expected one map entry per import statement for " + filename);
    doc.importedDocuments()
        .forEach(
            (key, importedDoc) -> org.junit.jupiter.api.Assertions.assertNotNull(importedDoc, key));
  }
}
