/** Call statement nodes for the TypeScript WDL model. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';
import { WdlStatementComponentType, type WdlStatement } from './wdl-statement.js';

/** Single call input binding such as `x = expr`. */
export class WdlCallInput extends WdlStringKeyValue {
  /** Creates a call input binding from its input name and optional expression. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Models a workflow call statement. */
export class WdlCall implements WdlStatement, WdlWorkflowElement {
  private readonly targetPathValues: string[] = [];
  private readonly inputValues: WdlCallInput[] = [];
  private readonly afterDependencyValues: string[] = [];
  private aliasValue: string | undefined;
  private legacyInputColonUsedValue = false;

  /** Returns the target path segments for the invoked task or workflow. */
  public targetPath(): string[] {
    return this.targetPathValues;
  }
  /** Returns the target path as dotted source text. */
  public targetPathAsString(): string {
    return this.targetPathValues.join('.');
  }
  /** Returns the call alias if one was declared. */
  public getAlias(): string | undefined {
    return this.aliasValue;
  }
  /** Sets the call alias. */
  public setAlias(alias: string | undefined): void {
    this.aliasValue = alias;
  }
  /** Returns the ordered input bindings supplied to the call. */
  public inputs(): WdlCallInput[] {
    return this.inputValues;
  }
  /** Returns the ordered `after` dependencies for the call. */
  public afterDependencies(): string[] {
    return this.afterDependencyValues;
  }
  /** Returns whether the legacy `input:` prefix syntax was used. */
  public isLegacyInputColonUsed(): boolean {
    return this.legacyInputColonUsedValue;
  }
  /** Sets whether the legacy `input:` prefix syntax was used. */
  public setLegacyInputColonUsed(legacy: boolean): void {
    this.legacyInputColonUsedValue = legacy;
  }
  /** Returns the broad statement family for this node. */
  public componentType(): WdlStatementComponentType {
    return WdlStatementComponentType.CALL;
  }
}
