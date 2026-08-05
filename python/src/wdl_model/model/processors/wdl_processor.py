"""Visitor-style processor contract for the full Python WDL object model."""

from __future__ import annotations

from abc import ABC, abstractmethod

from wdl_model.model.definitions import (
    WdlEnum,
    WdlStruct,
    WdlStructMember,
    WdlTask,
    WdlWorkflow,
)
from wdl_model.model.sections import (
    WdlCommand,
    WdlInput,
    WdlMetadata,
    WdlOutput,
    WdlParameterMetadata,
    WdlRequirements,
    WdlRuntime,
    WdlTaskHints,
    WdlWorkflowHints,
)
from wdl_model.model.statements import (
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
)

from ..wdl_document import WdlDocument
from ..wdl_version import WdlVersion


class WdlProcessor(ABC):
    """Abstract processor contract for documents, definitions, sections, and statements.

    Most callers should subclass ``WdlProcessorBase`` rather than implement this interface from
    scratch.
    """

    @abstractmethod
    def processDocument(self, node: WdlDocument) -> None: ...

    @abstractmethod
    def processVersion(self, ctx: WdlDocument, node: WdlVersion) -> None: ...

    @abstractmethod
    def processImportStandard(
        self, ctx: WdlDocument, node: WdlImportStandard
    ) -> None: ...

    @abstractmethod
    def processImportMembers(
        self, ctx: WdlDocument, node: WdlImportMembers
    ) -> None: ...

    @abstractmethod
    def processImportStar(self, ctx: WdlDocument, node: WdlImportStar) -> None: ...

    @abstractmethod
    def processEnum(self, ctx: WdlDocument, node: WdlEnum) -> None: ...

    @abstractmethod
    def processStruct(self, ctx: WdlDocument, node: WdlStruct) -> None: ...

    @abstractmethod
    def processStructMember(self, ctx: WdlStruct, node: WdlStructMember) -> None: ...

    @abstractmethod
    def processStructParameterMetadata(
        self, ctx: WdlStruct, node: WdlParameterMetadata
    ) -> None: ...

    @abstractmethod
    def processStructMetadata(self, ctx: WdlStruct, node: WdlMetadata) -> None: ...

    @abstractmethod
    def processTask(self, ctx: WdlDocument, node: WdlTask) -> None: ...

    @abstractmethod
    def processTaskDeclaration(
        self, ctx: WdlTask, node: WdlBoundDeclaration
    ) -> None: ...

    @abstractmethod
    def processTaskInput(self, ctx: WdlTask, node: WdlInput) -> None: ...

    @abstractmethod
    def processTaskOutput(self, ctx: WdlTask, node: WdlOutput) -> None: ...

    @abstractmethod
    def processTaskCommand(self, ctx: WdlTask, node: WdlCommand) -> None: ...

    @abstractmethod
    def processTaskParameterMetadata(
        self, ctx: WdlTask, node: WdlParameterMetadata
    ) -> None: ...

    @abstractmethod
    def processTaskMetadata(self, ctx: WdlTask, node: WdlMetadata) -> None: ...

    @abstractmethod
    def processTaskRequirements(self, ctx: WdlTask, node: WdlRequirements) -> None: ...

    @abstractmethod
    def processTaskRuntime(self, ctx: WdlTask, node: WdlRuntime) -> None: ...

    @abstractmethod
    def processTaskHints(self, ctx: WdlTask, node: WdlTaskHints) -> None: ...

    @abstractmethod
    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None: ...

    @abstractmethod
    def processWorkflowDeclaration(
        self, ctx: WdlWorkflow, node: WdlBoundDeclaration
    ) -> None: ...

    @abstractmethod
    def processWorkflowInput(self, ctx: WdlWorkflow, node: WdlInput) -> None: ...

    @abstractmethod
    def processWorkflowOutput(self, ctx: WdlWorkflow, node: WdlOutput) -> None: ...

    @abstractmethod
    def processWorkflowMetadata(self, ctx: WdlWorkflow, node: WdlMetadata) -> None: ...

    @abstractmethod
    def processWorkflowParameterMetadata(
        self, ctx: WdlWorkflow, node: WdlParameterMetadata
    ) -> None: ...

    @abstractmethod
    def processWorkflowCall(self, ctx: WdlWorkflow, node: WdlCall) -> None: ...

    @abstractmethod
    def processWorkflowConditional(
        self, ctx: WdlWorkflow, node: WdlConditional
    ) -> None: ...

    @abstractmethod
    def processWorkflowScatter(self, ctx: WdlWorkflow, node: WdlScatter) -> None: ...

    @abstractmethod
    def processWorkflowHints(
        self, ctx: WdlWorkflow, node: WdlWorkflowHints
    ) -> None: ...
