macro_rules! run_test {
    ($test_name:literal) => {
        let mut input_filepath = std::path::PathBuf::from($test_name);
        let mut target_filepath = input_filepath.clone();

        input_filepath.set_extension("acsl");
        target_filepath.set_extension("hlsl");

        let input_code = std::fs::read_to_string(input_filepath).unwrap();
        let target_code = std::fs::read_to_string(target_filepath).unwrap();

        let compiled_code = acsl::compile_hlsl(input_code).unwrap();

        assert_eq!(compiled_code, target_code);
    };
}

#[test]
fn simple_1() {
    run_test!("tests/simple");
}
