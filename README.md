# copykitten
`copykitten` provides a robust, dependency-free way to use the system clipboard in Python.

# Installation
You can install `copykitten` from PyPI using `pip` or any other Python package manager.

```sh
pip install copykitten
```

# Usage
The package API consists of three Python functions: `copy`, `paste`, `clear`. In a nutshell:

```python
import copykitten

copykitten.copy("The kitten says meow")

content = copykitten.paste()
print(content) # "The kitten says meow"

copykitten.clear()
content = copykitten.paste()

print(content) # ""
```

# Rationale
At the time of writing, there are very few Python packages that handle the clipboard. Most of them are simply no longer 
maintained (including the most popular solution around the web, [pyperclip](https://github.com/asweigart/pyperclip)).

They all depend on external command-line tools like xclip/pbcopy or libraries like PyQt/GTK. You have to make sure these 
dependencies are installed on the target machine, otherwise they won’t work. 

There are some solutions using the Tkinter library, which comes with the standard Python suite. However, these solutions 
are fragile and may leave your app unresponsive.

Copykitten is a lightweight wrapper around the Rust [arboard](https://github.com/1Password/arboard) library. It comes with pre-built wheels for Linux, 
MacOS, Windows, and ARM64, so you don't have to worry about anything.

# What's in a name?
You can’t even imagine, how many Python packages devoted to the clipboard management there are on PyPI! Most of them 
are abandoned for a decade, and all the neat obvious names (and even some rather creative ones) are already taken. 
So I had no choice, but to invent this tongue-in-cheek name. Also, my wife approved it.