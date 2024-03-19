# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..types.classes.cls_blah import Blah
from ..types.classes.cls_classoptionaloutput2 import ClassOptionalOutput2
from ..types.partial.classes.cls_blah import PartialBlah
from ..types.partial.classes.cls_classoptionaloutput2 import PartialClassOptionalOutput2
from baml_core.stream import AsyncStream
from baml_lib._impl.functions import BaseBAMLFunction
from typing import AsyncIterator, Callable, Optional, Protocol, runtime_checkable


IFnClassOptionalOutput2Output = Optional[ClassOptionalOutput2]

@runtime_checkable
class IFnClassOptionalOutput2(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: str

    Returns:
        Optional[ClassOptionalOutput2]
    """

    async def __call__(self, arg: str, /) -> Optional[ClassOptionalOutput2]:
        ...

   

@runtime_checkable
class IFnClassOptionalOutput2Stream(Protocol):
    """
    This is the interface for a stream function.

    Args:
        arg: str

    Returns:
        AsyncStream[Optional[ClassOptionalOutput2], PartialClassOptionalOutput2]
    """

    def __call__(self, arg: str, /) -> AsyncStream[Optional[ClassOptionalOutput2], PartialClassOptionalOutput2]:
        ...
class IBAMLFnClassOptionalOutput2(BaseBAMLFunction[Optional[ClassOptionalOutput2], PartialClassOptionalOutput2]):
    def __init__(self) -> None:
        super().__init__(
            "FnClassOptionalOutput2",
            IFnClassOptionalOutput2,
            ["v1"],
        )

    async def __call__(self, *args, **kwargs) -> Optional[ClassOptionalOutput2]:
        return await self.get_impl("v1").run(*args, **kwargs)
    
    def stream(self, *args, **kwargs) -> AsyncStream[Optional[ClassOptionalOutput2], PartialClassOptionalOutput2]:
        res = self.get_impl("v1").stream(*args, **kwargs)
        return res

BAMLFnClassOptionalOutput2 = IBAMLFnClassOptionalOutput2()

__all__ = [ "BAMLFnClassOptionalOutput2" ]