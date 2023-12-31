use rustpython::vm::{
    builtins::{
        PyBaseException, PyBaseExceptionRef, PyBool, PyBytes, PyDict, PyInt, PyList, PyTuple,
    },
    bytecode::BasicBag,
    compiler::CodeObject,
    convert::ToPyObject,
    frozen::FrozenCodeObject,
    types::Representable,
    AsObject, PyObjectRef, PyPayload, VirtualMachine,
};
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn run_py(code: &[u8], globals: &[u8]) -> Result<Vec<u8>, String> {
    let code = std::str::from_utf8(code).map_err(|err| err.to_string())?;
    let globals = ciborium::de::from_reader::<ciborium::Value, _>(globals)
        .map_err(|err| err.to_string())?
        .into_map()
        .map_err(|_| "globals not a map".to_string())?;

    let interpreter = rustpython::InterpreterConfig::default()
        .init_stdlib()
        .interpreter();
    let result = interpreter.enter(|vm| {
        let globals = globals
            .into_iter()
            .map(|(key, value)| {
                let key = key
                    .into_text()
                    .map_err(|_| "key not a string".to_string())?;
                let obj = cbor_to_py(vm, value).map_err(|err| pyerr_to_string(err, vm))?;
                Ok::<_, String>((key, obj))
            })
            .collect::<Result<Vec<_>, _>>()?;
        run_with_vm(vm, code, &globals).map_err(|err| pyerr_to_string(err, vm))
    })?;

    let mut buffer = vec![];
    ciborium::ser::into_writer(&result, &mut buffer).map_err(|err| err.to_string())?;
    Ok(buffer)
}

#[wasm_func]
pub fn compile_py(code: &[u8]) -> Result<Vec<u8>, String> {
    let code = std::str::from_utf8(code).map_err(|err| err.to_string())?;
    let interpreter = rustpython::InterpreterConfig::default()
        .init_stdlib()
        .interpreter();
    let result = interpreter.enter(|vm| {
        let code = vm
            .compile(
                code,
                rustpython::vm::compiler::Mode::Exec,
                "<main>".to_string(),
            )
            .map_err(|err| pyerr_to_string(vm.new_syntax_error(&err, None), vm))?;
        let frozen_code = FrozenCodeObject::encode(&code);
        Ok::<_, String>(frozen_code)
    })?;

    Ok(result.bytes)
}

#[wasm_func]
fn call_compiled(frozen_code: &[u8], fn_name: &[u8], args: &[u8]) -> Result<Vec<u8>, String> {
    let fn_name = std::str::from_utf8(fn_name).map_err(|err| err.to_string())?;
    let args = ciborium::de::from_reader::<ciborium::Value, _>(args)
        .map_err(|err| err.to_string())?
        .into_array()
        .map_err(|_| "args not an array".to_string())?;
    let code = FrozenCodeObject { bytes: frozen_code }.decode(BasicBag);

    let interpreter = rustpython::InterpreterConfig::default()
        .init_stdlib()
        .interpreter();
    let result = interpreter.enter(|vm| {
        run_compiled_with_vm(vm, code, fn_name, args).map_err(|err| pyerr_to_string(err, vm))
    })?;

    let mut buffer = vec![];
    ciborium::ser::into_writer(&result, &mut buffer).map_err(|err| err.to_string())?;
    Ok(buffer)
}

fn pyerr_to_string(err: PyBaseExceptionRef, vm: &VirtualMachine) -> String {
    PyBaseException::repr_str(&err, vm).unwrap_or("Error during print".to_string())
}

fn cbor_to_py(
    vm: &VirtualMachine,
    val: ciborium::Value,
) -> Result<PyObjectRef, PyBaseExceptionRef> {
    match val {
        ciborium::Value::Null => Ok(vm.ctx.none()),
        ciborium::Value::Bool(b) => Ok(vm.ctx.new_bool(b).to_pyobject(vm)),
        ciborium::Value::Integer(i) => Ok(vm.ctx.new_int(i128::from(i)).to_pyobject(vm)),
        ciborium::Value::Float(f) => Ok(vm.ctx.new_float(f).to_pyobject(vm)),
        ciborium::Value::Bytes(b) => Ok(vm.ctx.new_bytes(b).to_pyobject(vm)),
        ciborium::Value::Text(s) => Ok(vm.ctx.new_str(s).to_pyobject(vm)),
        ciborium::Value::Array(arr) => {
            let items = arr
                .into_iter()
                .map(|item| cbor_to_py(vm, item))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(vm.ctx.new_list(items).to_pyobject(vm))
        }
        ciborium::Value::Map(map) => {
            let dict = vm.ctx.new_dict();
            for (key, value) in map {
                let key = cbor_to_py(vm, key)?;
                let value = cbor_to_py(vm, value)?;
                dict.set_item(key.as_object(), value, vm)?;
            }
            Ok(dict.to_pyobject(vm))
        }
        _ => Err(vm.new_type_error("Unsupported type".to_string())),
    }
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
    if let Ok(bytes) = obj.clone().downcast::<PyBytes>() {
        return Ok(bytes.as_bytes().into());
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

fn run_with_vm(
    vm: &VirtualMachine,
    code: &str,
    globals: &[(String, PyObjectRef)],
) -> Result<ciborium::Value, PyBaseExceptionRef> {
    let scope = vm.new_scope_with_builtins();
    for (name, value) in globals {
        scope.globals.set_item(name, value.clone(), vm)?;
    }
    let obj = vm.run_block_expr(scope, code)?;
    Ok(py_to_cbor(vm, obj)?)
}

fn run_compiled_with_vm(
    vm: &VirtualMachine,
    code: CodeObject,
    fn_name: &str,
    args: Vec<ciborium::Value>,
) -> Result<ciborium::Value, PyBaseExceptionRef> {
    let code = vm.ctx.new_code(code);
    let scope = vm.new_scope_with_builtins();
    vm.run_code_obj(code, scope.clone())?;
    let fn_obj = scope.globals.get_item(fn_name, vm)?;
    let args = args
        .into_iter()
        .map(|arg| cbor_to_py(vm, arg))
        .collect::<Result<Vec<_>, _>>()?;
    let result = fn_obj.call(args, vm)?;
    Ok(py_to_cbor(vm, result)?)
}
