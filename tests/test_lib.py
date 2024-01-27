import os
from time import sleep
from unittest import TestCase

import copykitten
from tests.utils import read_clipboard, write_clipboard

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
