import subprocess
import sys

try:
    clipboard_read_cmd, clipboard_write_cmd = dict(
        win32=("Get-Clipboard", ("Set-Clipboard", "-Value")),
        linux=(("xsel", "-b"), ("xsel", "-b")),
        darwin=("pbpaste", "pbcopy"),
    )[sys.platform]
except KeyError:
    print("Cannot run tests, unknown platform.", file=sys.stderr)
    raise SystemExit(1)


def read_clipboard() -> str:
    return subprocess.check_output(clipboard_read_cmd).decode()


def write_clipboard(content: str) -> None:
    subprocess.check_output(clipboard_write_cmd, encoding="utf-8", input=content)
