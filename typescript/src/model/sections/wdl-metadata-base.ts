/** Metadata section nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlNode } from '../base/wdl-node.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';

/** Single metadata entry such as `description: "..."`. */
export class WdlMetadataEntry extends WdlStringKeyValue {
  /** Creates a metadata entry from its key and optional expression value. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Base container for ordered metadata entries. */
export class WdlMetadataBase implements WdlNode {
  private readonly elementValues: WdlMetadataEntry[] = [];
  /** Returns the ordered metadata entries in the section. */
  public elements(): WdlMetadataEntry[] {
    return this.elementValues;
  }
}

/** General metadata section. */
export class WdlMetadata extends WdlMetadataBase {}

/** Parameter metadata section. */
export class WdlParameterMetadata extends WdlMetadataBase {}
