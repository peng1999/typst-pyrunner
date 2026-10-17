#let py = plugin("./typst-pyrunner.wasm")

#let _extract(code) = {
  if type(code) == content {
    code.text
  } else {
    code
  }
}

/// Execute a Python code block and return its last expression.
///
/// `code` may be provided as a string or as a raw code block. Values passed
/// through `globals` are exposed to the Python runtime as global variables.
///
/// === Example
///
/// ````example
/// #let code = ```python
/// f'{a+b=}'
/// ```
///
/// #py.block(code, globals: (a: 1, b: 2))
///
/// #py.block(code, globals: (a: "1", b: "2"))
/// ````
///
/// -> any
#let block(
  /// The Python source code to execute.
  /// -> str | content
  code,
  /// Global variables made available to the Python runtime.
  /// -> dictionary
  globals: (:),
) = {
  let code = _extract(code)
  cbor(py.run_py(bytes(code), cbor.encode(globals)))
}

/// Compile Python source code to bytecode.
///
/// The returned value can later be passed to @call to invoke functions
/// defined in the compiled module repeatedly.
///
/// === Example
///
/// ````example
/// #let compiled = py.compile(
/// ```python
/// def sum_all(*array):
///     return sum(array)
/// ```)
///
/// #py.call(compiled, "sum_all", 1, 2, 3)
/// ````
///
/// -> bytes
#let compile(
  /// The Python source code to compile.
  /// -> str | content
  code,
) = {
  let code = _extract(code)
  py.compile_py(bytes(code))
}

/// Call a function from compiled Python bytecode.
///
/// The `compiled` argument must come from @compile. All variadic arguments
/// are forwarded positionally to the Python function.
///
/// === Example
///
/// ````example
/// #let compiled = py.compile(
/// ```python
/// def find_emails(string):
///     import re
///     return re.findall(
///         r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b",
///         string,
///     )
/// ```)
///
/// #let txt = "Email john.doe@example.com or jane.doe@example.net."
/// #py.call(compiled, "find_emails", txt)
/// ````
///
/// -> any
#let call(
  /// Compiled Python bytecode returned by @compile.
  /// -> bytes
  compiled,
  /// The name of the Python function to invoke.
  /// -> str
  fn-name,
  /// Positional arguments forwarded to the Python function.
  /// -> any
  ..args,
) = {
  cbor(py.call_compiled(compiled, bytes(fn-name), cbor.encode(args.pos())))
}
