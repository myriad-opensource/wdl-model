/** Visitor-style processor contract for the full TypeScript WDL model. */
import type { WdlDocument } from '../wdl-document.js';
import type { WdlVersion } from '../wdl-version.js';
import type { WdlEnum } from '../definitions/wdl-enum.js';
import type { WdlStruct, WdlStructMember } from '../definitions/wdl-struct.js';
import type { WdlTask } from '../definitions/wdl-task.js';
import type { WdlWorkflow } from '../definitions/wdl-workflow.js';
import type { WdlCommand } from '../sections/wdl-command.js';
import type { WdlTaskHints, WdlWorkflowHints } from '../sections/wdl-hints.js';
import type { WdlInput } from '../sections/wdl-input.js';
import type { WdlMetadata, WdlParameterMetadata } from '../sections/wdl-metadata-base.js';
import type { WdlOutput } from '../sections/wdl-output.js';
import type { WdlRequirements } from '../sections/wdl-requirements.js';
import type { WdlRuntime } from '../sections/wdl-runtime.js';
import type { WdlCall } from '../statements/wdl-call.js';
import type { WdlConditional } from '../statements/wdl-conditional.js';
import type { WdlBoundDeclaration } from '../statements/wdl-declaration.js';
import type {
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
} from '../statements/wdl-import.js';
import type { WdlScatter } from '../statements/wdl-scatter.js';

export interface WdlProcessor {
  /** Processes the document root. */
  processDocument(node: WdlDocument): void;
  /** Processes the version node. */
  processVersion(ctx: WdlDocument, node: WdlVersion): void;
  /** Processes a standard import node. */
  processImportStandard(ctx: WdlDocument, node: WdlImportStandard): void;
  /** Processes a member-list import node. */
  processImportMembers(ctx: WdlDocument, node: WdlImportMembers): void;
  /** Processes a star import node. */
  processImportStar(ctx: WdlDocument, node: WdlImportStar): void;
  /** Processes an enum definition node. */
  processEnum(ctx: WdlDocument, node: WdlEnum): void;
  /** Processes a struct definition node. */
  processStruct(ctx: WdlDocument, node: WdlStruct): void;
  /** Processes a struct member node. */
  processStructMember(ctx: WdlStruct, node: WdlStructMember): void;
  /** Processes struct parameter metadata. */
  processStructParameterMetadata(ctx: WdlStruct, node: WdlParameterMetadata): void;
  /** Processes struct metadata. */
  processStructMetadata(ctx: WdlStruct, node: WdlMetadata): void;
  /** Processes a task definition node. */
  processTask(ctx: WdlDocument, node: WdlTask): void;
  /** Processes a task declaration node. */
  processTaskDeclaration(ctx: WdlTask, node: WdlBoundDeclaration): void;
  /** Processes a task input section. */
  processTaskInput(ctx: WdlTask, node: WdlInput): void;
  /** Processes a task output section. */
  processTaskOutput(ctx: WdlTask, node: WdlOutput): void;
  /** Processes a task command section. */
  processTaskCommand(ctx: WdlTask, node: WdlCommand): void;
  /** Processes task parameter metadata. */
  processTaskParameterMetadata(ctx: WdlTask, node: WdlParameterMetadata): void;
  /** Processes task metadata. */
  processTaskMetadata(ctx: WdlTask, node: WdlMetadata): void;
  /** Processes task requirements. */
  processTaskRequirements(ctx: WdlTask, node: WdlRequirements): void;
  /** Processes task runtime data. */
  processTaskRuntime(ctx: WdlTask, node: WdlRuntime): void;
  /** Processes task hints. */
  processTaskHints(ctx: WdlTask, node: WdlTaskHints): void;
  /** Processes a workflow definition node. */
  processWorkflow(ctx: WdlDocument, node: WdlWorkflow): void;
  /** Processes a workflow declaration node. */
  processWorkflowDeclaration(ctx: WdlWorkflow, node: WdlBoundDeclaration): void;
  /** Processes a workflow input section. */
  processWorkflowInput(ctx: WdlWorkflow, node: WdlInput): void;
  /** Processes a workflow output section. */
  processWorkflowOutput(ctx: WdlWorkflow, node: WdlOutput): void;
  /** Processes workflow metadata. */
  processWorkflowMetadata(ctx: WdlWorkflow, node: WdlMetadata): void;
  /** Processes workflow parameter metadata. */
  processWorkflowParameterMetadata(ctx: WdlWorkflow, node: WdlParameterMetadata): void;
  /** Processes a workflow call statement. */
  processWorkflowCall(ctx: WdlWorkflow, node: WdlCall): void;
  /** Processes a workflow conditional statement. */
  processWorkflowConditional(ctx: WdlWorkflow, node: WdlConditional): void;
  /** Processes a workflow scatter statement. */
  processWorkflowScatter(ctx: WdlWorkflow, node: WdlScatter): void;
  /** Processes workflow hints. */
  processWorkflowHints(ctx: WdlWorkflow, node: WdlWorkflowHints): void;
}
