//! V8 context utilities.

use v8::{Context, HandleScope, Local};

/// Create a new V8 execution context.
pub fn create_context<'s>(scope: &mut HandleScope<'s, ()>) -> Local<'s, Context> {
    Context::new(scope)
}
