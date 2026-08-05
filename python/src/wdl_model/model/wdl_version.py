"""Supported WDL language versions for the Python object model and validators."""

from __future__ import annotations

from enum import Enum


class WdlVersion(Enum):
    """Enumeration of the WDL versions supported by this library."""

    V1_0 = (1, 0, "1.0")
    V1_1 = (1, 1, "1.1")
    V1_2 = (1, 2, "1.2")
    V1_3 = (1, 3, "1.3")

    def __init__(self, major: int, minor: int, versionString: str):
        self.major = major
        self.minor = minor
        self.versionString = versionString

    def getMajor(self) -> int:
        """Return the major version component."""
        return self.major

    def getMinor(self) -> int:
        """Return the minor version component."""
        return self.minor

    def getVersionString(self) -> str:
        """Return the source-level WDL version string, for example ``1.3``."""
        return self.versionString

    @classmethod
    def fromString(cls, versionString: str) -> "WdlVersion":
        """Convert a source-level version string into the matching enum value."""
        for item in cls:
            if item.versionString == versionString:
                return item
        raise ValueError(f"Unsupported WDL version: {versionString}")
