/** User-defined type reference nodes. */
import { WdlType, WdlTypeComponentType } from './wdl-type.js';

/** Models a reference to a named user-defined type such as a struct or enum. */
export class WdlTypeReferenceType extends WdlType {
  /** Creates a type reference from its target type name and optional flag. */
  public constructor(
    private referencedTypeName = '',
    optional = false,
  ) {
    super(optional);
  }

  /** Returns the broad type family for this node. */
  public componentType(): WdlTypeComponentType {
    return WdlTypeComponentType.TYPE_REFERENCE;
  }

  /** Returns the referenced type name. */
  public referenceName(): string {
    return this.referencedTypeName;
  }

  /** Sets the referenced type name. */
  public setReferenceName(referencedTypeName: string): void {
    this.referencedTypeName = referencedTypeName;
  }
}
