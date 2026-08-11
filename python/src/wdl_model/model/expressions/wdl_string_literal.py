"""String literal and placeholder expression nodes.

WDL strings are richer than plain scalar literals because they can contain interpolation
placeholders and command-template fragments.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from enum import Enum

from .wdl_expression import ComponentType, WdlExpression


class Delimiter(Enum):
    """Source-level string delimiter forms used by WDL literals and command templates."""

    SINGLE_QUOTED = '"'
    DOUBLE_ANGLE = "<<<>>>"


class WdlStringComponent:
    """Base interface for a component inside a parsed WDL string literal."""

    class ComponentType(Enum):
        TEXT = "TEXT"
        ESCAPE = "ESCAPE"
        TOKEN = "TOKEN"
        PLACEHOLDER = "PLACEHOLDER"

    def componentType(self) -> "WdlStringComponent.ComponentType":
        raise NotImplementedError


@dataclass
class WdlStringText(WdlStringComponent):
    """Raw text fragment inside a string literal."""

    text: str | None = None

    def componentType(self) -> WdlStringComponent.ComponentType:
        return WdlStringComponent.ComponentType.TEXT


@dataclass
class WdlStringEscape(WdlStringComponent):
    """Escape sequence fragment inside a string literal."""

    escapeText: str | None = None

    def componentType(self) -> WdlStringComponent.ComponentType:
        return WdlStringComponent.ComponentType.ESCAPE


@dataclass
class WdlStringToken(WdlStringComponent):
    """Token-preserving fragment used when rebuilding string text."""

    tokenText: str | None = None

    def componentType(self) -> WdlStringComponent.ComponentType:
        return WdlStringComponent.ComponentType.TOKEN


class PlaceHolderSymbol(Enum):
    """Supported placeholder sigils for WDL string interpolation."""

    TILDE = "~"
    DOLLAR = "$"

    def getWdlString(self) -> str:
        return self.value


class Type(Enum):
    """Placeholder option families supported by older WDL placeholder syntax."""

    DEFAULT = "default"
    TRUE_FALSE = "true_false"


@dataclass
class WdlStringPlaceholderOption:
    """Option block attached to a string placeholder, such as default or true/false."""

    type: Type
    value: "WdlStringLiteral" | None = None
    trueValue: "WdlStringLiteral" | None = None
    falseValue: "WdlStringLiteral" | None = None

    def getType(self) -> Type:
        return self.type

    def getValue(self) -> "WdlStringLiteral" | None:
        return self.value

    def setValue(self, value: "WdlStringLiteral") -> None:
        self.value = value

    def getTrueValue(self) -> "WdlStringLiteral" | None:
        return self.trueValue

    def setTrueValue(self, value: "WdlStringLiteral") -> None:
        self.trueValue = value

    def getFalseValue(self) -> "WdlStringLiteral" | None:
        return self.falseValue

    def setFalseValue(self, value: "WdlStringLiteral") -> None:
        self.falseValue = value


@dataclass
class WdlStringPlaceholder(WdlStringComponent):
    """Interpolated placeholder embedded in a string literal or command template."""

    option: WdlStringPlaceholderOption | None = None
    expression: WdlExpression | None = None
    symbol: PlaceHolderSymbol = PlaceHolderSymbol.TILDE

    def componentType(self) -> WdlStringComponent.ComponentType:
        return WdlStringComponent.ComponentType.PLACEHOLDER


@dataclass
class WdlStringLiteral(WdlExpression):
    """String literal expression composed of ordered text and placeholder fragments."""

    delimiter: Delimiter = Delimiter.SINGLE_QUOTED
    _components: deque[WdlStringComponent] = field(default_factory=deque)

    def components(self) -> deque[WdlStringComponent]:
        """Return the ordered string components that make up the literal."""
        return self._components

    def componentType(self) -> ComponentType:
        return ComponentType.LITERAL

    def __str__(self) -> str:
        chunks: list[str] = []
        for component in self._components:
            if isinstance(component, WdlStringText):
                chunks.append(component.text or "")
            elif isinstance(component, WdlStringEscape):
                chunks.append(component.escapeText or "")
            elif isinstance(component, WdlStringToken):
                chunks.append(component.tokenText or "")
            elif isinstance(component, WdlStringPlaceholder):
                chunks.append(
                    f"{component.symbol.value}{{{component.expression or ''}}}"
                )
        return "".join(chunks)
