/** Node-specific file loader wrapper for the core WDL parser. */
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

import type { WdlDocument } from './wdl-document.js';
import { type WdlValidator, WdlV1Loader } from './wdl-v1-loader.js';
import { WdlImportResolverBase } from './resolvers/wdl-import-resolver-base.js';
import { WdlImportResolverFilesystem } from './resolvers/wdl-import-resolver-filesystem.js';

export class WdlV1NodeLoader {
  /** Reads a UTF-8 WDL file from disk and parses it with the core loader. */
  public static loadFromFile(
    filePath: string,
    validator?: WdlValidator,
    importResolver: WdlImportResolverBase = new WdlImportResolverFilesystem(),
  ): WdlDocument {
    const absolutePath = resolve(filePath);
    return WdlV1Loader.loadFromString(
      readFileSync(absolutePath, 'utf8'),
      validator,
      pathToFileURL(absolutePath).toString(),
      importResolver,
    );
  }
}
