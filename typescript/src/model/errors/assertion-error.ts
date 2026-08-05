/** Assertion error used for internal parser invariants that should be unreachable. */
export class AssertionError extends Error {
  public constructor(message: string) {
    super(message);
    this.name = 'AssertionError';
  }
}
