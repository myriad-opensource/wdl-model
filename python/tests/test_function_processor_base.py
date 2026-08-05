from __future__ import annotations

from wdl_model.model.expressions import WdlFunctionCallOperation
from wdl_model.model.processors import WdlFunctionProcessorBase


class _RecordingFunctionProcessor(WdlFunctionProcessorBase):
    def __init__(self) -> None:
        self.events: list[str] = []

    def processFloor(self, functionCall: WdlFunctionCallOperation) -> None:
        self.events.append("floor")

    def processNonstandard(self, functionCall: WdlFunctionCallOperation) -> None:
        self.events.append("nonstandard")


def test_dispatches_to_function_specific_methods() -> None:
    processor = _RecordingFunctionProcessor()

    processor.processFunctionCall(WdlFunctionCallOperation("floor"))
    processor.processFunctionCall(WdlFunctionCallOperation("my_custom_function"))

    assert processor.events == ["floor", "nonstandard"]
