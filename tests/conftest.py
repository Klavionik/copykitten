import sys
from io import BytesIO
from pathlib import Path

import pytest
from PIL import Image

from tests.clipboard import (
    ReadClipboard,
    ReadClipboardImage,
    WriteClipboard,
    WriteClipboardImage,
    read_image_linux,
    read_linux,
    write_image_linux,
    write_linux, read_win, write_win, read_image_win, write_image_win, read_macos, read_image_macos, write_macos, write_image_macos
)

IMAGE_DIR = Path(__file__).resolve().parent / "fixtures"
JPG_IMAGE = IMAGE_DIR / "kitten.jpeg"
PNG_IMAGE = IMAGE_DIR / "kitten.png"


DEFAULT_ITERATIONS = 100
DEFAULT_SLEEP_TIME = 0.1


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
