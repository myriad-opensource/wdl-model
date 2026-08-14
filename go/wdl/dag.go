package wdl

import (
	"context"

	antlr "github.com/antlr4-go/antlr/v4"
	grammar "github.com/myriad-opensource/wdl-model/go/grammar/wdl1"
)

// WorkflowDagNode represents a single call in a workflow execution DAG.
type WorkflowDagNode struct {
	// CallAlias is the effective call name (declared alias, or last segment of target path).
	CallAlias string
	// TargetPath is the dotted call target as written in the WDL source.
	TargetPath string
	// Line is the 1-based source line of the call statement.
	Line int
	// Column is the 0-based source column of the call statement.
	Column int
	// EndLine is the 1-based source line of the last token of the call statement.
	EndLine int
	// EndColumn is the 0-based exclusive-end column of the last token.
	EndColumn int
}

// WorkflowDag is the execution dependency graph for a single WDL workflow.
//
// Build via [BuildWorkflowDag]; then call [WorkflowDag.IsAcyclic] and
// [WorkflowDag.TopologicalOrder] for safe traversal.
type WorkflowDag struct {
	nodes            []WorkflowDagNode
	dependencies     map[string][]string
	topologicalOrder []WorkflowDagNode
	cycles           [][]string
}

// BuildWorkflowDag extracts the call dependency graph for the named workflow
// from the document's parse tree.
//
// Dependency edges come from:
//   - Explicit [after] clauses on calls.
//   - Data dependencies: expressions in call inputs that reference a known call
//     alias using the pattern alias.output.
//
// Calls nested inside scatter or conditional blocks are included as first-class
// nodes.
func BuildWorkflowDag(_ context.Context, doc *Document, workflowName string) (*WorkflowDag, bool) {
	if doc.ParseTree == nil {
		return nil, false
	}
	docCtx, ok := doc.ParseTree.(grammar.IDocumentContext)
	if !ok {
		return nil, false
	}

	var workflowCtx grammar.IWorkflowDefinitionContext
	for _, elem := range docCtx.AllDocumentElement() {
		wf := elem.WorkflowDefinition()
		if wf == nil {
			continue
		}
		if strictIdentifierText(wf.StrictIdentifier()) == workflowName {
			workflowCtx = wf
			break
		}
	}
	if workflowCtx == nil {
		return nil, false
	}

	nodes := collectWorkflowCalls(workflowCtx.AllWorkflowElement())

	knownAliases := make(map[string]bool, len(nodes))
	for _, n := range nodes {
		knownAliases[n.CallAlias] = true
	}

	deps := make(map[string][]string, len(nodes))
	for _, n := range nodes {
		var nodeDeps []string
		// find the call statement context for this node
		callCtx := findCallContext(workflowCtx.AllWorkflowElement(), n.CallAlias)
		if callCtx != nil {
			for _, afterCtx := range callCtx.AllCallAfterClause() {
				if ident := afterCtx.StrictIdentifier(); ident != nil {
					dep := strictIdentifierText(ident)
					if knownAliases[dep] {
						nodeDeps = append(nodeDeps, dep)
					}
				}
			}
			if inputBlock := callCtx.CallInputBlock(); inputBlock != nil {
				for _, item := range inputBlock.AllCallInputItem() {
					if expr := item.Expression(); expr != nil {
						collectDepsFromExpr(expr, knownAliases, &nodeDeps)
					}
				}
			}
		}
		deps[n.CallAlias] = nodeDeps
	}

	sorted, cycles := kahnSortGo(nodes, deps)
	return &WorkflowDag{
		nodes:            nodes,
		dependencies:     deps,
		topologicalOrder: sorted,
		cycles:           cycles,
	}, true
}

// Nodes returns all call nodes in source order.
func (d *WorkflowDag) Nodes() []WorkflowDagNode { return d.nodes }

// Dependencies returns the dependency map: alias → aliases that must run first.
func (d *WorkflowDag) Dependencies() map[string][]string { return d.dependencies }

// TopologicalOrder returns calls ordered so dependencies always precede dependents.
// If the graph has cycles, the returned slice will be shorter than Nodes.
func (d *WorkflowDag) TopologicalOrder() []WorkflowDagNode { return d.topologicalOrder }

// IsAcyclic returns true when no dependency cycles exist.
func (d *WorkflowDag) IsAcyclic() bool { return len(d.cycles) == 0 }

// Cycles returns any detected cycles as slices of call aliases.
func (d *WorkflowDag) Cycles() [][]string { return d.cycles }

// -------------------------------------------------------------------------
// internal helpers
// -------------------------------------------------------------------------

func callAlias(ctx grammar.ICallStatementContext) string {
	if a := ctx.CallAlias(); a != nil {
		if ident := a.StrictIdentifier(); ident != nil {
			return strictIdentifierText(ident)
		}
	}
	target := ctx.CallTarget()
	if target == nil {
		return ""
	}
	ids := target.AllStrictIdentifier()
	if len(ids) == 0 {
		return ""
	}
	return strictIdentifierText(ids[len(ids)-1])
}

func callTargetPath(ctx grammar.ICallStatementContext) string {
	target := ctx.CallTarget()
	if target == nil {
		return ""
	}
	ids := target.AllStrictIdentifier()
	parts := make([]string, 0, len(ids))
	for _, id := range ids {
		parts = append(parts, strictIdentifierText(id))
	}
	result := ""
	for i, p := range parts {
		if i > 0 {
			result += "."
		}
		result += p
	}
	return result
}

func collectWorkflowCalls(elements []grammar.IWorkflowElementContext) []WorkflowDagNode {
	var out []WorkflowDagNode
	for _, elem := range elements {
		collectCallsFromWorkflowElem(elem, &out)
	}
	return out
}

func collectCallsFromWorkflowElem(elem grammar.IWorkflowElementContext, out *[]WorkflowDagNode) {
	if cs := workflowElemCallStatement(elem); cs != nil {
		ctx := cs.CallStatement()
		if ctx == nil {
			return
		}
		*out = append(*out, WorkflowDagNode{
			CallAlias:  callAlias(ctx),
			TargetPath: callTargetPath(ctx),
			Line:       startLine(ctx),
			Column:     startColumn(ctx),
			EndLine:    endLine(ctx),
			EndColumn:  endColumn(ctx),
		})
		return
	}
	if sc := workflowElemScatter(elem); sc != nil {
		scatter := sc.ScatterStatement()
		if scatter == nil {
			return
		}
		if body := scatter.ScatterBody(); body != nil {
			for _, stmt := range body.AllWorkflowStatement() {
				collectCallsFromWorkflowStmt(stmt, out)
			}
		}
		return
	}
	if cond := workflowElemConditional(elem); cond != nil {
		cs := cond.ConditionalStatement()
		if cs == nil {
			return
		}
		for _, stmt := range cs.AllWorkflowStatement() {
			collectCallsFromWorkflowStmt(stmt, out)
		}
	}
}

func collectCallsFromWorkflowStmt(stmt grammar.IWorkflowStatementContext, out *[]WorkflowDagNode) {
	if stmt == nil {
		return
	}
	if cs := stmt.CallStatement(); cs != nil {
		*out = append(*out, WorkflowDagNode{
			CallAlias:  callAlias(cs),
			TargetPath: callTargetPath(cs),
			Line:       startLine(cs),
			Column:     startColumn(cs),
			EndLine:    endLine(cs),
			EndColumn:  endColumn(cs),
		})
		return
	}
	if sc := stmt.ScatterStatement(); sc != nil {
		if body := sc.ScatterBody(); body != nil {
			for _, inner := range body.AllWorkflowStatement() {
				collectCallsFromWorkflowStmt(inner, out)
			}
		}
		return
	}
	if cond := stmt.ConditionalStatement(); cond != nil {
		for _, inner := range cond.AllWorkflowStatement() {
			collectCallsFromWorkflowStmt(inner, out)
		}
	}
}

func findCallContext(
	elements []grammar.IWorkflowElementContext,
	alias string,
) grammar.ICallStatementContext {
	for _, elem := range elements {
		if cs := workflowElemCallStatement(elem); cs != nil {
			ctx := cs.CallStatement()
			if ctx != nil && callAlias(ctx) == alias {
				return ctx
			}
			continue
		}
		if sc := workflowElemScatter(elem); sc != nil {
			scatter := sc.ScatterStatement()
			if scatter != nil {
				if body := scatter.ScatterBody(); body != nil {
					if found := findCallInStmts(body.AllWorkflowStatement(), alias); found != nil {
						return found
					}
				}
			}
			continue
		}
		if cond := workflowElemConditional(elem); cond != nil {
			cs := cond.ConditionalStatement()
			if cs != nil {
				if found := findCallInStmts(cs.AllWorkflowStatement(), alias); found != nil {
					return found
				}
			}
		}
	}
	return nil
}

func findCallInStmts(stmts []grammar.IWorkflowStatementContext, alias string) grammar.ICallStatementContext {
	for _, stmt := range stmts {
		if stmt == nil {
			continue
		}
		if cs := stmt.CallStatement(); cs != nil && callAlias(cs) == alias {
			return cs
		}
		if sc := stmt.ScatterStatement(); sc != nil {
			if body := sc.ScatterBody(); body != nil {
				if found := findCallInStmts(body.AllWorkflowStatement(), alias); found != nil {
					return found
				}
			}
		}
		if cond := stmt.ConditionalStatement(); cond != nil {
			if found := findCallInStmts(cond.AllWorkflowStatement(), alias); found != nil {
				return found
			}
		}
	}
	return nil
}

// workflowElemCallStatement / Scatter / Conditional are type-assertion helpers
// because IWorkflowElementContext uses labelled alternative subtypes.

func workflowElemCallStatement(e grammar.IWorkflowElementContext) *grammar.WorkflowCallStatementContext {
	v, _ := e.(*grammar.WorkflowCallStatementContext)
	return v
}

func workflowElemScatter(e grammar.IWorkflowElementContext) *grammar.WorkflowScatterStatementContext {
	v, _ := e.(*grammar.WorkflowScatterStatementContext)
	return v
}

func workflowElemConditional(e grammar.IWorkflowElementContext) *grammar.WorkflowConditionalStatementContext {
	v, _ := e.(*grammar.WorkflowConditionalStatementContext)
	return v
}

// collectDepsFromExpr scans an expression tree for member-access patterns like
// alias.output where alias is a known call alias and records the dependency.
func collectDepsFromExpr(expr antlr.Tree, known map[string]bool, out *[]string) {
	if expr == nil {
		return
	}
	if v, ok := expr.(*grammar.PostfixExprFieldContext); ok {
		root := postfixRoot(v.PostfixExpression())
		if callExpr, ok2 := root.(*grammar.CallExpressionContext); ok2 {
			name := callExpr.GetText()
			if known[name] {
				appendUniq(out, name)
			}
		}
	}
	for i := 0; i < expr.GetChildCount(); i++ {
		collectDepsFromExpr(expr.GetChild(i), known, out)
	}
}

func postfixRoot(expr grammar.IPostfixExpressionContext) antlr.Tree {
	for {
		field, ok := expr.(*grammar.PostfixExprFieldContext)
		if !ok {
			return expr
		}
		expr = field.PostfixExpression()
	}
}

func appendUniq(slice *[]string, s string) {
	for _, existing := range *slice {
		if existing == s {
			return
		}
	}
	*slice = append(*slice, s)
}

// kahnSortGo performs Kahn's topological sort on the call nodes.
func kahnSortGo(nodes []WorkflowDagNode, deps map[string][]string) ([]WorkflowDagNode, [][]string) {
	byAlias := make(map[string]WorkflowDagNode, len(nodes))
	for _, n := range nodes {
		byAlias[n.CallAlias] = n
	}

	inDegree := make(map[string]int, len(nodes))
	dependents := make(map[string][]string, len(nodes))
	for _, n := range nodes {
		inDegree[n.CallAlias] = 0
		dependents[n.CallAlias] = nil
	}
	for alias, nodeDeps := range deps {
		for _, dep := range nodeDeps {
			if _, ok := byAlias[dep]; ok {
				inDegree[alias]++
				dependents[dep] = append(dependents[dep], alias)
			}
		}
	}

	var ready []string
	for alias, deg := range inDegree {
		if deg == 0 {
			ready = append(ready, alias)
		}
	}

	var sorted []WorkflowDagNode
	for len(ready) > 0 {
		alias := ready[0]
		ready = ready[1:]
		sorted = append(sorted, byAlias[alias])
		for _, dep := range dependents[alias] {
			inDegree[dep]--
			if inDegree[dep] == 0 {
				ready = append(ready, dep)
			}
		}
	}

	var cycles [][]string
	if len(sorted) < len(nodes) {
		remaining := make(map[string]bool)
		for _, n := range nodes {
			remaining[n.CallAlias] = true
		}
		for _, n := range sorted {
			delete(remaining, n.CallAlias)
		}
		visited := make(map[string]bool)
		for start := range remaining {
			if !visited[start] {
				dfsCycleGo(start, deps, remaining, visited, nil, &cycles)
			}
		}
	}
	return sorted, cycles
}

func dfsCycleGo(
	current string,
	deps map[string][]string,
	candidates map[string]bool,
	visited map[string]bool,
	stack []string,
	out *[][]string,
) {
	visited[current] = true
	stack = append(stack, current)

	for _, dep := range deps[current] {
		if !candidates[dep] {
			continue
		}
		inStack := false
		cycleStart := -1
		for i, s := range stack {
			if s == dep {
				inStack = true
				cycleStart = i
				break
			}
		}
		if inStack {
			cycle := make([]string, len(stack)-cycleStart)
			copy(cycle, stack[cycleStart:])
			*out = append(*out, cycle)
		} else if !visited[dep] {
			dfsCycleGo(dep, deps, candidates, visited, stack, out)
		}
	}
}
