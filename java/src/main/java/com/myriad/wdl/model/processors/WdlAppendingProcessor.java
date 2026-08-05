package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructMember;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.sections.WdlCommand;
import com.myriad.wdl.model.sections.WdlHints;
import com.myriad.wdl.model.sections.WdlHints.WdlTaskHints;
import com.myriad.wdl.model.sections.WdlHints.WdlWorkflowHints;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlMetadata;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlParameterMetadata;
import com.myriad.wdl.model.sections.WdlOutput;
import com.myriad.wdl.model.sections.WdlRequirements;
import com.myriad.wdl.model.sections.WdlRuntime;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlConditional.WdlConditionalElseIf;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMembers;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStandard;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStar;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.statements.WdlStatement;
import java.io.IOError;
import java.io.IOException;
import java.util.stream.Collectors;

/**
 * Processor that renders the Java WDL object model back into source text.
 *
 * <p>This class is a concrete example of extending {@link WdlProcessorBase}: it traverses the
 * model and appends a WDL representation to an {@link Appendable}. It is useful both as a utility
 * and as readable reference code for consumers who want to build custom traversals.
 */
public class WdlAppendingProcessor extends WdlProcessorBase {

  protected Appendable out;

  /** Creates a processor that appends rendered WDL to the supplied output target. */
  public WdlAppendingProcessor(Appendable out) {
    this.out = out;
  }

  protected WdlAppendingProcessor append(CharSequence c) {
    try {
      out.append(c);
      return this;
    } catch (IOException e) {
      throw new IOError(e);
    }
  }

  protected WdlAppendingProcessor append(char c) {
    try {
      out.append(c);
      return this;
    } catch (IOException e) {
      throw new IOError(e);
    }
  }

  @Override
  public void processVersion(WdlDocument ctx, WdlVersion node) {
    append("version ").append(node.getVersionString()).append('\n');
  }

  @Override
  public void processImport(WdlDocument ctx, WdlImportStandard node) {
    append("import ")
        .append(stringLiteralToWdl(node.getSource(), true))
        .append(node.getAlias() == null ? "" : " as " + node.getAlias());
    if (!node.members().isEmpty()) {
      append('\n');
      append(
          String.join(
              "\n",
              node.members().stream()
                  .map(a -> "  alias " + a.getMember() + " as " + a.getAlias())
                  .collect(Collectors.toList())));
    }
    append('\n');
  }

  @Override
  public void processImport(WdlDocument ctx, WdlImportMembers node) {
    append("import { ");
    append(
        String.join(
            ", ",
            node.members().stream()
                .map(m -> m.getMember() + (m.getAlias() == null ? "" : " as " + m.getAlias()))
                .collect(Collectors.toList())));
    append(" } from ").append(stringLiteralToWdl(node.getSource(), true)).append('\n');
  }

  @Override
  public void processImport(WdlDocument ctx, WdlImportStar node) {
    append("import * from ").append(stringLiteralToWdl(node.getSource(), true)).append('\n');
  }

  @Override
  public void processEnum(WdlDocument ctx, WdlEnum node) {
    append("enum ").append(node.getName());
    if (node.getValueType() != null) {
      append('[').append(typeToWdl(node.getValueType())).append(']');
    }
    append(" {\n");
    append(
        String.join(
            ",\n",
            node.elements().stream()
                .map(
                    c -> {
                      return "  "
                          + c.getKey()
                          + (c.getValue() == null ? "" : " = " + expressionToWdl(c.getValue()));
                    })
                .collect(Collectors.toList())));
    append("\n}\n");
  }

  @Override
  public void processStruct(WdlDocument ctx, WdlStruct node) {
    append("struct ").append(node.getName()).append("{\n");
    super.processStruct(ctx, node);
    append("}\n");
  }

  @Override
  public void processStructMember(WdlStruct ctx, WdlStructMember node) {
    append("  ").append(typeToWdl(node.getType())).append(" ").append(node.getName()).append("\n");
  }

  @Override
  public void processStructParameterMetadata(WdlStruct ctx, WdlParameterMetadata node) {
    processParameterMetadata(node);
  }

  @Override
  public void processStructMetadata(WdlStruct ctx, WdlMetadata node) {
    processMetadata(node);
  }

  @Override
  public void processTask(WdlDocument ctx, WdlTask node) {
    append("task ").append(node.getName()).append("{\n");
    super.processTask(ctx, node);
    append("}\n");
  }

  @Override
  public void processTaskDeclaration(WdlTask ctx, WdlBoundDeclaration node) {
    append("  ").append(declarationToWdl(node)).append("\n");
  }

  @Override
  public void processTaskInput(WdlTask ctx, WdlInput node) {
    processInput(node);
  }

  @Override
  public void processTaskOutput(WdlTask ctx, WdlOutput node) {
    processOutput(node);
  }

  @Override
  public void processTaskCommand(WdlTask ctx, WdlCommand node) {
    append("  command ").append(node.isMultiline() ? "<<<" : "{");
    append(stringLiteralToWdl(node.getCommandText(), false));
    // append("\n  ")
    append(node.isMultiline() ? ">>>" : "}").append("\n");
  }

  @Override
  public void processTaskParameterMetadata(WdlTask ctx, WdlParameterMetadata node) {
    processParameterMetadata(node);
  }

  @Override
  public void processTaskMetadata(WdlTask ctx, WdlMetadata node) {
    processMetadata(node);
  }

  @Override
  public void processTaskRequirements(WdlTask ctx, WdlRequirements node) {
    append("  requirements {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(e -> "    " + e.getKey() + ": " + expressionToWdl(e.getValue()))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  @Override
  public void processTaskRuntime(WdlTask ctx, WdlRuntime node) {
    append("  runtime {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(e -> "    " + e.getKey() + ": " + expressionToWdl(e.getValue()))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  @Override
  public void processTaskHints(WdlTask ctx, WdlTaskHints node) {
    processHints(node);
  }

  @Override
  public void processWorkflow(WdlDocument ctx, WdlWorkflow node) {
    append("workflow ").append(node.getName()).append("{\n");
    super.processWorkflow(ctx, node);
    append("}\n");
  }

  @Override
  public void processWorkflowInput(WdlWorkflow ctx, WdlInput node) {
    processInput(node);
  }

  @Override
  public void processWorkflowOutput(WdlWorkflow ctx, WdlOutput node) {
    processOutput(node);
  }

  @Override
  public void processWorkflowParameterMetadata(WdlWorkflow ctx, WdlParameterMetadata node) {
    processParameterMetadata(node);
  }

  @Override
  public void processWorkflowMetadata(WdlWorkflow ctx, WdlMetadata node) {
    processMetadata(node);
  }

  @Override
  public void processWorkflowHints(WdlWorkflow ctx, WdlWorkflowHints node) {
    processHints(node);
  }

  @Override
  public void processWorkflowDeclaration(WdlWorkflow ctx, WdlBoundDeclaration node) {
    processStatement(node, 0);
  }

  @Override
  public void processWorkflowCall(WdlWorkflow ctx, WdlCall node) {
    processStatement(node, 0);
  }

  @Override
  public void processWorkflowScatter(WdlWorkflow ctx, WdlScatter node) {
    processStatement(node, 0);
  }

  @Override
  public void processWorkflowConditional(WdlWorkflow ctx, WdlConditional node) {
    processStatement(node, 0);
  }

  protected void processStatement(WdlStatement stmt, int indentLevel) {
    switch (stmt.componentType()) {
      case DECLARATION:
        processStatementDeclaration((WdlBoundDeclaration) stmt, indentLevel);
        break;
      case CALL:
        processStatementCall((WdlCall) stmt, indentLevel);
        break;
      case SCATTER:
        processStatementScatter((WdlScatter) stmt, indentLevel);
        break;
      case CONDITIONAL:
        processStatementConditional((WdlConditional) stmt, indentLevel);
        break;
      default:
        throw new IllegalStateException("Unhandled statement type:" + stmt.componentType());
    }
  }

  protected void indent(int indentLevel) {
    for (int i = 0; i < indentLevel + 1; i++) {
      append("  ");
    }
  }

  protected void processStatementDeclaration(WdlBoundDeclaration node, int indentLevel) {
    indent(indentLevel);
    append(declarationToWdl(node)).append("\n");
  }

  protected void processStatementCall(WdlCall node, int indentLevel) {
    indent(indentLevel);
    append("call ")
        .append(node.targetPathAsString())
        .append(node.getAlias() == null ? "" : " as " + node.getAlias());
    node.afterDependencies()
        .forEach(
            a -> {
              append(" after ").append(a);
            });
    if (!node.inputs().isEmpty()) {
      append("  {").append(node.isLegacyInputColonUsed() ? " input: " : "");
      append(
          String.join(
              ", ",
              node.inputs().stream()
                  .map(i -> keyValueToWdl(i, " = "))
                  .collect(Collectors.toList())));
      append("  }");
    }
    append("\n");
  }

  protected void processStatementScatter(WdlScatter node, int indentLevel) {
    indent(indentLevel);
    append("scatter (")
        .append(node.getName())
        .append(" in ")
        .append(expressionToWdl(node.getCollection()))
        .append(") {\n");
    node.statements().forEach(s -> processStatement(s, indentLevel + 1));
    indent(indentLevel);
    append("}\n");
  }

  protected void processStatementConditional(WdlConditional node, int indentLevel) {
    indent(indentLevel);
    append("if (").append(expressionToWdl(node.getCondition())).append(") {\n");
    node.thenStatements().forEach(s -> processStatement(s, indentLevel + 1));
    indent(indentLevel);
    append("}");
    if (!node.elseIfs().isEmpty()) {
      for (WdlConditionalElseIf elseIf : node.elseIfs()) {
        append(" else if (").append(expressionToWdl(elseIf.getCondition())).append(") {\n");
        elseIf.thenStatements().forEach(s -> processStatement(s, indentLevel + 1));

        indent(indentLevel);
        append("}");
      }
    }
    if (!node.elseStatements().isEmpty()) {
      append(" else {\n");
      node.elseStatements().forEach(s -> processStatement(s, indentLevel + 1));
      indent(indentLevel);
      append("}");
    }
    append("\n");
  }

  protected void processInput(WdlInput node) {
    append("  input {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(d -> "    " + declarationToWdl(d))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  protected void processOutput(WdlOutput node) {
    append("  output {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(d -> "    " + declarationToWdl(d))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  protected void processParameterMetadata(WdlParameterMetadata node) {
    append("  parameter_meta {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(i -> "    " + i.getKey() + ":" + expressionToWdl(i.getValue()))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  protected void processMetadata(WdlMetadata node) {
    append("  meta {\n")
        .append(
            String.join(
                "\n",
                node.elements().stream()
                    .map(i -> "    " + i.getKey() + ":" + expressionToWdl(i.getValue()))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }

  protected void processHints(WdlHints<?> hints) {
    append("  hints {\n")
        .append(
            String.join(
                "\n",
                hints.elements().stream()
                    .map(h -> "    " + keyValueToWdl(h))
                    .collect(Collectors.toList())))
        .append("\n  }\n");
  }
}
