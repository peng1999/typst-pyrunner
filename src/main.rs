use rustpython::vm::{
    builtins::{PyBaseException, PyBaseExceptionRef},
    types::Representable,
    VirtualMachine,
};
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

// This is to give a entry point for the wasm file, so the compiler won't add __wasm_call_ctors to
// all exported functions.
fn main() {}

#[wasm_func]
pub fn run_py(code: &[u8]) -> Result<Vec<u8>, String> {
    let code = std::str::from_utf8(code).map_err(|err| err.to_string())?;
    let interpreter = rustpython::InterpreterConfig::default()
        .init_stdlib()
        .interpreter();
    let result = interpreter.enter(|vm| {
        run_with_vm(vm, code).map_err(|err: PyBaseExceptionRef| {
            PyBaseException::repr_str(&err, vm).unwrap_or("Error during print".to_string())
        })
    })?;
    Ok(result.into_bytes())
}

fn run_with_vm(vm: &VirtualMachine, code: &str) -> Result<String, PyBaseExceptionRef> {
    let scope = vm.new_scope_with_builtins();
    let code = vm
        .compile(
            code,
            rustpython::vm::compiler::Mode::BlockExpr,
            "<main>".to_string(),
        )
        .map_err(|err| vm.new_syntax_error(&err, Some(code)))?;
    let obj = vm.run_code_obj(code, scope)?;
    Ok(obj.str(vm)?.to_string())
}
