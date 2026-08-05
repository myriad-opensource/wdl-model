"""Visitor-style processor contract for WDL function-call expressions."""

from __future__ import annotations

from abc import ABC, abstractmethod

from wdl_model.model.expressions import WdlFunction, WdlFunctionCallOperation


class WdlFunctionProcessor(ABC):
    """Abstract dispatch interface for the WDL standard-library function catalog."""

    @abstractmethod
    def processFunctionCall(
        self, functionCall: WdlFunctionCallOperation | None
    ) -> None: ...

    def dispatchByFunction(self, functionCall: WdlFunctionCallOperation | None) -> None:
        """Dispatch a function-call node to the matching per-function hook."""
        if functionCall is None:
            return

        function = functionCall.getFunction()

        if function == WdlFunction.FLOOR:
            self.processFloor(functionCall)
        elif function == WdlFunction.CEIL:
            self.processCeil(functionCall)
        elif function == WdlFunction.ROUND:
            self.processRound(functionCall)
        elif function == WdlFunction.MIN:
            self.processMin(functionCall)
        elif function == WdlFunction.MAX:
            self.processMax(functionCall)
        elif function == WdlFunction.SUB:
            self.processSub(functionCall)
        elif function == WdlFunction.STDOUT:
            self.processStdout(functionCall)
        elif function == WdlFunction.STDERR:
            self.processStderr(functionCall)
        elif function == WdlFunction.READ_LINES:
            self.processReadLines(functionCall)
        elif function == WdlFunction.READ_TSV:
            self.processReadTsv(functionCall)
        elif function == WdlFunction.READ_MAP:
            self.processReadMap(functionCall)
        elif function == WdlFunction.READ_OBJECT:
            self.processReadObject(functionCall)
        elif function == WdlFunction.READ_OBJECTS:
            self.processReadObjects(functionCall)
        elif function == WdlFunction.READ_JSON:
            self.processReadJson(functionCall)
        elif function == WdlFunction.READ_INT:
            self.processReadInt(functionCall)
        elif function == WdlFunction.READ_FLOAT:
            self.processReadFloat(functionCall)
        elif function == WdlFunction.READ_STRING:
            self.processReadString(functionCall)
        elif function == WdlFunction.READ_BOOLEAN:
            self.processReadBoolean(functionCall)
        elif function == WdlFunction.WRITE_LINES:
            self.processWriteLines(functionCall)
        elif function == WdlFunction.WRITE_TSV:
            self.processWriteTsv(functionCall)
        elif function == WdlFunction.WRITE_MAP:
            self.processWriteMap(functionCall)
        elif function == WdlFunction.WRITE_OBJECT:
            self.processWriteObject(functionCall)
        elif function == WdlFunction.WRITE_OBJECTS:
            self.processWriteObjects(functionCall)
        elif function == WdlFunction.WRITE_JSON:
            self.processWriteJson(functionCall)
        elif function == WdlFunction.GLOB:
            self.processGlob(functionCall)
        elif function == WdlFunction.SIZE:
            self.processSize(functionCall)
        elif function == WdlFunction.BASENAME:
            self.processBasename(functionCall)
        elif function == WdlFunction.PREFIX:
            self.processPrefix(functionCall)
        elif function == WdlFunction.SUFFIX:
            self.processSuffix(functionCall)
        elif function == WdlFunction.QUOTE:
            self.processQuote(functionCall)
        elif function == WdlFunction.SQUOTE:
            self.processSquote(functionCall)
        elif function == WdlFunction.SEP:
            self.processSep(functionCall)
        elif function == WdlFunction.LENGTH:
            self.processLength(functionCall)
        elif function == WdlFunction.RANGE:
            self.processRange(functionCall)
        elif function == WdlFunction.CHUNK:
            self.processChunk(functionCall)
        elif function == WdlFunction.CROSS:
            self.processCross(functionCall)
        elif function == WdlFunction.ZIP:
            self.processZip(functionCall)
        elif function == WdlFunction.UNZIP:
            self.processUnzip(functionCall)
        elif function == WdlFunction.TRANSPOSE:
            self.processTranspose(functionCall)
        elif function == WdlFunction.FLATTEN:
            self.processFlatten(functionCall)
        elif function == WdlFunction.SELECT_FIRST:
            self.processSelectFirst(functionCall)
        elif function == WdlFunction.SELECT_ALL:
            self.processSelectAll(functionCall)
        elif function == WdlFunction.CONTAINS:
            self.processContains(functionCall)
        elif function == WdlFunction.CONTAINS_KEY:
            self.processContainsKey(functionCall)
        elif function == WdlFunction.KEYS:
            self.processKeys(functionCall)
        elif function == WdlFunction.VALUES:
            self.processValues(functionCall)
        elif function == WdlFunction.AS_PAIRS:
            self.processAsPairs(functionCall)
        elif function == WdlFunction.AS_MAP:
            self.processAsMap(functionCall)
        elif function == WdlFunction.COLLECT_BY_KEY:
            self.processCollectByKey(functionCall)
        elif function == WdlFunction.MATCHES:
            self.processMatches(functionCall)
        elif function == WdlFunction.FIND:
            self.processFind(functionCall)
        elif function == WdlFunction.DEFINED:
            self.processDefined(functionCall)
        elif function == WdlFunction.JOIN_PATHS:
            self.processJoinPaths(functionCall)
        elif function == WdlFunction.VALUE:
            self.processValue(functionCall)
        elif function == WdlFunction.NONSTANDARD:
            self.processNonstandard(functionCall)
        else:
            raise RuntimeError(f"Unhandled function {function}")

    def processFloor(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processCeil(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processRound(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processMin(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processMax(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSub(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processStdout(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processStderr(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadLines(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadTsv(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadMap(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadObject(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadObjects(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadJson(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadInt(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadFloat(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadString(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processReadBoolean(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteLines(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteTsv(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteMap(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteObject(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteObjects(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processWriteJson(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processGlob(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSize(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processBasename(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processPrefix(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSuffix(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processQuote(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSquote(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSep(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processLength(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processRange(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processChunk(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processCross(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processZip(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processUnzip(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processTranspose(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processFlatten(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSelectFirst(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processSelectAll(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processContains(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processContainsKey(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processKeys(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processValues(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processAsPairs(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processAsMap(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processCollectByKey(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processMatches(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processFind(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processDefined(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processJoinPaths(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processValue(self, functionCall: WdlFunctionCallOperation) -> None:
        pass

    def processNonstandard(self, functionCall: WdlFunctionCallOperation) -> None:
        pass
