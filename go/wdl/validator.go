package wdl

import (
	"context"
	"regexp"
	"strings"

	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

// Validator validates a parsed WDL document.
type Validator interface {
	Validate(ctx context.Context, doc *Document) error
}

// SemanticValidatorConfig controls baseline validation behavior.
type SemanticValidatorConfig struct {
	ThrowOnWarnings bool
}

// SemanticValidator performs baseline semantic checks.
//
// It is the default validator used by package-level Validate when no validator
// is provided.
type SemanticValidator struct {
	throwOnWarnings bool
}

// NewSemanticValidator creates a baseline semantic validator.
//
// Set ThrowOnWarnings to true if warning-only diagnostics should fail
// validation.
func NewSemanticValidator(config SemanticValidatorConfig) *SemanticValidator {
	throwOnWarnings := config.ThrowOnWarnings
	if !config.ThrowOnWarnings {
		throwOnWarnings = false
	}
	return &SemanticValidator{throwOnWarnings: throwOnWarnings}
}

// SetThrowOnWarnings configures whether warning-only diagnostics should fail.
func (v *SemanticValidator) SetThrowOnWarnings(enabled bool) *SemanticValidator {
	v.throwOnWarnings = enabled
	return v
}

// ValidateDocument runs semantic checks and returns all collected diagnostics.
//
// Unlike Validate, this method does not apply warning throw policy.
func (v *SemanticValidator) ValidateDocument(_ context.Context, doc *Document) error {
	diagnostics := make([]Diagnostic, 0)

	byName := map[string]Declaration{}
	visibleNames := map[string]Declaration{}
	workflowCount := 0
	for _, decl := range doc.Declarations {
		if decl.Kind == DeclarationWorkflow {
			workflowCount++
		}
		if previous, exists := byName[decl.Name]; exists {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeDuplicateDefinition,
				Severity:  SeverityError,
				Message:   "duplicate top-level definition: " + decl.Name + " (already defined as " + string(previous.Kind) + ")",
				AtLine:    decl.Line,
				AtCol:     decl.Column,
			})
			continue
		}
		byName[decl.Name] = decl
		visibleNames[decl.Name] = decl
	}

	usedImportNames := map[string]ImportRecord{}
	for _, imp := range doc.ImportStatements {
		importedDoc := doc.ImportedDocs[imp.ResolvedLocation]
		if importedDoc == nil && imp.ResolvedLocation != "" {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeUnknownReference,
				Severity:  SeverityError,
				Message:   "unresolved import: " + imp.RawLocation,
				AtLine:    imp.Line,
				AtCol:     imp.Column,
			})
		}
		if importedDoc != nil && doc.Version.LessThan(importedDoc.Version) {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeFunctionNotAvailableInVersion,
				Severity:  SeverityError,
				Message:   "imported document version " + importedDoc.Version.String() + " is newer than importing document version " + doc.Version.String(),
				AtLine:    imp.Line,
				AtCol:     imp.Column,
			})
		}

		importedNames := map[string]struct{}{}
		if importedDoc != nil {
			for _, importedDecl := range importedDoc.Declarations {
				importedNames[importedDecl.Name] = struct{}{}
			}
		}

		if imp.NamespaceAlias != "" {
			if existingDecl, exists := visibleNames[imp.NamespaceAlias]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeGenericSemanticError,
					Severity:  SeverityError,
					Message:   "import namespace alias conflicts with declaration: " + imp.NamespaceAlias + " (" + string(existingDecl.Kind) + ")",
					AtLine:    imp.Line,
					AtCol:     imp.Column,
				})
			} else if existingImport, exists := usedImportNames[imp.NamespaceAlias]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeGenericSemanticError,
					Severity:  SeverityError,
					Message:   "duplicate import namespace alias: " + imp.NamespaceAlias + " (already used by import " + existingImport.RawLocation + ")",
					AtLine:    imp.Line,
					AtCol:     imp.Column,
				})
			} else {
				usedImportNames[imp.NamespaceAlias] = imp
			}
		}

		for _, alias := range imp.Aliases {
			if len(importedNames) > 0 {
				if _, exists := importedNames[alias.Name]; !exists {
					diagnostics = append(diagnostics, SemanticError{
						CodeValue: CodeUnknownReference,
						Severity:  SeverityError,
						Message:   "unknown imported symbol: " + alias.Name,
						AtLine:    imp.Line,
						AtCol:     imp.Column,
					})
				}
			}

			visible := alias.Name
			if alias.Alias != "" {
				visible = alias.Alias
			}
			if visible == "" {
				continue
			}
			if existingDecl, exists := visibleNames[visible]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeGenericSemanticError,
					Severity:  SeverityError,
					Message:   "imported symbol conflicts with declaration: " + visible + " (" + string(existingDecl.Kind) + ")",
					AtLine:    imp.Line,
					AtCol:     imp.Column,
				})
			} else if existingImport, exists := usedImportNames[visible]; exists {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeGenericSemanticError,
					Severity:  SeverityError,
					Message:   "duplicate imported symbol alias: " + visible + " (already used by import " + existingImport.RawLocation + ")",
					AtLine:    imp.Line,
					AtCol:     imp.Column,
				})
			} else {
				usedImportNames[visible] = imp
			}
		}
	}

	if workflowCount != 1 {
		diagnostics = append(diagnostics, SemanticError{
			CodeValue: CodeGenericSemanticError,
			Severity:  SeverityError,
			Message:   "a document must define exactly one workflow",
			AtLine:    1,
			AtCol:     0,
		})
	}

	if len(diagnostics) == 0 {
		return nil
	}
	return Exception{Diagnostics: diagnostics}
}

// Validate runs semantic checks and applies warning throw policy.
func (v *SemanticValidator) Validate(ctx context.Context, doc *Document) error {
	err := v.ValidateDocument(ctx, doc)
	if err == nil {
		return nil
	}

	ex, ok := err.(Exception)
	if !ok {
		return err
	}

	if v.throwOnWarnings {
		return ex
	}
	if ex.HasErrors() {
		return ex
	}
	return nil
}

// StaticValidator adds deterministic static checks on top of semantic checks.
type StaticValidator struct {
	*SemanticValidator
}

// NewStaticValidator creates a static validator.
func NewStaticValidator(config SemanticValidatorConfig) *StaticValidator {
	return &StaticValidator{SemanticValidator: NewSemanticValidator(config)}
}

// Validate runs semantic and static checks, then applies warning throw policy.
func (v *StaticValidator) Validate(ctx context.Context, doc *Document) error {
	semanticErr := v.SemanticValidator.ValidateDocument(ctx, doc)
	diagnostics := make([]Diagnostic, 0)
	if semanticErr != nil {
		ex, ok := semanticErr.(Exception)
		if !ok {
			return semanticErr
		}
		diagnostics = append(diagnostics, ex.Diagnostics...)
	}

	diagnostics = append(diagnostics, staticDiagnostics(doc)...)
	if len(diagnostics) == 0 {
		return nil
	}

	ex := Exception{Diagnostics: diagnostics}
	if v.throwOnWarnings || ex.HasErrors() {
		return ex
	}
	return nil
}

// LintingValidator adds usage and deprecation diagnostics.
type LintingValidator struct {
	*StaticValidator
}

// NewLintingValidator creates a linting validator.
func NewLintingValidator(config SemanticValidatorConfig) *LintingValidator {
	return &LintingValidator{StaticValidator: NewStaticValidator(config)}
}

// Validate runs semantic, static, and lint checks.
func (v *LintingValidator) Validate(ctx context.Context, doc *Document) error {
	semanticErr := v.StaticValidator.Validate(ctx, doc)

	diagnostics := make([]Diagnostic, 0)
	if semanticErr != nil {
		if ex, ok := semanticErr.(Exception); ok {
			diagnostics = append(diagnostics, ex.Diagnostics...)
		} else {
			return semanticErr
		}
	}

	bodyWithoutImports := stripImportLines(doc.RawSource)
	diagnostics = append(diagnostics, sourceDeprecationDiagnostics(doc.RawSource)...)

	for _, decl := range collectTaskBoundDeclarations(doc) {
		if identifierUsageCount(doc.RawSource, decl.Name) <= 1 {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintUnusedTaskDeclaration,
				Severity:  SeverityWarning,
				Message:   "unused task declaration: " + decl.Name,
				AtLine:    decl.Line,
				AtCol:     decl.Column,
			})
		}
	}

	for _, decl := range collectWorkflowBoundDeclarations(doc) {
		if identifierUsageCount(doc.RawSource, decl.Name) <= 1 {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintUnusedWorkflowDeclaration,
				Severity:  SeverityWarning,
				Message:   "unused workflow declaration: " + decl.Name,
				AtLine:    decl.Line,
				AtCol:     decl.Column,
			})
		}
	}

	for _, scatterVar := range collectScatterVariables(doc) {
		if identifierUsageCount(doc.RawSource, scatterVar.Name) <= 1 {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintUnusedScatterVariable,
				Severity:  SeverityWarning,
				Message:   "unused scatter variable: " + scatterVar.Name,
				AtLine:    scatterVar.Line,
				AtCol:     scatterVar.Column,
			})
		}
	}

	for _, imp := range doc.ImportStatements {
		if strings.HasPrefix(strings.ToLower(imp.RawLocation), "file://") {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintDeprecatedFeature,
				Severity:  SeverityWarning,
				Message:   "deprecated import URI scheme file://; prefer plain relative or absolute paths",
				AtLine:    imp.Line,
				AtCol:     imp.Column,
			})
		}

		if imp.NamespaceAlias != "" && !strings.Contains(bodyWithoutImports, imp.NamespaceAlias+".") {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintUnusedImport,
				Severity:  SeverityWarning,
				Message:   "unused import namespace alias: " + imp.NamespaceAlias,
				AtLine:    imp.Line,
				AtCol:     imp.Column,
			})
		}

		for _, alias := range imp.Aliases {
			name := alias.Name
			if alias.Alias != "" {
				name = alias.Alias
			}
			if name == "" {
				continue
			}
			if !strings.Contains(bodyWithoutImports, name) {
				diagnostics = append(diagnostics, SemanticError{
					CodeValue: CodeLintUnusedImport,
					Severity:  SeverityWarning,
					Message:   "unused imported symbol: " + name,
					AtLine:    imp.Line,
					AtCol:     imp.Column,
				})
			}
		}
	}

	if len(diagnostics) == 0 {
		return nil
	}

	ex := Exception{Diagnostics: diagnostics}
	if v.throwOnWarnings || ex.HasErrors() {
		return ex
	}
	return nil
}

type namedLocation struct {
	Name   string
	Line   int
	Column int
}

func identifierUsageCount(source string, name string) int {
	if name == "" {
		return 0
	}
	re := regexp.MustCompile(`\b` + regexp.QuoteMeta(name) + `\b`)
	return len(re.FindAllStringIndex(source, -1))
}

func collectScatterVariables(doc *Document) []namedLocation {
	root, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok || root == nil {
		return nil
	}
	vars := make([]namedLocation, 0)
	for _, el := range root.AllDocumentElement() {
		wf := el.WorkflowDefinition()
		if wf == nil {
			continue
		}
		for _, we := range wf.AllWorkflowElement() {
			sc, ok := we.(*grammar.WorkflowScatterStatementContext)
			if !ok {
				continue
			}
			stmt := sc.ScatterStatement()
			name := strictIdentifierText(stmt.StrictIdentifier())
			if name == "" {
				continue
			}
			vars = append(vars, namedLocation{Name: name, Line: startLine(stmt), Column: startColumn(stmt)})
		}
	}
	return vars
}

func collectTaskBoundDeclarations(doc *Document) []namedLocation {
	root, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok || root == nil {
		return nil
	}
	decls := make([]namedLocation, 0)
	for _, el := range root.AllDocumentElement() {
		task := el.TaskDefinition()
		if task == nil {
			continue
		}
		for _, te := range task.AllTaskElement() {
			d, ok := te.(*grammar.TaskDeclarationContext)
			if !ok {
				continue
			}
			bd := d.BoundDeclaration()
			if bd == nil {
				continue
			}
			name := strictIdentifierText(bd.StrictIdentifier())
			if name == "" {
				continue
			}
			decls = append(decls, namedLocation{Name: name, Line: startLine(bd), Column: startColumn(bd)})
		}
	}
	return decls
}

func collectWorkflowBoundDeclarations(doc *Document) []namedLocation {
	root, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok || root == nil {
		return nil
	}
	decls := make([]namedLocation, 0)
	for _, el := range root.AllDocumentElement() {
		wf := el.WorkflowDefinition()
		if wf == nil {
			continue
		}
		for _, we := range wf.AllWorkflowElement() {
			d, ok := we.(*grammar.WorkflowDeclarationContext)
			if !ok {
				continue
			}
			bd := d.BoundDeclaration()
			if bd == nil {
				continue
			}
			name := strictIdentifierText(bd.StrictIdentifier())
			if name == "" {
				continue
			}
			decls = append(decls, namedLocation{Name: name, Line: startLine(bd), Column: startColumn(bd)})
		}
	}
	return decls
}

func stripImportLines(source string) string {
	lines := strings.Split(source, "\n")
	out := make([]string, 0, len(lines))
	for _, line := range lines {
		if strings.HasPrefix(strings.TrimSpace(line), "import ") {
			continue
		}
		out = append(out, line)
	}
	return strings.Join(out, "\n")
}

func sourceDeprecationDiagnostics(source string) []Diagnostic {
	type depRule struct {
		re      *regexp.Regexp
		message string
	}

	rules := []depRule{
		{re: regexp.MustCompile(`(?m)^\s*runtime\s*\{`), message: "deprecated runtime section usage"},
		{re: regexp.MustCompile(`\bObject\??\b`), message: "deprecated Object type usage"},
		{re: regexp.MustCompile(`(?m)^\s*docker\s*:`), message: "deprecated requirements key docker; prefer container"},
		{re: regexp.MustCompile(`~\{\s*sep\s*=`), message: "deprecated placeholder option form"},
	}

	diagnostics := make([]Diagnostic, 0)
	for _, rule := range rules {
		matches := rule.re.FindAllStringIndex(source, -1)
		for _, m := range matches {
			line, col := offsetToLineCol(source, m[0])
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeLintDeprecatedFeature,
				Severity:  SeverityWarning,
				Message:   rule.message,
				AtLine:    line,
				AtCol:     col,
			})
		}
	}

	return diagnostics
}

func offsetToLineCol(source string, offset int) (int, int) {
	if offset <= 0 {
		return 1, 0
	}
	line := 1
	col := 0
	for i, r := range source {
		if i >= offset {
			break
		}
		if r == '\n' {
			line++
			col = 0
			continue
		}
		col++
	}
	return line, col
}
