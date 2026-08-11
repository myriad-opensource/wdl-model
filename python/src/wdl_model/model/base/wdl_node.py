"""Base node type for the Python WDL object model.

The library models a WDL document as a tree of nodes representing top-level definitions,
statements, sections, expressions, and types. See the WDL 1.3 specification sections "WDL
Documents", "Task Definition", and "Workflow Definition" for the source-language concepts.
"""


class WdlNode:
    """Marker base class matching Java WdlNode interface."""
