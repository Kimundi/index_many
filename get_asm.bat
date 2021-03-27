cd codegen_test
cargo asm codegen_test::index_many_mut_hardcoded --no-color > checked.asm
cargo asm codegen_test::index_many_mut_hardcoded_unchecked --no-color > unchecked.asm
cargo asm codegen_test::index_many_mut_hardcoded_sorted --no-color > sorted.asm
