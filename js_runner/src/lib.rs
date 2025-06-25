use std::{cell::RefCell};
use deno_core::{JsRuntime, RuntimeOptions};

thread_local! {
    static JS_RUNTIME: RefCell<JsRuntime> = RefCell::new(
        JsRuntime::new(RuntimeOptions::default())
    );
}

pub fn exec_script(script: String) -> Result<String, Box<dyn std::error::Error>> {
    let result: Result<String, Box<dyn std::error::Error>> = JS_RUNTIME.with(|cell| {
        let mut runtime = cell.borrow_mut();
        
        runtime.execute_script("<init>", script)
            .map_err(|e| format!("Erro ao executar script: {e}"))?;

        let result = runtime
            .execute_script("<run>", "run()")
            .map_err(|e| format!("Erro ao executar script: {e}"))?;

        let scope = &mut runtime.handle_scope();
        let local = result.open(scope);

        let rust_string = if local.is_string() {
            local.to_rust_string_lossy(scope)
        } else {
            let to_string = local
                .to_string(scope)
                .ok_or("Erro ao converter retorno JS para string")?; // &str vira Box<dyn Error>
            to_string.to_rust_string_lossy(scope)
        };

        Ok(rust_string)
    });

    result
}