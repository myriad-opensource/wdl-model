package org.openwdl.wdl.model;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.errors.WdlImportException;
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

  @Test
  void throwsOnDirectCircularImports() {
    Path root = FIXTURES_ROOT.resolve("circular").resolve("root.wdl");

    WdlImportException ex =
        assertThrows(WdlImportException.class, () -> WdlV1Loader.load(root.toFile()));

    String message = ex.toDebugMessage();
    assertTrue(message.contains("Circular import detected"));
    assertTrue(message.contains("root.wdl"));
    assertTrue(message.contains("child.wdl"));
  }

  @Test
  void throwsOnCircularImportsWithRelativePathNormalization() {
    Path root = FIXTURES_ROOT.resolve("circular_relative").resolve("root.wdl");

    WdlImportException ex =
        assertThrows(WdlImportException.class, () -> WdlV1Loader.load(root.toFile()));

    String message = ex.toDebugMessage();
    assertTrue(message.contains("Circular import detected"));
    assertTrue(message.contains("root.wdl"));
    assertTrue(
        message.contains("nested/child.wdl")
            || message.contains("nested%2Fchild.wdl")
            || message.contains("nested\\child.wdl"));
  }
}
