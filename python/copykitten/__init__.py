from typing import Tuple

from ._copykitten import CopykittenError
from ._copykitten import clear as _clear
from ._copykitten import copy as _copy
from ._copykitten import copy_image as _copy_image
from ._copykitten import copy_image_wait as _copy_image_wait
from ._copykitten import copy_wait as _copy_wait
from ._copykitten import paste as _paste
from ._copykitten import paste_image as _paste_image

__all__ = ["copy", "paste", "clear", "copy_image", "paste_image", "CopykittenError"]

CopykittenError.__doc__ = """\
Raised if anything went wrong during any clipboard operation.
arboard errors are mapped to this exception as well as mutex panics
and clipboard initialization errors in the underlying Rust library.

More on arboard errors: https://docs.rs/arboard/latest/arboard/enum.Error.html
"""


def copy(content: str, *, detach: bool = False) -> None:
    """
    Copies passed text content into the clipboard.
    Content must be a valid UTF-8 string.

    :param content: Text to copy.
    :param detach: Spawn a background process to keep the content available after exit.
    :raises CopykittenError: Raised if copying failed.
    :raises TypeError: Raised if the content is not a string.
    """
    if detach:
        _copy_wait(content)
    else:
        _copy(content)


def paste() -> str:
    """
    Returns the current text content of the clipboard
    as a UTF-8 string.

    :return: Clipboard content.
    :raises CopykittenError: Raised if fetching clipboard content failed
      or the clipboard is empty (on Windows and macOS).
    """
    return _paste()


def clear() -> None:
    """
    Clears the clipboard. Calling `copykitten.paste()` after this
    may raise an error on Windows and macOS due to the empty clipboard.

    :raises CopykittenError: Raised if the clear operation failed.
    """
    _clear()


def copy_image(content: bytes, width: int, height: int, *, detach: bool = False) -> None:
    """
    Copies given image data to the clipboard.

    :param content: Raw RGBA image data.
    :param width: Image width.
    :param height: Image height.
    :param detach: Spawn a background process to keep the image available after exit.
    :raises CopykittenError: Raised if the image cannot be copied into the clipboard.
    :raises TypeError: Raised if `content` is not bytes or `width`/`height` is not an integer.
    """
    if detach:
        _copy_image_wait(content, width, height)
    else:
        _copy_image(content, width, height)


def paste_image() -> Tuple[bytes, int, int]:
    """
    Returns image data from the clipboard.

    :raises CopykittenError: Raised if there's no image in the clipboard or
    the image is of incorrect format.
    :return: A 3-tuple of raw RGBA pixels, width, and height.
    """
    return _paste_image()
