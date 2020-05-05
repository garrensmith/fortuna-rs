use rusty_v8 as v8;
use rusty_v8::OwnedStartupData;
use std::convert::TryFrom;
use std::thread;
use std::time::{Duration, Instant};

include!(concat!(env!("OUT_DIR"), "/js_startup_code.rs"));

// enum Action {
//     Rewrite,
//     Eval,
//     Call
// }

// pub struct Instruction {
//     action: Action,
//     script: String,
//     // args: Vec<String>,
//     // timeout: uint32,
// }
//
// impl From<JsRequest> for Instruction {
//     fn from(request: JSRequest) -> Self {
//         Instruction {
//             action: Action::Eval,
//             script: request.script
//         }
//     }
// }
pub struct FortunaIsolate {
    isolate: v8::OwnedIsolate,
    global_context: v8::Global<v8::Context>,
}

pub struct JSEnv {
    startup_data: OwnedStartupData,
}

pub fn print() {
    println!("hello");
}

// fn data_is_true_callback(
//     _scope: v8::FunctionCallbackScope,
//     args: v8::FunctionCallbackArguments,
//     _rv: v8::ReturnValue,
// ) {
//     let data = args.data();
//     assert!(data.is_some());
//     let data = data.unwrap();
//     assert!(data.is_true());
// }

fn print_callback(
    scope: v8::FunctionCallbackScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    for i in 0..args.length() {
        let arg1 = args.get(i).to_string(scope).unwrap();
        println!("{:?}", arg1.to_rust_string_lossy(scope));
    }
    rv.set(v8::Boolean::new(scope, true).into())
}

impl JSEnv {
    pub fn new() -> JSEnv {
        let startup_data = JSEnv::create_startup_data();
        JSEnv { startup_data }
    }

    pub fn create_isolate(&self) -> FortunaIsolate {
        let start = Instant::now();
        let isolate = FortunaIsolate::new(&self.startup_data);
        println!(
            "Time elapsed in to create isolate is: {:?}",
            start.elapsed()
        );
        isolate
    }

    // adapted from Deno https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs#L1714
    fn create_startup_data() -> v8::OwnedStartupData {
        let mut snapshot_creator = v8::SnapshotCreator::new(None);
        {
            // TODO(ry) this shouldn't be necessary. workaround unfinished business in
            // the scope type system.
            let mut isolate = unsafe { snapshot_creator.get_owned_isolate() };

            let mut hs = v8::HandleScope::new(&mut isolate);
            let scope = hs.enter();

            let context = v8::Context::new(scope);
            let mut cs = v8::ContextScope::new(scope, context);
            let scope = cs.enter();
            // let source = v8::String::new(scope, "a = 1 + 2; function f() {return 'hello'}").unwrap();
            let source = v8::String::new(scope, JS_CODE).unwrap();
            let mut script = v8::Script::compile(scope, context, source, None).unwrap();
            script.run(scope, context).unwrap();

            snapshot_creator.set_default_context(context);
            std::mem::forget(isolate); // TODO(ry) this shouldn't be necessary.
        }

        snapshot_creator
            .create_blob(v8::FunctionCodeHandling::Clear)
            .unwrap()
    }
}

impl FortunaIsolate {
    pub fn new(_startup_data: &v8::OwnedStartupData) -> FortunaIsolate {
        // let isolate = FortunaIsolate::create_v8_isolate(&startup_data);
        Self::create_isolate()
    }

    pub fn new_clean() -> FortunaIsolate {
        Self::create_isolate()
    }

    // fn create_v8_isolate(snapshot_blob: &v8::OwnedStartupData) -> v8::OwnedIsolate {
    fn create_isolate() -> FortunaIsolate {
        let safe_obj: v8::PropertyAttribute = v8::DONT_DELETE + v8::DONT_ENUM + v8::READ_ONLY;

        let mut global_context = v8::Global::<v8::Context>::new();
        let mut create_params = v8::Isolate::create_params();
        create_params.set_array_buffer_allocator(v8::new_default_allocator());
        // create_params.set_snapshot_blob(&snapshot_blob);
        let mut isolate = v8::Isolate::new(create_params);

        let mut handle_scope = v8::HandleScope::new(&mut isolate);
        let scope = handle_scope.enter();

        let context = v8::Context::new(scope);

        let mut cs = v8::ContextScope::new(scope, context);
        let scope = cs.enter();

        // Add default map functions
        let source = v8::String::new(scope, JS_CODE).unwrap();
        let mut script = v8::Script::compile(scope, context, source, None).unwrap();
        script.run(scope, context).unwrap();

        global_context.set(scope, context);

        FortunaIsolate {
            isolate,
            global_context,
        }

        // let mut cs = v8::ContextScope::new(scope, context);
        // let scope = cs.enter();

        // let object_templ = v8::ObjectTemplate::new(scope);
        // let function_templ = v8::FunctionTemplate::new(scope, print_callback);
        // let name = v8::String::new(scope, "print").unwrap();
        // object_templ.set_with_attr(name.into(), function_templ.into(), safe_obj);
        // let context = scope.get_current_context().unwrap();

        // let global = context.global(scope);
        // let name = v8::String::new(scope, "f").unwrap();
        // let func = global.get(scope, context, name.into()).unwrap();
        // let a = v8::Local::<v8::Function>::try_from(func).unwrap();
        // let receiver = context.global(scope);
        // let resp = a.call(scope, context, receiver.into(), &[]).unwrap();
        // let result = resp.to_string(scope).unwrap();
        // println!("result: {}", result.to_rust_string_lossy(scope));

        // // let context = v8::Context::new_from_template(scope, object_templ);
        // let mut cs = v8::ContextScope::new(scope, context);
        // let scope = cs.enter();
        //
        // let code = v8::String::new(scope, "'Hello' + ' World!'; print(a, 1,2)").unwrap();
        // println!("javascript code: {}", code.to_rust_string_lossy(scope));
        //
        // let mut script = v8::Script::compile(scope, context, code, None).unwrap();
        // let result = script.run(scope, context).unwrap();
        // let result = result.to_string(scope).unwrap();

        // isolate
    }

    pub fn eval(&mut self, script_str: &str, _args: &[String]) -> String {
        // println!("script {:?}", script_str);
        let mut hs = v8::HandleScope::new(&mut self.isolate);
        let scope = hs.enter();
        let context = self.global_context.get(scope).unwrap();
        // let context = v8::Context::new(scope);
        let mut cs = v8::ContextScope::new(scope, context);
        let scope = cs.enter();
        let source = v8::String::new(scope, script_str).unwrap();
        let mut script = v8::Script::compile(scope, context, source, None).unwrap();
        let result = script.run(scope, context).unwrap();
        let result_json_string = v8::json::stringify(context, result).unwrap();
        let result_string = result_json_string.to_rust_string_lossy(scope);
        println!("result eval: {}", result_string);

        if result_string == "undefined".to_string() {
            return "null".to_string();
        }
        result_string
    }

    pub fn call(&mut self, raw_fun_name: &str, args: &[String]) -> String {
        println!("Call {:?} args {:?}", raw_fun_name, args);

        let mut hs = v8::HandleScope::new(&mut self.isolate);
        let scope = hs.enter();
        let context = self.global_context.get(scope).unwrap();
        let mut cs = v8::ContextScope::new(scope, context);
        let scope = cs.enter();

        let global = context.global(scope);
        let name = v8::String::new(scope, raw_fun_name).unwrap();
        let val_func = global.get(scope, context, name.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(val_func).unwrap();
        let receiver = context.global(scope);

        let val_args: Vec<v8::Local<v8::Value>> = args
            .iter()
            .map(|arg| {
                let v8_arg = v8::String::new(scope, arg).unwrap();
                v8::Local::<v8::Value>::try_from(v8_arg).unwrap()
            })
            .collect();

        let resp = func
            .call(scope, context, receiver.into(), val_args.as_slice())
            .unwrap();
        let result = v8::json::stringify(context, resp).unwrap();
        let result_string = result.to_rust_string_lossy(scope);
        println!("result: {}", result_string);
        result_string
    }
}

pub fn init() {
    let platform = v8::new_default_platform();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

// Not really needed
pub fn shutdown() {
    unsafe {
        v8::V8::shutdown_platform();
        v8::V8::dispose();
    }
}
