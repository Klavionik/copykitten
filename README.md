# copykitten
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
[![PyPI - Version](https://img.shields.io/pypi/v/copykitten)](https://pypi.org/project/copykitten)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/copykitten)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/copykitten)](https://pypistats.org/packages/copykitten)


A robust, dependency-free way to use the system clipboard in Python.

# Installation
`copykitten` supports Python >= 3.8.

You can install `copykitten` from PyPI using `pip` or any other Python package manager.

```sh
pip install copykitten
```

# Usage
## Text
To copy or paste text content, use `copykitten.copy` and `copykitten.paste` functions.

```python
import copykitten

copykitten.copy("The kitten says meow")
```

```python
import copykitten

text = copykitten.paste()
```

## Image
To copy or paste images, use `copykitten.copy_image` and `copykitten.paste_image` functions.
Working with images is a bit complex, so read further.

```python
import copykitten
from PIL import Image

image = Image.open("image.png")
pixels = image.tobytes()

copykitten.copy_image(pixels, image.width, image.height)
```

```python
import copykitten
from PIL import Image

pixels, width, height = copykitten.paste_image()
image = Image.frombytes(mode="RGBA", size=(width, height), data=pixels)
image.save("image.png")
```

To copy an image to the clipboard, you have to pass three arguments - pixel data, width, and height.
Pixel data must be a `bytes` object containing the raw RGBA value for each pixel. You can easily get it using an imaging
library like [Pillow](https://github.com/python-pillow/Pillow).

If your image is not of RGBA type (like a typical JPEG, which is RGB), you first have to convert it to RGBA, otherwise
`copy_image` will raise an exception.

When pasting an image from the clipboard you will receive a 3-tuple of (pixels, width, height). Pixels here are the same
RGBA `bytes` object. Please note that it is not guaranteed that any image copied to the clipboard by another program
will be successfully pasted with `copykitten`.

You can read more about the [data format](https://docs.rs/arboard/latest/arboard/struct.ImageData.html) and the
[implications](https://docs.rs/arboard/latest/arboard/struct.Clipboard.html#method.get_image) of working with images in
the `arboard` documentation.

## Clear
To clear the clipboard, use `copykitten.clear` function.

```python
import copykitten

copykitten.clear()
```

# Rationale
At the time of writing, there are very few Python packages that handle the clipboard. Most of them are simply no longer
maintained (including the most popular solution around the web, [pyperclip](https://github.com/asweigart/pyperclip)).

They all depend on external command-line tools like xclip/pbcopy or libraries like PyQt/GTK. You have to make sure these
dependencies are installed on the target machine, otherwise they won’t work.

There are some solutions using the Tkinter library, which comes with the standard Python suite. However, these solutions
are fragile and may leave your app unresponsive.

Copykitten is a lightweight wrapper around the Rust [arboard](https://github.com/1Password/arboard) library. It comes
with pre-built wheels for Linux (x64, ARM64), macOS (x64, ARM64), and Windows (x64), so you don't have to worry about
anything.

# What's in a name?
You can’t even imagine, how many Python packages devoted to the clipboard management there are on PyPI! Most of them
are abandoned for a decade, and all the neat obvious names (and even some rather creative ones) are already taken.
So I had no choice, but to invent this tongue-in-cheek name. Also, my wife approved it.
