#![feature(exit_status_error)]

use std::{collections::HashMap, path::Path, process::Output};

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
    let codegen_crate_name = "codegen_crate";
    let doc_assembly_path = Path::new("_doc_assembly.rs");

    let tempdir = tempdir().unwrap();
    let tempdir = tempdir.path().join(codegen_crate_name);
    std::fs::create_dir_all(&tempdir).unwrap();
    println!("generated into temporary directory {}", tempdir.display());

    run_raw("cargo init --lib . --vcs none", &tempdir, false);
    let crate_path = std::env::current_dir().unwrap();
    let crate_name = crate_path.file_name().unwrap().to_str().unwrap().to_owned();
    let crate_path_str = crate_path
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
        codegen_crate_name, crate_name, crate_path_str
    );
    std::fs::write(tempdir.join("Cargo.toml"), toml).unwrap();
    std::fs::write(tempdir.join("src").join("lib.rs"), "").unwrap();
    run_raw("cargo build --release", &tempdir, false);

    let output_dir = Path::new("./.codegen");
    std::fs::create_dir_all(&output_dir).unwrap();
    clear_asm(&output_dir);

    let header = codegen_test::HEADER;

    let mut doc_module = format!(
        r##"
    //! This module contains example functions with the generated assembly in
    //! their docs.

    use crate::*;

    {}

    "##,
        header
    );

    let mut duplicates = HashMap::new();

    for (gi, &(i, name, source, body)) in codegen_test::FUNCTIONS.iter().enumerate() {
        println!("Check {}...", name);
        let code = format!("use {}::*;\n{}\n{}", crate_name, header, source);
        std::fs::write(tempdir.join("src").join("lib.rs"), code).unwrap();
        run_raw("cargo fmt", &tempdir, false);

        let s = format!(r"cargo asm {}::{} --no-color", codegen_crate_name, name);
        let out = run_raw(&s, &tempdir, true);

        let asm = match out.status.exit_ok() {
            Ok(_) => {
                let asm = String::from_utf8(out.stdout).unwrap();

                let v: &mut Vec<_> = duplicates.entry(asm.replace(name, "<name>")).or_default();
                v.push((gi, i, name));

                doc_module.push_str(&format!(
                    r##"
                    /// Body: `{}`
                    ///
                    /// # Assembly (x86_64)
                    /// ```x86asm
                    {}
                    /// ```
                    {}
                    "##,
                    body.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" "),
                    asm.lines()
                        .map(|l| format!("/// {}\n", l))
                        .collect::<String>()
                        .trim(),
                    source
                ));

                asm
            }
            Err(e) => {
                eprintln!("Error[{}] with `{}`", e.code().unwrap_or(0), s);
                eprintln!("{}", String::from_utf8(out.stderr).unwrap());
                "<error>".to_owned()
            }
        };

        std::fs::write(output_dir.join(format!("{}_{}.asm", i, name)), asm).unwrap();
    }

    let mut duplicates_vec = Vec::new();
    for v in duplicates.values() {
        if v.len() > 1 {
            duplicates_vec.push(v);
        }
    }
    duplicates_vec.sort_by_key(|v| v[0].0);

    println!();
    println!("Duplicates:");
    for v in duplicates_vec {
        for (_gi, i, e) in v {
            println!("  {} {}", i, e);
        }
        println!();
    }

    println!("Writing to docs of crate...");
    let doc_module = format!("{}\n", doc_module.trim());
    std::fs::write(crate_path.join("src").join(doc_assembly_path), doc_module).unwrap();
    run_raw("cargo fmt", &crate_path, false);

    println!();
    println!("DONE");
}
