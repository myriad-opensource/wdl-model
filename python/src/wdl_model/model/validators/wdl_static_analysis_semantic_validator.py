"""Deterministic whole-document static analysis for Python WDL documents.

This layer extends baseline semantic validation with duplicate-definition checks, unknown type and
call-target checks, nested workflow structure validation, stricter function signatures, and
operator/type compatibility checks. The synthetic validator tests demonstrate failures that are
static-only rather than ordinary load-time semantic errors.
"""

from __future__ import annotations

from wdl_model.model.definitions import WdlEnum, WdlStruct, WdlTask, WdlWorkflow
from wdl_model.model.errors import WdlSemanticErrorCode
from wdl_model.model.expressions import (
    WdlBinaryOperation,
    WdlBinaryOperator,
    WdlExpression,
    WdlFunction,
    WdlFunctionCallOperation,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlUnaryOperator,
)
from wdl_model.model.sections import WdlInput, WdlOutput
from wdl_model.model.statements import (
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
)
from wdl_model.model.types import Type as WdlPrimitiveTypeEnum
from wdl_model.model.types import (
    WdlArrayType,
    WdlMapType,
    WdlPairType,
    WdlPrimitiveType,
    WdlType,
    WdlTypeReferenceType,
)
from wdl_model.model.wdl_document import WdlDocument

from .wdl_semantic_validator import WdlSemanticValidator


class WdlStaticAnalysisSemanticValidator(WdlSemanticValidator):
    """Static-analysis validator with deterministic whole-document checks.

    This class intentionally keeps static checks separate from baseline semantic
    checks so callers can choose stricter validation without changing parse/load
    behavior.
    """

    _FUNCTION_SIGNATURES: dict[WdlFunction, list[tuple[str, ...]]] = {
        WdlFunction.FLOOR: [("NUMBER",)],
        WdlFunction.CEIL: [("NUMBER",)],
        WdlFunction.ROUND: [("NUMBER",)],
        WdlFunction.MIN: [("NUMBER", "NUMBER")],
        WdlFunction.MAX: [("NUMBER", "NUMBER")],
        WdlFunction.SUB: [
            ("STRING", "STRING", "STRING"),
            ("STRING", "STRING", "STRING", "STRING"),
        ],
        WdlFunction.STDOUT: [()],
        WdlFunction.STDERR: [()],
        WdlFunction.READ_LINES: [("FILE",)],
        WdlFunction.READ_MAP: [("FILE",)],
        WdlFunction.READ_OBJECT: [("FILE",)],
        WdlFunction.READ_OBJECTS: [("FILE",)],
        WdlFunction.READ_JSON: [("ANY",)],
        WdlFunction.READ_INT: [("FILE",)],
        WdlFunction.READ_FLOAT: [("FILE",)],
        WdlFunction.READ_STRING: [("FILE",)],
        WdlFunction.READ_BOOLEAN: [("FILE",)],
        WdlFunction.WRITE_LINES: [("ARRAY_STRING",)],
        WdlFunction.WRITE_TSV: [("ARRAY_ARRAY_ANY",)],
        WdlFunction.WRITE_MAP: [("MAP_STRING_STRING",)],
        WdlFunction.WRITE_OBJECT: [("OBJECT",)],
        WdlFunction.WRITE_OBJECTS: [("ARRAY_OBJECT",)],
        WdlFunction.WRITE_JSON: [("ANY",)],
        WdlFunction.GLOB: [("STRING",)],
        WdlFunction.SIZE: [("FILE_OR_DIRECTORY",), ("ANY", "STRING")],
        WdlFunction.BASENAME: [("FILE_OR_DIRECTORY",), ("STRING", "STRING")],
        WdlFunction.PREFIX: [("STRING", "ARRAY_ANY")],
        WdlFunction.SUFFIX: [("STRING", "ARRAY_ANY")],
        WdlFunction.QUOTE: [("ARRAY_ANY",)],
        WdlFunction.SQUOTE: [("ARRAY_ANY",)],
        WdlFunction.SEP: [("STRING", "ARRAY_ANY")],
        WdlFunction.LENGTH: [("ANY",)],
        WdlFunction.RANGE: [("INT",)],
        WdlFunction.CHUNK: [("ARRAY_ANY", "INT")],
        WdlFunction.CROSS: [("ARRAY_ANY", "ARRAY_ANY")],
        WdlFunction.ZIP: [("ARRAY_ANY", "ARRAY_ANY")],
        WdlFunction.UNZIP: [("ARRAY_PAIR",)],
        WdlFunction.TRANSPOSE: [("ARRAY_ARRAY_ANY",)],
        WdlFunction.FLATTEN: [("ARRAY_ARRAY_ANY",)],
        WdlFunction.SELECT_FIRST: [
            ("ARRAY_OPTIONAL_ANY",),
            ("ARRAY_OPTIONAL_ANY", "ANY"),
        ],
        WdlFunction.SELECT_ALL: [("ARRAY_OPTIONAL_ANY",)],
        WdlFunction.CONTAINS: [("ARRAY_ANY", "ANY"), ("STRING", "STRING")],
        WdlFunction.CONTAINS_KEY: [("MAP_ANY_ANY", "ANY")],
        WdlFunction.KEYS: [("MAP_ANY_ANY",)],
        WdlFunction.VALUES: [("MAP_ANY_ANY",)],
        WdlFunction.AS_PAIRS: [("MAP_ANY_ANY",)],
        WdlFunction.AS_MAP: [("ARRAY_PAIR",)],
        WdlFunction.COLLECT_BY_KEY: [("ARRAY_PAIR",)],
        WdlFunction.MATCHES: [("STRING", "STRING")],
        WdlFunction.FIND: [("STRING", "STRING")],
        WdlFunction.DEFINED: [("ANY_OPTIONAL",)],
        WdlFunction.JOIN_PATHS: [("FILE_OR_DIRECTORY", "STRING")],
        WdlFunction.VALUE: [("ANY",)],
    }

    _FUNCTION_ARITY: dict[WdlFunction, tuple[int, int | None]] = {
        WdlFunction.FLOOR: (1, 1),
        WdlFunction.CEIL: (1, 1),
        WdlFunction.ROUND: (1, 1),
        WdlFunction.MIN: (2, 2),
        WdlFunction.MAX: (2, 2),
        WdlFunction.SUB: (3, 4),
        WdlFunction.STDOUT: (0, 0),
        WdlFunction.STDERR: (0, 0),
        WdlFunction.READ_LINES: (1, 1),
        WdlFunction.READ_TSV: (1, 2),
        WdlFunction.READ_MAP: (1, 1),
        WdlFunction.READ_OBJECT: (1, 1),
        WdlFunction.READ_OBJECTS: (1, 1),
        WdlFunction.READ_JSON: (1, 1),
        WdlFunction.READ_INT: (1, 1),
        WdlFunction.READ_FLOAT: (1, 1),
        WdlFunction.READ_STRING: (1, 1),
        WdlFunction.READ_BOOLEAN: (1, 1),
        WdlFunction.WRITE_LINES: (1, 1),
        WdlFunction.WRITE_TSV: (1, 1),
        WdlFunction.WRITE_MAP: (1, 1),
        WdlFunction.WRITE_OBJECT: (1, 1),
        WdlFunction.WRITE_OBJECTS: (1, 1),
        WdlFunction.WRITE_JSON: (1, 1),
        WdlFunction.GLOB: (1, 1),
        WdlFunction.SIZE: (1, 2),
        WdlFunction.BASENAME: (1, 2),
        WdlFunction.PREFIX: (2, 2),
        WdlFunction.SUFFIX: (2, 2),
        WdlFunction.QUOTE: (1, 1),
        WdlFunction.SQUOTE: (1, 1),
        WdlFunction.SEP: (2, 2),
        WdlFunction.LENGTH: (1, 1),
        WdlFunction.RANGE: (1, 1),
        WdlFunction.CHUNK: (2, 2),
        WdlFunction.CROSS: (2, 2),
        WdlFunction.ZIP: (2, 2),
        WdlFunction.UNZIP: (1, 1),
        WdlFunction.TRANSPOSE: (1, 1),
        WdlFunction.FLATTEN: (1, 1),
        WdlFunction.SELECT_FIRST: (1, 2),
        WdlFunction.SELECT_ALL: (1, 1),
        WdlFunction.CONTAINS: (2, 2),
        WdlFunction.CONTAINS_KEY: (2, 2),
        WdlFunction.KEYS: (1, 1),
        WdlFunction.VALUES: (1, 1),
        WdlFunction.AS_PAIRS: (1, 1),
        WdlFunction.AS_MAP: (1, 1),
        WdlFunction.COLLECT_BY_KEY: (1, 1),
        WdlFunction.MATCHES: (2, 2),
        WdlFunction.FIND: (2, 2),
        WdlFunction.DEFINED: (1, 1),
        WdlFunction.JOIN_PATHS: (2, None),
        WdlFunction.VALUE: (1, 1),
    }

    def __init__(self) -> None:
        super().__init__()
        self._known_callable_targets: set[str] = set()
        self._known_type_names: set[str] = set()

    def validateDocument(self, document: WdlDocument) -> None:
        """Run static analysis, then baseline semantic validation.

        Maintainer note:
        - Names are pre-indexed (including imports) so checks are deterministic.
        - Duplicate detection is scoped by declaration kind.
        """

        self._known_callable_targets = set()
        self._known_type_names = set()
        top_level_names: set[str] = set()
        for element in document.elements():
            if isinstance(element, WdlTask) and element.name is not None:
                self._known_callable_targets.add(element.name)
                key = f"task:{element.name}"
                if key in top_level_names:
                    self._add_error(
                        f"Duplicate task definition: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                top_level_names.add(key)
            elif isinstance(element, WdlWorkflow) and element.name is not None:
                self._known_callable_targets.add(element.name)
                key = f"workflow:{element.name}"
                if key in top_level_names:
                    self._add_error(
                        f"Duplicate workflow definition: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                top_level_names.add(key)
            elif isinstance(element, WdlStruct) and element.name is not None:
                self._known_type_names.add(element.name)
                key = f"struct:{element.name}"
                if key in top_level_names:
                    self._add_error(
                        f"Duplicate struct definition: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                top_level_names.add(key)
            elif isinstance(element, WdlEnum) and element.name is not None:
                self._known_type_names.add(element.name)
                key = f"enum:{element.name}"
                if key in top_level_names:
                    self._add_error(
                        f"Duplicate enum definition: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                top_level_names.add(key)

        for imp in document.importStatements():
            imported = self._resolve_imported_document(document, imp)
            if imported is None:
                continue

            if isinstance(imp, WdlImportStandard):
                namespace = self._import_namespace(imp)
                for task in imported.tasks():
                    if task.name is not None:
                        self._known_callable_targets.add(f"{namespace}.{task.name}")
                for workflow in imported.workflows():
                    if workflow.name is not None:
                        self._known_callable_targets.add(f"{namespace}.{workflow.name}")
                aliases = self._import_aliases(imp)
                for struct in imported.structs():
                    if struct.name is not None:
                        self._known_type_names.add(
                            aliases.get(struct.name, struct.name)
                        )
                for enum in imported.enums():
                    if enum.name is not None:
                        self._known_type_names.add(aliases.get(enum.name, enum.name))

            elif isinstance(imp, WdlImportStar):
                for task in imported.tasks():
                    if task.name is not None:
                        self._known_callable_targets.add(task.name)
                for workflow in imported.workflows():
                    if workflow.name is not None:
                        self._known_callable_targets.add(workflow.name)
                for struct in imported.structs():
                    if struct.name is not None:
                        self._known_type_names.add(struct.name)
                for enum in imported.enums():
                    if enum.name is not None:
                        self._known_type_names.add(enum.name)

            elif isinstance(imp, WdlImportMembers):
                for member in imp.members():
                    local_name = member.alias if member.alias else member.member
                    if local_name is None:
                        continue
                    kind = self._imported_symbol_kind(imported, member.member or "")
                    if kind in {"task", "workflow"}:
                        self._known_callable_targets.add(local_name)
                    elif kind in {"struct", "enum"}:
                        self._known_type_names.add(local_name)

        for element in document.elements():
            if isinstance(element, WdlStruct) and element.name is not None:
                for member in element.elements():
                    member_type = getattr(member, "type", None)
                    member_name = getattr(member, "name", None)
                    self._validate_known_type_reference(
                        member_type,
                        f"struct '{element.name}' member '{member_name}'",
                    )

        super().validateDocument(document)

    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None:
        seen_call_names: set[str] = set()
        seen_declarations: set[str] = set()

        for element in node.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    self._validate_known_type_reference(
                        declaration.type,
                        f"workflow input '{declaration.name}'",
                    )
                    if (
                        declaration.name is not None
                        and declaration.name in seen_declarations
                    ):
                        self._add_error(
                            f"Duplicate workflow declaration: '{declaration.name}'",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    elif declaration.name is not None:
                        seen_declarations.add(declaration.name)

            elif isinstance(element, WdlBoundDeclaration):
                self._validate_known_type_reference(
                    element.type,
                    f"workflow declaration '{element.name}'",
                )
                if element.name is not None and element.name in seen_declarations:
                    self._add_error(
                        f"Duplicate workflow declaration: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif element.name is not None:
                    seen_declarations.add(element.name)

            elif isinstance(element, WdlCall):
                target = element.targetPathAsString() if element.targetPath() else None
                unqualified_target = (
                    element.targetPath()[-1] if element.targetPath() else None
                )
                call_name = element.alias or target

                if (
                    target is not None
                    and target not in self._known_callable_targets
                    and (unqualified_target not in self._known_callable_targets)
                ):
                    self._add_error(
                        f"Call target '{target}' is not defined",
                        code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                    )

                if call_name is not None and call_name in seen_call_names:
                    self._add_error(
                        f"Duplicate call name in workflow: '{call_name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif call_name is not None:
                    seen_call_names.add(call_name)

                seen_call_inputs: set[str] = set()
                for call_input in element.inputs():
                    key = call_input.getKey()
                    if key is None:
                        continue
                    if key in seen_call_inputs:
                        self._add_error(
                            f"Duplicate call input key '{key}' in call '{call_name or '<unnamed>'}'",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    else:
                        seen_call_inputs.add(key)

                for dep in element.afterDependencies():
                    if dep not in seen_call_names:
                        self._add_error(
                            f"Call '{call_name or '<unnamed>'}' has unknown or forward after dependency '{dep}'",
                            code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                        )

        self._validate_nested_workflow_structure(node)

        super().processWorkflow(ctx, node)

    def processTask(self, ctx: WdlDocument, node: WdlTask) -> None:
        task_declarations: set[str] = set()

        for element in node.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    self._validate_known_type_reference(
                        declaration.type,
                        f"task '{node.name}' input '{declaration.name}'",
                    )
                    if (
                        declaration.name is not None
                        and declaration.name in task_declarations
                    ):
                        self._add_error(
                            f"Duplicate task declaration in '{node.name}': '{declaration.name}'",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    elif declaration.name is not None:
                        task_declarations.add(declaration.name)

            elif isinstance(element, WdlBoundDeclaration):
                self._validate_known_type_reference(
                    element.type,
                    f"task '{node.name}' declaration '{element.name}'",
                )
                if element.name is not None and element.name in task_declarations:
                    self._add_error(
                        f"Duplicate task declaration in '{node.name}': '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif element.name is not None:
                    task_declarations.add(element.name)

            elif isinstance(element, WdlOutput):
                output_names: set[str] = set()
                for declaration in element.elements():
                    self._validate_known_type_reference(
                        declaration.type,
                        f"task '{node.name}' output '{declaration.name}'",
                    )
                    if (
                        declaration.name is not None
                        and declaration.name in output_names
                    ):
                        self._add_error(
                            f"Duplicate task output in '{node.name}': '{declaration.name}'",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    elif declaration.name is not None:
                        output_names.add(declaration.name)

        super().processTask(ctx, node)

    def _validateExpression(self, expr: WdlExpression | None) -> None:
        super()._validateExpression(expr)

        if expr is None:
            return

        if isinstance(expr, WdlBinaryOperation):
            left = self._infer_type(expr.left)
            right = self._infer_type(expr.right)

            if expr.operator in {
                WdlBinaryOperator.LOGICAL_OR,
                WdlBinaryOperator.LOGICAL_AND,
            }:
                if (
                    left is not None
                    and not self._is_primitive(left, WdlPrimitiveTypeEnum.BOOLEAN)
                ) or (
                    right is not None
                    and not self._is_primitive(right, WdlPrimitiveTypeEnum.BOOLEAN)
                ):
                    self._add_error(
                        "Logical operators require Boolean operands",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

            if expr.operator in {
                WdlBinaryOperator.MULTIPLY,
                WdlBinaryOperator.DIVIDE,
                WdlBinaryOperator.MODULUS,
                WdlBinaryOperator.POWER,
                WdlBinaryOperator.SUBTRACT,
            }:
                if (left is not None and not self._is_numeric(left)) or (
                    right is not None and not self._is_numeric(right)
                ):
                    self._add_error(
                        "Numeric operator requires Int or Float operands",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

            if (
                expr.operator == WdlBinaryOperator.ADD
                and left is not None
                and right is not None
            ):
                if not (
                    (self._is_numeric(left) and self._is_numeric(right))
                    or self._is_primitive(left, WdlPrimitiveTypeEnum.STRING)
                    or self._is_primitive(right, WdlPrimitiveTypeEnum.STRING)
                ):
                    self._add_error(
                        "'+' requires numeric operands or string concatenation",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

            if expr.operator in {WdlBinaryOperator.EQUAL, WdlBinaryOperator.NOT_EQUAL}:
                if left is not None and right is not None:
                    if not self._is_type_assignable(
                        left, right
                    ) and not self._is_type_assignable(right, left):
                        self._add_error(
                            "Equality comparison operands are incompatible",
                            code=WdlSemanticErrorCode.TYPE_MISMATCH,
                        )

            if expr.operator in {
                WdlBinaryOperator.LESS,
                WdlBinaryOperator.LESS_EQUAL,
                WdlBinaryOperator.GREATER,
                WdlBinaryOperator.GREATER_EQUAL,
            }:
                if (
                    left is not None
                    and right is not None
                    and not self._are_order_comparable(left, right)
                ):
                    self._add_error(
                        "Ordering comparison operands are incompatible",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

        if isinstance(expr, WdlUnaryOperation):
            operand = self._infer_type(expr.operand)
            if operand is None:
                return
            if expr.operator == WdlUnaryOperator.NOT and not self._is_primitive(
                operand, WdlPrimitiveTypeEnum.BOOLEAN
            ):
                self._add_error(
                    "'!' requires a Boolean operand",
                    code=WdlSemanticErrorCode.TYPE_MISMATCH,
                )
            if expr.operator == WdlUnaryOperator.MINUS and not self._is_numeric(
                operand
            ):
                self._add_error(
                    "Unary '-' requires an Int or Float operand",
                    code=WdlSemanticErrorCode.TYPE_MISMATCH,
                )

        if isinstance(expr, WdlTernaryOperation):
            condition = self._infer_type(expr.condition)
            if condition is not None and not self._is_primitive(
                condition, WdlPrimitiveTypeEnum.BOOLEAN
            ):
                self._add_error(
                    "Ternary condition must be Boolean",
                    code=WdlSemanticErrorCode.TYPE_MISMATCH,
                )

            true_type = self._infer_type(expr.trueValue)
            false_type = self._infer_type(expr.falseValue)
            if true_type is not None and false_type is not None:
                if not self._is_type_assignable(
                    true_type, false_type
                ) and not self._is_type_assignable(false_type, true_type):
                    self._add_error(
                        "Ternary branches have incompatible types",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

        if isinstance(expr, WdlFunctionCallOperation):
            self._validate_generic_function_call(expr)

    def processKeys(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "keys expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        arg_type = self._infer_type(functionCall.arguments()[0])
        if arg_type is not None and not isinstance(arg_type, WdlMapType):
            self._add_error(
                "keys expects a map argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processValues(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "values expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        arg_type = self._infer_type(functionCall.arguments()[0])
        if arg_type is not None and not isinstance(arg_type, WdlMapType):
            self._add_error(
                "values expects a map argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processRange(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "range expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        arg_type = self._infer_type(functionCall.arguments()[0])
        if arg_type is not None and not (
            isinstance(arg_type, WdlPrimitiveType)
            and arg_type.primitiveType() == WdlPrimitiveTypeEnum.INT
        ):
            self._add_error(
                "range expects an Int argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processSelectAll(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "select_all expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        arg_type = self._infer_type(functionCall.arguments()[0])
        if arg_type is not None and not isinstance(arg_type, WdlArrayType):
            self._add_error(
                "select_all expects an array argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processReadInt(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_int", functionCall)

    def processReadFloat(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_float", functionCall)

    def processReadString(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_string", functionCall)

    def processReadBoolean(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_boolean", functionCall)

    def processReadLines(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_lines", functionCall)

    def processReadTsv(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_tsv", functionCall)

    def processReadMap(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_map", functionCall)

    def processReadObject(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_object", functionCall)

    def processReadObjects(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_objects", functionCall)

    def processReadJson(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("read_json", functionCall)

    def processGlob(self, functionCall: WdlFunctionCallOperation) -> None:
        self._validate_single_path_like_arg("glob", functionCall)

    def _validate_single_path_like_arg(
        self, name: str, function_call: WdlFunctionCallOperation
    ) -> None:
        if len(function_call.arguments()) != 1:
            self._add_error(
                f"{name} expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return

        arg_type = self._infer_type(function_call.arguments()[0])
        if arg_type is not None and not self._is_path_like_type(arg_type):
            self._add_error(
                f"{name} expects a String/File/Directory argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def _is_path_like_type(self, t: WdlType) -> bool:
        if not isinstance(t, WdlPrimitiveType):
            return False
        primitive = t.primitiveType()
        return primitive in {
            WdlPrimitiveTypeEnum.STRING,
            WdlPrimitiveTypeEnum.FILE,
            WdlPrimitiveTypeEnum.DIRECTORY,
        }

    def _is_numeric(self, t: WdlType | None) -> bool:
        return self._is_primitive(t, WdlPrimitiveTypeEnum.INT) or self._is_primitive(
            t, WdlPrimitiveTypeEnum.FLOAT
        )

    def _are_order_comparable(self, left: WdlType, right: WdlType) -> bool:
        if self._is_numeric(left) and self._is_numeric(right):
            return True
        return self._is_primitive(
            left, WdlPrimitiveTypeEnum.STRING
        ) and self._is_primitive(right, WdlPrimitiveTypeEnum.STRING)

    def _validate_generic_function_call(
        self, function_call: WdlFunctionCallOperation
    ) -> None:
        fn = function_call.getFunction()
        if fn == WdlFunction.NONSTANDARD:
            return

        argc = len(function_call.arguments())
        limits = self._FUNCTION_ARITY.get(fn)
        if limits is not None:
            min_arity, max_arity = limits
            if argc < min_arity:
                self._add_error(
                    f"{fn.toWdlString()} expects at least {min_arity} argument(s)",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            elif max_arity is not None and argc > max_arity:
                if min_arity == max_arity:
                    self._add_error(
                        f"{fn.toWdlString()} expects exactly {min_arity} argument(s)",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )
                else:
                    self._add_error(
                        f"{fn.toWdlString()} expects between {min_arity} and {max_arity} arguments",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )

        signatures = self._FUNCTION_SIGNATURES.get(fn, [])
        if signatures:
            same_arity_signatures = [sig for sig in signatures if len(sig) == argc]
            if same_arity_signatures:
                any_compatible = False
                for sig in same_arity_signatures:
                    compatible = True
                    for i, arg in enumerate(function_call.arguments()):
                        arg_type = self._infer_type(arg)
                        if arg_type is not None and not self._matches_signature_type(
                            arg_type, sig[i]
                        ):
                            compatible = False
                            break
                    if compatible:
                        any_compatible = True
                        break
                if not any_compatible:
                    self._add_error(
                        f"Argument types are incompatible for function '{fn.toWdlString()}'",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )

        if fn in {WdlFunction.MIN, WdlFunction.MAX} and argc >= 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if (left is not None and not self._is_numeric(left)) or (
                right is not None and not self._is_numeric(right)
            ):
                self._add_error(
                    f"{fn.toWdlString()} expects numeric arguments",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.SEP and argc == 2:
            delim = self._infer_type(function_call.arguments()[0])
            arr = self._infer_type(function_call.arguments()[1])
            if delim is not None and not self._is_primitive(
                delim, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "sep delimiter must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if arr is not None and not isinstance(arr, WdlArrayType):
                self._add_error(
                    "sep second argument must be an Array",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn in {WdlFunction.PREFIX, WdlFunction.SUFFIX} and argc == 2:
            s = self._infer_type(function_call.arguments()[0])
            arr = self._infer_type(function_call.arguments()[1])
            if s is not None and not self._is_primitive(s, WdlPrimitiveTypeEnum.STRING):
                self._add_error(
                    f"{fn.toWdlString()} first argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if arr is not None and not isinstance(arr, WdlArrayType):
                self._add_error(
                    f"{fn.toWdlString()} second argument must be Array",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn in {WdlFunction.QUOTE, WdlFunction.SQUOTE} and argc == 1:
            arr = self._infer_type(function_call.arguments()[0])
            if arr is not None and not isinstance(arr, WdlArrayType):
                self._add_error(
                    f"{fn.toWdlString()} expects an Array argument",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.TRANSPOSE and argc == 1:
            arr = self._infer_type(function_call.arguments()[0])
            if arr is not None and not (
                isinstance(arr, WdlArrayType)
                and isinstance(arr.memberType(), WdlArrayType)
            ):
                self._add_error(
                    "transpose expects Array[Array[X]]",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.FLATTEN and argc == 1:
            arr = self._infer_type(function_call.arguments()[0])
            if arr is not None and not (
                isinstance(arr, WdlArrayType)
                and isinstance(arr.memberType(), WdlArrayType)
            ):
                self._add_error(
                    "flatten expects Array[Array[X]]",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.CHUNK and argc == 2:
            arr = self._infer_type(function_call.arguments()[0])
            count = self._infer_type(function_call.arguments()[1])
            if arr is not None and not isinstance(arr, WdlArrayType):
                self._add_error(
                    "chunk first argument must be Array",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if count is not None and not self._is_primitive(
                count, WdlPrimitiveTypeEnum.INT
            ):
                self._add_error(
                    "chunk second argument must be Int",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.CROSS and argc == 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if (left is not None and not isinstance(left, WdlArrayType)) or (
                right is not None and not isinstance(right, WdlArrayType)
            ):
                self._add_error(
                    "cross expects two array arguments",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.UNZIP and argc == 1:
            arr = self._infer_type(function_call.arguments()[0])
            if arr is not None and not (
                isinstance(arr, WdlArrayType)
                and arr.memberType() is not None
                and isinstance(arr.memberType(), WdlPairType)
            ):
                self._add_error(
                    "unzip expects Array[Pair[X,Y]]",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.AS_PAIRS and argc == 1:
            map_t = self._infer_type(function_call.arguments()[0])
            if map_t is not None and not isinstance(map_t, WdlMapType):
                self._add_error(
                    "as_pairs expects a Map argument",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.COLLECT_BY_KEY and argc == 1:
            arr = self._infer_type(function_call.arguments()[0])
            if arr is not None and not (
                isinstance(arr, WdlArrayType)
                and arr.memberType() is not None
                and isinstance(arr.memberType(), WdlPairType)
            ):
                self._add_error(
                    "collect_by_key expects Array[Pair[K,V]]",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.MATCHES and argc == 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if left is not None and not self._is_primitive(
                left, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "matches first argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if right is not None and not self._is_primitive(
                right, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "matches second argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.FIND and argc == 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if left is not None and not self._is_primitive(
                left, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "find first argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if right is not None and not self._is_primitive(
                right, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "find second argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.CONTAINS and argc == 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if isinstance(left, WdlArrayType) and right is not None:
                m = left.memberType()
                if m is not None and not self._is_type_assignable(m, right):
                    self._add_error(
                        "contains argument type is incompatible with array member type",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )
            elif left is not None:
                if not self._is_primitive(left, WdlPrimitiveTypeEnum.STRING):
                    self._add_error(
                        "contains first argument must be Array or String",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )
                elif right is not None and not self._is_primitive(
                    right, WdlPrimitiveTypeEnum.STRING
                ):
                    self._add_error(
                        "contains second argument must be String when first argument is String",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )

        if fn == WdlFunction.CONTAINS_KEY and argc == 2:
            map_t = self._infer_type(function_call.arguments()[0])
            key_t = self._infer_type(function_call.arguments()[1])
            if map_t is not None and not isinstance(map_t, WdlMapType):
                self._add_error(
                    "contains_key first argument must be a Map",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            elif isinstance(map_t, WdlMapType) and key_t is not None:
                expected = map_t.keyType()
                if expected is not None and not self._is_type_assignable(
                    expected, key_t
                ):
                    self._add_error(
                        "contains_key key type is incompatible with map key type",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )

        if fn == WdlFunction.SIZE and argc >= 1:
            t = self._infer_type(function_call.arguments()[0])
            if t is not None and not self._is_path_like_type(t):
                self._add_error(
                    "size first argument must be String/File/Directory",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            if argc >= 2:
                unit = self._infer_type(function_call.arguments()[1])
                if unit is not None and not self._is_primitive(
                    unit, WdlPrimitiveTypeEnum.STRING
                ):
                    self._add_error(
                        "size second argument must be String",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )

        if fn == WdlFunction.BASENAME and argc >= 2:
            base = self._infer_type(function_call.arguments()[0])
            if base is not None and not self._is_path_like_type(base):
                self._add_error(
                    "basename first argument must be String/File/Directory",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            suffix = self._infer_type(function_call.arguments()[1])
            if suffix is not None and not self._is_primitive(
                suffix, WdlPrimitiveTypeEnum.STRING
            ):
                self._add_error(
                    "basename second argument must be String",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.BASENAME and argc == 1:
            base = self._infer_type(function_call.arguments()[0])
            if base is not None and not self._is_path_like_type(base):
                self._add_error(
                    "basename first argument must be String/File/Directory",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if fn == WdlFunction.JOIN_PATHS and argc >= 2:
            first = self._infer_type(function_call.arguments()[0])
            if first is not None and not self._is_path_like_type(first):
                self._add_error(
                    "join_paths first argument must be String/File/Directory",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            for arg in list(function_call.arguments())[1:]:
                arg_type = self._infer_type(arg)
                if arg_type is not None and not self._is_primitive(
                    arg_type, WdlPrimitiveTypeEnum.STRING
                ):
                    self._add_error(
                        "join_paths arguments after the first must be String",
                        code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                    )
                    break

    def _matches_signature_type(self, actual: WdlType, sig: str) -> bool:
        if sig in {"ANY", "ANY_OPTIONAL"}:
            return True
        if sig == "NUMBER":
            return self._is_numeric(actual)
        if sig == "BOOLEAN":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.BOOLEAN)
        if sig == "INT":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.INT)
        if sig == "FLOAT":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.FLOAT)
        if sig in {"STRING", "STRING_OPTIONAL"}:
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.STRING)
        if sig == "FILE":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.FILE)
        if sig == "DIRECTORY":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.DIRECTORY)
        if sig == "FILE_OR_DIRECTORY":
            return (
                self._is_primitive(actual, WdlPrimitiveTypeEnum.FILE)
                or self._is_primitive(actual, WdlPrimitiveTypeEnum.DIRECTORY)
                or self._is_primitive(actual, WdlPrimitiveTypeEnum.STRING)
            )
        if sig == "OBJECT":
            return self._is_primitive(actual, WdlPrimitiveTypeEnum.OBJECT)
        if sig in {
            "ARRAY_ANY",
            "ARRAY_FILE",
            "ARRAY_OPTIONAL_ANY",
            "ARRAY_INT",
            "ARRAY_STRING",
            "ARRAY_OBJECT",
            "ARRAY_PAIR",
            "ARRAY_ARRAY_ANY",
            "ARRAY_ARRAY_STRING",
        }:
            return self._matches_array_signature(actual, sig)
        if sig in {"MAP_ANY_ANY", "MAP_ANY_ARRAY", "MAP_STRING_STRING"}:
            return self._matches_map_signature(actual, sig)
        if sig == "PAIR_ARRAY":
            return (
                isinstance(actual, WdlPairType)
                and isinstance(actual.leftType(), WdlArrayType)
                and isinstance(actual.rightType(), WdlArrayType)
            )
        return True

    def _matches_array_signature(self, actual: WdlType, sig: str) -> bool:
        if not isinstance(actual, WdlArrayType):
            return False
        member = actual.memberType()
        if sig in {"ARRAY_ANY", "ARRAY_OPTIONAL_ANY"}:
            return True
        if sig == "ARRAY_FILE":
            return member is not None and self._is_primitive(
                member, WdlPrimitiveTypeEnum.FILE
            )
        if sig == "ARRAY_INT":
            return member is not None and self._is_primitive(
                member, WdlPrimitiveTypeEnum.INT
            )
        if sig == "ARRAY_STRING":
            return member is not None and self._is_primitive(
                member, WdlPrimitiveTypeEnum.STRING
            )
        if sig == "ARRAY_OBJECT":
            return member is not None and self._is_primitive(
                member, WdlPrimitiveTypeEnum.OBJECT
            )
        if sig == "ARRAY_PAIR":
            return isinstance(member, WdlPairType)
        if sig == "ARRAY_ARRAY_ANY":
            return isinstance(member, WdlArrayType)
        if sig == "ARRAY_ARRAY_STRING":
            return (
                isinstance(member, WdlArrayType)
                and member.memberType() is not None
                and self._is_primitive(member.memberType(), WdlPrimitiveTypeEnum.STRING)
            )
        return True

    def _matches_map_signature(self, actual: WdlType, sig: str) -> bool:
        if not isinstance(actual, WdlMapType):
            return False
        if sig == "MAP_ANY_ANY":
            return True
        if sig == "MAP_ANY_ARRAY":
            return isinstance(actual.valueType(), WdlArrayType)
        if sig == "MAP_STRING_STRING":
            return self._is_primitive(
                actual.keyType(), WdlPrimitiveTypeEnum.STRING
            ) and self._is_primitive(actual.valueType(), WdlPrimitiveTypeEnum.STRING)
        return True

    def _validate_known_type_reference(self, t: WdlType | None, location: str) -> None:
        if t is None:
            return
        if isinstance(t, WdlTypeReferenceType):
            ref = t.referenceName()
            if ref is not None and ref not in self._known_type_names:
                self._add_error(
                    f"Unknown type reference '{ref}' in {location}",
                    code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                )
            return
        if isinstance(t, WdlArrayType):
            self._validate_known_type_reference(t.memberType(), location)
            return
        if isinstance(t, WdlMapType):
            self._validate_known_type_reference(t.keyType(), location)
            self._validate_known_type_reference(t.valueType(), location)
            return
        if isinstance(t, WdlPairType):
            self._validate_known_type_reference(t.leftType(), location)
            self._validate_known_type_reference(t.rightType(), location)

    def _validate_nested_workflow_structure(self, workflow: WdlWorkflow) -> None:
        available_calls: set[str] = set()
        names_in_block: set[str] = set()

        for element in workflow.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    if (
                        declaration.name is not None
                        and declaration.name in names_in_block
                    ):
                        self._add_error(
                            f"Duplicate workflow declaration: '{declaration.name}'",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    elif declaration.name is not None:
                        names_in_block.add(declaration.name)
            elif isinstance(element, WdlBoundDeclaration):
                if element.name is not None and element.name in names_in_block:
                    self._add_error(
                        f"Duplicate workflow declaration: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif element.name is not None:
                    names_in_block.add(element.name)
            elif isinstance(element, WdlCall):
                self._validate_call_structure(element, names_in_block, available_calls)
            elif isinstance(element, WdlScatter):
                if element.name is not None and element.name in names_in_block:
                    self._add_error(
                        f"Duplicate workflow declaration: '{element.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif element.name is not None:
                    names_in_block.add(element.name)
                self._validate_nested_statements(
                    element.statements(), available_calls, "scatter"
                )
            elif isinstance(element, WdlConditional):
                self._validate_conditional_structure(
                    element, available_calls, "conditional"
                )

    def _validate_nested_statements(
        self,
        statements: list | tuple | set | object,
        inherited_calls: set[str],
        context_label: str,
    ) -> None:
        names_in_block: set[str] = set()
        available_calls = set(inherited_calls)

        for statement in statements:
            if isinstance(statement, WdlBoundDeclaration):
                if statement.name is not None and statement.name in names_in_block:
                    self._add_error(
                        f"Duplicate declaration in {context_label}: '{statement.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif statement.name is not None:
                    names_in_block.add(statement.name)
            elif isinstance(statement, WdlCall):
                self._validate_call_structure(
                    statement, names_in_block, available_calls
                )
            elif isinstance(statement, WdlScatter):
                if statement.name is not None and statement.name in names_in_block:
                    self._add_error(
                        f"Duplicate declaration in {context_label}: '{statement.name}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                elif statement.name is not None:
                    names_in_block.add(statement.name)
                self._validate_nested_statements(
                    statement.statements(), available_calls, "scatter"
                )
            elif isinstance(statement, WdlConditional):
                self._validate_conditional_structure(
                    statement, available_calls, "conditional"
                )

    def _validate_conditional_structure(
        self, conditional: WdlConditional, available_calls: set[str], context_label: str
    ) -> None:
        self._validate_nested_statements(
            conditional.thenStatements(), available_calls, f"{context_label} then"
        )
        for else_if in conditional.elseIfs():
            self._validate_nested_statements(
                else_if.thenStatements(), available_calls, f"{context_label} else-if"
            )
        self._validate_nested_statements(
            conditional.elseStatements(), available_calls, f"{context_label} else"
        )

    def _validate_call_structure(
        self, call: WdlCall, names_in_block: set[str], available_calls: set[str]
    ) -> None:
        target = call.targetPath()[-1] if call.targetPath() else None
        call_name = call.alias or target

        if call_name is not None and call_name in names_in_block:
            self._add_error(
                f"Duplicate call name in workflow: '{call_name}'",
                code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            )
        elif call_name is not None:
            names_in_block.add(call_name)

        seen_call_inputs: set[str] = set()
        for call_input in call.inputs():
            key = call_input.getKey()
            if key is None:
                continue
            if key in seen_call_inputs:
                self._add_error(
                    f"Duplicate call input key '{key}' in call '{call_name or '<unnamed>'}'",
                    code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                )
            else:
                seen_call_inputs.add(key)

        for dep in call.afterDependencies():
            if dep not in available_calls:
                self._add_error(
                    f"Call '{call_name or '<unnamed>'}' has unknown or forward after dependency '{dep}'",
                    code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                )

        if call_name is not None:
            available_calls.add(call_name)
