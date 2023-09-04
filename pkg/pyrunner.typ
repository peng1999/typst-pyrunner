#let py = plugin("./typst-pyrunner.wasm")

#let python(code) = {
  let code = if type(code) == "content" {
    code.text
  } else {
    code
  }
  str(py.run_py(bytes(code)))
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
