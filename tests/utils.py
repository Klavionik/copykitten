import subprocess
import sys

from PIL import Image

IMAGE_SIZE = 10

try:
    clipboard_read_cmd, clipboard_write_cmd = dict(
        win32=(("powershell.exe", "Get-Clipboard"), ("powershell.exe", "Set-Clipboard")),
        linux=(("xclip", "-sel", "clipboard", "-o"), ("xclip", "-sel", "clipboard", "-i")),
        darwin=("pbpaste", "pbcopy"),
    )[sys.platform]
except KeyError:
    print("Cannot run tests, unknown platform.", file=sys.stderr)
    raise SystemExit(1)

try:
    clipboard_read_image_cmd, clipboard_write_image_cmd = dict(
        linux=(
            ("xclip", "-sel", "clipboard", "-o", "-target", "image/png"),
            ("xclip", "-sel", "clipboard", "-i", "-target", "image/png"),
        ),
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
        subprocess.run(clipboard_write_cmd, input=content, text=True)


def read_clipboard_image() -> bytes:
    return subprocess.check_output(clipboard_read_image_cmd)


def write_clipboard_image(content: bytes) -> None:
    subprocess.run(clipboard_write_image_cmd, input=content)


def generate_image() -> Image:
    return Image.new(mode="RGBA", size=(IMAGE_SIZE, IMAGE_SIZE), color="red")
