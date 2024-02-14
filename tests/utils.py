import pathlib
import subprocess
import sys
import tempfile

IMAGE_DIR = pathlib.Path().resolve().parent / "fixtures"
JPEG_IMAGE = IMAGE_DIR / "kitten.jpeg"
PNG_IMAGE = IMAGE_DIR / "kitten.png"

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
        darwin=(
            ("osascript", "-e", "get the clipboard as «class PNGf»"),
            (
                "osascript",
                "-e",
                "on run args",
                "-e",
                "set the clipboard to (read POSIX file (first item of args) as JPEG picture)",
                "-e",
                "end",
            ),
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
    data = subprocess.check_output(clipboard_read_image_cmd)

    if sys.platform == "darwin":
        # On macOS `data` looks like this: '«data PNGf<hex-string>»\n'.
        # So it has to be stripped and converted from hex.
        hex_string = data[11:-3].decode()
        return bytes.fromhex(hex_string)

    return data


def write_clipboard_image(content: bytes) -> None:
    if sys.platform == "darwin":
        # AppleScript can't read from stdin, but can read from a file.
        with tempfile.NamedTemporaryFile(suffix=".png") as tmp:
            tmp.write(content)
            tmp.seek(0)
            cmd = (*clipboard_write_image_cmd, tmp.name)
            subprocess.run(cmd, check=True)
    else:
        subprocess.run(clipboard_write_image_cmd, input=content, check=True)
