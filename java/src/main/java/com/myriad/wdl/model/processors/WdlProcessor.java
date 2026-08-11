package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructMember;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.sections.WdlCommand;
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
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMembers;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStandard;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStar;
import com.myriad.wdl.model.statements.WdlScatter;

/**
 * Visitor-style processor contract for the full Java WDL object model.
 *
 * <p>Implement this interface when you want explicit callbacks for the major document, definition,
 * section, and workflow statement nodes. {@link WdlProcessorBase} provides the usual source-order
 * traversal so most callers should extend that instead of implementing everything directly.
 */
public interface WdlProcessor {

  void processDocument(WdlDocument node);

  void processVersion(WdlDocument ctx, WdlVersion node);

  void processImport(WdlDocument ctx, WdlImportStandard node);

  void processImport(WdlDocument ctx, WdlImportMembers node);

  void processImport(WdlDocument ctx, WdlImportStar node);

  void processEnum(WdlDocument ctx, WdlEnum node);

  void processStruct(WdlDocument ctx, WdlStruct node);

  void processStructMember(WdlStruct ctx, WdlStructMember node);

  void processStructParameterMetadata(WdlStruct ctx, WdlParameterMetadata node);

  void processStructMetadata(WdlStruct ctx, WdlMetadata node);

  void processTask(WdlDocument ctx, WdlTask node);

  void processTaskDeclaration(WdlTask ctx, WdlBoundDeclaration node);

  void processTaskInput(WdlTask ctx, WdlInput node);

  void processTaskOutput(WdlTask ctx, WdlOutput node);

  void processTaskCommand(WdlTask ctx, WdlCommand node);

  void processTaskParameterMetadata(WdlTask ctx, WdlParameterMetadata node);

  void processTaskMetadata(WdlTask ctx, WdlMetadata node);

  void processTaskRequirements(WdlTask ctx, WdlRequirements node);

  void processTaskRuntime(WdlTask ctx, WdlRuntime node);

  void processTaskHints(WdlTask ctx, WdlTaskHints node);

  void processWorkflow(WdlDocument ctx, WdlWorkflow node);

  void processWorkflowDeclaration(WdlWorkflow ctx, WdlBoundDeclaration node);

  void processWorkflowInput(WdlWorkflow ctx, WdlInput node);

  void processWorkflowOutput(WdlWorkflow ctx, WdlOutput node);

  void processWorkflowMetadata(WdlWorkflow ctx, WdlMetadata node);

  void processWorkflowParameterMetadata(WdlWorkflow ctx, WdlParameterMetadata node);

  void processWorkflowCall(WdlWorkflow ctx, WdlCall node);

  void processWorkflowConditional(WdlWorkflow ctx, WdlConditional node);

  void processWorkflowScatter(WdlWorkflow ctx, WdlScatter node);

  void processWorkflowHints(WdlWorkflow ctx, WdlWorkflowHints node);
}
