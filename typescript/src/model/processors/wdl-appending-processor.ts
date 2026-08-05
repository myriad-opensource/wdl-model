/** Processor that renders the TypeScript WDL model back into source text. */
import { WdlProcessorBase } from './wdl-processor-base.js';

export class WdlAppendingProcessor extends WdlProcessorBase {
  private readonly chunks: string[] = [];

  /** Appends raw text to the internal buffer and returns `this` for chaining. */
  public append(value: string): this {
    this.chunks.push(value);
    return this;
  }

  /** Returns the accumulated WDL text. */
  public getValue(): string {
    return this.chunks.join('');
  }
}
