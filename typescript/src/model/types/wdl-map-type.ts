/** Map type nodes. */
import { WdlType, WdlTypeComponentType } from './wdl-type.js';

/** Models a map type such as `Map[String, Int]`. */
export class WdlMapType extends WdlType {
  /** Creates a map type from its key type, value type, and optional flag. */
  public constructor(
    private mapKeyType?: WdlType,
    private mapValueType?: WdlType,
    optional = false,
  ) {
    super(optional);
  }

  /** Returns the broad type family for this node. */
  public componentType(): WdlTypeComponentType {
    return WdlTypeComponentType.MAP;
  }

  /** Returns the map key type. */
  public keyType(): WdlType | undefined {
    return this.mapKeyType;
  }

  /** Sets the map key type. */
  public setKeyType(mapKeyType: WdlType | undefined): void {
    this.mapKeyType = mapKeyType;
  }

  /** Returns the map value type. */
  public valueType(): WdlType | undefined {
    return this.mapValueType;
  }

  /** Sets the map value type. */
  public setValueType(mapValueType: WdlType | undefined): void {
    this.mapValueType = mapValueType;
  }
}
