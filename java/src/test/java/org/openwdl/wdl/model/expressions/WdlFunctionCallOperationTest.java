package org.openwdl.wdl.model.expressions;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import org.junit.jupiter.api.Test;

class WdlFunctionCallOperationTest {

  @Test
  void setFunctionNameResolvesKnownFunction() {
    WdlFunctionCallOperation op = new WdlFunctionCallOperation("zip");

    assertEquals("zip", op.getFunctionName());
    assertEquals(WdlFunctionCallOperation.WdlFunction.ZIP, op.getFunction());
  }

  @Test
  void setFunctionNameKeepsCustomNonstandardFunctionName() {
    WdlFunctionCallOperation op = new WdlFunctionCallOperation("my_custom_fn");

    assertEquals("my_custom_fn", op.getFunctionName());
    assertEquals(WdlFunctionCallOperation.WdlFunction.NONSTANDARD, op.getFunction());
  }

  @Test
  void setFunctionSetsCanonicalNameForStandardFunction() {
    WdlFunctionCallOperation op = new WdlFunctionCallOperation("my_custom_fn");

    op.setFunction(WdlFunctionCallOperation.WdlFunction.JOIN_PATHS);

    assertEquals("join_paths", op.getFunctionName());
    assertEquals(WdlFunctionCallOperation.WdlFunction.JOIN_PATHS, op.getFunction());
  }
}
