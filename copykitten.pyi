class CopykittenError(Exception):
    """
    Raised if anything went wrong during any clipboard operation.
    Possible reasons: invalid content, clipboard is unavailable, etc.
    """
    pass


def copy(content: str) -> None:
    """
    Copies passed text content into the clipboard.
    Content must be a valid UTF-8 string.

    :param content: Text to copy.
    :raises CopykittenError
    """
    ...


def paste() -> str:
    """
    Returns the current text content of the clipboard
    as a UTF-8 string.

    :return: Clipboard content.
    :raises CopykittenError
    """
    ...


def clear() -> None:
    """
    Clears the clipboard (basically, sets it to an empty string).

    :raises CopykittenError
    """
    ...
