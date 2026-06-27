//! ES module loading utilities.

use v8::script_compiler::{self, Source};
use v8::{
    ContextScope, Global, HandleScope, Local, Module, ScriptOrigin, String as V8String, Value,
};

/// Load and evaluate an ES module from source text.
pub fn load_module_from_source(
    scope: &mut ContextScope<'_, HandleScope<'_>>,
    path: &str,
    content: &str,
) -> Result<Global<Module>, std::string::String> {
    let source = V8String::new(scope, content).ok_or("Failed to create source string")?;
    let path_str = V8String::new(scope, path).ok_or("Failed to create path string")?;
    let source_map_url: Local<Value> = v8::undefined(scope).into();

    let origin = ScriptOrigin::new(
        scope,
        path_str.into(),
        0,
        0,
        false,
        0,
        source_map_url,
        false,
        false,
        true,
    );

    let source = Source::new(source, Some(&origin));
    let module =
        script_compiler::compile_module(scope, source).ok_or("Module parsing failed")?;

    module
        .instantiate_module(scope, |_, _, _, _| None)
        .ok_or("Module instantiation failed")?;

    module
        .evaluate(scope)
        .ok_or("Module evaluation failed")?;

    Ok(Global::new(scope, module))
}

/// Convert a V8 value to a Rust string for display.
pub fn value_to_string(
    scope: &mut ContextScope<'_, HandleScope<'_>>,
    value: v8::Local<Value>,
) -> String {
    value
        .to_string(scope)
        .map(|s| s.to_rust_string_lossy(scope))
        .unwrap_or_else(|| "[object]".to_string())
}
