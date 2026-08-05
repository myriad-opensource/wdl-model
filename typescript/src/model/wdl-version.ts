/** Supported WDL language versions. */
import type { WdlNode } from './base/wdl-node.js';

/** Models the supported WDL version set used by the parser and validators. */
export class WdlVersion implements WdlNode {
  public static readonly V1_0 = new WdlVersion(1, 0, '1.0');
  public static readonly V1_1 = new WdlVersion(1, 1, '1.1');
  public static readonly V1_2 = new WdlVersion(1, 2, '1.2');
  public static readonly V1_3 = new WdlVersion(1, 3, '1.3');

  /** Returns all supported version values. */
  public static values(): readonly WdlVersion[] {
    return [WdlVersion.V1_0, WdlVersion.V1_1, WdlVersion.V1_2, WdlVersion.V1_3];
  }

  /** Converts a source-level version string such as `1.3` into the matching model value. */
  public static fromString(versionString: string): WdlVersion {
    const match = WdlVersion.values().find((value) => value.versionString === versionString);
    if (!match) {
      throw new Error(`Unsupported WDL version: ${versionString}`);
    }
    return match;
  }

  /** Creates a version value from its major/minor components and display string. */
  private constructor(
    public readonly major: number,
    public readonly minor: number,
    public readonly versionString: string,
  ) {}

  /** Returns the source-level version string. */
  public getVersionString(): string {
    return this.versionString;
  }
}
