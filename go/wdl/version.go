package wdl

import "fmt"

// Version represents a WDL language version.
type Version struct {
	Major int
	Minor int
}

var (
	// Version10 is WDL language version 1.0.
	Version10 = Version{Major: 1, Minor: 0}
	// Version11 is WDL language version 1.1.
	Version11 = Version{Major: 1, Minor: 1}
	// Version12 is WDL language version 1.2.
	Version12 = Version{Major: 1, Minor: 2}
	// Version13 is WDL language version 1.3.
	Version13 = Version{Major: 1, Minor: 3}
)

// String returns the canonical "major.minor" representation.
func (v Version) String() string {
	return fmt.Sprintf("%d.%d", v.Major, v.Minor)
}

// LessThan reports whether v is older than other.
func (v Version) LessThan(other Version) bool {
	if v.Major != other.Major {
		return v.Major < other.Major
	}
	return v.Minor < other.Minor
}

// ParseVersion parses a supported WDL version string (for example, "1.3").
func ParseVersion(s string) (Version, error) {
	switch s {
	case "1.0":
		return Version10, nil
	case "1.1":
		return Version11, nil
	case "1.2":
		return Version12, nil
	case "1.3":
		return Version13, nil
	default:
		return Version{}, fmt.Errorf("unsupported WDL version: %s", s)
	}
}
