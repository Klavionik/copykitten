import io
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Callable, Generic, List, TypeVar, cast

from PIL import Image

ReadClipboard = Callable[[], str]
WriteClipboard = Callable[[str], None]
ReadClipboardImage = Callable[[], Image.Image]
WriteClipboardImage = Callable[[Image.Image], None]
ReadClipboardFileList = Callable[[], List[Path]]
WriteClipboardFileList = Callable[[List[str]], None]

T = TypeVar("T")


class Resolver(Generic[T]):
    platform_mapping = {"win32": "win", "linux": "linux", "darwin": "macos"}

    def __set_name__(self, _, name):
        self.action = name

    def __get__(self, _, __) -> T:
        variables = globals()
        platform = self.platform_mapping[sys.platform]
        function_name = f"{self.action}_{platform}"

        try:
            return cast(T, variables[function_name])
        except KeyError:
            raise RuntimeError(f"Cannot find suitable clipboard for {sys.platform}.")


class Clipboard:
    read = Resolver[ReadClipboard]()
    write = Resolver[WriteClipboard]()
    read_image = Resolver[ReadClipboardImage]()
    write_image = Resolver[WriteClipboardImage]()
    read_file_list = Resolver[ReadClipboardFileList]()
    write_file_list = Resolver[WriteClipboardFileList]()


def read_macos() -> str:
    return subprocess.check_output("pbpaste").decode().strip()


def write_macos(content: str) -> None:
    subprocess.run("pbcopy", input=content, text=True)


def read_image_macos() -> Image.Image:
    data = subprocess.check_output(("osascript", "-e", "get the clipboard as «class PNGf»"))
    # On macOS data looks like this: '«data PNGf<hex-string>»\n'.
    # So it has to be stripped and converted from hex.
    hex_string = data[11:-3].decode()
    content = io.BytesIO(bytes.fromhex(hex_string))
    return Image.open(content, formats=["png"])


def write_image_macos(img: Image.Image) -> None:
    # AppleScript can't read from stdin, but can read from a file.
    with tempfile.NamedTemporaryFile(suffix=".png") as tmp:
        img.save(tmp)
        cmd = (
            "osascript",
            "-e",
            "on run args",
            "-e",
            "set the clipboard to (read POSIX file (first item of args) as JPEG picture)",
            "-e",
            "end",
            tmp.name,
        )

        subprocess.run(cmd, check=True)


def _require_appkit():
    try:
        from AppKit import NSFilenamesPboardType, NSPasteboard

        return NSPasteboard, NSFilenamesPboardType
    except ImportError:
        import pytest

        pytest.skip("AppKit not available (requires pyobjc)")


def read_file_list_macos() -> List[str]:
    NSPasteboard, NSFilenamesPboardType = _require_appkit()

    pb = NSPasteboard.generalPasteboard()
    types = pb.types()
    if NSFilenamesPboardType in types:
        file_paths = pb.propertyListForType_(NSFilenamesPboardType)
        return list(file_paths)
    return []


def write_file_list_macos(file_list: List[str]) -> None:
    NSPasteboard, NSFilenamesPboardType = _require_appkit()

    pb = NSPasteboard.generalPasteboard()
    pb.declareTypes_owner_([NSFilenamesPboardType], None)
    pb.setPropertyList_forType_(file_list, NSFilenamesPboardType)


def read_win() -> str:
    return subprocess.check_output(("powershell.exe", "Get-Clipboard")).decode().strip()


def write_win(content: str) -> None:
    # Set-Clipboard won't read input piped from a Python process.
    # Another cmdlet `clip` does read from a pipe, but adds 4 unexpected
    # null bytes to the content.
    subprocess.check_call(("powershell.exe", "Set-Clipboard", content))


def read_image_win() -> Image.Image:
    tmp_dir = tempfile.gettempdir()
    tmp_file = Path(tmp_dir) / "pasted_image.png"
    subprocess.run(
        (
            "powershell.exe",
            "Add-Type",
            "-Assembly",
            "System.Drawing,",
            "System.Windows.Forms;",
            '[System.Windows.Forms.Clipboard]::GetImage().Save("%s")' % tmp_file,
        ),
        check=True,
    )

    try:
        img = Image.open(tmp_file)
        img.load()
    finally:
        tmp_file.unlink()

    return img


def write_image_win(img: Image.Image) -> None:
    tmp_dir = tempfile.gettempdir()
    tmp_file = Path(tmp_dir) / "copied_image.png"

    try:
        img.save(tmp_file)
        subprocess.run(
            (
                "powershell.exe",
                "Add-Type",
                "-Assembly",
                "System.Drawing,",
                "System.Windows.Forms;",
                '[System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile("%s"))'
                % tmp_file,
            ),
            check=True,
        )
    finally:
        tmp_file.unlink()


def read_file_list_win() -> List[str]:
    result = subprocess.check_output(
        (
            "powershell.exe",
            "-NoProfile",
            "-Command",
            "Add-Type -Assembly System.Windows.Forms; "
            "[System.Windows.Forms.Clipboard]::GetFileDropList() | ForEach-Object { $_ }",
        )
    )
    return [line.strip() for line in result.decode().splitlines() if line.strip()]


def write_file_list_win(file_list: List[str]) -> None:
    # Build a PowerShell StringCollection from the provided paths and set it as clipboard.
    paths_ps = ", ".join("'%s'" % p.replace("'", "''") for p in file_list)
    script = (
        "Add-Type -Assembly System.Windows.Forms; "
        "$col = New-Object System.Collections.Specialized.StringCollection; "
        "$col.AddRange(@(%s)); " % paths_ps
        + "[System.Windows.Forms.Clipboard]::SetFileDropList($col)"
    )
    subprocess.run(("powershell.exe", "-NoProfile", "-Command", script), check=True)


def read_file_list_linux() -> List[str]:
    result = subprocess.check_output(
        ("xclip", "-sel", "clipboard", "-o", "-target", "text/uri-list")
    )
    paths = []
    for line in result.decode().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("file://"):
            from urllib.parse import unquote

            paths.append(unquote(line[7:]))
        else:
            paths.append(line)
    return paths


def write_file_list_linux(file_list: List[str]) -> None:
    from urllib.parse import quote

    uri_list = "\n".join("file://" + quote(p, safe="/") for p in file_list)
    subprocess.run(
        ("xclip", "-sel", "clipboard", "-i", "-target", "text/uri-list"),
        input=uri_list.encode(),
        check=True,
    )


def read_linux() -> str:
    return subprocess.check_output(("xclip", "-sel", "clipboard", "-o")).decode()


def write_linux(content: str) -> None:
    subprocess.run(("xclip", "-sel", "clipboard", "-i"), input=content, text=True)


def read_image_linux() -> Image.Image:
    data = subprocess.check_output(("xclip", "-sel", "clipboard", "-o", "-target", "image/png"))
    buffer = io.BytesIO(data)
    return Image.open(buffer, formats=["png"])


def write_image_linux(img: Image.Image) -> None:
    buffer = io.BytesIO()
    img.save(buffer, format="png")
    subprocess.run(
        ("xclip", "-sel", "clipboard", "-i", "-target", "image/png"),
        input=buffer.getvalue(),
        check=True,
    )
