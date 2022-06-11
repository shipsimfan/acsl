macro_rules! run_test {
    ($test_name:literal) => {
        let mut input_filepath = std::path::PathBuf::from($test_name);
        let mut target_filepath_hlsl = input_filepath.clone();
        let target_filepath_glsl_vertex =
            std::path::PathBuf::from(format!("{}_vertex.glsl", $test_name));
        let target_filepath_glsl_fragment =
            std::path::PathBuf::from(format!("{}_frag.glsl", $test_name));

        input_filepath.set_extension("acsl");
        target_filepath_hlsl.set_extension("hlsl");

        let input_code = std::fs::read_to_string(input_filepath).unwrap();
        let target_code_hlsl = std::fs::read_to_string(target_filepath_hlsl).unwrap();
        let target_code_glsl_vertex = std::fs::read_to_string(target_filepath_glsl_vertex).unwrap();
        let target_code_glsl_fragment =
            std::fs::read_to_string(target_filepath_glsl_fragment).unwrap();

        let compiled_code_hlsl = acsl::compile_hlsl(&input_code).unwrap();
        let (compiled_code_glsl_vertex, compiled_code_glsl_fragment) =
            acsl::compile_glsl(&input_code).unwrap();

        assert_eq!(compiled_code_hlsl, target_code_hlsl);
        assert_eq!(compiled_code_glsl_vertex, target_code_glsl_vertex);
        assert_eq!(compiled_code_glsl_fragment, target_code_glsl_fragment);
    };
}

#[test]
fn simple_1() {
    run_test!("tests/simple");
}
