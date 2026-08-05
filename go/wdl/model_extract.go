package wdl

import (
	"path/filepath"
	"strconv"
	"strings"

	antlr "github.com/antlr4-go/antlr/v4"
	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

type extractedModel struct {
	version      Version
	imports      []ImportRecord
	declarations []Declaration
}

func extractModelFromDocument(root grammar.IDocumentContext) extractedModel {
	version := Version13
	if vs := root.VersionStatement(); vs != nil && vs.FLOAT() != nil {
		if parsed, err := ParseVersion(vs.FLOAT().GetText()); err == nil {
			version = parsed
		}
	}

	model := extractedModel{
		version:      version,
		imports:      []ImportRecord{},
		declarations: []Declaration{},
	}

	for _, element := range root.AllDocumentElement() {
		if decl, ok := declarationFromElement(element); ok {
			model.declarations = append(model.declarations, decl)
		}
		if imp, ok := importFromElement(element); ok {
			model.imports = append(model.imports, imp)
		}
	}

	return model
}

func declarationFromElement(element grammar.IDocumentElementContext) (Declaration, bool) {
	if task := element.TaskDefinition(); task != nil {
		return Declaration{
			Kind:   DeclarationTask,
			Name:   strictIdentifierText(task.StrictIdentifier()),
			Line:   startLine(task),
			Column: startColumn(task),
		}, true
	}
	if workflow := element.WorkflowDefinition(); workflow != nil {
		return Declaration{
			Kind:   DeclarationWorkflow,
			Name:   strictIdentifierText(workflow.StrictIdentifier()),
			Line:   startLine(workflow),
			Column: startColumn(workflow),
		}, true
	}
	if structDef := element.StructDefinition(); structDef != nil {
		return Declaration{
			Kind:   DeclarationStruct,
			Name:   strictIdentifierText(structDef.StrictIdentifier()),
			Line:   startLine(structDef),
			Column: startColumn(structDef),
		}, true
	}
	if enumDef := element.EnumDefinition(); enumDef != nil {
		return Declaration{
			Kind:   DeclarationEnum,
			Name:   strictIdentifierText(enumDef.StrictIdentifier()),
			Line:   startLine(enumDef),
			Column: startColumn(enumDef),
		}, true
	}
	return Declaration{}, false
}

func importFromElement(element grammar.IDocumentElementContext) (ImportRecord, bool) {
	stmt := element.ImportStatement()
	if stmt == nil {
		return ImportRecord{}, false
	}

	record := ImportRecord{Line: startLine(stmt), Column: startColumn(stmt), Aliases: []ImportMember{}}
	switch v := stmt.(type) {
	case *grammar.ImportStatementStandardContext:
		record.RawLocation = importURILiteralText(v.ImportUriLiteral())
		if ident := v.StrictIdentifier(); ident != nil {
			record.NamespaceAlias = strictIdentifierText(ident)
		} else {
			record.NamespaceAlias = defaultNamespaceAlias(record.RawLocation)
		}
		for _, alias := range v.AllImportAlias() {
			items := alias.AllStrictIdentifier()
			if len(items) == 2 {
				record.Aliases = append(record.Aliases, ImportMember{Name: strictIdentifierText(items[0]), Alias: strictIdentifierText(items[1])})
			}
		}
	case *grammar.ImportStatementMembersContext:
		record.RawLocation = importURILiteralText(v.ImportUriLiteral())
		members := v.ImportMembers()
		if members != nil {
			for _, m := range members.AllImportMember() {
				ids := m.AllStrictIdentifier()
				if len(ids) == 0 {
					continue
				}
				im := ImportMember{Name: strictIdentifierText(ids[0])}
				if len(ids) > 1 {
					im.Alias = strictIdentifierText(ids[1])
				}
				record.Aliases = append(record.Aliases, im)
			}
		}
	case *grammar.ImportStatementStarContext:
		record.RawLocation = importURILiteralText(v.ImportUriLiteral())
		record.ImportAllMembers = true
	}

	if record.RawLocation == "" {
		return ImportRecord{}, false
	}
	return record, true
}

func importURILiteralText(ctx grammar.IImportUriLiteralContext) string {
	if ctx == nil {
		return ""
	}
	raw := ctx.GetText()
	decoded, err := strconv.Unquote(raw)
	if err != nil {
		return raw
	}
	return decoded
}

func strictIdentifierText(ctx grammar.IStrictIdentifierContext) string {
	if ctx == nil {
		return ""
	}
	return ctx.GetText()
}

func defaultNamespaceAlias(importLocation string) string {
	trimmed := strings.TrimSpace(importLocation)
	if trimmed == "" {
		return ""
	}
	base := filepath.Base(trimmed)
	if base == "." || base == string(filepath.Separator) {
		return ""
	}
	ext := filepath.Ext(base)
	if ext != "" {
		base = strings.TrimSuffix(base, ext)
	}
	return base
}

func startLine(ctx antlr.ParserRuleContext) int {
	if ctx == nil || ctx.GetStart() == nil {
		return 0
	}
	return ctx.GetStart().GetLine()
}

func startColumn(ctx antlr.ParserRuleContext) int {
	if ctx == nil || ctx.GetStart() == nil {
		return 0
	}
	return ctx.GetStart().GetColumn()
}
