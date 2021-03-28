cd codegen_test
cargo asm codegen_test::index_many_mut_hardcoded_checked --no-color > checked.asm
cargo asm codegen_test::index_many_mut_hardcoded_unchecked --no-color > unchecked.asm
cargo asm codegen_test::index_many_mut_hardcoded_option --no-color > option.asm
cargo asm codegen_test::index_many_mut_hardcoded_presorted --no-color > presorted.asm
cargo asm codegen_test::index_many_mut_hardcoded_unsorted --no-color > unsorted.asm

cargo asm codegen_test::index_many_mut_hardcoded_checked_simple --no-color > checked_simple.asm
cargo asm codegen_test::index_many_mut_hardcoded_unchecked_simple --no-color > unchecked_simple.asm
cargo asm codegen_test::index_many_mut_hardcoded_option_simple --no-color > option_simple.asm
cargo asm --no-color > all.asm
