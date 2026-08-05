/** Import statement nodes for the TypeScript WDL model. */
import type { WdlStringLiteral } from '../expressions/wdl-string-literal.js';

/** Base class for the three WDL import forms. */
export abstract class WdlImport {
  private importIdentifierValue: string | undefined;
  private sourceTextValue: string | undefined;

  /** Creates an import from its source literal. */
  public constructor(protected sourceValue?: WdlStringLiteral) {}
  /** Returns the source literal naming the imported document. */
  public getSource(): WdlStringLiteral | undefined {
    return this.sourceValue;
  }
  /** Sets the source literal naming the imported document. */
  public setSource(source: WdlStringLiteral | undefined): void {
    this.sourceValue = source;
  }

  /** Returns the resolved identifier used to index imported documents. */
  public getImportIdentifier(): string | undefined {
    return this.importIdentifierValue;
  }

  /** Sets the resolved identifier used to index imported documents. */
  public setImportIdentifier(importIdentifier: string | undefined): void {
    this.importIdentifierValue = importIdentifier;
  }

  /** Returns the raw source text loaded for this import, when available. */
  public getSourceText(): string | undefined {
    return this.sourceTextValue;
  }

  /** Sets the raw source text loaded for this import. */
  public setSourceText(sourceText: string | undefined): void {
    this.sourceTextValue = sourceText;
  }
}

/** Models a single imported member and optional local alias. */
export class WdlImportMember {
  /** Creates an import member from its source name and optional local alias. */
  public constructor(
    private memberValue?: string,
    private aliasValue?: string,
  ) {}
  /** Returns the imported member name. */
  public getMember(): string | undefined {
    return this.memberValue;
  }
  /** Sets the imported member name. */
  public setMember(member: string | undefined): void {
    this.memberValue = member;
  }
  /** Returns the local alias for the imported member. */
  public getAlias(): string | undefined {
    return this.aliasValue;
  }
  /** Sets the local alias for the imported member. */
  public setAlias(alias: string | undefined): void {
    this.aliasValue = alias;
  }
}

/** Models the standard `import "x" as y` form with optional alias clauses. */
export class WdlImportStandard extends WdlImport {
  private readonly memberValues: WdlImportMember[] = [];
  private aliasValue: string | undefined;
  /** Returns the import namespace alias. */
  public getAlias(): string | undefined {
    return this.aliasValue;
  }
  /** Sets the import namespace alias. */
  public setAlias(alias: string | undefined): void {
    this.aliasValue = alias;
  }
  /** Returns the ordered imported type/member alias clauses attached to the import. */
  public members(): WdlImportMember[] {
    return this.memberValues;
  }
}

/** Models `import * from "x"`. */
export class WdlImportStar extends WdlImport {}

/** Models `import { a, b as c } from "x"`. */
export class WdlImportMembers extends WdlImport {
  private readonly memberValues: WdlImportMember[] = [];
  /** Returns the ordered imported members selected by this import. */
  public members(): WdlImportMember[] {
    return this.memberValues;
  }
}
