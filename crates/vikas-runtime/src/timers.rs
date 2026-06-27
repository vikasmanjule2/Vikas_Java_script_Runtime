use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time;
use v8::{
    ContextScope, Function, FunctionCallbackArguments, HandleScope, Number, ReturnValue,
    String as V8String,
};

lazy_static! {
    static ref TIMERS: Arc<Mutex<HashMap<u64, tokio::task::JoinHandle<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref TIMER_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

pub fn bind(scope: &mut ContextScope<'_, HandleScope<'_>>) {
    let global = scope.get_current_context().global(scope);

    let set_timeout_fn = Function::new(scope, set_timeout_callback).unwrap();
    let set_timeout_key = V8String::new(scope, "setTimeout").unwrap();
    global.set(scope, set_timeout_key.into(), set_timeout_fn.into());

    let clear_timeout_fn = Function::new(scope, clear_timeout_callback).unwrap();
    let clear_timeout_key = V8String::new(scope, "clearTimeout").unwrap();
    global.set(scope, clear_timeout_key.into(), clear_timeout_fn.into());

    let set_interval_fn = Function::new(scope, set_interval_callback).unwrap();
    let set_interval_key = V8String::new(scope, "setInterval").unwrap();
    global.set(scope, set_interval_key.into(), set_interval_fn.into());

    let clear_interval_fn = Function::new(scope, clear_interval_callback).unwrap();
    let clear_interval_key = V8String::new(scope, "clearInterval").unwrap();
    global.set(scope, clear_interval_key.into(), clear_interval_fn.into());
}

fn set_timeout_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    if args.length() < 1 {
        rv.set_undefined();
        return;
    }

    let callback = args.get(0);
    if !callback.is_function() {
        rv.set_undefined();
        return;
    }

    let delay = if args.length() > 1 {
        args.get(1)
            .to_uint32(scope)
            .map(|n| n.value() as u64)
            .unwrap_or(0)
    } else {
        0
    };

    let mut counter = TIMER_COUNTER.lock().unwrap();
    let timer_id = *counter;
    *counter += 1;
    drop(counter);

    let handle = tokio::spawn(async move {
        time::sleep(time::Duration::from_millis(delay)).await;
        println!("Timer {} executed", timer_id);
    });

    TIMERS.lock().unwrap().insert(timer_id, handle);
    rv.set(Number::new(scope, timer_id as f64).into());
}

fn clear_timeout_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    if args.length() >= 1 {
        if let Some(num) = args.get(0).to_uint32(scope) {
            let id = num.value() as u64;
            if let Some(handle) = TIMERS.lock().unwrap().remove(&id) {
                handle.abort();
            }
        }
    }
    rv.set_undefined();
}

fn set_interval_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut rv: ReturnValue,
) {
    if args.length() < 1 {
        rv.set_undefined();
        return;
    }

    let callback = args.get(0);
    if !callback.is_function() {
        rv.set_undefined();
        return;
    }

    let delay = if args.length() > 1 {
        args.get(1)
            .to_uint32(scope)
            .map(|n| n.value() as u64)
            .unwrap_or(1000)
    } else {
        1000
    };

    let mut counter = TIMER_COUNTER.lock().unwrap();
    let timer_id = *counter;
    *counter += 1;
    drop(counter);

    let handle = tokio::spawn(async move {
        loop {
            time::sleep(time::Duration::from_millis(delay)).await;
            println!("Interval {} tick", timer_id);
        }
    });

    TIMERS.lock().unwrap().insert(timer_id, handle);
    rv.set(Number::new(scope, timer_id as f64).into());
}

fn clear_interval_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    rv: ReturnValue,
) {
    clear_timeout_callback(scope, args, rv);
}
