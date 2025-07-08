use deno_core::{JsRuntime, RuntimeOptions, v8};

pub fn exec_script(
    script: String,
    args: Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    runtime
        .execute_script("<init>", script)
        .map_err(|e| format!("{e}"))?;

    let mut function = String::from("run()");
    if !args.is_empty() {
        function = String::from(format!("run({})", args.join(", ")));
    }

    let result = runtime
        .execute_script("<run>", function)
        .map_err(|e| format!("{e}"))?;

    let scope = &mut runtime.handle_scope();
    let value = v8::Local::new(scope, result);

    if value.is_string() {
        Ok(value.to_rust_string_lossy(scope))
    } else if value.is_object() {
        let global = scope.get_current_context().global(scope);

        let json_key = v8::String::new(scope, "JSON").unwrap();
        let json_key_val = json_key.into();

        let json_obj = global
            .get(scope, json_key_val)
            .and_then(|val| v8::Local::<v8::Object>::try_from(val).ok())
            .ok_or("Não foi possível acessar JSON")?;

        let stringify_key = v8::String::new(scope, "stringify").unwrap();
        let stringify_key_val = stringify_key.into();

        let stringify_fn = json_obj
            .get(scope, stringify_key_val)
            .and_then(|val| v8::Local::<v8::Function>::try_from(val).ok())
            .ok_or("Não foi possível acessar JSON.stringify")?;

        let args = [value];
        let json_str = stringify_fn
            .call(scope, json_obj.into(), &args)
            .and_then(|val| v8::Local::<v8::String>::try_from(val).ok())
            .ok_or("Erro ao converter objeto para JSON string")?;

        Ok(json_str.to_rust_string_lossy(scope))
    } else {
        Ok(value
            .to_string(scope)
            .ok_or("Erro ao converter valor JS para string")?
            .to_rust_string_lossy(scope))
    }
}
