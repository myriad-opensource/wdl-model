/** Struct definition nodes for the TypeScript WDL model. */
import type { WdlNode } from '../base/wdl-node.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';
import type { WdlType } from '../types/wdl-type.js';

/** Marker for nodes that may appear directly inside a struct definition. */
export interface WdlStructElement extends WdlNode {}

/** Models a named typed member declared inside a WDL struct. */
export class WdlStructMember implements WdlStructElement {
  /** Creates a struct member from its type and member name. */
  public constructor(
    private typeValue?: WdlType,
    private nameValue?: string,
  ) {}

  /** Returns the member type. */
  public getType(): WdlType | undefined {
    return this.typeValue;
  }
  /** Sets the member type. */
  public setType(type: WdlType | undefined): void {
    this.typeValue = type;
  }
  /** Returns the member name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the member name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
}

/** Models a WDL struct definition. */
export class WdlStruct {
  private readonly elementValues: WdlStructElement[] = [];
  private sourceRangeValue: WdlSourceRange | undefined;

  /** Creates a struct from its optional declared name. */
  public constructor(private nameValue?: string) {}

  /** Returns the declared struct name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the declared struct name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns the source range of this struct in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this struct. */
  public setSourceRange(range: WdlSourceRange | undefined): void {
    this.sourceRangeValue = range;
  }
  /** Returns the ordered struct elements, usually members and metadata sections. */
  public elements(): WdlStructElement[] {
    return this.elementValues;
  }

  /** Returns whether a member with the supplied name exists. */
  public hasMember(memberName: string | undefined): boolean {
    return this.member(memberName) !== undefined;
  }

  /** Returns the declared member by name, if present. */
  public member(memberName: string | undefined): WdlStructMember | undefined {
    if (!memberName || !memberName.trim()) return undefined;
    for (const element of this.elementValues) {
      if (element instanceof WdlStructMember && element.getName() === memberName) return element;
    }
    return undefined;
  }

  /** Returns the declared member type by name, if present. */
  public memberType(memberName: string | undefined): WdlType | undefined {
    return this.member(memberName)?.getType();
  }
}
