#![feature(exit_status_error)]

use std::path::Path;

fn run(s: &str) -> String {
    let args = s.split_whitespace().collect::<Vec<_>>();
    let out = std::process::Command::new(&args[0])
        .args(&args[1..])
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let ret = match out.status.exit_ok() {
        Ok(_) => String::from_utf8(out.stdout).unwrap(),
        Err(e) => {
            eprintln!("Error[{}] with `{}`", e.code().unwrap_or(0), s);
            //eprintln!("{}", String::from_utf8(out.stderr).unwrap());
            String::new()
        }
    };

    let p = ret.find("_ZN12codegen_test13").unwrap_or(ret.len());
    ret[..p].to_owned()
}

fn main() {
    let p = Path::new("./codegen_test");
    for p in std::fs::read_dir(p).unwrap() {
        let p = p.unwrap().path();
        if p.extension() == Some("asm".as_ref()) {
            std::fs::remove_file(p).unwrap();
        }
    }

    std::fs::write(p.join("all.asm"), run("cargo asm --no-color")).unwrap();

    for &(i, name) in codegen_test::FUNCTIONS {
        std::fs::write(
            p.join(format!("{}_{}.asm", i, name)),
            run(&format!(r"cargo asm codegen_test::{} --no-color", name)),
        )
        .unwrap();
    }

    println!();
    println!("DONE");
}
