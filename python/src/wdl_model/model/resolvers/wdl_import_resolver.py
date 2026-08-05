"""Backward-compatible alias resolver using the httpx transport implementation."""

from .wdl_import_resolver_httpx import WdlImportResolverHttpx


class WdlImportResolver(WdlImportResolverHttpx):
    """Compatibility wrapper preserving the existing resolver class name."""
