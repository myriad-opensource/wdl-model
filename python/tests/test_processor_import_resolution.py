from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from wdl_model.model import WdlV1Loader
from wdl_model.model.processors import WdlProcessorBase

FIXTURES_ROOT = Path("wdl_tests") / "processor_imports"


@dataclass
class ProbeProcessor(WdlProcessorBase):
    def tasks(self, doc, call_target: str):
        return self.resolveImportedTasks(doc, call_target)

    def workflows(self, doc, call_target: str):
        return self.resolveImportedWorkflows(doc, call_target)

    def structs(self, doc, type_name: str):
        return self.resolveImportedStructs(doc, type_name)

    def enums(self, doc, type_name: str):
        return self.resolveImportedEnums(doc, type_name)

    def imported_doc(self, doc, imp):
        return self.resolveImportedDocument(doc, imp)


def test_resolves_imported_call_targets_and_types_across_import_forms() -> None:
    root = FIXTURES_ROOT / "root.wdl"

    root_doc = WdlV1Loader.load_from_file(root)
    processor = ProbeProcessor()

    lib_tasks = processor.tasks(root_doc, "lib.lib_task")
    assert len(lib_tasks) == 1
    assert lib_tasks[0].import_namespace == "lib"
    assert lib_tasks[0].imported_name == "lib_task"

    star_tasks = processor.tasks(root_doc, "star_task")
    assert len(star_tasks) == 1
    assert star_tasks[0].local_name == "star_task"

    member_tasks = processor.tasks(root_doc, "local_task")
    assert len(member_tasks) == 1
    assert member_tasks[0].imported_name == "selected_task"

    workflows = processor.workflows(root_doc, "local_flow")
    assert len(workflows) == 1
    assert workflows[0].imported_name == "selected_flow"

    aliased_structs = processor.structs(root_doc, "Patient")
    assert len(aliased_structs) == 1
    assert aliased_structs[0].imported_name == "Person"

    star_structs = processor.structs(root_doc, "StarStruct")
    assert len(star_structs) == 1

    member_structs = processor.structs(root_doc, "LocalStruct")
    assert len(member_structs) == 1
    assert member_structs[0].imported_name == "SelectedStruct"

    aliased_enums = processor.enums(root_doc, "ImportStatus")
    assert len(aliased_enums) == 1
    assert aliased_enums[0].imported_name == "Status"

    member_enums = processor.enums(root_doc, "LocalEnum")
    assert len(member_enums) == 1
    assert member_enums[0].imported_name == "SelectedEnum"

    assert root_doc.importStatements()
    assert processor.imported_doc(root_doc, root_doc.importStatements()[0]) is not None
