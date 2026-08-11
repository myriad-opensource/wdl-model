from __future__ import annotations

from wdl_model.model.expressions import (
    Delimiter,
    WdlUnaryOperator,
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlBinaryOperator,
    WdlFloatLiteral,
    WdlIntLiteral,
    WdlMapEntry,
    WdlMapLiteral,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStringPlaceholderOption,
    WdlStringPlaceholderOptionType,
    WdlStringText,
    WdlUnaryOperation,
    WdlVariable,
)
from wdl_model.model.processors import WdlExpressionProcessorBase


class _RecordingExpressionProcessor(WdlExpressionProcessorBase):
    def __init__(self) -> None:
        self.events: list[str] = []

    def enterExpression(self, expression) -> None:  # type: ignore[override]
        name = expression.__class__.__name__
        if name.startswith("Wdl"):
            name = name[3:]
        if name.endswith("Operation"):
            name = name[: -len("Operation")]
        if name.endswith("Literal"):
            name = name[: -len("Literal")]
        self.events.append(name.upper())

    def enterStringComponent(self, context, component) -> None:  # type: ignore[override]
        if isinstance(component, WdlStringText):
            self.events.append("SC:TEXT")
        elif isinstance(component, WdlStringPlaceholder):
            self.events.append("SC:PLACEHOLDER")


def test_walks_expressions_depth_first_using_component_type_dispatch() -> None:
    root = WdlArrayLiteral()

    root.entries().append(
        WdlBinaryOperation(WdlIntLiteral(1), WdlBinaryOperator.ADD, WdlIntLiteral(2))
    )

    default_value = WdlStringLiteral(Delimiter.SINGLE_QUOTED)
    default_value.components().append(WdlStringText("d"))

    placeholder = WdlStringPlaceholder(
        option=WdlStringPlaceholderOption(
            type=WdlStringPlaceholderOptionType.DEFAULT, value=default_value
        ),
        expression=WdlVariable("v"),
    )

    string_lit = WdlStringLiteral(Delimiter.SINGLE_QUOTED)
    string_lit.components().append(WdlStringText("pre"))
    string_lit.components().append(placeholder)
    root.entries().append(string_lit)

    map_lit = WdlMapLiteral()
    map_lit.entries().append(
        WdlMapEntry(
            key=WdlVariable("k"),
            value=WdlUnaryOperation(WdlUnaryOperator.MINUS, WdlFloatLiteral(3.0)),
        )
    )
    root.entries().append(map_lit)

    processor = _RecordingExpressionProcessor()
    processor.processExpression(root)

    assert processor.events == [
        "ARRAY",
        "BINARY",
        "INT",
        "INT",
        "STRING",
        "SC:TEXT",
        "SC:PLACEHOLDER",
        "STRING",
        "SC:TEXT",
        "VARIABLE",
        "MAP",
        "VARIABLE",
        "UNARY",
        "FLOAT",
    ]
