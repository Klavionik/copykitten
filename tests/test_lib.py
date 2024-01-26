import subprocess
from time import sleep
from unittest import TestCase

import copykitten


def read_clipboard() -> str:
    return subprocess.check_output(["xsel", "-b"]).decode()


class TestCopy(TestCase):
    def test_copy(self):
        for i in range(300):
            text = f"text{i}"

            with self.subTest(text=text):
                copykitten.copy(text)
                sleep(0.1)
                actual = read_clipboard()

                self.assertEqual(actual, text)
