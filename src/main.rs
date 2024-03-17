use std::time::Duration;
/*
mod compiler;

const CODE: &'static str = r#"
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
"#;

fn main() {
    let code = compiler::ts_to_js("index.ts", CODE).0;
    println!("{}", code);
    let _script = js_sandbox::Script::from_string(&code).unwrap().with_timeout(Duration::from_secs(1));
}
*/

// const CODE: &'static str = r#"
// (() => console.log("hi"))//.toString()
// "#;
// let value = js_sandbox::eval_json(CODE).unwrap(); println!("{}", value);

// const CODE: &'static str = include_str!("main.js");

fn main() {
    String::from_utf8(std::process::Command::new("tsc").arg("main.ts").output().unwrap().stderr).unwrap();
    let code = std::fs::read_to_string("main.js").unwrap();
    let mut script = js_sandbox::Script::from_string(&format!("const a = (\n{}\n);", code)).unwrap().with_timeout(Duration::from_secs(1));
    script.call::<_, ()>("a", ()).unwrap();
}
