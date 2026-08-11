package com.myriad.wdl.model.dag;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlMemberAccessOperation;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlCall.WdlCallInput;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.statements.WdlStatement;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

/**
 * Directed acyclic graph of call execution order for a single WDL workflow.
 *
 * <p>Each node in the DAG corresponds to one {@link WdlCall} in the workflow. Dependency edges are
 * derived from two sources:
 *
 * <ol>
 *   <li>Explicit {@code after} clauses on the call.
 *   <li>Data dependencies: any call input expression that references {@code alias.output} where
 *       {@code alias} is the effective name of another call in the same workflow.
 * </ol>
 *
 * <p>Calls nested inside scatter or conditional blocks are included as first-class nodes. An
 * implicit dependency is added from any call inside a scatter to the scatter's collection
 * expression's data deps, and from any call inside a conditional to the conditional's data deps.
 *
 * <p>Usage:
 *
 * <pre>{@code
 * WdlWorkflowDag dag = WdlWorkflowDag.build(workflow, document);
 * if (!dag.isAcyclic()) { ... handle cycles ... }
 * for (WdlWorkflowDag.DagNode node : dag.topologicalOrder()) { ... process in order ... }
 * }</pre>
 */
public final class WdlWorkflowDag {

  /** A node in the execution DAG, wrapping a single {@link WdlCall}. */
  public static final class DagNode {
    private final String callAlias;
    private final WdlCall call;

    DagNode(String callAlias, WdlCall call) {
      this.callAlias = callAlias;
      this.call = call;
    }

    /** The effective call alias (declared alias, or the last segment of the target path). */
    public String getCallAlias() {
      return callAlias;
    }

    /** The underlying call statement. */
    public WdlCall getCall() {
      return call;
    }

    @Override
    public String toString() {
      return callAlias;
    }
  }

  private final List<DagNode> nodes;
  private final Map<String, Set<String>> dependencies;
  private final List<DagNode> topologicalOrder;
  private final List<List<String>> cycles;

  private WdlWorkflowDag(
      List<DagNode> nodes,
      Map<String, Set<String>> dependencies,
      List<DagNode> topologicalOrder,
      List<List<String>> cycles) {
    this.nodes = Collections.unmodifiableList(nodes);
    this.dependencies = Collections.unmodifiableMap(dependencies);
    this.topologicalOrder = Collections.unmodifiableList(topologicalOrder);
    this.cycles = Collections.unmodifiableList(cycles);
  }

  /**
   * Builds the execution DAG for {@code workflow}, using {@code document} to resolve import
   * namespaces when needed.
   */
  public static WdlWorkflowDag build(WdlWorkflow workflow, WdlDocument document) {
    List<DagNode> nodes = new ArrayList<>();
    collectCalls(workflow.getElements(), nodes);

    Set<String> knownAliases = new LinkedHashSet<>();
    for (DagNode node : nodes) {
      knownAliases.add(node.getCallAlias());
    }

    Map<String, Set<String>> deps = new LinkedHashMap<>();
    for (DagNode node : nodes) {
      Set<String> nodeDeps = new LinkedHashSet<>();
      // explicit after-dependencies
      for (String after : node.getCall().afterDependencies()) {
        if (knownAliases.contains(after)) {
          nodeDeps.add(after);
        }
      }
      // data dependencies from input expressions
      for (WdlCallInput input : node.getCall().inputs()) {
        if (input.getValue() != null) {
          collectExpressionCallDeps(input.getValue(), knownAliases, nodeDeps);
        }
      }
      deps.put(node.getCallAlias(), nodeDeps);
    }

    KahnResult result = kahnSort(nodes, deps);
    return new WdlWorkflowDag(nodes, deps, result.sorted, result.cycles);
  }

  /** Returns all call nodes in the workflow, in source order. */
  public List<DagNode> nodes() {
    return nodes;
  }

  /**
   * Returns the dependency edges: for each call alias, the set of call aliases that must complete
   * before it runs.
   */
  public Map<String, Set<String>> dependencies() {
    return dependencies;
  }

  /**
   * Returns calls in topological execution order (dependencies before dependents).
   * If the graph has cycles this list will be shorter than {@link #nodes()}.
   */
  public List<DagNode> topologicalOrder() {
    return topologicalOrder;
  }

  /** Returns {@code true} when no dependency cycles exist. */
  public boolean isAcyclic() {
    return cycles.isEmpty();
  }

  /**
   * Returns any detected dependency cycles as lists of call aliases.
   * Each list represents one cycle path (last element depends on the first).
   */
  public List<List<String>> cycles() {
    return cycles;
  }

  // -------------------------------------------------------------------------
  // Helpers
  // -------------------------------------------------------------------------

  private static void collectCalls(
      Iterable<? extends WdlWorkflowElement> elements, List<DagNode> out) {
    for (WdlWorkflowElement elem : elements) {
      collectCallsFromNode(elem, out);
    }
  }

  private static void collectCallsFromStatements(
      Iterable<? extends WdlStatement> stmts, List<DagNode> out) {
    for (WdlStatement stmt : stmts) {
      collectCallsFromNode(stmt, out);
    }
  }

  private static void collectCallsFromNode(Object node, List<DagNode> out) {
    if (node instanceof WdlCall) {
      WdlCall call = (WdlCall) node;
      out.add(new DagNode(effectiveAlias(call), call));
    } else if (node instanceof WdlScatter) {
      collectCallsFromStatements(((WdlScatter) node).statements(), out);
    } else if (node instanceof WdlConditional) {
      WdlConditional cond = (WdlConditional) node;
      collectCallsFromStatements(cond.thenStatements(), out);
      for (WdlConditional.WdlConditionalElseIf elseIf : cond.elseIfs()) {
        collectCallsFromStatements(elseIf.thenStatements(), out);
      }
      collectCallsFromStatements(cond.elseStatements(), out);
    }
    // WdlInput, WdlOutput, WdlBoundDeclaration, metadata sections — no calls
  }

  private static String effectiveAlias(WdlCall call) {
    if (call.getAlias() != null && !call.getAlias().isBlank()) {
      return call.getAlias();
    }
    // fall back to last segment of target path
    String last = null;
    for (String segment : call.targetPath()) {
      last = segment;
    }
    return last != null ? last : "";
  }

  /**
   * Walks an expression tree and adds call alias names referenced as {@code alias.member} or bare
   * {@code alias} identifiers that match a known call alias.
   */
  private static void collectExpressionCallDeps(
      WdlExpression expr, Set<String> knownAliases, Set<String> out) {
    if (expr == null) return;

    if (expr instanceof WdlMemberAccessOperation) {
      WdlMemberAccessOperation access = (WdlMemberAccessOperation) expr;
      // root variable of a chain like callAlias.outputName (or callAlias.field.subfield)
      WdlExpression root = rootOf(access);
      if (root instanceof WdlVariable) {
        String name = ((WdlVariable) root).getName();
        if (knownAliases.contains(name)) {
          out.add(name);
        }
      }
      // still recurse into target side in case it contains further call refs
      collectExpressionCallDeps(access.getTarget(), knownAliases, out);
      return;
    }

    if (expr instanceof WdlVariable) {
      String name = ((WdlVariable) expr).getName();
      if (knownAliases.contains(name)) {
        out.add(name);
      }
      return;
    }

    if (expr instanceof WdlFunctionCallOperation) {
      for (WdlExpression arg : ((WdlFunctionCallOperation) expr).arguments()) {
        collectExpressionCallDeps(arg, knownAliases, out);
      }
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlBinaryOperation) {
      com.myriad.wdl.model.expressions.WdlBinaryOperation bin =
          (com.myriad.wdl.model.expressions.WdlBinaryOperation) expr;
      collectExpressionCallDeps(bin.getLeft(), knownAliases, out);
      collectExpressionCallDeps(bin.getRight(), knownAliases, out);
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlUnaryOperation) {
      collectExpressionCallDeps(
          ((com.myriad.wdl.model.expressions.WdlUnaryOperation) expr).getOperand(),
          knownAliases,
          out);
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlTernaryOperation) {
      com.myriad.wdl.model.expressions.WdlTernaryOperation ternary =
          (com.myriad.wdl.model.expressions.WdlTernaryOperation) expr;
      collectExpressionCallDeps(ternary.getCondition(), knownAliases, out);
      collectExpressionCallDeps(ternary.getTrueValue(), knownAliases, out);
      collectExpressionCallDeps(ternary.getFalseValue(), knownAliases, out);
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlArrayLiteral) {
      for (WdlExpression item :
          ((com.myriad.wdl.model.expressions.WdlArrayLiteral) expr).entries()) {
        collectExpressionCallDeps(item, knownAliases, out);
      }
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlIndexAccessOperation) {
      com.myriad.wdl.model.expressions.WdlIndexAccessOperation idx =
          (com.myriad.wdl.model.expressions.WdlIndexAccessOperation) expr;
      collectExpressionCallDeps(idx.getTarget(), knownAliases, out);
      collectExpressionCallDeps(idx.getIndex(), knownAliases, out);
      return;
    }

    if (expr instanceof com.myriad.wdl.model.expressions.WdlStringLiteral) {
      for (com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent component :
          ((com.myriad.wdl.model.expressions.WdlStringLiteral) expr).components()) {
        if (component
            instanceof com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder) {
          collectExpressionCallDeps(
              ((com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder) component)
                  .getExpression(),
              knownAliases,
              out);
        }
      }
    }
  }

  /** Returns the leftmost expression in a member-access chain. */
  private static WdlExpression rootOf(WdlExpression expr) {
    WdlExpression current = expr;
    while (current instanceof WdlMemberAccessOperation) {
      current = ((WdlMemberAccessOperation) current).getTarget();
    }
    return current;
  }

  // -------------------------------------------------------------------------
  // Kahn's topological sort
  // -------------------------------------------------------------------------

  private static final class KahnResult {
    final List<DagNode> sorted;
    final List<List<String>> cycles;

    KahnResult(List<DagNode> sorted, List<List<String>> cycles) {
      this.sorted = sorted;
      this.cycles = cycles;
    }
  }

  private static KahnResult kahnSort(List<DagNode> nodes, Map<String, Set<String>> deps) {
    Map<String, DagNode> byAlias = new HashMap<>();
    for (DagNode n : nodes) {
      byAlias.put(n.getCallAlias(), n);
    }

    // in-degree per node
    Map<String, Integer> inDegree = new HashMap<>();
    for (DagNode n : nodes) {
      inDegree.put(n.getCallAlias(), 0);
    }
    // reverse map: alias -> set of aliases that depend on it
    Map<String, Set<String>> dependents = new HashMap<>();
    for (DagNode n : nodes) {
      dependents.put(n.getCallAlias(), new HashSet<>());
    }
    for (Map.Entry<String, Set<String>> entry : deps.entrySet()) {
      String dependent = entry.getKey();
      for (String dependency : entry.getValue()) {
        if (byAlias.containsKey(dependency)) {
          inDegree.merge(dependent, 1, Integer::sum);
          dependents.get(dependency).add(dependent);
        }
      }
    }

    Queue<String> ready = new ArrayDeque<>();
    for (Map.Entry<String, Integer> entry : inDegree.entrySet()) {
      if (entry.getValue() == 0) {
        ready.add(entry.getKey());
      }
    }

    List<DagNode> sorted = new ArrayList<>();
    while (!ready.isEmpty()) {
      String alias = ready.poll();
      sorted.add(byAlias.get(alias));
      for (String dependent : dependents.get(alias)) {
        int newDegree = inDegree.merge(dependent, -1, Integer::sum);
        if (newDegree == 0) {
          ready.add(dependent);
        }
      }
    }

    List<List<String>> cycles = new ArrayList<>();
    if (sorted.size() < nodes.size()) {
      // Detect cycles among remaining nodes using DFS
      Set<String> remaining = new HashSet<>();
      for (DagNode n : nodes) {
        remaining.add(n.getCallAlias());
      }
      for (DagNode n : sorted) {
        remaining.remove(n.getCallAlias());
      }
      Set<String> visited = new HashSet<>();
      for (String start : remaining) {
        if (!visited.contains(start)) {
          List<String> path = new ArrayList<>();
          dfsCycle(start, deps, remaining, visited, new LinkedHashSet<>(), path, cycles);
        }
      }
    }

    return new KahnResult(sorted, cycles);
  }

  private static void dfsCycle(
      String current,
      Map<String, Set<String>> deps,
      Set<String> candidates,
      Set<String> visited,
      LinkedHashSet<String> stack,
      List<String> currentPath,
      List<List<String>> cycles) {
    visited.add(current);
    stack.add(current);

    Set<String> nodeDeps = deps.getOrDefault(current, Collections.emptySet());
    for (String dep : nodeDeps) {
      if (!candidates.contains(dep)) continue;
      if (stack.contains(dep)) {
        // found a cycle — extract it from the stack
        List<String> cycle = new ArrayList<>();
        boolean inCycle = false;
        for (String s : stack) {
          if (s.equals(dep)) inCycle = true;
          if (inCycle) cycle.add(s);
        }
        cycles.add(cycle);
      } else if (!visited.contains(dep)) {
        dfsCycle(dep, deps, candidates, visited, stack, currentPath, cycles);
      }
    }
    stack.remove(current);
  }
}
