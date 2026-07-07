from typing import Iterator

import pytest
from PIL import Image

from tests.clipboard import (
    Clipboard,
    ReadClipboard,
    ReadClipboardFileList,
    ReadClipboardImage,
    WriteClipboard,
    WriteClipboardFileList,
    WriteClipboardImage,
)


@pytest.fixture(scope="session")
def test_image() -> Image.Image:
    return Image.new(mode="RGBA", size=(10, 10), color="red")


@pytest.fixture(scope="session")
def clipboard() -> Clipboard:
    return Clipboard()


@pytest.fixture(scope="session")
def read_clipboard(clipboard) -> ReadClipboard:
    return clipboard.read


@pytest.fixture(scope="session")
def write_clipboard(clipboard) -> WriteClipboard:
    return clipboard.write


@pytest.fixture(scope="session")
def read_clipboard_image(clipboard) -> ReadClipboardImage:
    return clipboard.read_image


@pytest.fixture(scope="session")
def write_clipboard_image(clipboard) -> WriteClipboardImage:
    return clipboard.write_image


@pytest.fixture(scope="session")
def paste_file_list(clipboard) -> WriteClipboardFileList:
    return clipboard.write_file_list


@pytest.fixture(scope="session")
def read_clipboard_file_list(clipboard) -> ReadClipboardFileList:
    return clipboard.read_file_list


@pytest.fixture(autouse=True)
def clear_clipboard(clipboard) -> Iterator[None]:
    yield

    clipboard.write("")
