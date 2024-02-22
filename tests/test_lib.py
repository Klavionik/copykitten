import os
import sys
from time import sleep

import pytest
from PIL import Image

import copykitten
from tests.clipboard import (
    ReadClipboard,
    ReadClipboardImage,
    WriteClipboard,
    WriteClipboardImage,
)

DEFAULT_ITERATIONS = 50
DEFAULT_SLEEP_TIME = 0.1


try:
    ITERATIONS = int(os.getenv("COPYKITTEN_TEST_ITERATIONS", DEFAULT_ITERATIONS))
except Exception:
    ITERATIONS = DEFAULT_ITERATIONS

# Why sleep in tests?
# It may take a bit for the OS to finish the clipboard operation,
# so there have to be a short delay before asserting the result.
# Otherwise, tests may randomly fail.
try:
    SLEEP_TIME = float(os.getenv("COPYKITTEN_TEST_SLEEP_TIME", DEFAULT_SLEEP_TIME))
except Exception:
    SLEEP_TIME = DEFAULT_SLEEP_TIME


@pytest.mark.repeat(ITERATIONS)
def test_copy_text(read_clipboard: ReadClipboard):
    copykitten.copy("text")
    sleep(SLEEP_TIME)

    actual = read_clipboard()

    assert actual == "text"


@pytest.mark.repeat(ITERATIONS)
def test_clear(read_clipboard: ReadClipboard, write_clipboard: WriteClipboard):
    write_clipboard("text")
    sleep(SLEEP_TIME)
    copykitten.clear()
    sleep(SLEEP_TIME)

    actual = read_clipboard()

    assert actual == ""


@pytest.mark.repeat(ITERATIONS)
def test_paste_text(write_clipboard: WriteClipboard):
    write_clipboard("text")
    sleep(SLEEP_TIME)

    actual = copykitten.paste()

    assert actual == "text"


def test_copy_image(test_image: Image.Image, read_clipboard_image: ReadClipboardImage):
    test_image_bytes = test_image.tobytes()
    copykitten.copy_image(test_image_bytes, test_image.width, test_image.height)
    sleep(SLEEP_TIME)

    pasted_image = read_clipboard_image()

    assert test_image_bytes == pasted_image.tobytes()


@pytest.mark.skipif(
    sys.platform == "win32", reason="No way to reliably assert result on Windows yet"
)
def test_paste_image(test_image: Image.Image, write_clipboard_image: WriteClipboardImage):
    write_clipboard_image(test_image)
    sleep(SLEEP_TIME)

    pasted_image, width, height = copykitten.paste_image()

    assert test_image.tobytes() == pasted_image
    assert width == test_image.width
    assert height == test_image.height
