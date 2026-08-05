"""Default function-call dispatch helper for the Python WDL object model."""

from __future__ import annotations

from wdl_model.model.expressions import WdlFunctionCallOperation

from .wdl_function_processor import WdlFunctionProcessor


class WdlFunctionProcessorBase(WdlFunctionProcessor):
    """Base implementation that forwards a function call into ``dispatchByFunction``."""

    def processFunctionCall(
        self, functionCall: WdlFunctionCallOperation | None
    ) -> None:
        if functionCall is None:
            return
        self.dispatchByFunction(functionCall)
