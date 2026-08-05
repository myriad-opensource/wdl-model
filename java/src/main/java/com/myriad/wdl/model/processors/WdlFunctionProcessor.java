package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;

/** Processor contract for function-call expressions using WdlFunction enum dispatch. */
public interface WdlFunctionProcessor {

  /** Process a function-call expression. Implementations own traversal strategy. */
  void processFunctionCall(WdlFunctionCallOperation functionCall);

  /** Dispatch helper based on {@link WdlFunctionCallOperation#getFunction()}. */
  default void dispatchByFunction(WdlFunctionCallOperation functionCall) {
    if (functionCall == null) {
      return;
    }

    switch (functionCall.getFunction()) {
      case FLOOR:
        processFloor(functionCall);
        break;
      case CEIL:
        processCeil(functionCall);
        break;
      case ROUND:
        processRound(functionCall);
        break;
      case MIN:
        processMin(functionCall);
        break;
      case MAX:
        processMax(functionCall);
        break;
      case SUB:
        processSub(functionCall);
        break;
      case STDOUT:
        processStdout(functionCall);
        break;
      case STDERR:
        processStderr(functionCall);
        break;
      case READ_LINES:
        processReadLines(functionCall);
        break;
      case READ_TSV:
        processReadTsv(functionCall);
        break;
      case READ_MAP:
        processReadMap(functionCall);
        break;
      case READ_OBJECT:
        processReadObject(functionCall);
        break;
      case READ_OBJECTS:
        processReadObjects(functionCall);
        break;
      case READ_JSON:
        processReadJson(functionCall);
        break;
      case READ_INT:
        processReadInt(functionCall);
        break;
      case READ_FLOAT:
        processReadFloat(functionCall);
        break;
      case READ_STRING:
        processReadString(functionCall);
        break;
      case READ_BOOLEAN:
        processReadBoolean(functionCall);
        break;
      case WRITE_LINES:
        processWriteLines(functionCall);
        break;
      case WRITE_TSV:
        processWriteTsv(functionCall);
        break;
      case WRITE_MAP:
        processWriteMap(functionCall);
        break;
      case WRITE_OBJECT:
        processWriteObject(functionCall);
        break;
      case WRITE_OBJECTS:
        processWriteObjects(functionCall);
        break;
      case WRITE_JSON:
        processWriteJson(functionCall);
        break;
      case GLOB:
        processGlob(functionCall);
        break;
      case SIZE:
        processSize(functionCall);
        break;
      case BASENAME:
        processBasename(functionCall);
        break;
      case PREFIX:
        processPrefix(functionCall);
        break;
      case SUFFIX:
        processSuffix(functionCall);
        break;
      case QUOTE:
        processQuote(functionCall);
        break;
      case SQUOTE:
        processSquote(functionCall);
        break;
      case SEP:
        processSep(functionCall);
        break;
      case LENGTH:
        processLength(functionCall);
        break;
      case RANGE:
        processRange(functionCall);
        break;
      case CHUNK:
        processChunk(functionCall);
        break;
      case CROSS:
        processCross(functionCall);
        break;
      case ZIP:
        processZip(functionCall);
        break;
      case UNZIP:
        processUnzip(functionCall);
        break;
      case TRANSPOSE:
        processTranspose(functionCall);
        break;
      case FLATTEN:
        processFlatten(functionCall);
        break;
      case SELECT_FIRST:
        processSelectFirst(functionCall);
        break;
      case SELECT_ALL:
        processSelectAll(functionCall);
        break;
      case CONTAINS:
        processContains(functionCall);
        break;
      case CONTAINS_KEY:
        processContainsKey(functionCall);
        break;
      case KEYS:
        processKeys(functionCall);
        break;
      case VALUES:
        processValues(functionCall);
        break;
      case AS_PAIRS:
        processAsPairs(functionCall);
        break;
      case AS_MAP:
        processAsMap(functionCall);
        break;
      case COLLECT_BY_KEY:
        processCollectByKey(functionCall);
        break;
      case MATCHES:
        processMatches(functionCall);
        break;
      case FIND:
        processFind(functionCall);
        break;
      case DEFINED:
        processDefined(functionCall);
        break;
      case JOIN_PATHS:
        processJoinPaths(functionCall);
        break;
      case VALUE:
        processValue(functionCall);
        break;
      case NONSTANDARD:
        processNonstandard(functionCall);
        break;
      default:
        throw new IllegalStateException("Unhandled function: " + functionCall.getFunction());
    }
  }

  default void processFloor(WdlFunctionCallOperation functionCall) {}

  default void processCeil(WdlFunctionCallOperation functionCall) {}

  default void processRound(WdlFunctionCallOperation functionCall) {}

  default void processMin(WdlFunctionCallOperation functionCall) {}

  default void processMax(WdlFunctionCallOperation functionCall) {}

  default void processSub(WdlFunctionCallOperation functionCall) {}

  default void processStdout(WdlFunctionCallOperation functionCall) {}

  default void processStderr(WdlFunctionCallOperation functionCall) {}

  default void processReadLines(WdlFunctionCallOperation functionCall) {}

  default void processReadTsv(WdlFunctionCallOperation functionCall) {}

  default void processReadMap(WdlFunctionCallOperation functionCall) {}

  default void processReadObject(WdlFunctionCallOperation functionCall) {}

  default void processReadObjects(WdlFunctionCallOperation functionCall) {}

  default void processReadJson(WdlFunctionCallOperation functionCall) {}

  default void processReadInt(WdlFunctionCallOperation functionCall) {}

  default void processReadFloat(WdlFunctionCallOperation functionCall) {}

  default void processReadString(WdlFunctionCallOperation functionCall) {}

  default void processReadBoolean(WdlFunctionCallOperation functionCall) {}

  default void processWriteLines(WdlFunctionCallOperation functionCall) {}

  default void processWriteTsv(WdlFunctionCallOperation functionCall) {}

  default void processWriteMap(WdlFunctionCallOperation functionCall) {}

  default void processWriteObject(WdlFunctionCallOperation functionCall) {}

  default void processWriteObjects(WdlFunctionCallOperation functionCall) {}

  default void processWriteJson(WdlFunctionCallOperation functionCall) {}

  default void processGlob(WdlFunctionCallOperation functionCall) {}

  default void processSize(WdlFunctionCallOperation functionCall) {}

  default void processBasename(WdlFunctionCallOperation functionCall) {}

  default void processPrefix(WdlFunctionCallOperation functionCall) {}

  default void processSuffix(WdlFunctionCallOperation functionCall) {}

  default void processQuote(WdlFunctionCallOperation functionCall) {}

  default void processSquote(WdlFunctionCallOperation functionCall) {}

  default void processSep(WdlFunctionCallOperation functionCall) {}

  default void processLength(WdlFunctionCallOperation functionCall) {}

  default void processRange(WdlFunctionCallOperation functionCall) {}

  default void processChunk(WdlFunctionCallOperation functionCall) {}

  default void processCross(WdlFunctionCallOperation functionCall) {}

  default void processZip(WdlFunctionCallOperation functionCall) {}

  default void processUnzip(WdlFunctionCallOperation functionCall) {}

  default void processTranspose(WdlFunctionCallOperation functionCall) {}

  default void processFlatten(WdlFunctionCallOperation functionCall) {}

  default void processSelectFirst(WdlFunctionCallOperation functionCall) {}

  default void processSelectAll(WdlFunctionCallOperation functionCall) {}

  default void processContains(WdlFunctionCallOperation functionCall) {}

  default void processContainsKey(WdlFunctionCallOperation functionCall) {}

  default void processKeys(WdlFunctionCallOperation functionCall) {}

  default void processValues(WdlFunctionCallOperation functionCall) {}

  default void processAsPairs(WdlFunctionCallOperation functionCall) {}

  default void processAsMap(WdlFunctionCallOperation functionCall) {}

  default void processCollectByKey(WdlFunctionCallOperation functionCall) {}

  default void processMatches(WdlFunctionCallOperation functionCall) {}

  default void processFind(WdlFunctionCallOperation functionCall) {}

  default void processDefined(WdlFunctionCallOperation functionCall) {}

  default void processJoinPaths(WdlFunctionCallOperation functionCall) {}

  default void processValue(WdlFunctionCallOperation functionCall) {}

  default void processNonstandard(WdlFunctionCallOperation functionCall) {}
}
