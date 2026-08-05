"""Transport-agnostic base resolver for WDL imports."""

from __future__ import annotations

from pathlib import Path
from urllib.parse import urljoin, urlparse

from wdl_model.model.errors import WdlImportException


class WdlImportResolverBase:
    """Resolve import references and dispatch protocol-specific loading."""

    def resolve_import(
        self, current_document_location: str | None, import_location: str
    ) -> str:
        """Resolve an import reference relative to the current document and load its content."""
        resolved = self.resolve_import_location(
            current_document_location, import_location
        )
        return self.dispatch_import(
            current_document_location, resolved, import_location
        )

    def resolve_import_location(
        self, current_document_location: str | None, import_location: str
    ) -> str:
        """Resolve an import reference into a stable location identifier."""
        return self.resolve_import_uri(current_document_location, import_location)

    def dispatch_import(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        """Dispatch import loading based on URI scheme."""
        parsed = urlparse(import_uri)
        scheme = parsed.scheme.lower()
        if scheme == "":
            return self.load_bare_path(
                current_document_location, import_uri, original_import_location
            )
        if scheme == "http":
            return self.load_http(
                current_document_location, import_uri, original_import_location
            )
        if scheme == "https":
            return self.load_https(
                current_document_location, import_uri, original_import_location
            )
        if scheme == "file":
            return self.load_file(
                current_document_location, import_uri, original_import_location
            )
        raise WdlImportException(
            f"Unsupported import URI protocol: {parsed.scheme}",
            original_import_location,
        )

    def load_http(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        """Load an ``http://`` import and return source text.

        Subclasses should raise ``WdlImportException`` with enough context for
        users to understand which import failed.
        """
        raise NotImplementedError

    def load_https(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        """Load an ``https://`` import and return source text.

        Subclasses should validate TLS according to project security policy and
        raise ``WdlImportException`` on fetch errors.
        """
        raise NotImplementedError

    def load_file(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        parsed = urlparse(import_uri)
        path = Path(parsed.path)
        try:
            return path.read_text(encoding="utf-8")
        except OSError as exc:
            raise WdlImportException(
                "Unable to read file import", original_import_location, exc
            ) from exc

    def load_bare_path(
        self,
        current_document_location: str | None,
        resolved_path: str,
        original_import_location: str,
    ) -> str:
        if not resolved_path.strip():
            raise WdlImportException(
                "Invalid filesystem import path", original_import_location
            )
        try:
            return Path(resolved_path).read_text(encoding="utf-8")
        except OSError as exc:
            raise WdlImportException(
                "Unable to read filesystem import", original_import_location, exc
            ) from exc

    def resolve_import_uri(
        self, current_document_location: str | None, import_location: str
    ) -> str:
        parsed_import = urlparse(import_location)
        if parsed_import.scheme:
            return import_location

        if current_document_location is None:
            if import_location.startswith("/"):
                return Path(import_location).as_uri()
            return import_location

        base = urlparse(current_document_location)
        if base.scheme in {"http", "https"}:
            return urljoin(current_document_location, import_location)
        if base.scheme == "file":
            base_path = Path(base.path)
            if base_path.name:
                base_path = base_path.parent
            return str((base_path / import_location).resolve())

        current_path = Path(current_document_location)
        parent = current_path.parent if current_path.name else current_path
        return str((parent / import_location).resolve())
