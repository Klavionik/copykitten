class CopykittenError(Exception):
    """
    Raised if anything went wrong during any clipboard operation.
    arboard errors are mapped to this exception as well as mutex panics
    and clipboard initialization errors in the underlying Rust library.

    More on arboard errors: https://docs.rs/arboard/latest/arboard/enum.Error.html
    """
    pass


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
    :raises CopykittenError: Raised if fetching clipboard content failed.
    """
    ...


def clear() -> None:
    """
    Clears the clipboard (basically, sets it to an empty string).

    :raises CopykittenError: Raised if the clear operation failed.
    """
    ...
