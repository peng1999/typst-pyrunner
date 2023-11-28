#let py = plugin("./typst-pyrunner.wasm")

#let python(code, globals: (:)) = {
  let code = if type(code) == "content" {
    code.text
  } else {
    code
  }
  cbor.decode(py.run_py(bytes(code), cbor.encode(globals)))
}

// Usage:
//
// #let code = ```
// def fib(n):
//   return 1 if n <= 2 else fib(n-1) + fib(n-2)
//
// fib(5)
// ```
//
// #python(code)
