/** Generic key/value helper nodes used across the TypeScript WDL model. */
import type { WdlExpression } from '../expressions/wdl-expression.js';
import type { WdlNode } from './wdl-node.js';

/** Base class for ordered key/value nodes such as metadata entries or call inputs. */
export abstract class WdlKeyValue<K, V> implements WdlNode {
  /** Creates a key/value node from its key and value payloads. */
  public constructor(
    private keyValue?: K,
    private valueValue?: V,
  ) {}

  /** Returns the key portion of the node. */
  public getKey(): K | undefined {
    return this.keyValue;
  }

  /** Sets the key portion of the node. */
  public setKey(key: K | undefined): void {
    this.keyValue = key;
  }

  /** Returns the value portion of the node. */
  public getValue(): V | undefined {
    return this.valueValue;
  }

  /** Sets the value portion of the node. */
  public setValue(value: V | undefined): void {
    this.valueValue = value;
  }
}

/** String-keyed expression value pair. */
export abstract class WdlStringKeyValue extends WdlKeyValue<string, WdlExpression> {}

/** Expression-keyed expression value pair. */
export abstract class WdlExpressionKeyValue extends WdlKeyValue<WdlExpression, WdlExpression> {}
