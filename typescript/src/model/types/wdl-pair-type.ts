/** Pair type nodes. */
import { WdlType, WdlTypeComponentType } from './wdl-type.js';

/** Models a pair type such as `Pair[String, Int]`. */
export class WdlPairType extends WdlType {
  /** Creates a pair type from its left type, right type, and optional flag. */
  public constructor(
    private pairLeftType?: WdlType,
    private pairRightType?: WdlType,
    optional = false,
  ) {
    super(optional);
  }

  /** Returns the broad type family for this node. */
  public componentType(): WdlTypeComponentType {
    return WdlTypeComponentType.PAIR;
  }

  /** Returns the left member type. */
  public leftType(): WdlType | undefined {
    return this.pairLeftType;
  }

  /** Sets the left member type. */
  public setLeftType(pairLeftType: WdlType | undefined): void {
    this.pairLeftType = pairLeftType;
  }

  /** Returns the right member type. */
  public rightType(): WdlType | undefined {
    return this.pairRightType;
  }

  /** Sets the right member type. */
  public setRightType(pairRightType: WdlType | undefined): void {
    this.pairRightType = pairRightType;
  }
}
