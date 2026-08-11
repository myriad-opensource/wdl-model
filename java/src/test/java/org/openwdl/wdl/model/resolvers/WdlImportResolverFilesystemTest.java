package org.openwdl.wdl.model.resolvers;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.myriad.wdl.model.errors.WdlImportException;
import com.myriad.wdl.model.resolvers.WdlImportResolverFilesystem;
import java.net.URI;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class WdlImportResolverFilesystemTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "resolver_filesystem");

  @Test
  void resolvesRelativePathAgainstCurrentDocumentLocation() throws Exception {
    Path root = FIXTURES_ROOT.resolve("root.wdl");

    WdlImportResolverFilesystem resolver = new WdlImportResolverFilesystem();
    String text = resolver.resolveImport(root.toUri(), "sub/imported.wdl");

    assertEquals("version 1.3\n", text);
  }

  @Test
  void resolvesFileSchemeImport() throws Exception {
    Path imported = FIXTURES_ROOT.resolve("sub").resolve("imported.wdl");

    WdlImportResolverFilesystem resolver = new WdlImportResolverFilesystem();
    String text = resolver.resolveImport(null, imported.toUri().toString());

    assertEquals("version 1.3\n", text);
  }

  @Test
  void rejectsHttpImports() {
    WdlImportResolverFilesystem resolver = new WdlImportResolverFilesystem();
    assertThrows(
        WdlImportException.class,
        () ->
            resolver.resolveImport(URI.create("file:///tmp/root.wdl"), "http://example.com/a.wdl"));
  }

  @Test
  void rejectsHttpsImports() {
    WdlImportResolverFilesystem resolver = new WdlImportResolverFilesystem();
    assertThrows(
        WdlImportException.class,
        () ->
            resolver.resolveImport(
                URI.create("file:///tmp/root.wdl"), "https://example.com/a.wdl"));
  }
}
