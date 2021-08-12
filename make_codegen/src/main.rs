#![feature(exit_status_error)]

use std::{path::Path, process::Output};

use tempfile::tempdir;

fn run_raw(s: &str, cwd: &Path, pipe: bool) -> Output {
    let args = s.split_whitespace().collect::<Vec<_>>();
    let mut cmd = std::process::Command::new(&args[0]);
    if pipe {
        cmd.stderr(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
    }
    cmd.args(&args[1..])
        .current_dir(cwd)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
}

fn clear_asm(p: &Path) {
    for p in std::fs::read_dir(p).unwrap() {
        let p = p.unwrap().path();
        if p.extension() == Some("asm".as_ref()) {
            std::fs::remove_file(p).unwrap();
        }
    }
}

fn main() {
    let output_dir = Path::new("./.codegen");
    let codegen_crate_name = "codegen_crate";

    std::fs::create_dir_all(&output_dir).unwrap();

    let tempdir = tempdir().unwrap();
    let tempdir = tempdir.path().join(codegen_crate_name);
    std::fs::create_dir_all(&tempdir).unwrap();

    println!("generated into temporary directory {}", tempdir.display());

    run_raw("cargo init --lib . --vcs none", &tempdir, false);

    // TODO: Escape path properly
    let crate_path = std::env::current_dir().unwrap();
    let crate_name = crate_path.file_name().unwrap().to_str().unwrap().to_owned();
    let crate_path = crate_path
        .as_os_str()
        .to_str()
        .unwrap()
        .escape_default()
        .to_string();

    let toml = format!(
        r##"
        [package]
        name = "{}"
        version = "0.1.0"
        edition = "2018"

        [dependencies]
        {} = {{ version = "*", path = "{}"}}
    "##,
        codegen_crate_name, crate_name, crate_path
    );

    std::fs::write(tempdir.join("Cargo.toml"), toml).unwrap();
    std::fs::write(tempdir.join("src").join("lib.rs"), "").unwrap();

    run_raw("cargo build --release", &tempdir, false);

    clear_asm(&output_dir);

    let header = codegen_test::HEADER;
    for &(i, name, source) in codegen_test::FUNCTIONS {
        println!("Check {}...", name);
        let code = format!("{}\n{}", header, source);
        std::fs::write(tempdir.join("src").join("lib.rs"), code).unwrap();
        run_raw("cargo fmt", &tempdir, false);

        let s = format!(r"cargo asm {}::{} --no-color", codegen_crate_name, name);
        let out = run_raw(&s, &tempdir, true);

        let ret = match out.status.exit_ok() {
            Ok(_) => String::from_utf8(out.stdout).unwrap(),
            Err(e) => {
                eprintln!("Error[{}] with `{}`", e.code().unwrap_or(0), s);
                eprintln!("{}", String::from_utf8(out.stderr).unwrap());
                "<error>".to_owned()
            }
        };

        std::fs::write(output_dir.join(format!("{}_{}.asm", i, name)), ret).unwrap();
    }

    println!();
    println!("DONE");
}
