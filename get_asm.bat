cd codegen_test
del .\*.asm

cargo asm --no-color > all.asm

cargo asm codegen_test::checked_simple --no-color > 0_checked_simple.asm
cargo asm codegen_test::unchecked_simple --no-color > 0_unchecked_simple.asm
cargo asm codegen_test::option_simple --no-color > 1_option_simple.asm
cargo asm codegen_test::option_simple_unwrap --no-color > 0_option_simple_unwrap.asm
cargo asm codegen_test::checked --no-color > 0_checked.asm
cargo asm codegen_test::unchecked --no-color > 0_unchecked.asm
cargo asm codegen_test::option --no-color > 1_option.asm
cargo asm codegen_test::option_unwrap --no-color > 0_option_unwrap.asm
cargo asm codegen_test::result --no-color > 1_result.asm
cargo asm codegen_test::result_kind --no-color > 1_result_kind.asm
cargo asm codegen_test::result_option --no-color > 1_result_option.asm
cargo asm codegen_test::result_unwrap --no-color > 0_result_unwrap.asm
cargo asm codegen_test::presorted --no-color > 0_presorted.asm
cargo asm codegen_test::unsorted --no-color > 0_unsorted.asm
cargo asm codegen_test::checked_usize_trait --no-color > 0_checked_usize_trait.asm
cargo asm codegen_test::unchecked_usize_trait --no-color > 0_unchecked_usize_trait.asm
cargo asm codegen_test::option_usize_trait --no-color > 0_option_usize_trait.asm
cargo asm codegen_test::checked_range_trait --no-color > 0_checked_range_trait.asm
cargo asm codegen_test::unchecked_range_trait --no-color > 0_unchecked_range_trait.asm
cargo asm codegen_test::option_range_trait --no-color > 0_option_range_trait.asm
