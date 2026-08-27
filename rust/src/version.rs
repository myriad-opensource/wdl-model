//! Supported WDL language versions.

/// Supported WDL language versions (WDL 1.x family).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WdlVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
}

impl WdlVersion {
    pub fn major(self) -> u32 {
        1
    }

    pub fn minor(self) -> u32 {
        match self {
            WdlVersion::V1_0 => 0,
            WdlVersion::V1_1 => 1,
            WdlVersion::V1_2 => 2,
            WdlVersion::V1_3 => 3,
        }
    }

    pub fn version_string(self) -> &'static str {
        match self {
            WdlVersion::V1_0 => "1.0",
            WdlVersion::V1_1 => "1.1",
            WdlVersion::V1_2 => "1.2",
            WdlVersion::V1_3 => "1.3",
        }
    }

    /// Converts a source-level version string such as `"1.3"` into the matching variant.
    /// Returns `None` if the string is not a known version.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1.0" => Some(WdlVersion::V1_0),
            "1.1" => Some(WdlVersion::V1_1),
            "1.2" => Some(WdlVersion::V1_2),
            "1.3" => Some(WdlVersion::V1_3),
            _ => None,
        }
    }
}

impl std::fmt::Display for WdlVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version_string())
    }
}
