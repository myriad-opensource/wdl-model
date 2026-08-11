from __future__ import annotations

import pytest
import httpx

from wdl_model.model.errors import WdlImportException
from wdl_model.model.resolvers import WdlImportResolverHttpx


def test_resolves_http_import_with_injected_httpx_client() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        assert request.url == httpx.URL("http://example.com/workflow.wdl")
        return httpx.Response(200, text="version 1.3\n")

    client = httpx.Client(transport=httpx.MockTransport(handler))
    resolver = WdlImportResolverHttpx(http_client=client)

    assert (
        resolver.resolve_import(None, "http://example.com/workflow.wdl")
        == "version 1.3\n"
    )


def test_resolves_https_import_with_injected_httpx_client() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        assert request.url == httpx.URL("https://example.com/workflow.wdl")
        return httpx.Response(200, text="version 1.3\n")

    client = httpx.Client(transport=httpx.MockTransport(handler))
    resolver = WdlImportResolverHttpx(http_client=client)

    assert (
        resolver.resolve_import(None, "https://example.com/workflow.wdl")
        == "version 1.3\n"
    )


def test_raises_import_exception_on_http_error_status() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(404, text="not found")

    client = httpx.Client(transport=httpx.MockTransport(handler))
    resolver = WdlImportResolverHttpx(http_client=client)

    with pytest.raises(WdlImportException):
        resolver.resolve_import(None, "http://example.com/missing.wdl")


def test_raises_import_exception_for_unsupported_protocol() -> None:
    resolver = WdlImportResolverHttpx()

    with pytest.raises(WdlImportException):
        resolver.resolve_import("file:///tmp/root.wdl", "git://repo/workflow.wdl")
