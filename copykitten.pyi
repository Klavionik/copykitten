class CopykittenError(Exception):
    pass


def copy(content: str) -> None:
    """
    Copies passed content into the clipboard.

    :param content: Text to copy.
    :raise: CopykittenError
    """
    ...

def paste() -> str:
    """
    Returns the current content of the clipboard.

    :return: Clipboard content.
    :raise: CopykittenError
    """
    ...


def clear() -> None:
    """
    Clears the clipboard.

    :raise: CopykittenError
    """
    ...
