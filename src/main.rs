use rustpython::vm::{
    builtins::{PyBaseException, PyBaseExceptionRef, PyBool, PyDict, PyInt, PyList, PyTuple},
    types::Representable,
    AsObject, PyObjectRef, PyPayload, VirtualMachine,
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
    let mut buffer = vec![];
    ciborium::ser::into_writer(&result, &mut buffer).map_err(|err| err.to_string())?;
    Ok(buffer)
}

fn py_to_cbor(
    vm: &VirtualMachine,
    obj: PyObjectRef,
) -> Result<ciborium::Value, PyBaseExceptionRef> {
    if obj.is(&vm.ctx.none()) {
        return Ok(ciborium::Value::Null);
    }
    if let Ok(num) = obj.clone().downcast::<PyInt>() {
        let int = num.try_to_primitive::<i64>(vm)?;
        if obj.is_instance(PyBool::class(&vm.ctx).as_object(), vm)? {
            return Ok((int != 0).into());
        }
        return Ok(int.into());
    }
    if let Ok(list) = obj.clone().downcast::<PyTuple>() {
        let values = list
            .into_iter()
            .map(|item| py_to_cbor(vm, item.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(values.into());
    }
    if let Ok(list) = obj.clone().downcast::<PyList>() {
        let values = list
            .borrow_vec()
            .iter()
            .map(|item| py_to_cbor(vm, item.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(values.into());
    };
    if let Ok(dict) = obj.clone().downcast::<PyDict>() {
        let kvs = dict
            .into_iter()
            .map(|(key, value)| {
                let key = ciborium::Value::from(key.str(vm)?.to_string());
                let value = py_to_cbor(vm, value.clone())?;
                Ok((key, value))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(kvs.into());
    };
    Ok(obj.str(vm)?.to_string().into())
}

fn run_with_vm(vm: &VirtualMachine, code: &str) -> Result<ciborium::Value, PyBaseExceptionRef> {
    let scope = vm.new_scope_with_builtins();
    let code = vm
        .compile(
            code,
            rustpython::vm::compiler::Mode::BlockExpr,
            "<main>".to_string(),
        )
        .map_err(|err| vm.new_syntax_error(&err, Some(code)))?;
    let obj = vm.run_code_obj(code, scope)?;
    Ok(py_to_cbor(vm, obj)?)
}
