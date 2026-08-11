from __future__ import annotations

from pathlib import Path

from wdl_model.model import WdlV1Loader
from wdl_model.model.processors import WdlAppendingProcessor

FIXTURES_ROOT = Path("wdl_tests") / "appending_processor"


def test_appending_processor_renders_recursive_workflow_statements() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "recursive_workflow_statements.wdl"
    )
    processor = WdlAppendingProcessor()
    processor.processDocument(document)

    rendered = processor.getValue()

    assert "if (x == 1) {" in rendered
    assert "scatter (n in [1, 2]) {" in rendered
    assert "Int y = n" in rendered
    assert "{ ... }" not in rendered


def test_appending_processor_renders_metadata_content() -> None:
    document = WdlV1Loader.load_from_file(FIXTURES_ROOT / "metadata_content.wdl")
    processor = WdlAppendingProcessor()
    processor.processDocument(document)

    rendered = processor.getValue()

    assert "meta {" in rendered
    assert "author:" in rendered
    assert "parameter_meta {" in rendered
    assert "x:" in rendered
    assert "meta { ... }" not in rendered
    assert "parameter_meta { ... }" not in rendered
