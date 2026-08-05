/** Base diagnostic types for parsing and validation failures. */
export abstract class WdlError {
  /** Creates a diagnostic from its message and source location. */
  public constructor(
    public readonly message: string,
    public readonly line: number,
    public readonly charPositionInLine: number,
  ) {}

  /** Returns a compact debug-oriented rendering of the diagnostic. */
  public toDebugMessage(): string {
    return `${this.constructor.name}:${this.line}:${this.charPositionInLine}:${this.message}`;
  }
}
