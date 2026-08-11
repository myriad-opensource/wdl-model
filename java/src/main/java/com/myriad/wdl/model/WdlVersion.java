package com.myriad.wdl.model;

import com.myriad.wdl.model.base.WdlNode;
import java.util.Arrays;
import lombok.Getter;

/**
 * Supported WDL language versions.
 *
 * <p>The repository currently models the WDL 1.x family, with validation rules that can depend on
 * when a language feature or standard-library function was introduced.
 */
public enum WdlVersion implements WdlNode {
  V1_0(1, 0, "1.0"),
  V1_1(1, 1, "1.1"),
  V1_2(1, 2, "1.2"),
  V1_3(1, 3, "1.3");

  @Getter private final int major;
  @Getter private final int minor;
  @Getter private final String versionString;

  WdlVersion(int major, int minor, String versionString) {
    this.major = major;
    this.minor = minor;
    this.versionString = versionString;
  }

  /** Converts a source-level version string such as {@code 1.3} into the matching enum. */
  public static WdlVersion fromString(String versionString) {
    return Arrays.stream(WdlVersion.values())
        .filter(v -> v.versionString.equals(versionString))
        .findFirst()
        .orElse(null);
  }
}
