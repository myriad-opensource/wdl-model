from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model.errors import WdlImportException
from wdl_model.model.resolvers import WdlImportResolverFilesystem

FIXTURES_ROOT = Path("wdl_tests") / "resolver_filesystem"


def test_resolves_relative_path_against_current_document_location() -> None:
    root = FIXTURES_ROOT / "root.wdl"

    resolver = WdlImportResolverFilesystem()
    text = resolver.resolve_import(root.resolve().as_uri(), "sub/imported.wdl")

    assert text == "version 1.3\n"


def test_resolves_file_scheme_import() -> None:
    imported = FIXTURES_ROOT / "sub" / "imported.wdl"

    resolver = WdlImportResolverFilesystem()
    text = resolver.resolve_import(None, imported.resolve().as_uri())

    assert text == "version 1.3\n"


def test_rejects_http_imports() -> None:
    resolver = WdlImportResolverFilesystem()
    with pytest.raises(WdlImportException):
        resolver.resolve_import("file:///tmp/root.wdl", "http://example.com/a.wdl")


def test_rejects_https_imports() -> None:
    resolver = WdlImportResolverFilesystem()
    with pytest.raises(WdlImportException):
        resolver.resolve_import("file:///tmp/root.wdl", "https://example.com/a.wdl")
