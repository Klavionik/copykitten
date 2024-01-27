import subprocess
import sys

try:
    clipboard_read_cmd, clipboard_write_cmd = dict(
        win32=(("powershell.exe", "Get-Clipboard"), ("powershell.exe", "Set-Clipboard")),
        linux=(("xsel", "-b"), ("xsel", "-b")),
        darwin=("pbpaste", "pbcopy"),
    )[sys.platform]
except KeyError:
    print("Cannot run tests, unknown platform.", file=sys.stderr)
    raise SystemExit(1)


def read_clipboard() -> str:
    return subprocess.check_output(clipboard_read_cmd).decode().strip()


def write_clipboard(content: str) -> None:
    # For some reason, Set-Clipboard won't read input
    # piped from a Python process. Another cmdlet `clip`
    # does read from a pipe, but adds 4 unexpected null bytes
    # to the content.
    # I chose to special-case this call, rather than stripping
    # null bytes in tests.
    if sys.platform == "win32":
        subprocess.check_call((*clipboard_write_cmd, content))
    else:
        subprocess.check_output(clipboard_write_cmd, input=content, text=True)
