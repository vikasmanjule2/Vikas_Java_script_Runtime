use v8::{
    ContextScope, Function, FunctionCallbackArguments, HandleScope, ReturnValue, String as V8String,
};

pub fn bind(scope: &mut ContextScope<'_, HandleScope<'_>>) {
    let fetch_fn = Function::new(scope, fetch_callback).unwrap();
    let fetch_key = V8String::new(scope, "fetch").unwrap();
    scope
        .get_current_context()
        .global(scope)
        .set(scope, fetch_key.into(), fetch_fn.into());
}

fn fetch_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    if args.length() < 1 {
        rv.set_undefined();
        return;
    }

    let url = args
        .get(0)
        .to_string(scope)
        .map(|s| s.to_rust_string_lossy(scope))
        .unwrap_or_default();

    let result = format!(r#"{{"url": "{}", "status": 200, "ok": true}}"#, url);
    if let Some(result_string) = V8String::new(scope, &result) {
        rv.set(result_string.into());
    } else {
        rv.set_undefined();
    }
}
