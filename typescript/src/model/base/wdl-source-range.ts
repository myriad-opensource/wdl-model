/** Source text span attached to AST nodes from ANTLR token positions. */

/**
 * Source range for a parsed WDL node.
 *
 * All line values are 1-based; all column values are 0-based, matching ANTLR convention.
 * `endColumn` is the exclusive end position of the last token.
 */
export interface WdlSourceRange {
  readonly startLine: number;
  readonly startColumn: number;
  readonly endLine: number;
  readonly endColumn: number;
}
