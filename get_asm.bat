cd codegen_test
del .\*.asm

cargo asm --no-color > all.asm

cargo asm codegen_test::checked --no-color > 1_checked.asm
cargo asm codegen_test::unchecked --no-color > 2_unchecked.asm
cargo asm codegen_test::option --no-color > 3_option.asm
cargo asm codegen_test::presorted --no-color > 4_presorted.asm
cargo asm codegen_test::unsorted --no-color > 5_unsorted.asm
cargo asm codegen_test::option_unwrap() --no-color > 6_option_unwrap.asm

cargo asm codegen_test::checked_simple --no-color > 1_checked_simple.asm
cargo asm codegen_test::unchecked_simple --no-color > 2_unchecked_simple.asm
cargo asm codegen_test::option_simple --no-color > 3_option_simple.asm
cargo asm codegen_test::option_simple_unwrap() --no-color > 6_option_simple_unwrap.asm

cargo asm codegen_test::checked_usize_trait --no-color > 1_checked_usize_trait.asm
cargo asm codegen_test::unchecked_usize_trait --no-color > 2_unchecked_usize_trait.asm
cargo asm codegen_test::option_usize_trait --no-color > 3_option_usize_trait.asm

cargo asm codegen_test::checked_range_trait --no-color > 1_checked_range_trait.asm
cargo asm codegen_test::unchecked_range_trait --no-color > 2_unchecked_range_trait.asm
cargo asm codegen_test::option_range_trait --no-color > 3_option_range_trait.asm

cargo asm codegen_test::result() --no-color > 3_result.asm
cargo asm codegen_test::result_unwrap() --no-color > 6_result_unwrap.asm
cargo asm codegen_test::result_kind() --no-color > 7_result_kind.asm
cargo asm codegen_test::result_option() --no-color > 8_result_option.asm
