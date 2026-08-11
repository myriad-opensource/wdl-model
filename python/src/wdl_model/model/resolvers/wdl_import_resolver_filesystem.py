"""Filesystem-only WDL import resolver."""

from __future__ import annotations

from wdl_model.model.errors import WdlImportException

from .wdl_import_resolver_base import WdlImportResolverBase


class WdlImportResolverFilesystem(WdlImportResolverBase):
    """Reject network protocols and only allow file/bare-path imports."""

    def load_http(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        raise WdlImportException(
            "Filesystem resolver does not support http imports",
            original_import_location,
        )

    def load_https(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        raise WdlImportException(
            "Filesystem resolver does not support https imports",
            original_import_location,
        )
