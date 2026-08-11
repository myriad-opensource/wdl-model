/** Primitive type nodes and primitive type names. */
import { WdlType, WdlTypeComponentType } from './wdl-type.js';

/** Models a primitive WDL type such as `Int`, `String`, or `File`. */
export class WdlPrimitiveType extends WdlType {
  /** Creates a primitive type from its primitive name and optional flag. */
  public constructor(
    private primitiveValueType: WdlPrimitiveType.Type = WdlPrimitiveType.Type.STRING,
    optional = false,
  ) {
    super(optional);
  }

  /** Returns the broad type family for this node. */
  public componentType(): WdlTypeComponentType {
    return WdlTypeComponentType.PRIMITIVE;
  }

  /** Returns the primitive type name represented by this node. */
  public primitiveType(): WdlPrimitiveType.Type {
    return this.primitiveValueType;
  }

  /** Sets the primitive type name represented by this node. */
  public setPrimitiveType(primitiveValueType: WdlPrimitiveType.Type): void {
    this.primitiveValueType = primitiveValueType;
  }
}

export namespace WdlPrimitiveType {
  /** Primitive type names supported by the WDL specification. */
  export enum Type {
    BOOLEAN = 'Boolean',
    INT = 'Int',
    FLOAT = 'Float',
    STRING = 'String',
    FILE = 'File',
    DIRECTORY = 'Directory',
    OBJECT = 'Object',
  }
}
