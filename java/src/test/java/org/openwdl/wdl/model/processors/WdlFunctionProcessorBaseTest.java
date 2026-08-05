package org.openwdl.wdl.model.processors;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.processors.WdlFunctionProcessorBase;
import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;

class WdlFunctionProcessorBaseTest {

  @Test
  void dispatchesToFunctionSpecificMethods() {
    RecordingFunctionProcessor processor = new RecordingFunctionProcessor();

    processor.processFunctionCall(new WdlFunctionCallOperation("floor"));
    processor.processFunctionCall(new WdlFunctionCallOperation("my_custom_function"));

    assertEquals(List.of("floor", "nonstandard"), processor.events);
  }

  private static final class RecordingFunctionProcessor extends WdlFunctionProcessorBase {
    private final List<String> events = new ArrayList<>();

    @Override
    public void processFloor(WdlFunctionCallOperation functionCall) {
      events.add("floor");
    }

    @Override
    public void processNonstandard(WdlFunctionCallOperation functionCall) {
      events.add("nonstandard");
    }
  }
}
