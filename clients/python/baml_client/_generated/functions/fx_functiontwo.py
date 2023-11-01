# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: __DATE__
# Generated by: vbv

from ..._impl.functions import BaseBAMLFunction
from ..types.classes.cls_type1 import Type1
from ..types.classes.cls_type2 import Type2
from typing import Protocol, runtime_checkable


@runtime_checkable
class IFunctionTwo(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: Type1

    Returns:
        str
    """

    async def __call__(self, arg: Type1, /) -> str:
        ...


class IBAMLFunctionTwo(BaseBAMLFunction[str]):
    def __init__(self) -> None:
        super().__init__(
            "FunctionTwo",
            IFunctionTwo,
            [],
        )

BAMLFunctionTwo = IBAMLFunctionTwo()

__all__ = [ "BAMLFunctionTwo" ]
