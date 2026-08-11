"""Execution DAG for a WDL workflow — dependency ordering and cycle detection."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from typing import Iterator

from wdl_model.model.definitions import WdlWorkflow
from wdl_model.model.expressions import (
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlExpression,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlMemberAccessOperation,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlVariable,
)
from wdl_model.model.statements import WdlCall, WdlConditional, WdlScatter
from wdl_model.model.wdl_document import WdlDocument


@dataclass
class DagNode:
    """A node in the execution DAG, wrapping a single :class:`WdlCall`."""

    call_alias: str
    call: WdlCall


class WdlWorkflowDag:
    """Directed acyclic graph of call execution order for a single WDL workflow.

    Dependency edges come from:

    * Explicit ``after`` clauses on the call.
    * Data dependencies — any call input expression that references ``alias.output``
      where ``alias`` is the effective name of another call in the same workflow.

    Usage::

        dag = WdlWorkflowDag.build(workflow, document)
        if not dag.is_acyclic():
            print(dag.cycles())
        for node in dag.topological_order():
            ...
    """

    def __init__(
        self,
        nodes: list[DagNode],
        dependencies: dict[str, set[str]],
        topological_order: list[DagNode],
        cycles: list[list[str]],
    ) -> None:
        self._nodes = nodes
        self._dependencies = dependencies
        self._topological_order = topological_order
        self._cycles = cycles

    @classmethod
    def build(cls, workflow: WdlWorkflow, document: WdlDocument) -> "WdlWorkflowDag":
        """Build the execution DAG for *workflow*."""
        nodes: list[DagNode] = []
        _collect_calls(workflow.elements(), nodes)

        known_aliases: set[str] = {n.call_alias for n in nodes}

        deps: dict[str, set[str]] = {}
        for node in nodes:
            node_deps: set[str] = set()
            for after in node.call.afterDependencies():
                if after in known_aliases:
                    node_deps.add(after)
            for call_input in node.call.inputs():
                if call_input.getValue() is not None:
                    _collect_expr_call_deps(call_input.getValue(), known_aliases, node_deps)
            deps[node.call_alias] = node_deps

        sorted_nodes, detected_cycles = _kahn_sort(nodes, deps)
        return cls(nodes, deps, sorted_nodes, detected_cycles)

    def nodes(self) -> list[DagNode]:
        """Return all call nodes in source order."""
        return self._nodes

    def dependencies(self) -> dict[str, set[str]]:
        """Return dependency edges: alias → set of aliases that must complete first."""
        return self._dependencies

    def topological_order(self) -> list[DagNode]:
        """Return calls in topological execution order (dependencies before dependents)."""
        return self._topological_order

    def is_acyclic(self) -> bool:
        """Return ``True`` when the workflow graph has no dependency cycles."""
        return not self._cycles

    def cycles(self) -> list[list[str]]:
        """Return detected dependency cycles as lists of call aliases."""
        return self._cycles


# ---------------------------------------------------------------------------
# Internal helpers
# ---------------------------------------------------------------------------

def _effective_alias(call: WdlCall) -> str:
    if call.alias:
        return call.alias
    path = list(call.targetPath())
    return path[-1] if path else ""


def _collect_calls(elements: "deque | list", out: list[DagNode]) -> None:
    for elem in elements:
        if isinstance(elem, WdlCall):
            out.append(DagNode(call_alias=_effective_alias(elem), call=elem))
        elif isinstance(elem, WdlScatter):
            _collect_calls(elem.statements(), out)
        elif isinstance(elem, WdlConditional):
            _collect_calls(elem.thenStatements(), out)
            for else_if in elem.elseIfs():
                _collect_calls(else_if.thenStatements(), out)
            _collect_calls(elem.elseStatements(), out)


def _root_of(expr: WdlExpression) -> WdlExpression:
    while isinstance(expr, WdlMemberAccessOperation):
        expr = expr.target  # type: ignore[assignment]
    return expr


def _collect_expr_call_deps(
    expr: WdlExpression | None,
    known: set[str],
    out: set[str],
) -> None:
    if expr is None:
        return
    if isinstance(expr, WdlMemberAccessOperation):
        root = _root_of(expr)
        if isinstance(root, WdlVariable) and root.name in known:
            out.add(root.name)  # type: ignore[arg-type]
        _collect_expr_call_deps(expr.target, known, out)
        return
    if isinstance(expr, WdlVariable):
        if expr.name in known:
            out.add(expr.name)  # type: ignore[arg-type]
        return
    if isinstance(expr, WdlFunctionCallOperation):
        for arg in expr.arguments():
            _collect_expr_call_deps(arg, known, out)
        return
    if isinstance(expr, WdlBinaryOperation):
        _collect_expr_call_deps(expr.left, known, out)
        _collect_expr_call_deps(expr.right, known, out)
        return
    if isinstance(expr, WdlUnaryOperation):
        _collect_expr_call_deps(expr.operand, known, out)
        return
    if isinstance(expr, WdlTernaryOperation):
        _collect_expr_call_deps(expr.condition, known, out)
        _collect_expr_call_deps(expr.trueValue, known, out)
        _collect_expr_call_deps(expr.falseValue, known, out)
        return
    if isinstance(expr, WdlArrayLiteral):
        for item in expr.entries():
            _collect_expr_call_deps(item, known, out)
        return
    if isinstance(expr, WdlIndexAccessOperation):
        _collect_expr_call_deps(expr.target, known, out)
        _collect_expr_call_deps(expr.index, known, out)
        return
    if isinstance(expr, WdlStringLiteral):
        for component in expr.components():
            if isinstance(component, WdlStringPlaceholder):
                _collect_expr_call_deps(component.expression, known, out)


def _kahn_sort(
    nodes: list[DagNode],
    deps: dict[str, set[str]],
) -> tuple[list[DagNode], list[list[str]]]:
    by_alias = {n.call_alias: n for n in nodes}
    in_degree: dict[str, int] = {n.call_alias: 0 for n in nodes}
    dependents: dict[str, set[str]] = {n.call_alias: set() for n in nodes}

    for alias, node_deps in deps.items():
        for dep in node_deps:
            if dep in by_alias:
                in_degree[alias] = in_degree.get(alias, 0) + 1
                dependents[dep].add(alias)

    ready: deque[str] = deque(a for a, d in in_degree.items() if d == 0)
    sorted_nodes: list[DagNode] = []

    while ready:
        alias = ready.popleft()
        sorted_nodes.append(by_alias[alias])
        for dependent in dependents[alias]:
            in_degree[dependent] -= 1
            if in_degree[dependent] == 0:
                ready.append(dependent)

    detected: list[list[str]] = []
    if len(sorted_nodes) < len(nodes):
        remaining = {n.call_alias for n in nodes} - {n.call_alias for n in sorted_nodes}
        visited: set[str] = set()
        for start in remaining:
            if start not in visited:
                _dfs_cycle(start, deps, remaining, visited, [], detected)

    return sorted_nodes, detected


def _dfs_cycle(
    current: str,
    deps: dict[str, set[str]],
    candidates: set[str],
    visited: set[str],
    stack: list[str],
    out: list[list[str]],
) -> None:
    visited.add(current)
    stack.append(current)
    for dep in deps.get(current, set()):
        if dep not in candidates:
            continue
        if dep in stack:
            cycle_start = stack.index(dep)
            out.append(list(stack[cycle_start:]))
        elif dep not in visited:
            _dfs_cycle(dep, deps, candidates, visited, stack, out)
    stack.pop()
