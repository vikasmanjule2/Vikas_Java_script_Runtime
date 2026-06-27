use v8::{
    ContextScope, Function, FunctionCallbackArguments, HandleScope, Object, ReturnValue,
    String as V8String,
};

pub fn bind(scope: &mut ContextScope<'_, HandleScope<'_>>) {
    let console = Object::new(scope);

    let log_fn = Function::new(scope, log_callback).unwrap();
    let log_key = V8String::new(scope, "log").unwrap();
    console.set(scope, log_key.into(), log_fn.into());

    let error_fn = Function::new(scope, error_callback).unwrap();
    let error_key = V8String::new(scope, "error").unwrap();
    console.set(scope, error_key.into(), error_fn.into());

    let warn_fn = Function::new(scope, warn_callback).unwrap();
    let warn_key = V8String::new(scope, "warn").unwrap();
    console.set(scope, warn_key.into(), warn_fn.into());

    let info_fn = Function::new(scope, info_callback).unwrap();
    let info_key = V8String::new(scope, "info").unwrap();
    console.set(scope, info_key.into(), info_fn.into());

    let debug_fn = Function::new(scope, debug_callback).unwrap();
    let debug_key = V8String::new(scope, "debug").unwrap();
    console.set(scope, debug_key.into(), debug_fn.into());

    let global = scope.get_current_context().global(scope);
    let console_key = V8String::new(scope, "console").unwrap();
    global.set(scope, console_key.into(), console.into());
}

fn format_args(scope: &mut HandleScope, args: FunctionCallbackArguments) -> String {
    let mut output = String::new();
    for i in 0..args.length() {
        if i > 0 {
            output.push(' ');
        }
        let value = args.get(i);
        if let Some(str) = value.to_string(scope) {
            output.push_str(&str.to_rust_string_lossy(scope));
        } else {
            output.push_str(&format!("{:?}", value));
        }
    }
    output
}

fn log_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    println!("{}", format_args(scope, args));
    rv.set_undefined();
}

fn error_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    eprintln!("ERROR: {}", format_args(scope, args));
    rv.set_undefined();
}

fn warn_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    println!("WARN: {}", format_args(scope, args));
    rv.set_undefined();
}

fn info_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    println!("INFO: {}", format_args(scope, args));
    rv.set_undefined();
}

fn debug_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    println!("DEBUG: {}", format_args(scope, args));
    rv.set_undefined();
}
