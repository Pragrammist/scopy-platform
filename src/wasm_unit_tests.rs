mod del_this_tests {
    use wasmparser::validate;
    use wasmprinter::print_bytes;
    use crate::byte_maker::build_module;
    use crate::low_ir::convert_to_lowering_ir;
    use crate::scopy_ir::parse_modules;

    fn check_wasm(bytes: &[u8]) -> Result<(), String> {
        match validate(bytes) {
            Ok(_) => {
                println!("WASM valid");
                Ok(())
            }
            Err(e) => {
                eprintln!("WASM invalid: {e}");
                eprintln!("========== wasm dump ==========");
                match print_bytes(bytes) {
                    Ok(wat) => eprintln!("{wat}"),
                    Err(print_err) => eprintln!("[wasmprinter failed: {print_err}]"),
                }
                eprintln!("=================================");
                Err(e.to_string())
            }
        }
    }

    #[test]
    fn del_this_test() {
        let project = parse_modules();
        let d = convert_to_lowering_ir(&project);
        let wasm_bytes = build_module(d);
        let wasm_validate_res = check_wasm(wasm_bytes.as_slice());
        assert!(wasm_validate_res.is_ok());
    }
}