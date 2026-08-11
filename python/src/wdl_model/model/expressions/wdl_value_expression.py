from __future__ import annotations

from typing import Generic, TypeVar

from .wdl_expression import WdlExpression

T = TypeVar("T")


class WdlValueExpression(WdlExpression, Generic[T]):
    pass
