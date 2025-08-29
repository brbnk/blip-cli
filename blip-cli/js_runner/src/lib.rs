use std::rc::Rc;

use contexts::MANAGER_POOL;
use deno_core::{extension, futures, op2, v8, JsRuntime, PollEventLoopOptions, RuntimeOptions};

extension!(
    scriptv2,
    ops = [
        op_get_var,
        op_set_var,
        op_delete_var
    ],
    esm_entry_point = "ext:scriptv2/runtime.js",
    esm = [dir "src", "runtime.js"],
);

pub fn exec_script(
    script: String,
    args: Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![
            scriptv2::init()
        ],
        ..Default::default()
    });

    runtime
        .execute_script("<init>", script)
        .map_err(|e| format!("{e}"))?;

    let mut function = String::from("run()");
    if !args.is_empty() {
        function = String::from(format!("run({})", args.join(", ")));
    }

    let result_val = runtime
        .execute_script("<run>", function)
        .map_err(|e| format!("{e}"))?;

     let value_global = {
        let scope = &mut runtime.handle_scope();
        let local_value = v8::Local::new(scope, result_val);
        v8::Global::new(scope, local_value)
    };

    futures::executor::block_on(runtime.run_event_loop(PollEventLoopOptions {
        wait_for_inspector: false,
        pump_v8_message_loop: true,
    }))?;

    #[allow(deprecated)]
    let resolved = futures::executor::block_on(runtime.resolve_value(value_global))?;

    let scope = &mut runtime.handle_scope();
    let value = v8::Local::new(scope, resolved);
    
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

#[op2(async)]
#[string]
pub async fn op_get_var(#[string] key: String) -> Result<String, deno_core::error::CoreError> {
    Ok(match MANAGER_POOL.context.get(&key) {
        Some(s) => { 
            s
        },
        None => String::new(),
    })
}

#[op2(async)]
pub async fn op_set_var(#[string] key: String, #[string] value: String) -> Result<(), deno_core::error::CoreError> {
    MANAGER_POOL.context.set(&key, &value);
    Ok(())
}


#[op2(async)]
pub async fn op_delete_var(#[string] key: String) -> Result<(), deno_core::error::CoreError> {
    MANAGER_POOL.context.delete(&key);
    Ok(())
}