import pytest
from PIL import Image

from tests.clipboard import (
    Clipboard,
    ReadClipboard,
    ReadClipboardImage,
    WriteClipboard,
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
