fn main() {
    use std::fmt::Write;

    let mut s = String::new();

    writeln!(s, r"cd codegen_test").unwrap();
    writeln!(s, r"del .\*.asm").unwrap();
    writeln!(s).unwrap();
    writeln!(s, r"cargo asm --no-color > all.asm").unwrap();
    writeln!(s).unwrap();

    for &(i, name) in codegen_test::FUNCTIONS {
        writeln!(
            s,
            r"cargo asm codegen_test::{1}() --no-color > {0}_{1}.asm",
            i, name
        )
        .unwrap();
    }

    std::fs::write("./get_asm_2.bat", s).unwrap();
}
