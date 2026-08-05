package org.openwdl.wdl.model.processors;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.processors.WdlAppendingProcessor;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class WdlAppendingProcessorTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "appending_processor");

  @Test
  void rendersRecursiveWorkflowStatements() throws Exception {
    WdlDocument document =
        WdlV1Loader.load(FIXTURES_ROOT.resolve("recursive_workflow_statements.wdl").toFile());

    StringBuilder rendered = new StringBuilder();
    WdlAppendingProcessor processor = new WdlAppendingProcessor(rendered);
    processor.processDocument(document);

    String out = rendered.toString();
    assertTrue(out.contains("if (x == 1) {"));
    assertTrue(out.contains("scatter (n in [1, 2]) {"));
    assertTrue(out.contains("Int y = n"));
    assertFalse(out.contains("{ ... }"));
  }

  @Test
  void rendersMetadataContent() throws Exception {
    WdlDocument document = WdlV1Loader.load(FIXTURES_ROOT.resolve("metadata_content.wdl").toFile());

    StringBuilder rendered = new StringBuilder();
    WdlAppendingProcessor processor = new WdlAppendingProcessor(rendered);
    processor.processDocument(document);

    String out = rendered.toString();
    assertTrue(out.contains("meta {"));
    assertTrue(out.contains("author:"));
    assertTrue(out.contains("parameter_meta {"));
    assertTrue(out.contains("x:"));
    assertFalse(out.contains("meta { ... }"));
    assertFalse(out.contains("parameter_meta { ... }"));
  }
}
