package wdl

import (
	"context"

	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

// Processor receives high-level callbacks during document traversal.
//
// Implement this interface for simple traversal needs. For section-level and
// statement-level callbacks, implement ProcessorDetailed.
type Processor interface {
	EnterDocument(ctx context.Context, doc *Document) error
	ExitDocument(ctx context.Context, doc *Document) error
	HandleDeclaration(ctx context.Context, doc *Document, decl Declaration) error
	HandleImport(ctx context.Context, doc *Document, imp ImportRecord, imported *Document) error
}

// ProcessorDetailed adds fine-grained callbacks for source-order traversal.
//
// These callbacks expose generated parser contexts so advanced tooling can
// inspect section details without reparsing source.
type ProcessorDetailed interface {
	Processor
	HandleVersion(ctx context.Context, doc *Document, version Version) error
	HandleTask(ctx context.Context, doc *Document, decl Declaration) error
	HandleWorkflow(ctx context.Context, doc *Document, decl Declaration) error
	HandleStruct(ctx context.Context, doc *Document, decl Declaration) error
	HandleStructMember(ctx context.Context, doc *Document, structDecl Declaration, member grammar.IStructDeclarationContext) error
	HandleStructParameterMetadata(ctx context.Context, doc *Document, structDecl Declaration, section grammar.IParameterMetadataSectionContext) error
	HandleStructMetadata(ctx context.Context, doc *Document, structDecl Declaration, section grammar.IMetadataSectionContext) error
	HandleEnum(ctx context.Context, doc *Document, decl Declaration) error
	HandleImportStandard(ctx context.Context, doc *Document, imp ImportRecord, imported *Document) error
	HandleImportMembers(ctx context.Context, doc *Document, imp ImportRecord, imported *Document) error
	HandleImportStar(ctx context.Context, doc *Document, imp ImportRecord, imported *Document) error
	HandleTaskInputSection(ctx context.Context, doc *Document, task Declaration, section grammar.IInputSectionContext) error
	HandleTaskOutputSection(ctx context.Context, doc *Document, task Declaration, section grammar.IOutputSectionContext) error
	HandleTaskCommandSection(ctx context.Context, doc *Document, task Declaration, section grammar.ICommandSectionContext) error
	HandleTaskRuntimeSection(ctx context.Context, doc *Document, task Declaration, section grammar.IRuntimeSectionContext) error
	HandleTaskRequirementsSection(ctx context.Context, doc *Document, task Declaration, section grammar.IRequirementsSectionContext) error
	HandleTaskHintsSection(ctx context.Context, doc *Document, task Declaration, section grammar.IHintsSectionTaskContext) error
	HandleTaskMetadataSection(ctx context.Context, doc *Document, task Declaration, section grammar.IMetadataSectionContext) error
	HandleTaskParameterMetadataSection(ctx context.Context, doc *Document, task Declaration, section grammar.IParameterMetadataSectionContext) error
	HandleTaskDeclaration(ctx context.Context, doc *Document, task Declaration, decl grammar.IBoundDeclarationContext) error
	HandleWorkflowInputSection(ctx context.Context, doc *Document, workflow Declaration, section grammar.IInputSectionContext) error
	HandleWorkflowOutputSection(ctx context.Context, doc *Document, workflow Declaration, section grammar.IOutputSectionContext) error
	HandleWorkflowMetadataSection(ctx context.Context, doc *Document, workflow Declaration, section grammar.IMetadataSectionContext) error
	HandleWorkflowParameterMetadataSection(ctx context.Context, doc *Document, workflow Declaration, section grammar.IParameterMetadataSectionContext) error
	HandleWorkflowHintsSection(ctx context.Context, doc *Document, workflow Declaration, section grammar.IHintsSectionWorkflowContext) error
	HandleWorkflowDeclaration(ctx context.Context, doc *Document, workflow Declaration, decl grammar.IBoundDeclarationContext) error
	HandleWorkflowCall(ctx context.Context, doc *Document, workflow Declaration, call grammar.ICallStatementContext) error
	HandleWorkflowConditional(ctx context.Context, doc *Document, workflow Declaration, conditional grammar.IConditionalStatementContext) error
	HandleWorkflowScatter(ctx context.Context, doc *Document, workflow Declaration, scatter grammar.IScatterStatementContext) error
}

// ProcessorBase provides no-op callback defaults for custom processors.
//
// Embed ProcessorBase and override only the callbacks your tool needs.
type ProcessorBase struct{}

func (p *ProcessorBase) EnterDocument(_ context.Context, _ *Document) error { return nil }
func (p *ProcessorBase) ExitDocument(_ context.Context, _ *Document) error  { return nil }
func (p *ProcessorBase) HandleDeclaration(_ context.Context, _ *Document, _ Declaration) error {
	return nil
}
func (p *ProcessorBase) HandleImport(_ context.Context, _ *Document, _ ImportRecord, _ *Document) error {
	return nil
}
func (p *ProcessorBase) HandleVersion(_ context.Context, _ *Document, _ Version) error { return nil }
func (p *ProcessorBase) HandleTask(_ context.Context, _ *Document, _ Declaration) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflow(_ context.Context, _ *Document, _ Declaration) error {
	return nil
}
func (p *ProcessorBase) HandleStruct(_ context.Context, _ *Document, _ Declaration) error { return nil }
func (p *ProcessorBase) HandleStructMember(_ context.Context, _ *Document, _ Declaration, _ grammar.IStructDeclarationContext) error {
	return nil
}
func (p *ProcessorBase) HandleStructParameterMetadata(_ context.Context, _ *Document, _ Declaration, _ grammar.IParameterMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleStructMetadata(_ context.Context, _ *Document, _ Declaration, _ grammar.IMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleEnum(_ context.Context, _ *Document, _ Declaration) error { return nil }
func (p *ProcessorBase) HandleImportStandard(_ context.Context, _ *Document, _ ImportRecord, _ *Document) error {
	return nil
}
func (p *ProcessorBase) HandleImportMembers(_ context.Context, _ *Document, _ ImportRecord, _ *Document) error {
	return nil
}
func (p *ProcessorBase) HandleImportStar(_ context.Context, _ *Document, _ ImportRecord, _ *Document) error {
	return nil
}
func (p *ProcessorBase) HandleTaskInputSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IInputSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskOutputSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IOutputSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskCommandSection(_ context.Context, _ *Document, _ Declaration, _ grammar.ICommandSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskRuntimeSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IRuntimeSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskRequirementsSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IRequirementsSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskHintsSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IHintsSectionTaskContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskMetadataSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskParameterMetadataSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IParameterMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleTaskDeclaration(_ context.Context, _ *Document, _ Declaration, _ grammar.IBoundDeclarationContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowInputSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IInputSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowOutputSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IOutputSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowMetadataSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowParameterMetadataSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IParameterMetadataSectionContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowHintsSection(_ context.Context, _ *Document, _ Declaration, _ grammar.IHintsSectionWorkflowContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowDeclaration(_ context.Context, _ *Document, _ Declaration, _ grammar.IBoundDeclarationContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowCall(_ context.Context, _ *Document, _ Declaration, _ grammar.ICallStatementContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowConditional(_ context.Context, _ *Document, _ Declaration, _ grammar.IConditionalStatementContext) error {
	return nil
}
func (p *ProcessorBase) HandleWorkflowScatter(_ context.Context, _ *Document, _ Declaration, _ grammar.IScatterStatementContext) error {
	return nil
}

// TraverseDocument walks a document and its imports once each.
//
// The walk is depth-first. A document is visited at most once per traversal.
func TraverseDocument(ctx context.Context, root *Document, processor Processor) error {
	if root == nil || processor == nil {
		return nil
	}
	visited := map[*Document]struct{}{}
	return traverseRecursive(ctx, root, processor, visited)
}

func traverseRecursive(ctx context.Context, doc *Document, processor Processor, visited map[*Document]struct{}) error {
	if _, seen := visited[doc]; seen {
		return nil
	}
	visited[doc] = struct{}{}

	if err := processor.EnterDocument(ctx, doc); err != nil {
		return err
	}

	if detailed, ok := processor.(ProcessorDetailed); ok {
		if err := detailed.HandleVersion(ctx, doc, doc.Version); err != nil {
			return err
		}
	}

	if err := processDocumentElementsInSourceOrder(ctx, doc, processor); err != nil {
		return err
	}

	for _, imp := range doc.ImportStatements {
		imported := doc.ImportedDocs[imp.ResolvedLocation]
		if imported != nil {
			if err := traverseRecursive(ctx, imported, processor, visited); err != nil {
				return err
			}
		}
	}

	if err := processor.ExitDocument(ctx, doc); err != nil {
		return err
	}
	return nil
}

func processDocumentElementsInSourceOrder(ctx context.Context, doc *Document, processor Processor) error {
	root, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok || root == nil {
		for _, decl := range doc.Declarations {
			if err := dispatchDeclaration(ctx, doc, processor, decl); err != nil {
				return err
			}
		}
		for _, imp := range doc.ImportStatements {
			imported := doc.ImportedDocs[imp.ResolvedLocation]
			if err := dispatchImport(ctx, doc, processor, imp, imported, ""); err != nil {
				return err
			}
		}
		return nil
	}

	declIndex := 0
	importIndex := 0
	for _, element := range root.AllDocumentElement() {
		if element.ImportStatement() != nil {
			if importIndex >= len(doc.ImportStatements) {
				continue
			}
			imp := doc.ImportStatements[importIndex]
			importIndex++
			imported := doc.ImportedDocs[imp.ResolvedLocation]
			kind := importKind(element.ImportStatement())
			if err := dispatchImport(ctx, doc, processor, imp, imported, kind); err != nil {
				return err
			}
			continue
		}

		if declIndex >= len(doc.Declarations) {
			continue
		}
		decl := doc.Declarations[declIndex]
		declIndex++
		if err := dispatchDeclaration(ctx, doc, processor, decl); err != nil {
			return err
		}

		detailed, ok := processor.(ProcessorDetailed)
		if !ok {
			continue
		}
		if task := element.TaskDefinition(); task != nil {
			if err := dispatchTaskElements(ctx, doc, detailed, decl, task); err != nil {
				return err
			}
		}
		if workflow := element.WorkflowDefinition(); workflow != nil {
			if err := dispatchWorkflowElements(ctx, doc, detailed, decl, workflow); err != nil {
				return err
			}
		}
		if structDef := element.StructDefinition(); structDef != nil {
			if err := dispatchStructElements(ctx, doc, detailed, decl, structDef); err != nil {
				return err
			}
		}
	}

	return nil
}

func dispatchDeclaration(ctx context.Context, doc *Document, processor Processor, decl Declaration) error {
	if err := processor.HandleDeclaration(ctx, doc, decl); err != nil {
		return err
	}

	detailed, ok := processor.(ProcessorDetailed)
	if !ok {
		return nil
	}

	switch decl.Kind {
	case DeclarationTask:
		return detailed.HandleTask(ctx, doc, decl)
	case DeclarationWorkflow:
		return detailed.HandleWorkflow(ctx, doc, decl)
	case DeclarationStruct:
		return detailed.HandleStruct(ctx, doc, decl)
	case DeclarationEnum:
		return detailed.HandleEnum(ctx, doc, decl)
	default:
		return nil
	}
}

func dispatchImport(ctx context.Context, doc *Document, processor Processor, imp ImportRecord, imported *Document, kind string) error {
	if err := processor.HandleImport(ctx, doc, imp, imported); err != nil {
		return err
	}

	detailed, ok := processor.(ProcessorDetailed)
	if !ok {
		return nil
	}

	switch kind {
	case "standard":
		return detailed.HandleImportStandard(ctx, doc, imp, imported)
	case "members":
		return detailed.HandleImportMembers(ctx, doc, imp, imported)
	case "star":
		return detailed.HandleImportStar(ctx, doc, imp, imported)
	default:
		if imp.ImportAllMembers {
			return detailed.HandleImportStar(ctx, doc, imp, imported)
		}
		if imp.NamespaceAlias != "" {
			return detailed.HandleImportStandard(ctx, doc, imp, imported)
		}
		return detailed.HandleImportMembers(ctx, doc, imp, imported)
	}
}

func importKind(stmt grammar.IImportStatementContext) string {
	switch stmt.(type) {
	case *grammar.ImportStatementStandardContext:
		return "standard"
	case *grammar.ImportStatementMembersContext:
		return "members"
	case *grammar.ImportStatementStarContext:
		return "star"
	default:
		return ""
	}
}

func dispatchTaskElements(ctx context.Context, doc *Document, processor ProcessorDetailed, taskDecl Declaration, task grammar.ITaskDefinitionContext) error {
	for _, element := range task.AllTaskElement() {
		switch e := element.(type) {
		case *grammar.TaskDeclarationContext:
			if err := processor.HandleTaskDeclaration(ctx, doc, taskDecl, e.BoundDeclaration()); err != nil {
				return err
			}
		case *grammar.TaskInputSectionContext:
			if err := processor.HandleTaskInputSection(ctx, doc, taskDecl, e.InputSection()); err != nil {
				return err
			}
		case *grammar.TaskOutputSectionContext:
			if err := processor.HandleTaskOutputSection(ctx, doc, taskDecl, e.OutputSection()); err != nil {
				return err
			}
		case *grammar.TaskCommandSectionContext:
			if err := processor.HandleTaskCommandSection(ctx, doc, taskDecl, e.CommandSection()); err != nil {
				return err
			}
		case *grammar.TaskRuntimeSectionContext:
			if err := processor.HandleTaskRuntimeSection(ctx, doc, taskDecl, e.RuntimeSection()); err != nil {
				return err
			}
		case *grammar.TaskRequirementsSectionContext:
			if err := processor.HandleTaskRequirementsSection(ctx, doc, taskDecl, e.RequirementsSection()); err != nil {
				return err
			}
		case *grammar.TaskHintsSectionContext:
			if err := processor.HandleTaskHintsSection(ctx, doc, taskDecl, e.HintsSectionTask()); err != nil {
				return err
			}
		case *grammar.TaskMetadataSectionContext:
			if err := processor.HandleTaskMetadataSection(ctx, doc, taskDecl, e.MetadataSection()); err != nil {
				return err
			}
		case *grammar.TaskParameterMetadataSectionContext:
			if err := processor.HandleTaskParameterMetadataSection(ctx, doc, taskDecl, e.ParameterMetadataSection()); err != nil {
				return err
			}
		}
	}
	return nil
}

func dispatchStructElements(ctx context.Context, doc *Document, processor ProcessorDetailed, structDecl Declaration, structDef grammar.IStructDefinitionContext) error {
	for _, item := range structDef.AllStructItem() {
		switch v := item.(type) {
		case *grammar.StructItemMemberDeclarationContext:
			if err := processor.HandleStructMember(ctx, doc, structDecl, v.StructDeclaration()); err != nil {
				return err
			}
		case *grammar.StructItemParameterMetadataContext:
			if err := processor.HandleStructParameterMetadata(ctx, doc, structDecl, v.ParameterMetadataSection()); err != nil {
				return err
			}
		case *grammar.StructItemMetadataContext:
			if err := processor.HandleStructMetadata(ctx, doc, structDecl, v.MetadataSection()); err != nil {
				return err
			}
		}
	}
	return nil
}

func dispatchWorkflowElements(ctx context.Context, doc *Document, processor ProcessorDetailed, workflowDecl Declaration, workflow grammar.IWorkflowDefinitionContext) error {
	for _, element := range workflow.AllWorkflowElement() {
		switch e := element.(type) {
		case *grammar.WorkflowDeclarationContext:
			if err := processor.HandleWorkflowDeclaration(ctx, doc, workflowDecl, e.BoundDeclaration()); err != nil {
				return err
			}
		case *grammar.WorkflowInputSectionContext:
			if err := processor.HandleWorkflowInputSection(ctx, doc, workflowDecl, e.InputSection()); err != nil {
				return err
			}
		case *grammar.WorkflowOutputSectionContext:
			if err := processor.HandleWorkflowOutputSection(ctx, doc, workflowDecl, e.OutputSection()); err != nil {
				return err
			}
		case *grammar.WorkflowMetadataSectionContext:
			if err := processor.HandleWorkflowMetadataSection(ctx, doc, workflowDecl, e.MetadataSection()); err != nil {
				return err
			}
		case *grammar.WorkflowParameterMetadataSectionContext:
			if err := processor.HandleWorkflowParameterMetadataSection(ctx, doc, workflowDecl, e.ParameterMetadataSection()); err != nil {
				return err
			}
		case *grammar.WorkflowHintsSectionContext:
			if err := processor.HandleWorkflowHintsSection(ctx, doc, workflowDecl, e.HintsSectionWorkflow()); err != nil {
				return err
			}
		case *grammar.WorkflowCallStatementContext:
			if err := processor.HandleWorkflowCall(ctx, doc, workflowDecl, e.CallStatement()); err != nil {
				return err
			}
		case *grammar.WorkflowConditionalStatementContext:
			if err := dispatchConditional(ctx, doc, processor, workflowDecl, e.ConditionalStatement()); err != nil {
				return err
			}
		case *grammar.WorkflowScatterStatementContext:
			if err := dispatchScatter(ctx, doc, processor, workflowDecl, e.ScatterStatement()); err != nil {
				return err
			}
		}
	}
	return nil
}

func dispatchConditional(ctx context.Context, doc *Document, processor ProcessorDetailed, workflowDecl Declaration, conditional grammar.IConditionalStatementContext) error {
	if err := processor.HandleWorkflowConditional(ctx, doc, workflowDecl, conditional); err != nil {
		return err
	}
	for _, st := range conditional.AllWorkflowStatement() {
		if err := dispatchWorkflowStatement(ctx, doc, processor, workflowDecl, st); err != nil {
			return err
		}
	}
	for _, elseIf := range conditional.AllConditionalElseIfClause() {
		for _, st := range elseIf.AllWorkflowStatement() {
			if err := dispatchWorkflowStatement(ctx, doc, processor, workflowDecl, st); err != nil {
				return err
			}
		}
	}
	if elseClause := conditional.ConditionalElseClause(); elseClause != nil {
		for _, st := range elseClause.AllWorkflowStatement() {
			if err := dispatchWorkflowStatement(ctx, doc, processor, workflowDecl, st); err != nil {
				return err
			}
		}
	}
	return nil
}

func dispatchScatter(ctx context.Context, doc *Document, processor ProcessorDetailed, workflowDecl Declaration, scatter grammar.IScatterStatementContext) error {
	if err := processor.HandleWorkflowScatter(ctx, doc, workflowDecl, scatter); err != nil {
		return err
	}
	for _, st := range scatter.ScatterBody().AllWorkflowStatement() {
		if err := dispatchWorkflowStatement(ctx, doc, processor, workflowDecl, st); err != nil {
			return err
		}
	}
	return nil
}

func dispatchWorkflowStatement(ctx context.Context, doc *Document, processor ProcessorDetailed, workflowDecl Declaration, statement grammar.IWorkflowStatementContext) error {
	if statement == nil {
		return nil
	}
	if decl := statement.BoundDeclaration(); decl != nil {
		if err := processor.HandleWorkflowDeclaration(ctx, doc, workflowDecl, decl); err != nil {
			return err
		}
	}
	if call := statement.CallStatement(); call != nil {
		if err := processor.HandleWorkflowCall(ctx, doc, workflowDecl, call); err != nil {
			return err
		}
	}
	if conditional := statement.ConditionalStatement(); conditional != nil {
		if err := dispatchConditional(ctx, doc, processor, workflowDecl, conditional); err != nil {
			return err
		}
	}
	if scatter := statement.ScatterStatement(); scatter != nil {
		if err := dispatchScatter(ctx, doc, processor, workflowDecl, scatter); err != nil {
			return err
		}
	}
	return nil
}
