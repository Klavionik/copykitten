import sys

import pytest
from PIL import Image

from tests.clipboard import (
    ReadClipboard,
    ReadClipboardImage,
    WriteClipboard,
    WriteClipboardImage,
    read_image_linux,
    read_image_macos,
    read_image_win,
    read_linux,
    read_macos,
    read_win,
    write_image_linux,
    write_image_macos,
    write_image_win,
    write_linux,
    write_macos,
    write_win,
)


@pytest.fixture(scope="session")
def test_image() -> Image.Image:
    return Image.new(mode="RGBA", size=(10, 10), color="red")


@pytest.fixture(scope="session")
def read_clipboard() -> ReadClipboard:
    if sys.platform == "linux":
        return read_linux

    if sys.platform == "win32":
        return read_win

    if sys.platform == "darwin":
        return read_macos

    raise RuntimeError("Cannot run tests, platform not supported.")


@pytest.fixture(scope="session")
def write_clipboard() -> WriteClipboard:
    if sys.platform == "linux":
        return write_linux

    if sys.platform == "win32":
        return write_win

    if sys.platform == "darwin":
        return write_macos

    raise RuntimeError("Cannot run tests, platform not supported.")


@pytest.fixture(scope="session")
def read_clipboard_image() -> ReadClipboardImage:
    if sys.platform == "linux":
        return read_image_linux

    if sys.platform == "win32":
        return read_image_win

    if sys.platform == "darwin":
        return read_image_macos

    raise RuntimeError("Cannot run tests, platform not supported.")


@pytest.fixture(scope="session")
def write_clipboard_image() -> WriteClipboardImage:
    if sys.platform == "linux":
        return write_image_linux

    if sys.platform == "win32":
        return write_image_win

    if sys.platform == "darwin":
        return write_image_macos

    raise RuntimeError("Cannot run tests, platform not supported.")
