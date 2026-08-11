package com.myriad.wdl.model.resolvers;

import com.myriad.wdl.model.errors.WdlImportException;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * Base import resolver that dispatches URI-like import locations to protocol-specific handlers.
 *
 * <p>Built-in dispatch covers WDL 1.x protocols ({@code http://}, {@code https://}, and
 * deprecated {@code file://}) plus protocol-less paths resolved relative to the current document
 * location. Concrete subclasses provide transport-specific HTTP(S) behavior.
 */
public abstract class WdlImportResolverBase {

  /**
   * Resolves and loads import content.
   *
   * @param currentDocumentLocation location of the document that contains the import statement;
   *     may be null if unknown
   * @param importLocation import URI/path string from the WDL source
   */
  public String resolveImport(URI currentDocumentLocation, String importLocation)
      throws WdlImportException {
    URI resolved = resolveImportLocation(currentDocumentLocation, importLocation);
    return dispatchImport(currentDocumentLocation, resolved, importLocation);
  }

  /** Resolves an import location string into a URI identifier used by loader import maps. */
  public URI resolveImportLocation(URI currentDocumentLocation, String importLocation)
      throws WdlImportException {
    return resolveImportUri(currentDocumentLocation, importLocation);
  }

  /**
   * Dispatches a resolved import URI to a concrete loader.
   *
   * <p>Subclasses can override to support non-standard protocols. A typical pattern is to handle
   * custom schemes first, then call {@code super.dispatchImport(...)} for spec-compliant behavior.
   */
  protected String dispatchImport(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    String scheme = importUri.getScheme();
    if (scheme == null) {
      return loadBarePath(currentDocumentLocation, importUri.getPath(), originalImportLocation);
    }

    switch (scheme.toLowerCase()) {
      case "http":
        return loadHttp(currentDocumentLocation, importUri, originalImportLocation);
      case "https":
        return loadHttps(currentDocumentLocation, importUri, originalImportLocation);
      case "file":
        return loadFile(currentDocumentLocation, importUri, originalImportLocation);
      default:
        throw new WdlImportException(
            "Unsupported import URI protocol: " + importUri.getScheme(), originalImportLocation);
    }
  }

  protected abstract String loadHttp(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException;

  protected abstract String loadHttps(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException;

  protected String loadFile(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    try {
      return Files.readString(Path.of(importUri), StandardCharsets.UTF_8);
    } catch (IOException e) {
      throw ioFailure("Unable to read file import", originalImportLocation, e);
    }
  }

  protected String loadBarePath(
      URI currentDocumentLocation, String resolvedPath, String originalImportLocation)
      throws WdlImportException {
    if (resolvedPath == null || resolvedPath.isBlank()) {
      throw new WdlImportException("Invalid filesystem import path", originalImportLocation);
    }
    try {
      return Files.readString(Path.of(resolvedPath), StandardCharsets.UTF_8);
    } catch (IOException e) {
      throw ioFailure("Unable to read filesystem import", originalImportLocation, e);
    }
  }

  protected URI resolveImportUri(URI currentDocumentLocation, String importLocation)
      throws WdlImportException {
    try {
      URI importUri = new URI(importLocation);
      if (importUri.getScheme() != null) {
        return importUri;
      }

      if (currentDocumentLocation == null) {
        if (importLocation.startsWith("/")) {
          return new URI("file", null, importLocation, null);
        }
        return importUri;
      }

      if (importLocation.startsWith("/")) {
        return new URI(
            currentDocumentLocation.getScheme(),
            currentDocumentLocation.getAuthority(),
            importLocation,
            null,
            null);
      }

      URI base = currentDocumentLocation;
      if (base.getPath() != null && !base.getPath().endsWith("/")) {
        base = base.resolve(".");
      }
      return base.resolve(importUri);
    } catch (URISyntaxException e) {
      throw new WdlImportException("Invalid import URI", importLocation, e);
    }
  }

  protected static WdlImportException ioFailure(
      String message, String importLocation, IOException cause) {
    return new WdlImportException(message, importLocation, cause);
  }
}
