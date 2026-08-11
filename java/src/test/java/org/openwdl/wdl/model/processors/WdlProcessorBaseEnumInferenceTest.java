package org.openwdl.wdl.model.processors;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlEnum.WdlEnumChoice;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructMember;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeInference;
import java.util.Optional;
import org.junit.jupiter.api.Test;

class WdlProcessorBaseEnumInferenceTest {

  private static final class ProbeProcessor extends com.myriad.wdl.model.processors.WdlProcessorBase {
    Optional<WdlType> infer(WdlEnum e) {
      return inferEnumValueType(e);
    }
  }

  @Test
  void infersImplicitEnumTypeAsString() {
    WdlEnum en = new WdlEnum("Letters");
    en.elements().add(new WdlEnumChoice("A", null));
    en.elements().add(new WdlEnumChoice("B", null));

    Optional<WdlType> inferred = new ProbeProcessor().infer(en);
    Optional<WdlType> inferredDirect = WdlTypeInference.inferEnumValueType(en);

    assertTrue(inferred.isPresent());
    assertTrue(inferredDirect.isPresent());
    assertTrue(inferred.get() instanceof WdlPrimitiveType);
    assertEquals(WdlPrimitiveType.Type.STRING, ((WdlPrimitiveType) inferred.get()).primitiveType());
    assertEquals(WdlPrimitiveType.Type.STRING, ((WdlPrimitiveType) inferredDirect.get()).primitiveType());
  }

  @Test
  void widensIntAndFloatEnumChoicesToFloat() {
    WdlEnum en = new WdlEnum("Numbers");
    en.elements().add(new WdlEnumChoice("ONE", new WdlIntLiteral(1)));
    en.elements().add(new WdlEnumChoice("PI", new WdlFloatLiteral(3.14)));

    Optional<WdlType> inferred = new ProbeProcessor().infer(en);
    Optional<WdlType> inferredDirect = WdlTypeInference.inferEnumValueType(en);

    assertTrue(inferred.isPresent());
    assertTrue(inferredDirect.isPresent());
    assertTrue(inferred.get() instanceof WdlPrimitiveType);
    assertEquals(WdlPrimitiveType.Type.FLOAT, ((WdlPrimitiveType) inferred.get()).primitiveType());
    assertEquals(WdlPrimitiveType.Type.FLOAT, ((WdlPrimitiveType) inferredDirect.get()).primitiveType());
  }

  @Test
  void returnsEmptyForIncompatibleEnumChoiceTypes() {
    WdlEnum en = new WdlEnum("Bad");
    en.elements().add(new WdlEnumChoice("ONE", new WdlIntLiteral(1)));
    WdlFunctionCallOperation dynamic = new WdlFunctionCallOperation();
    dynamic.setFunctionName("foo");
    dynamic.setFunction(WdlFunctionCallOperation.WdlFunction.NONSTANDARD);
    en.elements().add(new WdlEnumChoice("DYNAMIC", dynamic));

    Optional<WdlType> inferred = new ProbeProcessor().infer(en);
    Optional<WdlType> inferredDirect = WdlTypeInference.inferEnumValueType(en);

    assertFalse(inferred.isPresent());
    assertFalse(inferredDirect.isPresent());
  }

  @Test
  void supportsLocalStructAndEnumIntrospectionHelpers() {
    WdlStruct struct = new WdlStruct("Person");
    struct.elements().add(new WdlStructMember(new WdlPrimitiveType(WdlPrimitiveType.Type.STRING, false), "name"));
    struct.elements().add(new WdlStructMember(new WdlPrimitiveType(WdlPrimitiveType.Type.INT, false), "age"));

    WdlEnum en = new WdlEnum("Status");
    en.elements().add(new WdlEnumChoice("NEW", null));
    en.elements().add(new WdlEnumChoice("DONE", null));

    assertTrue(struct.hasMember("name"));
    assertFalse(struct.hasMember("missing"));
    assertTrue(struct.memberType("age").isPresent());
    assertEquals(
        WdlPrimitiveType.Type.INT,
        ((WdlPrimitiveType) struct.memberType("age").orElseThrow()).primitiveType());

    assertTrue(en.hasChoice("DONE"));
    assertFalse(en.hasChoice("FAILED"));
    assertTrue(en.choice("NEW").isPresent());
  }
}
