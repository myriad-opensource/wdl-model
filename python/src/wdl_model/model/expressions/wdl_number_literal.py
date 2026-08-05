from __future__ import annotations

from typing import Generic, TypeVar

from .wdl_literal import WdlLiteral

T = TypeVar("T", int, float)


class WdlNumberLiteral(WdlLiteral[T], Generic[T]):
    pass
