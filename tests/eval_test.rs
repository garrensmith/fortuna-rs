
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

    let script = "var x = 3; x;";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "3");

    let script = "let x = () => \"hello\"; x();";
    let result = instance.eval(script, &[]);
    assert_eq!(result, "hello")
}