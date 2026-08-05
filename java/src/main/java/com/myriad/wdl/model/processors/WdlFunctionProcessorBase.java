package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;

/** Base function-call processor that dispatches by {@link WdlFunctionCallOperation.WdlFunction}. */
public abstract class WdlFunctionProcessorBase implements WdlFunctionProcessor {

  @Override
  public final void processFunctionCall(WdlFunctionCallOperation functionCall) {
    if (functionCall == null) {
      return;
    }
    dispatchByFunction(functionCall);
  }
}
