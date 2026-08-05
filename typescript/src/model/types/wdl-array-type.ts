/** Array type nodes. */
import { WdlType, WdlTypeComponentType } from './wdl-type.js';

/** Models an array type such as `Array[Int]` or `Array[String]+`. */
export class WdlArrayType extends WdlType {
  /** Creates an array type from its member type, non-empty flag, and optional flag. */
  public constructor(
    private arrayMemberType?: WdlType,
    private nonEmpty = false,
    optional = false,
  ) {
    super(optional);
  }

  /** Returns the broad type family for this node. */
  public componentType(): WdlTypeComponentType {
    return WdlTypeComponentType.ARRAY;
  }

  /** Returns the member type stored in the array. */
  public memberType(): WdlType | undefined {
    return this.arrayMemberType;
  }

  /** Sets the member type stored in the array. */
  public setMemberType(arrayMemberType: WdlType | undefined): void {
    this.arrayMemberType = arrayMemberType;
  }

  /** Returns whether the array has the WDL non-empty (`+`) marker. */
  public isNonEmpty(): boolean {
    return this.nonEmpty;
  }

  /** Sets whether the array has the WDL non-empty (`+`) marker. */
  public setNonEmpty(nonEmpty: boolean): void {
    this.nonEmpty = nonEmpty;
  }
}
