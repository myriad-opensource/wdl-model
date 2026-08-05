package org.openwdl.wdl.model.processors;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral.WdlMapEntry;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholderOption;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.processors.WdlExpressionProcessorBase;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.Test;

class WdlExpressionProcessorBaseTest {

  @Test
  void walksExpressionsDepthFirstUsingComponentTypeDispatch() {
    WdlArrayLiteral root = new WdlArrayLiteral();

    root.entries()
        .add(
            new WdlBinaryOperation(
                new WdlIntLiteral(1), WdlBinaryOperation.Operator.ADD, new WdlIntLiteral(2)));

    WdlStringLiteral defaultValue = new WdlStringLiteral(WdlStringLiteral.Delimiter.SINGLE_QUOTE);
    defaultValue.components().add(new WdlStringText("d"));

    WdlStringPlaceholder placeholder = new WdlStringPlaceholder();
    placeholder.setOption(
        new WdlStringPlaceholderOption(WdlStringPlaceholderOption.Type.DEFAULT, defaultValue));
    placeholder.setExpression(new WdlVariable("v"));

    WdlStringLiteral str = new WdlStringLiteral(WdlStringLiteral.Delimiter.DOUBLE_QUOTE);
    str.components().add(new WdlStringText("pre"));
    str.components().add(placeholder);
    root.entries().add(str);

    WdlMapLiteral map = new WdlMapLiteral();
    map.entries()
        .add(
            new WdlMapEntry(
                new WdlVariable("k"),
                new WdlUnaryOperation(
                    WdlUnaryOperation.Operator.NEGATIVE, new WdlFloatLiteral(3.0))));
    root.entries().add(map);

    RecordingExpressionProcessor processor = new RecordingExpressionProcessor();
    processor.processExpression(root);

    assertEquals(
        Arrays.asList(
            "ARRAY_LIT",
            "BINARY_OP",
            "INT_LIT",
            "INT_LIT",
            "STR_LIT",
            "SC:TEXT",
            "SC:PLACEHOLDER",
            "STR_LIT",
            "SC:TEXT",
            "VARIABLE",
            "MAP_LIT",
            "VARIABLE",
            "UNARY_OP",
            "FLOAT_LIT"),
        processor.events);
  }

  private static final class RecordingExpressionProcessor extends WdlExpressionProcessorBase {
    private final List<String> events = new ArrayList<>();

    @Override
    protected void enterExpression(com.myriad.wdl.model.expressions.WdlExpression expression) {
      events.add(expression.componentType().name());
    }

    @Override
    protected void enterStringComponent(WdlStringLiteral context, WdlStringComponent component) {
      events.add("SC:" + component.componentType().name());
    }
  }
}
