from __future__ import annotations

from pathlib import Path

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.resolvers import WdlImportResolverFilesystem

FIXTURES_ROOT = Path("wdl_tests") / "loader_imports"


def test_recursively_loads_imported_documents_into_map() -> None:
    root = FIXTURES_ROOT / "recursive" / "root.wdl"
    child = FIXTURES_ROOT / "recursive" / "child.wdl"
    grandchild = FIXTURES_ROOT / "recursive" / "grandchild.wdl"

    root_doc = WdlV1Loader.load_from_file(root)

    assert len(root_doc.importedDocuments()) == 1
    child_doc = next(iter(root_doc.importedDocuments().values()))
    assert child_doc is not None
    assert (
        Path(child_doc.getSourceLocation().replace("file://", "")).resolve()
        == child.resolve()
    )

    assert len(child_doc.importedDocuments()) == 1
    grandchild_doc = next(iter(child_doc.importedDocuments().values()))
    assert grandchild_doc is not None
    assert (
        Path(grandchild_doc.getSourceLocation().replace("file://", "")).resolve()
        == grandchild.resolve()
    )

    assert root_doc.importStatements()[0].sourceText is not None
    assert child_doc.importStatements()[0].sourceText is not None


def test_loads_from_source_code_with_source_location_resolver_then_validator() -> None:
    root = FIXTURES_ROOT / "string_input" / "root.wdl"
    root_source = root.read_text(encoding="utf-8")

    root_doc = WdlV1Loader.load_from_string(
        root_source,
        source_location=root.resolve().as_uri(),
        import_resolver=WdlImportResolverFilesystem(),
        validator=WdlSemanticValidator(),
    )

    assert len(root_doc.importStatements()) == 1
    assert len(root_doc.importedDocuments()) == 1
    assert next(iter(root_doc.importedDocuments().values())) is not None
