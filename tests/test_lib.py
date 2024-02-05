import os
from io import BytesIO
from time import sleep
from unittest import TestCase

from PIL import Image

import copykitten
from tests.utils import (
    IMAGE_SIZE,
    generate_image,
    read_clipboard,
    read_clipboard_image,
    write_clipboard,
    write_clipboard_image,
)

DEFAULT_ITERATIONS = 100
DEFAULT_SLEEP_TIME = 0.1

try:
    ITERATIONS = int(os.getenv("COPYKITTEN_TEST_ITERATIONS", DEFAULT_ITERATIONS))
except Exception:
    ITERATIONS = DEFAULT_ITERATIONS

try:
    SLEEP_TIME = float(os.getenv("COPYKITTEN_TEST_SLEEP_TIME", DEFAULT_SLEEP_TIME))
except Exception:
    SLEEP_TIME = DEFAULT_SLEEP_TIME


class TestClipboard(TestCase):
    # Why sleep in tests?
    # It may take a bit for the OS to finish the clipboard operation,
    # so there have to be a short delay before asserting the result.
    # Otherwise, tests may randomly fail.
    def test_copy(self):
        for i in range(ITERATIONS):
            text = f"text{i}"

            with self.subTest(text=text):
                copykitten.copy(text)
                sleep(SLEEP_TIME)
                actual = read_clipboard()

                self.assertEqual(actual, text)

    def test_paste(self):
        for i in range(ITERATIONS):
            text = f"text{i}"

            with self.subTest(text=text):
                write_clipboard(text)
                sleep(SLEEP_TIME)
                actual = copykitten.paste()

                self.assertEqual(actual, text)

    def test_clear(self):
        for i in range(ITERATIONS):
            text = f"text{i}"

            with self.subTest(text=text):
                write_clipboard(text)
                sleep(SLEEP_TIME)
                copykitten.clear()
                sleep(SLEEP_TIME)
                actual = read_clipboard()

                self.assertEqual(actual, "")

    def test_copy_image(self):
        for _ in range(ITERATIONS):
            image = generate_image()
            image_bytes = image.tobytes()
            copykitten.copy_image(image_bytes, IMAGE_SIZE, IMAGE_SIZE)
            sleep(SLEEP_TIME)
            buffer = BytesIO(read_clipboard_image())
            actual = Image.open(buffer, formats=["png"])
            actual_bytes = actual.tobytes()

            self.assertEqual(image_bytes, actual_bytes)

    def test_paste_image(self):
        for _ in range(ITERATIONS):
            image = generate_image()
            buffer = BytesIO()
            image.save(buffer, format="png")
            write_clipboard_image(buffer.getvalue())
            sleep(SLEEP_TIME)
            img, width, height = copykitten.paste_image()

            self.assertEqual(image.tobytes(), img)
            self.assertEqual(width, IMAGE_SIZE)
            self.assertEqual(height, IMAGE_SIZE)
