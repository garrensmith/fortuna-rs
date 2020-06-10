use rusty_v8 as v8;
use std::convert::TryFrom;

// This is created in build.rs and is all the required js code added into
// a byte array
include!(concat!(env!("OUT_DIR"), "/js_startup_code.rs"));

// TODO: Handle errors properly

pub struct FortunaIsolate {
    isolate: v8::OwnedIsolate,
    global_context: v8::Global<v8::Context>,
}

pub struct JSEnv {
    pub startup_data: Vec<u8>,
}

pub fn print() {
    println!("hello");
}

// fn print_callback(
//     scope: v8::FunctionCallbackScope,
//     args: v8::FunctionCallbackArguments,
//     mut rv: v8::ReturnValue,
// ) {
//     for i in 0..args.length() {
//         let arg1 = args.get(i).to_string(scope).unwrap();
//         println!("{:?}", arg1.to_rust_string_lossy(scope));
//     }
//     rv.set(v8::Boolean::new(scope, true).into())
// }

impl JSEnv {
    pub fn new() -> JSEnv {
        let startup_data = JSEnv::create_startup_data();
        JSEnv {
            startup_data: startup_data.to_vec(),
        }
    }

    // adapted from Deno https://github.com/denoland/rusty_v8/blob/master/tests/test_api.rs#L1714
    fn create_startup_data() -> v8::StartupData {
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
    pub fn new_from_snapshot(data: &[u8]) -> FortunaIsolate {
        // let start = Instant::now();
        let isolate = FortunaIsolate::create_isolate(data.to_vec());
        // println!(
        //     "Time elapsed in to create isolate is: {:?}",
        //     start.elapsed()
        // );
        isolate
    }

    fn create_isolate(startup_data: Vec<u8>) -> FortunaIsolate {
        // let safe_obj: v8::PropertyAttribute = v8::DONT_DELETE + v8::DONT_ENUM + v8::READ_ONLY;

        let mut global_context = v8::Global::<v8::Context>::new();
        let create_params = v8::Isolate::create_params().snapshot_blob(startup_data);
        let mut isolate = v8::Isolate::new(create_params);

        let mut handle_scope = v8::HandleScope::new(&mut isolate);
        let scope = handle_scope.enter();

        let context = v8::Context::new(scope);

        // let mut cs = v8::ContextScope::new(scope, context);
        // let scope = cs.enter();
        //
        // // Add default map functions
        // let source = v8::String::new(scope, JS_CODE).unwrap();
        // let mut script = v8::Script::compile(scope, context, source, None).unwrap();
        // script.run(scope, context).unwrap();

        global_context.set(scope, context);

        FortunaIsolate {
            isolate,
            global_context,
        }
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
        // println!("result eval: {}", result_string);

        if result_string == "undefined" {
            return "null".to_string();
        }
        result_string
    }

    pub fn call(&mut self, raw_fun_name: &str, args: &[String]) -> String {

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
        // println!("result: {}", result_string);
        result_string
    }
}

pub fn init() {
    let platform = v8::new_default_platform().unwrap();
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
