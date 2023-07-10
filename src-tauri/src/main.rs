#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vm::PyResult;
use rustpython_vm as vm;

#[tauri::command]
fn run(input: &str) -> String {
    // Run RustPython interpreter
    vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    })
    .enter(|vm| -> PyResult<()> {
        let scope = vm.new_scope_with_builtins();
        let code_obj = vm
            .compile(input, vm::compiler::Mode::Exec, "<embedded>".to_owned())
            .map_err(|err| vm.new_syntax_error(&err, Some(input)))?;
        if let Err(exc) = vm.run_code_obj(code_obj, scope) {
            vm.print_exception(exc);
        }
        Ok(())
    }).unwrap();

    todo!()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
