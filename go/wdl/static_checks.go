package wdl

import (
	antlr "github.com/antlr4-go/antlr/v4"
	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

type staticSymbolTable struct {
	taskNames         map[string]struct{}
	typeNames         map[string]struct{}
	importNamespaces  map[string]struct{}
	importedCallables map[string]struct{}
}

// buildStaticSymbolTable collects visibility information used by static checks.
//
// Maintainer note:
// This table models names that are visible during workflow analysis, including
// imported aliases and star-import exports. It intentionally does not try to
// model control-flow-dependent visibility.
func buildStaticSymbolTable(doc *Document) staticSymbolTable {
	tasks := map[string]struct{}{}
	types := map[string]struct{}{}
	for _, decl := range doc.Declarations {
		switch decl.Kind {
		case DeclarationTask:
			tasks[decl.Name] = struct{}{}
		case DeclarationStruct, DeclarationEnum:
			types[decl.Name] = struct{}{}
		}
	}

	namespaces := map[string]struct{}{}
	callables := map[string]struct{}{}
	for _, imp := range doc.ImportStatements {
		if imp.NamespaceAlias != "" {
			namespaces[imp.NamespaceAlias] = struct{}{}
		}
		importedDoc := doc.ImportedDocs[imp.ResolvedLocation]
		if importedDoc != nil {
			importedByName := map[string]Declaration{}
			for _, decl := range importedDoc.Declarations {
				importedByName[decl.Name] = decl
			}
			if imp.ImportAllMembers {
				for _, decl := range importedDoc.Declarations {
					switch decl.Kind {
					case DeclarationTask:
						callables[decl.Name] = struct{}{}
					case DeclarationStruct, DeclarationEnum:
						types[decl.Name] = struct{}{}
					}
				}
			}
			for _, alias := range imp.Aliases {
				visible := alias.Name
				if alias.Alias != "" {
					visible = alias.Alias
				}
				decl, ok := importedByName[alias.Name]
				if !ok {
					continue
				}
				switch decl.Kind {
				case DeclarationTask:
					callables[visible] = struct{}{}
				case DeclarationStruct, DeclarationEnum:
					types[visible] = struct{}{}
				}
			}
		}
	}

	return staticSymbolTable{
		taskNames:         tasks,
		typeNames:         types,
		importNamespaces:  namespaces,
		importedCallables: callables,
	}
}

// staticDiagnostics composes deterministic static passes that run after
// baseline semantic validation.
//
// The order is intentional: function-version gates and expression typing run
// before workflow structure checks so downstream checks can rely on richer
// expression diagnostics already being present.
func staticDiagnostics(doc *Document) []Diagnostic {
	root, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok || root == nil {
		return nil
	}

	diagnostics := make([]Diagnostic, 0)
	symbols := buildStaticSymbolTable(doc)

	diagnostics = append(diagnostics, versionGatedFunctionDiagnostics(root, doc.Version)...)
	diagnostics = append(diagnostics, expressionStaticDiagnostics(root, doc.Version)...)

	for _, element := range root.AllDocumentElement() {
		workflow := element.WorkflowDefinition()
		if workflow == nil {
			continue
		}
		diagnostics = append(diagnostics, validateWorkflow(workflow, symbols)...)
	}

	return diagnostics
}

// versionGatedFunctionDiagnostics reports calls that are valid syntax but not
// available in the current WDL language version.
func versionGatedFunctionDiagnostics(root grammar.IDocumentContext, version Version) []Diagnostic {
	type gate struct {
		minVersion Version
		message    string
	}
	gates := map[string]gate{
		"contains_key": {minVersion: Version12, message: "function contains_key is not available before WDL 1.2"},
		"join_paths":   {minVersion: Version12, message: "function join_paths is not available before WDL 1.2"},
		"value":        {minVersion: Version13, message: "function value is not available before WDL 1.3"},
	}

	diagnostics := make([]Diagnostic, 0)
	for _, call := range collectCallExpressions(root) {
		name := strictIdentifierText(call.StrictIdentifier())
		rule, tracked := gates[name]
		if !tracked || !version.LessThan(rule.minVersion) {
			continue
		}
		diagnostics = append(diagnostics, SemanticError{
			CodeValue: CodeFunctionNotAvailableInVersion,
			Severity:  SeverityError,
			Message:   rule.message,
			AtLine:    startLine(call),
			AtCol:     startColumn(call),
		})
	}
	return diagnostics
}

func collectCallExpressions(root antlr.Tree) []grammar.ICallExpressionContext {
	if root == nil {
		return nil
	}
	out := make([]grammar.ICallExpressionContext, 0)
	var walk func(node antlr.Tree)
	walk = func(node antlr.Tree) {
		if node == nil {
			return
		}
		if call, ok := node.(grammar.ICallExpressionContext); ok {
			out = append(out, call)
		}
		for i := 0; i < node.GetChildCount(); i++ {
			walk(node.GetChild(i))
		}
	}
	walk(root)
	return out
}

func validateWorkflow(workflow grammar.IWorkflowDefinitionContext, symbols staticSymbolTable) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)
	availableCalls := map[string]struct{}{}
	scopeNames := map[string]struct{}{}

	for _, element := range workflow.AllWorkflowElement() {
		switch e := element.(type) {
		case *grammar.WorkflowInputSectionContext:
			for _, decl := range e.InputSection().AllDeclaration() {
				diagnostics = append(diagnostics, validateDeclarationType(decl.UnboundDeclaration(), decl.BoundDeclaration(), symbols)...)
			}
		case *grammar.WorkflowDeclarationContext:
			bd := e.BoundDeclaration()
			if bd != nil {
				diagnostics = append(diagnostics, validateDeclarationType(nil, bd, symbols)...)
				name := strictIdentifierText(bd.StrictIdentifier())
				if name != "" {
					if _, exists := scopeNames[name]; exists {
						diagnostics = append(diagnostics, SemanticError{
							CodeValue: CodeDuplicateDefinition,
							Severity:  SeverityError,
							Message:   "duplicate workflow declaration: " + name,
							AtLine:    startLine(bd),
							AtCol:     startColumn(bd),
						})
					} else {
						scopeNames[name] = struct{}{}
					}
				}
			}
		case *grammar.WorkflowCallStatementContext:
			call := e.CallStatement()
			diagnostics = append(diagnostics, validateCallStatement(call, symbols, availableCalls)...)
			if callName := callStatementName(call); callName != "" {
				availableCalls[callName] = struct{}{}
			}
		case *grammar.WorkflowScatterStatementContext:
			scatter := e.ScatterStatement()
			diagnostics = append(diagnostics, validateScatter(scatter, symbols, cloneSet(availableCalls))...)
		case *grammar.WorkflowConditionalStatementContext:
			cond := e.ConditionalStatement()
			diagnostics = append(diagnostics, validateConditional(cond, symbols, cloneSet(availableCalls))...)
		}
	}

	return diagnostics
}

func validateConditional(cond grammar.IConditionalStatementContext, symbols staticSymbolTable, callSeen map[string]struct{}) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)
	for _, st := range cond.AllWorkflowStatement() {
		diagnostics = append(diagnostics, validateWorkflowStatement(st, symbols, cloneSet(callSeen), map[string]struct{}{})...)
	}
	for _, elseIf := range cond.AllConditionalElseIfClause() {
		for _, st := range elseIf.AllWorkflowStatement() {
			diagnostics = append(diagnostics, validateWorkflowStatement(st, symbols, cloneSet(callSeen), map[string]struct{}{})...)
		}
	}
	if elseClause := cond.ConditionalElseClause(); elseClause != nil {
		for _, st := range elseClause.AllWorkflowStatement() {
			diagnostics = append(diagnostics, validateWorkflowStatement(st, symbols, cloneSet(callSeen), map[string]struct{}{})...)
		}
	}
	return diagnostics
}

func validateScatter(scatter grammar.IScatterStatementContext, symbols staticSymbolTable, callSeen map[string]struct{}) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)
	scope := map[string]struct{}{}
	if scatterVar := strictIdentifierText(scatter.StrictIdentifier()); scatterVar != "" {
		scope[scatterVar] = struct{}{}
	}
	for _, st := range scatter.ScatterBody().AllWorkflowStatement() {
		diagnostics = append(diagnostics, validateWorkflowStatement(st, symbols, callSeen, scope)...)
	}
	return diagnostics
}

func validateWorkflowStatement(stmt grammar.IWorkflowStatementContext, symbols staticSymbolTable, callSeen map[string]struct{}, scopeNames map[string]struct{}) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)

	if bd := stmt.BoundDeclaration(); bd != nil {
		diagnostics = append(diagnostics, validateDeclarationType(nil, bd, symbols)...)
		name := strictIdentifierText(bd.StrictIdentifier())
		if name != "" {
			if _, exists := scopeNames[name]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeDuplicateDefinition,
					Severity:  SeverityError,
					Message:   "duplicate declaration in scope: " + name,
					AtLine:    startLine(bd),
					AtCol:     startColumn(bd),
				})
			} else {
				scopeNames[name] = struct{}{}
			}
		}
	}

	if call := stmt.CallStatement(); call != nil {
		diagnostics = append(diagnostics, validateCallStatement(call, symbols, callSeen)...)
		if name := callStatementName(call); name != "" {
			callSeen[name] = struct{}{}
		}
	}

	if scatter := stmt.ScatterStatement(); scatter != nil {
		diagnostics = append(diagnostics, validateScatter(scatter, symbols, cloneSet(callSeen))...)
	}

	if cond := stmt.ConditionalStatement(); cond != nil {
		diagnostics = append(diagnostics, validateConditional(cond, symbols, cloneSet(callSeen))...)
	}

	return diagnostics
}

func validateDeclarationType(unbound grammar.IUnboundDeclarationContext, bound grammar.IBoundDeclarationContext, symbols staticSymbolTable) []Diagnostic {
	var typ grammar.ITypeContext
	var line int
	var col int
	if unbound != nil {
		typ = unbound.Type_()
		line = startLine(unbound)
		col = startColumn(unbound)
	}
	if bound != nil {
		typ = bound.Type_()
		line = startLine(bound)
		col = startColumn(bound)
	}
	if typ == nil || typ.TypeRefType() == nil {
		return nil
	}
	ref := strictIdentifierText(typ.TypeRefType().StrictIdentifier())
	if ref == "" {
		return nil
	}
	if _, ok := symbols.typeNames[ref]; ok {
		return nil
	}
	return []Diagnostic{SemanticError{
		CodeValue: CodeUnknownReference,
		Severity:  SeverityError,
		Message:   "unknown type reference: " + ref,
		AtLine:    line,
		AtCol:     col,
	}}
}

func validateCallStatement(call grammar.ICallStatementContext, symbols staticSymbolTable, callSeen map[string]struct{}) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)

	target := call.CallTarget()
	if target != nil {
		parts := target.AllStrictIdentifier()
		if len(parts) == 1 {
			name := strictIdentifierText(parts[0])
			_, localTask := symbols.taskNames[name]
			_, importedCallable := symbols.importedCallables[name]
			if !localTask && !importedCallable {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeUnknownReference,
					Severity:  SeverityError,
					Message:   "unknown call target: " + name,
					AtLine:    startLine(call),
					AtCol:     startColumn(call),
				})
			}
		} else if len(parts) > 1 {
			namespace := strictIdentifierText(parts[0])
			if _, ok := symbols.importNamespaces[namespace]; !ok {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeUnknownReference,
					Severity:  SeverityError,
					Message:   "unknown call namespace: " + namespace,
					AtLine:    startLine(call),
					AtCol:     startColumn(call),
				})
			}
		}
	}

	for _, after := range call.AllCallAfterClause() {
		name := strictIdentifierText(after.StrictIdentifier())
		if _, ok := callSeen[name]; !ok {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeUnknownReference,
				Severity:  SeverityError,
				Message:   "unknown after-call dependency: " + name,
				AtLine:    startLine(after),
				AtCol:     startColumn(after),
			})
		}
	}

	if input := call.CallInputBlock(); input != nil {
		seenInputs := map[string]struct{}{}
		for _, item := range input.AllCallInputItem() {
			name := strictIdentifierText(item.StrictIdentifier())
			if _, exists := seenInputs[name]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeDuplicateDefinition,
					Severity:  SeverityError,
					Message:   "duplicate call input assignment: " + name,
					AtLine:    startLine(item),
					AtCol:     startColumn(item),
				})
			} else {
				seenInputs[name] = struct{}{}
			}
		}
	}

	return diagnostics
}

func callStatementName(call grammar.ICallStatementContext) string {
	if call == nil {
		return ""
	}
	if alias := call.CallAlias(); alias != nil {
		if name := strictIdentifierText(alias.StrictIdentifier()); name != "" {
			return name
		}
	}
	target := call.CallTarget()
	if target == nil {
		return ""
	}
	parts := target.AllStrictIdentifier()
	if len(parts) == 0 {
		return ""
	}
	return strictIdentifierText(parts[len(parts)-1])
}

func cloneSet(in map[string]struct{}) map[string]struct{} {
	out := make(map[string]struct{}, len(in))
	for k := range in {
		out[k] = struct{}{}
	}
	return out
}
