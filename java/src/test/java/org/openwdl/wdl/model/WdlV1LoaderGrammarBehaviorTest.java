package org.openwdl.wdl.model;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlV1Loader;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlBooleanLiteral;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class WdlV1LoaderGrammarBehaviorTest {

  private static final Path FIXTURES_ROOT =
      Paths.get("src", "test", "resources", "wdl_tests", "grammar_behavior");

  @Test
  void parsesAdditiveChainsAsLeftAssociative() throws Exception {
    WdlBoundDeclaration declaration = firstWorkflowDeclaration("associativity_additive_chain.wdl");

    WdlBinaryOperation root =
        assertInstanceOf(WdlBinaryOperation.class, declaration.getExpression());
    assertEquals(WdlBinaryOperation.Operator.SUTRACT, root.getOperator());
    assertEquals(3L, assertInstanceOf(WdlIntLiteral.class, root.getRight()).getValue());

    WdlBinaryOperation left = assertInstanceOf(WdlBinaryOperation.class, root.getLeft());
    assertEquals(WdlBinaryOperation.Operator.SUTRACT, left.getOperator());
    assertEquals(1L, assertInstanceOf(WdlIntLiteral.class, left.getLeft()).getValue());
    assertEquals(2L, assertInstanceOf(WdlIntLiteral.class, left.getRight()).getValue());
  }

  @Test
  void parsesMultiplicativeChainsAsLeftAssociative() throws Exception {
    WdlBoundDeclaration declaration =
        firstWorkflowDeclaration("associativity_multiplicative_chain.wdl");

    WdlBinaryOperation root =
        assertInstanceOf(WdlBinaryOperation.class, declaration.getExpression());
    assertEquals(WdlBinaryOperation.Operator.DIVIDE, root.getOperator());
    assertEquals(2L, assertInstanceOf(WdlIntLiteral.class, root.getRight()).getValue());

    WdlBinaryOperation left = assertInstanceOf(WdlBinaryOperation.class, root.getLeft());
    assertEquals(WdlBinaryOperation.Operator.DIVIDE, left.getOperator());
    assertEquals(8L, assertInstanceOf(WdlIntLiteral.class, left.getLeft()).getValue());
    assertEquals(4L, assertInstanceOf(WdlIntLiteral.class, left.getRight()).getValue());
  }

  @Test
  void parsesLogicalOrChainsAsLeftAssociative() throws Exception {
    WdlBoundDeclaration declaration =
        firstWorkflowDeclaration("associativity_logical_or_chain.wdl");

    WdlBinaryOperation root =
        assertInstanceOf(WdlBinaryOperation.class, declaration.getExpression());
    assertEquals(WdlBinaryOperation.Operator.OR, root.getOperator());
    assertEquals(true, assertInstanceOf(WdlBooleanLiteral.class, root.getRight()).getValue());

    WdlBinaryOperation left = assertInstanceOf(WdlBinaryOperation.class, root.getLeft());
    assertEquals(WdlBinaryOperation.Operator.OR, left.getOperator());
    assertEquals(true, assertInstanceOf(WdlBooleanLiteral.class, left.getLeft()).getValue());
    assertEquals(false, assertInstanceOf(WdlBooleanLiteral.class, left.getRight()).getValue());
  }

  @Test
  void rejectsReservedKeywordAsDeclarationIdentifierTask() {
    Path fixture = FIXTURES_ROOT.resolve("keyword_decl_identifier_task.wdl");

    assertThrows(WdlException.class, () -> WdlV1Loader.load(fixture.toFile()));
  }

  @Test
  void rejectsReservedKeywordAsDeclarationIdentifierIf() {
    Path fixture = FIXTURES_ROOT.resolve("keyword_decl_identifier_if.wdl");

    assertThrows(WdlException.class, () -> WdlV1Loader.load(fixture.toFile()));
  }

  @Test
  void rejectsReservedKeywordAsTaskInputIdentifier() {
    Path fixture = FIXTURES_ROOT.resolve("keyword_task_input_in.wdl");

    assertThrows(WdlException.class, () -> WdlV1Loader.load(fixture.toFile()));
  }

  @Test
  void rejectsReservedKeywordAsMetadataKey() {
    Path fixture = FIXTURES_ROOT.resolve("keyword_metadata_key_version.wdl");

    assertThrows(WdlException.class, () -> WdlV1Loader.load(fixture.toFile()));
  }

  private static WdlBoundDeclaration firstWorkflowDeclaration(String fixtureFile) throws Exception {
    WdlDocument doc = WdlV1Loader.load(FIXTURES_ROOT.resolve(fixtureFile).toFile());
    WdlWorkflow workflow = doc.workflows().get(0);
    return (WdlBoundDeclaration) workflow.getElements().getFirst();
  }
}
