pub mod console;
pub mod context;
pub mod fetch;
pub mod isolate;
pub mod module;
pub mod timers;

use std::sync::Arc;
use std::sync::Once;
use tokio::sync::Mutex;
use v8::{
    ContextScope, Global, HandleScope, Local, Module, OwnedIsolate, Script, String as V8String,
};

pub struct VikasRuntime {
    context: Global<v8::Context>,
    modules: Arc<Mutex<Vec<Global<Module>>>>,
    isolate: OwnedIsolate,
}

static V8_INIT: Once = Once::new();

impl VikasRuntime {
    pub fn new() -> Self {
        V8_INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        let mut isolate = v8::Isolate::new(isolate::default_create_params());

        let context_global = {
            let scope = &mut HandleScope::new(&mut isolate);
            let context = context::create_context(scope);
            let scope = &mut ContextScope::new(scope, context);

            console::bind(scope);
            timers::bind(scope);
            fetch::bind(scope);

            Global::new(scope, context)
        };

        Self {
            context: context_global,
            modules: Arc::new(Mutex::new(Vec::new())),
            isolate,
        }
    }

    pub fn execute_script(&mut self, code: &str) -> Result<std::string::String, std::string::String> {
        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let source = V8String::new(scope, code).ok_or("Failed to create source string")?;
        let script = Script::compile(scope, source, None)
            .ok_or("Script compilation failed")?;

        match script.run(scope) {
            Some(result) => Ok(module::value_to_string(scope, result)),
            None => Err("Script execution failed".to_string()),
        }
    }

    pub fn load_module(&mut self, path: &str) -> Result<(), std::string::String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read module: {}", e))?;

        let scope = &mut HandleScope::new(&mut self.isolate);
        let context = Local::new(scope, &self.context);
        let scope = &mut ContextScope::new(scope, context);

        let module_global = module::load_module_from_source(scope, path, &content)?;
        self.modules.blocking_lock().push(module_global);

        Ok(())
    }

    pub fn create_http_server(&mut self) -> vikas_http::HttpServer {
        vikas_http::HttpServer::new()
    }
}

impl Default for VikasRuntime {
    fn default() -> Self {
        Self::new()
    }
}

