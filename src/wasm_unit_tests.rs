mod validate_byte_code_tests {
    use wasmparser::validate;
    use wasmprinter::print_bytes;
    use crate::byte_maker::{build_modules};
    use crate::low_ir::convert_to_lowering_ir;
    use crate::scopy_ir::parse_modules;
    use crate::semantic::{CodeModuleMetaData, CodeModuleSourceFileType};

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
    fn dump_pipeline_memory() {
        let project = parse_modules(vec![test_module_meta_memory()]);
        let d = convert_to_lowering_ir(&project);
        let wasm_bytes = build_modules(d);

        dump_memory(&wasm_bytes).unwrap();
    }


    #[test]
    fn validate_test_wasm_func(){
        let project = parse_modules(vec![test_module_meta_func()]);
        let d = convert_to_lowering_ir(&project);
        println!("WASM valid: {:?}", d);
        let wasm_bytes = build_modules(d);
    }



    fn dump_memory(wasm_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        use wasmtime::{Config, Engine, Instance, Module, Store};

        let mut config = Config::new();
        config.wasm_multi_value(true);
        let engine = Engine::new(&config)?;

        let module = Module::new(&engine, wasm_bytes)?;
        let mut store = Store::new(&engine, ());

        let instance = Instance::new(&mut store, &module, &[])?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or("module does not export 'memory'")?;

        let data = memory.data(&store);

        // 8 байт в строке, 32 строки (256 байт)
        for (i, chunk) in data.chunks(8).take(32).enumerate() {
            let offset = i * 8;

            print!("{offset:08x}: ");
            for b in chunk {
                print!("{b:02x} ");
            }
            for _ in chunk.len()..8 {
                print!("   ");
            }

            print!(" |");
            for b in chunk {
                let c = if b.is_ascii_graphic() || *b == b' ' {
                    *b as char
                } else {
                    '.'
                };
                print!("{c}");
            }
            println!("|");
        }

        Ok(())
    }

    #[test]
    fn validate_test_wasm() {
        let project = parse_modules(vec![test_module_meta()]);
        let d = convert_to_lowering_ir(&project);
        let wasm_bytes = build_modules(d);
        let wasm_validate_res = check_wasm(wasm_bytes.as_slice());
        assert!(wasm_validate_res.is_ok());
    }

    fn test_module_meta() -> CodeModuleMetaData{
        CodeModuleMetaData{
            code:  r#"
            const va00 = null;

            const va0 = {
                va1 = {
                    va2 = "test"
                }
            };


            const va1 = {
                va21={
                    va31= null,
                },
                va22={
                    va31={
                        va41= null,
                        va42="test",
                        va43=false,
                        va45=123,
                        va46=va0.va1.va2
                    }
                }
            };

            va1.va22.va31;

            export { va1 }
        "#.to_string(),
            name: "main.js".to_string(),
            module_meta_type: CodeModuleSourceFileType::Internal,
        }
    }


    fn test_module_meta_memory() -> CodeModuleMetaData{
        CodeModuleMetaData{
            code:  r#"
            const va00 = null;
            const va0 = {
                va1 = "va1"
            }
            const va1 = {
                va21={
                    var33=va0,
                    va31="va31",
                    va32=null,
                },
                va22={
                    va31={
                        va41= null,
                        va42="va42",
                        va43=false,
                        va45=123
                    }
                }
            };

            export { va1 }


        "#.to_string(),
            name: "main.js".to_string(),
            module_meta_type: CodeModuleSourceFileType::Internal,
        }
    }


    fn test_module_meta_func() -> CodeModuleMetaData{
        CodeModuleMetaData{
            code:  r#"
            const va1 = (result={

            }) => {

            }

            const res = va1();

            export { va1 }


        "#.to_string(),
            name: "main.js".to_string(),
            module_meta_type: CodeModuleSourceFileType::Internal,
        }
    }
}