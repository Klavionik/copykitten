import os
import subprocess
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
@pytest.mark.skipif(sys.platform == "linux", reason="Linux needs a dedicated test case")
def test_clear(read_clipboard: ReadClipboard, write_clipboard: WriteClipboard):
    write_clipboard("text")
    sleep(SLEEP_TIME)
    copykitten.clear()
    sleep(SLEEP_TIME)

    actual = read_clipboard()

    assert actual == ""


@pytest.mark.repeat(ITERATIONS)
@pytest.mark.skipif(sys.platform != "linux", reason="Different behavior on Linux")
def test_clear_linux(
    capfd: pytest.CaptureFixture[str],
    read_clipboard: ReadClipboard,
    write_clipboard: WriteClipboard,
):
    write_clipboard("text")
    sleep(SLEEP_TIME)
    copykitten.clear()
    sleep(SLEEP_TIME)

    with pytest.raises(subprocess.CalledProcessError) as exc, capfd.disabled():
        read_clipboard()

        assert exc.value.returncode == 1
        assert exc.value.stderr == "Error: target STRING not available"


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


def test_paste_image(test_image: Image.Image, write_clipboard_image: WriteClipboardImage):
    write_clipboard_image(test_image)
    sleep(SLEEP_TIME)

    pasted_image, width, height = copykitten.paste_image()

    assert test_image.tobytes() == pasted_image
    assert width == test_image.width
    assert height == test_image.height


@pytest.mark.skipif(sys.platform != "linux", reason="Detach is supported only on Linux")
def test_copy_no_detach(capfd: pytest.CaptureFixture[str], read_clipboard: ReadClipboard):
    subprocess.check_call(
        ["python", "-c", "import copykitten; copykitten.copy('text', detach=False)"]
    )

    # The clipboard content becomes unavailable due to the responsible process exiting.
    # In this case xclip returns an error. Also, disable pytest's stderr capturing
    # to properly check the exception.
    with pytest.raises(subprocess.CalledProcessError) as exc, capfd.disabled():
        read_clipboard()

        assert exc.value.returncode == 1
        assert exc.value.stderr == "Error: target STRING not available"


@pytest.mark.skipif(sys.platform != "linux", reason="Detach is supported only on Linux")
def test_copy_detach(read_clipboard: ReadClipboard):
    subprocess.check_call(
        ["python", "-c", "import copykitten; copykitten.copy('text', detach=True)"]
    )

    actual = read_clipboard()

    assert actual == "text"


@pytest.mark.skipif(sys.platform != "linux", reason="Detach is supported only on Linux")
def test_copy_detach_twice(read_clipboard: ReadClipboard):
    code = """\
import copykitten

copykitten.copy('text', detach=True)
copykitten.copy('another text', detach=True)
    """
    subprocess.check_call(["python", "-c", code])

    actual = read_clipboard()

    assert actual == "another text"


@pytest.mark.skipif(sys.platform != "linux", reason="Detach is supported only on Linux")
def test_copy_image_detach(test_image: Image.Image, read_clipboard_image: ReadClipboardImage):
    code = f"""\
import copykitten
from PIL import Image

image = {test_image.tobytes()}
copykitten.copy_image(image, {test_image.width}, {test_image.height}, detach=True)
    """
    subprocess.check_call(["python", "-c", code])

    pasted_image = read_clipboard_image()

    assert pasted_image.tobytes() == test_image.tobytes()


@pytest.mark.skipif(
    sys.platform == "linux", reason="Check that detach doesn't break things on Win/Mac"
)
def test_copy_detach_not_linux(read_clipboard: ReadClipboard):
    copykitten.copy("text", detach=True)

    actual = read_clipboard()

    assert actual == "text"
