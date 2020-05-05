use fortuna::*;
mod common;

#[test]
fn simple_evals() {
    // using common code.
    common::setup();

    let js_env = JSEnv::new();
    let mut instance = js_env.create_isolate();

    let script = "var x = 2; x;";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "2");

    let script = "var y = 3; y;";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "3");

    let script = "let my_fn = () => \"hello\"; my_fn();";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "\"hello\"");
}

#[test]
fn eval_and_call() {
    common::setup();

    let js_env = JSEnv::new();
    let mut instance = js_env.create_isolate();

    let script = "function double(x) {return x * 2;};";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "null");

    let call_result = instance.call("double", &["2".to_string()]);
    assert_eq!(call_result, "4");
}
