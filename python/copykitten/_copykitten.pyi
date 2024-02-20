class CopykittenError(Exception):
    """
    Raised if anything went wrong during any clipboard operation.
    arboard errors are mapped to this exception as well as mutex panics
    and clipboard initialization errors in the underlying Rust library.

    More on arboard errors: https://docs.rs/arboard/latest/arboard/enum.Error.html
    """
    ...


def copy(content: str) -> None:
    """
    Copies passed text content into the clipboard.
    Content must be a valid UTF-8 string.

    :param content: Text to copy.
    :raises CopykittenError: Raised if copying failed.
    :raises TypeError: Raised if the content is not a string.
    """
    ...


def paste() -> str:
    """
    Returns the current text content of the clipboard
    as a UTF-8 string.

    :return: Clipboard content.
    :raises CopykittenError: Raised if fetching clipboard content failed
      or the clipboard is empty (on Windows and macOS).
    """
    ...


def clear() -> None:
    """
    Clears the clipboard. Calling `copykitten.paste()` after this
    may raise an error on Windows and macOS due to the empty clipboard.

    :raises CopykittenError: Raised if the clear operation failed.
    """
    ...


def copy_image(content: bytes, width: int, height: int) -> None:
    """
    Copies given image data to the clipboard.

    :param content: Raw RGBA image data.
    :param width: Image width.
    :param height: Image height.
    :raises CopykittenError: Raised if the image cannot be copied into the clipboard.
    :raises TypeError: Raised if `content` is not bytes or `width`/`height` is not an integer.
    """
    ...


def paste_image() -> tuple[bytes, int, int]:
    """
    Returns image data from the clipboard.

    :raises CopykittenError: Raised if there's no image in the clipboard or the image is of incorrect format.
    :return: A 3-tuple of raw RGBA pixels, width, and height.
    """
    ...
