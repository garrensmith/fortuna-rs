use std::env;
use std::fs;
use std::fs::read_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_js_src_file()?;
    tonic_build::compile_protos("proto/ateles.proto")?;
    Ok(())
}

// Load all the files from js/ and create a string with them to be added to the
// snapshot isolate
fn create_js_src_file() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("js_startup_code.rs");
    let js_codes = read_dir("./js")
        .unwrap()
        .filter(|entry| entry.as_ref().unwrap().path().is_file())
        .map(|file_entry| {
            let name = file_entry.unwrap().path();
            println!("reading from file {:?}", name);
            fs::read_to_string(&name).unwrap()
        })
        .collect::<Vec<String>>()
        .join("");

    let code = format!("pub const JS_CODE: &str = r#\"{}\"#;", js_codes);

    fs::write(dest_path, code).unwrap();

    Ok(())
}
