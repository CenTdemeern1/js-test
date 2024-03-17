use std::time::Duration;

fn discordify(s: String) -> String {
    s.replace("\u{001b}[96m", "\u{001b}[34m")
        .replace("\u{001b}[93m", "\u{001b}[33m")
        .replace("\u{001b}[91m", "\u{001b}[31m")
        .replace("\u{001b}[90m", "\u{001b}[30m")
        .replace("\u{001b}[7m", "\u{001b}[30;47m")
}

fn main() {
    let tsc_output = String::from_utf8(
        std::process::Command::new("tsc")
            .args(["--target", "esNext", "--pretty", "main.ts"])
            .output()
            .unwrap()
            .stdout, // Why is this on stdout and not stderr?
    )
    .unwrap();
    println!("{}", tsc_output);
    std::fs::write("tsc_output.log", discordify(tsc_output)).unwrap();
    let code = std::fs::read_to_string("main.js").unwrap();
    let code = code.trim().strip_suffix(";").unwrap_or(&code);
    let mut script = js_sandbox::Script::from_string(&format!("const a = (\n{}\n);", code))
        .unwrap()
        .with_timeout(Duration::from_secs(1));
    script.call::<_, ()>("a", ()).unwrap();
}
