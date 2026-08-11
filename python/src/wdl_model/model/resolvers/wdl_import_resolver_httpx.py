"""httpx-backed WDL import resolver for HTTP(S) sources."""

from __future__ import annotations

from typing import Any

import httpx

from wdl_model.model.errors import WdlImportException

from .wdl_import_resolver_base import WdlImportResolverBase


class WdlImportResolverHttpx(WdlImportResolverBase):
    """Resolve HTTP(S) imports through httpx while reusing base path/file resolution logic."""

    def __init__(
        self,
        http_client: httpx.Client | None = None,
        *,
        verify: bool | str | Any = True,
        connect_timeout_seconds: float = 10.0,
        read_timeout_seconds: float = 30.0,
    ) -> None:
        self._http_client = http_client
        self._owns_http_client = http_client is None
        if self._http_client is None:
            timeout = httpx.Timeout(
                read_timeout_seconds, connect=connect_timeout_seconds
            )
            self._http_client = httpx.Client(
                verify=verify, timeout=timeout, follow_redirects=True
            )

    def close(self) -> None:
        """Close the internally-owned HTTP client, if any."""
        if self._owns_http_client and self._http_client is not None:
            self._http_client.close()

    def load_http(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        return self._load_from_http(import_uri, original_import_location)

    def load_https(
        self,
        current_document_location: str | None,
        import_uri: str,
        original_import_location: str,
    ) -> str:
        return self._load_from_http(import_uri, original_import_location)

    def _load_from_http(self, import_uri: str, original_import_location: str) -> str:
        assert self._http_client is not None
        try:
            response = self._http_client.get(import_uri)
            response.raise_for_status()
            return response.text
        except httpx.HTTPError as exc:
            raise WdlImportException(
                "Unable to load HTTP import", original_import_location, exc
            ) from exc
