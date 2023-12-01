# Typst Python Runner Plugin

Run python code in [typst](https://typst.app).

````typst
#import "@local/pyrunner:0.0.2" as py

#let compiled = py.compile(
```python
def find_emails(string):
    import re
    return re.findall(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b", string)
```)

#let txt = "My email address is john.doe@example.com and my friend's email address is jane.doe@example.net."

#py.call(compiled, "find_emails", txt)
````

Block mode is also available.

````typst
#let code = ```
f'{a+b=}'
```

#py.block(code, globals: (a: 1, b: 2))

#py.block(code, globals: (a: "1", b: "2"))
````

The result will be `a+b=3` and `a+b='12'`.

## Current limitations

- No file and network IO due to limitations of typst plugin
  - As a consequence, there is no way to import third-party modules. Only bundled stdlib modules are available.

## Use pre-built package

Download from [releases](https://github.com/peng1999/typst-pyrunner/releases) page and copy the files to `~/.local/share/typst/packages/local/pyrunner/0.0.1`.

## Build from source

Install [`wasi-stub`][]. You should use a slightly modified one. See [the related issue](https://github.com/astrale-sharp/wasm-minimal-protocol/issues/22#issuecomment-1827379467).

[`wasi-stub`]: https://github.com/astrale-sharp/wasm-minimal-protocol

<!-- ```
cargo install --git https://github.com/astrale-sharp/wasm-minimal-protocol.git wasi-stub
```-->

Build pyrunner.

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
make pkg/typst-pyrunner.wasm
```

Add to local package.

```
mkdir -p ~/.local/share/typst/packages/local/pyrunner/0.0.1
cp pkg/* ~/.local/share/typst/packages/local/pyrunner/0.0.1
```
