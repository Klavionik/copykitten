import subprocess

from copykitten import copy, paste, clear
from unittest import TestCase
from time import sleep


def read_clipboard() -> str:
    return subprocess.check_output(["xsel", '-b']).decode()


class TestCopy(TestCase):
    def test_copy(self):
        for i in range(100):
            text = "text" + str(i)

            with self.subTest(text=text):
                copy(text)
                sleep(0.3)
                actual = read_clipboard()

                self.assertEqual(actual, text)
