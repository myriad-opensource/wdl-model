package com.myriad.wdl.model.resolvers;

import com.myriad.wdl.model.errors.WdlImportException;
import java.net.URI;

/**
 * Filesystem-only resolver for import locations.
 *
 * <p>Rejects network protocols and delegates filesystem path handling to the base resolver.
 */
public class WdlImportResolverFilesystem extends WdlImportResolverBase {

  @Override
  protected String loadHttp(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    throw new WdlImportException(
        "Filesystem resolver does not support http imports", originalImportLocation);
  }

  @Override
  protected String loadHttps(
      URI currentDocumentLocation, URI importUri, String originalImportLocation)
      throws WdlImportException {
    throw new WdlImportException(
        "Filesystem resolver does not support https imports", originalImportLocation);
  }
}
