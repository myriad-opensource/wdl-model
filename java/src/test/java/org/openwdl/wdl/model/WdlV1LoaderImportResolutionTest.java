package org.openwdl.wdl.model;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.resolvers.WdlImportResolverFilesystem;
import com.myriad.wdl.model.validators.WdlValidator;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class WdlV1LoaderImportResolutionTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "loader_imports");

  @Test
  void recursivelyLoadsImportedDocumentsIntoMap() throws Exception {
    Path root = FIXTURES_ROOT.resolve("recursive").resolve("root.wdl");
    Path child = FIXTURES_ROOT.resolve("recursive").resolve("child.wdl");
    Path grandchild = FIXTURES_ROOT.resolve("recursive").resolve("grandchild.wdl");

    WdlDocument rootDoc = WdlV1Loader.load(root.toFile());

    assertEquals(1, rootDoc.importedDocuments().size());
    String childKey = rootDoc.importedDocuments().keySet().iterator().next();
    WdlDocument childDoc = rootDoc.importedDocuments().get(childKey);
    assertNotNull(childDoc);
    assertEquals(
        child.toFile().getAbsolutePath(),
        Path.of(childDoc.getSourceLocation()).toFile().getAbsolutePath());

    assertEquals(1, childDoc.importedDocuments().size());
    String grandchildKey = childDoc.importedDocuments().keySet().iterator().next();
    WdlDocument grandchildDoc = childDoc.importedDocuments().get(grandchildKey);
    assertNotNull(grandchildDoc);
    assertEquals(
        grandchild.toFile().getAbsolutePath(),
        Path.of(grandchildDoc.getSourceLocation()).toFile().getAbsolutePath());

    assertNotNull(rootDoc.importStatements().get(0).getSourceText());
    assertNotNull(childDoc.importStatements().get(0).getSourceText());
  }

  @Test
  void loadsFromSourceCodeWithSourceLocationResolverThenValidator() throws Exception {
    Path root = FIXTURES_ROOT.resolve("string_input").resolve("root.wdl");
    String rootSource = Files.readString(root);

    WdlDocument rootDoc =
        WdlV1Loader.load(
            rootSource, root.toUri(), new WdlImportResolverFilesystem(), new WdlValidator());

    assertEquals(1, rootDoc.importStatements().size());
    assertEquals(1, rootDoc.importedDocuments().size());
    assertNotNull(rootDoc.importedDocuments().values().iterator().next());
  }
}
