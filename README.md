# Typst Python Runner Plugin

Run python code in [typst](https://typst.app).

````typst
#import "@local/pyrunner:0.0.1": python

#python(```
import re

string = "My email address is john.doe@example.com and my friend's email address is jane.doe@example.net."

re.findall(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b", string)
```)
````

# Build from source

Install `wasi-stub`.

```
cargo install --git https://github.com/astrale-sharp/wasm-minimal-protocol.git wasi-stub
```

Build pyrunner.

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
wasi-stub target/wasm32-wasi/debug/typst-pyrunner.wasm -o pkg/typst-pyrunner.wasm
```

Add to local package.

```
mkdir -p ~/.local/share/typst/packages/local/pyrunner/0.0.1
cp pkg/* ~/.local/share/typst/packages/local/pyrunner/0.0.1
```