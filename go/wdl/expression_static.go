package wdl

import (
	"fmt"
	"strings"

	antlr "github.com/antlr4-go/antlr/v4"
	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

// This file intentionally implements a lightweight static type system that is
// separate from runtime expression evaluation.
//
// Maintainer note:
// - The validator is conservative: "unknown" avoids false positives when local
//   information is incomplete.
// - Workflow scopes are cloned across scatter/conditional branches so branch
//   declarations do not leak into siblings.
// - Diagnostics are emitted where declarations/expressions are seen rather than
//   deferred, to keep error locations intuitive for users.

type staticTypeKind string

const (
	staticTypeUnknown staticTypeKind = "unknown"
	staticTypeNone    staticTypeKind = "none"
	staticTypeBoolean staticTypeKind = "Boolean"
	staticTypeInt     staticTypeKind = "Int"
	staticTypeFloat   staticTypeKind = "Float"
	staticTypeString  staticTypeKind = "String"
	staticTypeFile    staticTypeKind = "File"
	staticTypeDir     staticTypeKind = "Directory"
	staticTypeArray   staticTypeKind = "Array"
	staticTypeMap     staticTypeKind = "Map"
	staticTypePair    staticTypeKind = "Pair"
	staticTypeObject  staticTypeKind = "Object"
	staticTypeRef     staticTypeKind = "Ref"
)

type staticType struct {
	kind     staticTypeKind
	optional bool
	name     string
	elem     *staticType
	key      *staticType
	value    *staticType
	left     *staticType
	right    *staticType
}

var staticStructMemberTypes = map[string]map[string]staticType{}

func expressionStaticDiagnostics(root grammar.IDocumentContext, version Version) []Diagnostic {
	if root == nil {
		return nil
	}
	previousStructTypes := staticStructMemberTypes
	staticStructMemberTypes = collectStructMemberTypes(root)
	defer func() {
		staticStructMemberTypes = previousStructTypes
	}()

	diagnostics := make([]Diagnostic, 0)
	for _, element := range root.AllDocumentElement() {
		workflow := element.WorkflowDefinition()
		if workflow == nil {
			continue
		}
		diagnostics = append(diagnostics, validateWorkflowExpressions(workflow, version)...)
	}
	return diagnostics
}

func collectStructMemberTypes(root grammar.IDocumentContext) map[string]map[string]staticType {
	out := map[string]map[string]staticType{}
	if root == nil {
		return out
	}
	for _, element := range root.AllDocumentElement() {
		structDef := element.StructDefinition()
		if structDef == nil {
			continue
		}
		structName := strictIdentifierText(structDef.StrictIdentifier())
		if structName == "" {
			continue
		}
		memberTypes := map[string]staticType{}
		for _, item := range structDef.AllStructItem() {
			memberCtx, ok := item.(*grammar.StructItemMemberDeclarationContext)
			if !ok || memberCtx.StructDeclaration() == nil {
				continue
			}
			member := memberCtx.StructDeclaration()
			memberName := strictIdentifierText(member.StrictIdentifier())
			if memberName == "" {
				continue
			}
			memberTypes[memberName] = typeFromContext(member.Type_())
		}
		out[structName] = memberTypes
	}
	return out
}

// validateWorkflowExpressions applies expression checks to workflow content.
//
// Tasks are validated by other layers. Keeping workflow checks here avoids
// duplicated declaration-scope logic and keeps static analysis deterministic.
func validateWorkflowExpressions(workflow grammar.IWorkflowDefinitionContext, version Version) []Diagnostic {
	scope := map[string]staticType{}
	diagnostics := make([]Diagnostic, 0)

	for _, element := range workflow.AllWorkflowElement() {
		switch e := element.(type) {
		case *grammar.WorkflowInputSectionContext:
			input := e.InputSection()
			if input == nil {
				continue
			}
			for _, decl := range input.AllDeclaration() {
				diagnostics = append(diagnostics, validateDeclarationExpression(decl.UnboundDeclaration(), decl.BoundDeclaration(), scope, version)...)
			}
		case *grammar.WorkflowDeclarationContext:
			diagnostics = append(diagnostics, validateDeclarationExpression(nil, e.BoundDeclaration(), scope, version)...)
		case *grammar.WorkflowScatterStatementContext:
			scatter := e.ScatterStatement()
			if scatter != nil {
				diagnostics = append(diagnostics, validateWorkflowStatementExpressions(scatter.ScatterBody().AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			}
		case *grammar.WorkflowConditionalStatementContext:
			cond := e.ConditionalStatement()
			if cond == nil {
				continue
			}
			diagnostics = append(diagnostics, validateExpressionBoolean(cond.Expression(), scope, "if expression condition must be Boolean")...)
			diagnostics = append(diagnostics, validateWorkflowStatementExpressions(cond.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			for _, elseIf := range cond.AllConditionalElseIfClause() {
				diagnostics = append(diagnostics, validateExpressionBoolean(elseIf.Expression(), scope, "if expression condition must be Boolean")...)
				diagnostics = append(diagnostics, validateWorkflowStatementExpressions(elseIf.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			}
			if elseClause := cond.ConditionalElseClause(); elseClause != nil {
				diagnostics = append(diagnostics, validateWorkflowStatementExpressions(elseClause.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			}
		}
	}

	return diagnostics
}

func validateWorkflowStatementExpressions(statements []grammar.IWorkflowStatementContext, scope map[string]staticType, version Version) []Diagnostic {
	diagnostics := make([]Diagnostic, 0)
	for _, st := range statements {
		if bd := st.BoundDeclaration(); bd != nil {
			diagnostics = append(diagnostics, validateDeclarationExpression(nil, bd, scope, version)...)
		}
		if scatter := st.ScatterStatement(); scatter != nil {
			diagnostics = append(diagnostics, validateWorkflowStatementExpressions(scatter.ScatterBody().AllWorkflowStatement(), cloneTypeScope(scope), version)...)
		}
		if cond := st.ConditionalStatement(); cond != nil {
			diagnostics = append(diagnostics, validateExpressionBoolean(cond.Expression(), scope, "if expression condition must be Boolean")...)
			diagnostics = append(diagnostics, validateWorkflowStatementExpressions(cond.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			for _, elseIf := range cond.AllConditionalElseIfClause() {
				diagnostics = append(diagnostics, validateExpressionBoolean(elseIf.Expression(), scope, "if expression condition must be Boolean")...)
				diagnostics = append(diagnostics, validateWorkflowStatementExpressions(elseIf.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			}
			if elseClause := cond.ConditionalElseClause(); elseClause != nil {
				diagnostics = append(diagnostics, validateWorkflowStatementExpressions(elseClause.AllWorkflowStatement(), cloneTypeScope(scope), version)...)
			}
		}
	}
	return diagnostics
}

// validateDeclarationExpression checks assignment compatibility and updates the
// current type scope with the declaration's type.
//
// The scope update always happens, even when assignment mismatches are found,
// so later diagnostics can continue with the declared type information.
func validateDeclarationExpression(unbound grammar.IUnboundDeclarationContext, bound grammar.IBoundDeclarationContext, scope map[string]staticType, version Version) []Diagnostic {
	var declaredName string
	var declaredTypeCtx grammar.ITypeContext
	var exprCtx grammar.IExpressionContext
	var declLine int
	var declCol int

	if unbound != nil {
		declaredTypeCtx = unbound.Type_()
		declaredName = strictIdentifierText(unbound.StrictIdentifier())
		declLine = startLine(unbound)
		declCol = startColumn(unbound)
	}
	if bound != nil {
		declaredTypeCtx = bound.Type_()
		declaredName = strictIdentifierText(bound.StrictIdentifier())
		exprCtx = bound.Expression()
		declLine = startLine(bound)
		declCol = startColumn(bound)
	}

	declaredType := typeFromContext(declaredTypeCtx)
	diagnostics := make([]Diagnostic, 0)
	if exprCtx != nil {
		actualType, exprDiagnostics := inferExpressionType(exprCtx, scope, version)
		diagnostics = append(diagnostics, exprDiagnostics...)
		if !isAssignable(declaredType, actualType) {
			diagnostics = append(diagnostics, SemanticError{
				CodeValue: CodeTypeMismatch,
				Severity:  SeverityError,
				Message:   fmt.Sprintf("type mismatch assigning %s to %s", renderStaticType(actualType), renderStaticType(declaredType)),
				AtLine:    declLine,
				AtCol:     declCol,
			})
		}
	}

	if declaredName != "" {
		scope[declaredName] = declaredType
	}

	return diagnostics
}

func validateExpressionBoolean(expr grammar.IExpressionContext, scope map[string]staticType, message string) []Diagnostic {
	t, diagnostics := inferExpressionType(expr, scope, Version13)
	if t.kind == staticTypeUnknown {
		return diagnostics
	}
	if t.kind != staticTypeBoolean {
		diagnostics = append(diagnostics, semanticTypeError(expr, message))
	}
	return diagnostics
}

func inferExpressionType(expr grammar.IExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	if expr == nil || expr.LogicalOrExpression() == nil {
		return staticType{kind: staticTypeUnknown}, nil
	}
	return inferLogicalOrExpression(expr.LogicalOrExpression(), scope, version)
}

func collectBinaryOperatorSymbols(ctx antlr.ParserRuleContext) []string {
	children := ctx.GetChildren()
	if len(children) < 3 {
		return nil
	}
	symbols := make([]string, 0, len(children)/2)
	for i := 1; i < len(children); i += 2 {
		if textNode, ok := children[i].(antlr.ParseTree); ok {
			symbols = append(symbols, textNode.GetText())
		}
	}
	return symbols
}

func inferLogicalOrExpression(expr grammar.ILogicalOrExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.LogicalOrExprOperationContext:
		operands := e.AllLogicalAndExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, diagnostics := inferLogicalAndExpression(operands[0], scope, version)
		if len(operands) == 1 {
			return left, diagnostics
		}
		for i := 1; i < len(operands); i++ {
			right, rightD := inferLogicalAndExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			if left.kind != staticTypeUnknown && left.kind != staticTypeBoolean {
				diagnostics = append(diagnostics, semanticTypeError(e, "logical operator requires Boolean operands"))
			}
			if right.kind != staticTypeUnknown && right.kind != staticTypeBoolean {
				diagnostics = append(diagnostics, semanticTypeError(e, "logical operator requires Boolean operands"))
			}
			left = staticType{kind: staticTypeBoolean}
		}
		return staticType{kind: staticTypeBoolean}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferLogicalAndExpression(expr grammar.ILogicalAndExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.LogicalAndExprOperationContext:
		operands := e.AllEqualityExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, diagnostics := inferEqualityExpression(operands[0], scope, version)
		if len(operands) == 1 {
			return left, diagnostics
		}
		for i := 1; i < len(operands); i++ {
			right, rightD := inferEqualityExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			if left.kind != staticTypeUnknown && left.kind != staticTypeBoolean {
				diagnostics = append(diagnostics, semanticTypeError(e, "logical operator requires Boolean operands"))
			}
			if right.kind != staticTypeUnknown && right.kind != staticTypeBoolean {
				diagnostics = append(diagnostics, semanticTypeError(e, "logical operator requires Boolean operands"))
			}
			left = staticType{kind: staticTypeBoolean}
		}
		return staticType{kind: staticTypeBoolean}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferEqualityExpression(expr grammar.IEqualityExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.EqualityExprOperationContext:
		operands := e.AllComparisonExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, diagnostics := inferComparisonExpression(operands[0], scope, version)
		if len(operands) == 1 {
			return left, diagnostics
		}
		for i := 1; i < len(operands); i++ {
			right, rightD := inferComparisonExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			if left.kind != staticTypeUnknown && right.kind != staticTypeUnknown && !areComparableForEquality(left, right) {
				diagnostics = append(diagnostics, semanticTypeError(e, "equality comparison requires compatible operand types"))
			}
			left = staticType{kind: staticTypeBoolean}
		}
		return staticType{kind: staticTypeBoolean}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferComparisonExpression(expr grammar.IComparisonExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.ComparisonExprOperationContext:
		operands := e.AllAdditiveExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, diagnostics := inferAdditiveExpression(operands[0], scope, version)
		if len(operands) == 1 {
			return left, diagnostics
		}
		for i := 1; i < len(operands); i++ {
			right, rightD := inferAdditiveExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			if left.kind != staticTypeUnknown && right.kind != staticTypeUnknown {
				if !isOrderablePrimitive(left) || !isOrderablePrimitive(right) || !areComparableForEquality(left, right) {
					diagnostics = append(diagnostics, semanticTypeError(e, "order comparison requires orderable primitive operands"))
				}
			}
			left = staticType{kind: staticTypeBoolean}
		}
		return staticType{kind: staticTypeBoolean}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferAdditiveExpression(expr grammar.IAdditiveExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.AdditiveExprOperationContext:
		operands := e.AllMultiplicativeExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		operators := collectBinaryOperatorSymbols(e)
		left, diagnostics := inferMultiplicativeExpression(operands[0], scope, version)
		for i := 1; i < len(operands); i++ {
			right, rightD := inferMultiplicativeExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			op := operators[i-1]
			if op == "-" {
				if !isNumeric(left) || !isNumeric(right) {
					diagnostics = append(diagnostics, semanticTypeError(e, "numeric operator requires numeric operands"))
					left = staticType{kind: staticTypeUnknown}
				} else {
					left = numericResultType(left, right)
				}
				continue
			}
			if isNumeric(left) && isNumeric(right) {
				left = numericResultType(left, right)
				continue
			}
			if left.kind == staticTypeString && right.kind == staticTypeString {
				left = staticType{kind: staticTypeString}
				continue
			}
			diagnostics = append(diagnostics, semanticTypeError(e, "additive operator requires numeric or String operands"))
			left = staticType{kind: staticTypeUnknown}
		}
		return left, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferMultiplicativeExpression(expr grammar.IMultiplicativeExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.MultiplicativeExprOperationContext:
		operands := e.AllPowerExpression()
		if len(operands) == 0 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, diagnostics := inferPowerExpression(operands[0], scope, version)
		for i := 1; i < len(operands); i++ {
			right, rightD := inferPowerExpression(operands[i], scope, version)
			diagnostics = append(diagnostics, rightD...)
			if !isNumeric(left) || !isNumeric(right) {
				diagnostics = append(diagnostics, semanticTypeError(e, "numeric operator requires numeric operands"))
				left = staticType{kind: staticTypeUnknown}
			} else {
				left = numericResultType(left, right)
			}
		}
		return left, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferPowerExpression(expr grammar.IPowerExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.PowerExprNoneContext:
		return inferUnaryExpression(e.UnaryExpression(), scope, version)
	case *grammar.PowerExprOperationContext:
		left, leftD := inferUnaryExpression(e.UnaryExpression(), scope, version)
		right, rightD := inferPowerExpression(e.PowerExpression(), scope, version)
		diagnostics := append(leftD, rightD...)
		if !isNumeric(left) || !isNumeric(right) {
			diagnostics = append(diagnostics, semanticTypeError(e, "numeric operator requires numeric operands"))
		}
		return numericResultType(left, right), diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferUnaryExpression(expr grammar.IUnaryExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.UnaryExprNoneContext:
		return inferPostfixExpression(e.PostfixExpression(), scope, version)
	case *grammar.UnaryExprOperationContext:
		inner, diagnostics := inferUnaryExpression(e.UnaryExpression(), scope, version)
		if e.EXCLAMATION() != nil {
			if inner.kind != staticTypeUnknown && inner.kind != staticTypeBoolean {
				diagnostics = append(diagnostics, semanticTypeError(e, "logical operator requires Boolean operands"))
			}
			return staticType{kind: staticTypeBoolean}, diagnostics
		}
		if inner.kind != staticTypeUnknown && !isNumeric(inner) {
			diagnostics = append(diagnostics, semanticTypeError(e, "numeric operator requires numeric operands"))
		}
		return inner, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferPostfixExpression(expr grammar.IPostfixExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	switch e := expr.(type) {
	case *grammar.PostfixExprNoneContext:
		return inferPrimaryExpression(e.PrimaryExpression(), scope, version)
	case *grammar.PostfixExprArrayIndexContext:
		target, targetD := inferPostfixExpression(e.PostfixExpression(), scope, version)
		index, indexD := inferExpressionType(e.Expression(), scope, version)
		diagnostics := append(targetD, indexD...)
		if index.kind != staticTypeUnknown && index.kind != staticTypeInt {
			diagnostics = append(diagnostics, semanticTypeError(e, "array index must be Int"))
		}
		if target.kind == staticTypeArray && target.elem != nil {
			return *target.elem, diagnostics
		}
		return staticType{kind: staticTypeUnknown}, diagnostics
	case *grammar.PostfixExprFieldContext:
		_, diagnostics := inferPostfixExpression(e.PostfixExpression(), scope, version)
		return staticType{kind: staticTypeUnknown}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, nil
	}
}

func inferPrimaryExpression(expr grammar.IPrimaryExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	if expr == nil {
		return staticType{kind: staticTypeUnknown}, nil
	}
	if expr.NoneLiteral() != nil {
		return staticType{kind: staticTypeNone}, nil
	}
	if expr.BooleanLiteral() != nil {
		return staticType{kind: staticTypeBoolean}, nil
	}
	if number := expr.NumberLiteral(); number != nil {
		if strings.Contains(number.GetText(), ".") {
			return staticType{kind: staticTypeFloat}, nil
		}
		return staticType{kind: staticTypeInt}, nil
	}
	if expr.StringLiteral() != nil {
		return staticType{kind: staticTypeString}, nil
	}
	if variable := expr.Variable(); variable != nil {
		name := strictIdentifierText(variable.StrictIdentifier())
		if name == "None" {
			return staticType{kind: staticTypeNone}, nil
		}
		if t, ok := scope[name]; ok {
			return t, nil
		}
		return staticType{kind: staticTypeUnknown}, nil
	}
	if array := expr.ArrayLiteral(); array != nil {
		return inferArrayLiteralType(array, scope, version)
	}
	if m := expr.MapLiteral(); m != nil {
		return inferMapLiteralType(m, scope, version)
	}
	if pair := expr.PairLiteral(); pair != nil {
		items := pair.AllExpression()
		if len(items) != 2 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		left, leftD := inferExpressionType(items[0], scope, version)
		right, rightD := inferExpressionType(items[1], scope, version)
		return staticType{kind: staticTypePair, left: ptrType(left), right: ptrType(right)}, append(leftD, rightD...)
	}
	if grouped := expr.GroupedExpression(); grouped != nil {
		return inferExpressionType(grouped.Expression(), scope, version)
	}
	if ifExpr := expr.IfExpression(); ifExpr != nil {
		parts := ifExpr.AllExpression()
		if len(parts) != 3 {
			return staticType{kind: staticTypeUnknown}, nil
		}
		cond, condD := inferExpressionType(parts[0], scope, version)
		thenT, thenD := inferExpressionType(parts[1], scope, version)
		elseT, elseD := inferExpressionType(parts[2], scope, version)
		diagnostics := append(append(condD, thenD...), elseD...)
		if cond.kind != staticTypeUnknown && cond.kind != staticTypeBoolean {
			diagnostics = append(diagnostics, semanticTypeError(ifExpr, "if expression condition must be Boolean"))
		}
		if areComparableForEquality(thenT, elseT) {
			return thenT, diagnostics
		}
		if thenT.kind == staticTypeUnknown {
			return elseT, diagnostics
		}
		if elseT.kind == staticTypeUnknown {
			return thenT, diagnostics
		}
		diagnostics = append(diagnostics, semanticTypeError(ifExpr, "if expression branches must have compatible types"))
		return staticType{kind: staticTypeUnknown}, diagnostics
	}
	if call := expr.CallExpression(); call != nil {
		return inferFunctionCallType(call, scope, version)
	}

	if expr.StructLiteral() != nil {
		name := strictIdentifierText(expr.StructLiteral().StrictIdentifier())
		if name != "" {
			return staticType{kind: staticTypeRef, name: name}, nil
		}
		return staticType{kind: staticTypeObject}, nil
	}
	if expr.ObjectLiteral() != nil {
		return staticType{kind: staticTypeObject}, nil
	}

	return staticType{kind: staticTypeUnknown}, nil
}

func inferArrayLiteralType(array grammar.IArrayLiteralContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	items := array.AllExpression()
	diagnostics := make([]Diagnostic, 0)
	if len(items) == 0 {
		return staticType{kind: staticTypeArray, elem: ptrType(staticType{kind: staticTypeUnknown})}, diagnostics
	}

	first, firstD := inferExpressionType(items[0], scope, version)
	diagnostics = append(diagnostics, firstD...)
	elemType := first
	for i := 1; i < len(items); i++ {
		next, nextD := inferExpressionType(items[i], scope, version)
		diagnostics = append(diagnostics, nextD...)
		if !areComparableForEquality(elemType, next) {
			diagnostics = append(diagnostics, semanticTypeError(items[i], "array literal elements must have compatible types"))
		}
	}
	return staticType{kind: staticTypeArray, elem: ptrType(elemType)}, diagnostics
}

func inferMapLiteralType(m grammar.IMapLiteralContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	items := m.AllMapLiteralItem()
	diagnostics := make([]Diagnostic, 0)
	if len(items) == 0 {
		return staticType{kind: staticTypeMap, key: ptrType(staticType{kind: staticTypeUnknown}), value: ptrType(staticType{kind: staticTypeUnknown})}, diagnostics
	}

	firstItem := items[0]
	firstExprs := firstItem.AllExpression()
	if len(firstExprs) != 2 {
		return staticType{kind: staticTypeMap, key: ptrType(staticType{kind: staticTypeUnknown}), value: ptrType(staticType{kind: staticTypeUnknown})}, diagnostics
	}
	keyType, keyD := inferExpressionType(firstExprs[0], scope, version)
	valueType, valueD := inferExpressionType(firstExprs[1], scope, version)
	diagnostics = append(diagnostics, keyD...)
	diagnostics = append(diagnostics, valueD...)

	for i := 1; i < len(items); i++ {
		exprs := items[i].AllExpression()
		if len(exprs) != 2 {
			continue
		}
		k, kd := inferExpressionType(exprs[0], scope, version)
		v, vd := inferExpressionType(exprs[1], scope, version)
		diagnostics = append(diagnostics, kd...)
		diagnostics = append(diagnostics, vd...)
		if !areComparableForEquality(keyType, k) {
			diagnostics = append(diagnostics, semanticTypeError(exprs[0], "map literal keys must have compatible types"))
		}
		if !areComparableForEquality(valueType, v) {
			diagnostics = append(diagnostics, semanticTypeError(exprs[1], "map literal values must have compatible types"))
		}
	}

	return staticType{kind: staticTypeMap, key: ptrType(keyType), value: ptrType(valueType)}, diagnostics
}

func inferFunctionCallType(call grammar.ICallExpressionContext, scope map[string]staticType, version Version) (staticType, []Diagnostic) {
	name := strictIdentifierText(call.StrictIdentifier())
	args := call.AllExpression()
	diagnostics := make([]Diagnostic, 0)
	argTypes := make([]staticType, 0, len(args))
	for _, arg := range args {
		t, ds := inferExpressionType(arg, scope, version)
		diagnostics = append(diagnostics, ds...)
		argTypes = append(argTypes, t)
	}

	errorAtCall := func(message string) {
		diagnostics = append(diagnostics, semanticCallArgumentError(call, message))
	}

	switch name {
	case "keys":
		if len(argTypes) != 1 || argTypes[0].kind != staticTypeMap || argTypes[0].key == nil {
			errorAtCall("invalid function arguments for keys")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		return staticType{kind: staticTypeArray, elem: ptrType(*argTypes[0].key)}, diagnostics
	case "range":
		if len(argTypes) != 1 || argTypes[0].kind != staticTypeInt {
			errorAtCall("invalid function arguments for range")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		return staticType{kind: staticTypeArray, elem: ptrType(staticType{kind: staticTypeInt})}, diagnostics
	case "contains":
		if len(argTypes) != 2 {
			errorAtCall("invalid function arguments for contains")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		if argTypes[0].kind == staticTypeArray {
			if argTypes[0].elem == nil || !isAssignable(*argTypes[0].elem, argTypes[1]) {
				errorAtCall("invalid function arguments for contains")
			}
			return staticType{kind: staticTypeBoolean}, diagnostics
		}
		if argTypes[0].kind == staticTypeString && argTypes[1].kind == staticTypeString {
			return staticType{kind: staticTypeBoolean}, diagnostics
		}
		errorAtCall("invalid function arguments for contains")
		return staticType{kind: staticTypeBoolean}, diagnostics
	case "size":
		if len(argTypes) < 1 || len(argTypes) > 2 {
			errorAtCall("invalid function arguments for size")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		if !(argTypes[0].kind == staticTypeString || argTypes[0].kind == staticTypeFile || argTypes[0].kind == staticTypeDir) {
			errorAtCall("invalid function arguments for size")
		}
		if len(argTypes) == 2 && argTypes[1].kind != staticTypeString {
			errorAtCall("invalid function arguments for size")
		}
		return staticType{kind: staticTypeFloat}, diagnostics
	case "chunk":
		if len(argTypes) != 2 || argTypes[0].kind != staticTypeArray || argTypes[1].kind != staticTypeInt || argTypes[0].elem == nil {
			errorAtCall("invalid function arguments for chunk")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		return staticType{kind: staticTypeArray, elem: ptrType(staticType{kind: staticTypeArray, elem: ptrType(*argTypes[0].elem)})}, diagnostics
	case "cross":
		if len(argTypes) != 2 || argTypes[0].kind != staticTypeArray || argTypes[1].kind != staticTypeArray || argTypes[0].elem == nil || argTypes[1].elem == nil {
			errorAtCall("invalid function arguments for cross")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		pair := staticType{kind: staticTypePair, left: ptrType(*argTypes[0].elem), right: ptrType(*argTypes[1].elem)}
		return staticType{kind: staticTypeArray, elem: ptrType(pair)}, diagnostics
	case "join_paths":
		if len(argTypes) < 2 {
			errorAtCall("invalid function arguments for join_paths")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		if !(argTypes[0].kind == staticTypeString || argTypes[0].kind == staticTypeFile || argTypes[0].kind == staticTypeDir) {
			errorAtCall("invalid function arguments for join_paths")
		}
		for i := 1; i < len(argTypes); i++ {
			if argTypes[i].kind != staticTypeString {
				errorAtCall("invalid function arguments for join_paths")
				break
			}
		}
		return staticType{kind: staticTypeFile}, diagnostics
	case "basename":
		if len(argTypes) < 1 || len(argTypes) > 2 {
			errorAtCall("invalid function arguments for basename")
			return staticType{kind: staticTypeUnknown}, diagnostics
		}
		if !(argTypes[0].kind == staticTypeString || argTypes[0].kind == staticTypeFile || argTypes[0].kind == staticTypeDir) {
			errorAtCall("invalid function arguments for basename")
		}
		if len(argTypes) == 2 && argTypes[1].kind != staticTypeString {
			errorAtCall("invalid function arguments for basename")
		}
		return staticType{kind: staticTypeString}, diagnostics
	default:
		return staticType{kind: staticTypeUnknown}, diagnostics
	}
}

func typeFromContext(ctx grammar.ITypeContext) staticType {
	if ctx == nil {
		return staticType{kind: staticTypeUnknown}
	}
	if p := ctx.PrimitiveType(); p != nil {
		optional := p.QUESTION_MARK() != nil
		switch {
		case p.KEYWORD_BOOLEAN_TYPE() != nil:
			return staticType{kind: staticTypeBoolean, optional: optional}
		case p.KEYWORD_INT_TYPE() != nil:
			return staticType{kind: staticTypeInt, optional: optional}
		case p.KEYWORD_FLOAT_TYPE() != nil:
			return staticType{kind: staticTypeFloat, optional: optional}
		case p.KEYWORD_STRING_TYPE() != nil:
			return staticType{kind: staticTypeString, optional: optional}
		case p.KEYWORD_FILE_TYPE() != nil:
			return staticType{kind: staticTypeFile, optional: optional}
		case p.KEYWORD_DIRECTORY_TYPE() != nil:
			return staticType{kind: staticTypeDir, optional: optional}
		}
	}
	if a := ctx.ArrayType(); a != nil {
		elem := typeFromContext(a.Type_())
		return staticType{kind: staticTypeArray, optional: a.QUESTION_MARK() != nil, elem: ptrType(elem)}
	}
	if m := ctx.MapType(); m != nil {
		key := primitiveTypeFromContext(m.PrimitiveType())
		value := typeFromContext(m.Type_())
		return staticType{kind: staticTypeMap, optional: m.QUESTION_MARK() != nil, key: ptrType(key), value: ptrType(value)}
	}
	if p := ctx.PairType(); p != nil {
		all := p.AllType_()
		if len(all) == 2 {
			left := typeFromContext(all[0])
			right := typeFromContext(all[1])
			return staticType{kind: staticTypePair, optional: p.QUESTION_MARK() != nil, left: ptrType(left), right: ptrType(right)}
		}
		return staticType{kind: staticTypePair, optional: p.QUESTION_MARK() != nil}
	}
	if o := ctx.ObjectType(); o != nil {
		return staticType{kind: staticTypeObject, optional: o.QUESTION_MARK() != nil}
	}
	if r := ctx.TypeRefType(); r != nil {
		return staticType{kind: staticTypeRef, optional: r.QUESTION_MARK() != nil, name: strictIdentifierText(r.StrictIdentifier())}
	}
	return staticType{kind: staticTypeUnknown}
}

func primitiveTypeFromContext(ctx grammar.IPrimitiveTypeContext) staticType {
	if ctx == nil {
		return staticType{kind: staticTypeUnknown}
	}
	optional := ctx.QUESTION_MARK() != nil
	switch {
	case ctx.KEYWORD_BOOLEAN_TYPE() != nil:
		return staticType{kind: staticTypeBoolean, optional: optional}
	case ctx.KEYWORD_INT_TYPE() != nil:
		return staticType{kind: staticTypeInt, optional: optional}
	case ctx.KEYWORD_FLOAT_TYPE() != nil:
		return staticType{kind: staticTypeFloat, optional: optional}
	case ctx.KEYWORD_STRING_TYPE() != nil:
		return staticType{kind: staticTypeString, optional: optional}
	case ctx.KEYWORD_FILE_TYPE() != nil:
		return staticType{kind: staticTypeFile, optional: optional}
	case ctx.KEYWORD_DIRECTORY_TYPE() != nil:
		return staticType{kind: staticTypeDir, optional: optional}
	default:
		return staticType{kind: staticTypeUnknown, optional: optional}
	}
}

func isAssignable(expected staticType, actual staticType) bool {
	if expected.kind == staticTypeUnknown || actual.kind == staticTypeUnknown {
		return true
	}
	if actual.kind == staticTypeNone {
		return expected.optional
	}
	if expected.optional && actual.optional {
		actual.optional = false
		expected.optional = false
	}
	if expected.kind == staticTypeFloat && actual.kind == staticTypeInt {
		return true
	}
	if (expected.kind == staticTypeFile || expected.kind == staticTypeDir) && actual.kind == staticTypeString {
		return true
	}
	if expected.kind != actual.kind {
		return false
	}
	switch expected.kind {
	case staticTypeArray:
		if expected.elem == nil || actual.elem == nil {
			return true
		}
		return isAssignable(*expected.elem, *actual.elem)
	case staticTypeMap:
		if expected.key == nil || expected.value == nil || actual.key == nil || actual.value == nil {
			return true
		}
		return isAssignable(*expected.key, *actual.key) && isAssignable(*expected.value, *actual.value)
	case staticTypePair:
		if expected.left == nil || expected.right == nil || actual.left == nil || actual.right == nil {
			return true
		}
		return isAssignable(*expected.left, *actual.left) && isAssignable(*expected.right, *actual.right)
	case staticTypeRef:
		if expected.name == actual.name {
			return true
		}
		return areStructRefsCompatible(expected.name, actual.name, map[string]struct{}{})
	default:
		return true
	}
}

func areStructRefsCompatible(expectedName string, actualName string, visiting map[string]struct{}) bool {
	if expectedName == actualName {
		return true
	}
	if expectedName == "" || actualName == "" {
		return false
	}
	expectedMembers, expectedOK := staticStructMemberTypes[expectedName]
	actualMembers, actualOK := staticStructMemberTypes[actualName]
	if !expectedOK || !actualOK {
		return false
	}

	pairKey := expectedName + "<=" + actualName
	if _, seen := visiting[pairKey]; seen {
		return true
	}
	visiting[pairKey] = struct{}{}

	for memberName, expectedMemberType := range expectedMembers {
		actualMemberType, ok := actualMembers[memberName]
		if !ok {
			return false
		}
		if expectedMemberType.kind == staticTypeRef && actualMemberType.kind == staticTypeRef {
			if !areStructRefsCompatible(expectedMemberType.name, actualMemberType.name, visiting) {
				return false
			}
			continue
		}
		if !isAssignable(expectedMemberType, actualMemberType) {
			return false
		}
	}

	return true
}

func areComparableForEquality(left staticType, right staticType) bool {
	if left.kind == staticTypeUnknown || right.kind == staticTypeUnknown {
		return true
	}
	if left.kind == staticTypeNone {
		return right.optional
	}
	if right.kind == staticTypeNone {
		return left.optional
	}
	if isNumeric(left) && isNumeric(right) {
		return true
	}
	if left.kind != right.kind {
		return false
	}
	switch left.kind {
	case staticTypeArray:
		if left.elem == nil || right.elem == nil {
			return true
		}
		return areComparableForEquality(*left.elem, *right.elem)
	case staticTypeMap:
		if left.key == nil || right.key == nil || left.value == nil || right.value == nil {
			return true
		}
		return areComparableForEquality(*left.key, *right.key) && areComparableForEquality(*left.value, *right.value)
	case staticTypePair:
		if left.left == nil || right.left == nil || left.right == nil || right.right == nil {
			return true
		}
		return areComparableForEquality(*left.left, *right.left) && areComparableForEquality(*left.right, *right.right)
	case staticTypeRef:
		return left.name == right.name
	default:
		return true
	}
}

func numericResultType(left staticType, right staticType) staticType {
	if left.kind == staticTypeFloat || right.kind == staticTypeFloat {
		return staticType{kind: staticTypeFloat}
	}
	if left.kind == staticTypeUnknown || right.kind == staticTypeUnknown {
		return staticType{kind: staticTypeUnknown}
	}
	return staticType{kind: staticTypeInt}
}

func isNumeric(t staticType) bool {
	return t.kind == staticTypeInt || t.kind == staticTypeFloat
}

func isOrderablePrimitive(t staticType) bool {
	switch t.kind {
	case staticTypeInt, staticTypeFloat, staticTypeString, staticTypeFile, staticTypeDir:
		return true
	default:
		return false
	}
}

func semanticTypeError(node antlr.ParserRuleContext, message string) Diagnostic {
	return SemanticError{
		CodeValue: CodeTypeMismatch,
		Severity:  SeverityError,
		Message:   message,
		AtLine:    startLine(node),
		AtCol:     startColumn(node),
	}
}

func semanticCallArgumentError(node antlr.ParserRuleContext, message string) Diagnostic {
	return SemanticError{
		CodeValue: CodeInvalidFunctionArguments,
		Severity:  SeverityError,
		Message:   message,
		AtLine:    startLine(node),
		AtCol:     startColumn(node),
	}
}

func ptrType(t staticType) *staticType {
	copy := t
	return &copy
}

func cloneTypeScope(in map[string]staticType) map[string]staticType {
	out := make(map[string]staticType, len(in))
	for k, v := range in {
		out[k] = v
	}
	return out
}

func renderStaticType(t staticType) string {
	suffix := ""
	if t.optional {
		suffix = "?"
	}
	switch t.kind {
	case staticTypeArray:
		if t.elem == nil {
			return "Array[Unknown]" + suffix
		}
		return "Array[" + renderStaticType(*t.elem) + "]" + suffix
	case staticTypeMap:
		k := "Unknown"
		v := "Unknown"
		if t.key != nil {
			k = renderStaticType(*t.key)
		}
		if t.value != nil {
			v = renderStaticType(*t.value)
		}
		return "Map[" + k + "," + v + "]" + suffix
	case staticTypePair:
		l := "Unknown"
		r := "Unknown"
		if t.left != nil {
			l = renderStaticType(*t.left)
		}
		if t.right != nil {
			r = renderStaticType(*t.right)
		}
		return "Pair[" + l + "," + r + "]" + suffix
	case staticTypeRef:
		if t.name == "" {
			return "Ref" + suffix
		}
		return t.name + suffix
	default:
		if t.kind == staticTypeUnknown {
			return "Unknown"
		}
		if t.kind == staticTypeNone {
			return "None"
		}
		return string(t.kind) + suffix
	}
}
