package org.openwdl.wdl.model.expressions;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import org.junit.jupiter.api.Test;

class WdlFunctionTest {

  @Test
  void fromWdlStringFindsKnownFunction() {
    WdlFunctionCallOperation.WdlFunction fn =
        WdlFunctionCallOperation.WdlFunction.fromWdlString("zip");
    assertEquals(WdlFunctionCallOperation.WdlFunction.ZIP, fn);
    assertEquals(2, fn.getMinArity());
    assertEquals(2, fn.getMaxArity());
  }

  @Test
  void fromWdlStringFallsBackToNonstandard() {
    assertEquals(
        WdlFunctionCallOperation.WdlFunction.NONSTANDARD,
        WdlFunctionCallOperation.WdlFunction.fromWdlString("my_custom_function"));
  }

  @Test
  void variadicArityUsesUnboundedSentinel() {
    WdlFunctionCallOperation.WdlFunction fn = WdlFunctionCallOperation.WdlFunction.JOIN_PATHS;
    assertTrue(fn.isVariadic());
    assertTrue(fn.supportsArity(2));
    assertTrue(fn.supportsArity(10));
    assertFalse(fn.supportsArity(1));
  }

  @Test
  void signaturesExposeKnownTypeHints() {
    WdlFunctionCallOperation.WdlFunction.FunctionSignature sig =
        WdlFunctionCallOperation.WdlFunction.READ_INT.getSignatures().get(0);

    assertEquals(WdlFunctionCallOperation.WdlFunction.T.INT, sig.getReturns());
    assertEquals(1, sig.getArgs().size());
    assertEquals(WdlFunctionCallOperation.WdlFunction.T.FILE, sig.getArgs().get(0));
  }

  @Test
  void versionMetadataIsExposed() {
    assertEquals(WdlVersion.V1_1, WdlFunctionCallOperation.WdlFunction.MIN.getAddedIn());
    assertEquals(WdlVersion.V1_2, WdlFunctionCallOperation.WdlFunction.JOIN_PATHS.getAddedIn());
    assertEquals(WdlVersion.V1_3, WdlFunctionCallOperation.WdlFunction.VALUE.getAddedIn());

    assertNull(WdlFunctionCallOperation.WdlFunction.MIN.getDeprecatedIn());
    assertNull(WdlFunctionCallOperation.WdlFunction.MIN.getRemovedIn());
  }

  @Test
  void nonstandardEntryIsVariadicAndVersionless() {
    WdlFunctionCallOperation.WdlFunction fn = WdlFunctionCallOperation.WdlFunction.NONSTANDARD;

    assertEquals("nonstandard", fn.toWdlString());
    assertTrue(fn.isVariadic());
    assertTrue(fn.supportsArity(0));
    assertTrue(fn.supportsArity(99));

    assertNull(fn.getAddedIn());
    assertNull(fn.getDeprecatedIn());
    assertNull(fn.getRemovedIn());
  }
}
