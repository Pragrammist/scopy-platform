use std::fs;
use std::path::PathBuf;
use crate::low_ir::{convert_to_lowering_ir};
use crate::scopy_ir::{parse_modules};
use crate::semantic::{CodeModuleMetaData, CodeModuleSourceFileType, ObjectIdent, ScopyModule};
use wasmtime::{ Result};
use std::error::Error;
use std::io::Write;
use walkdir::{DirEntry, WalkDir};
use crate::byte_maker::{build_modules};

mod unit_tests;
mod low_ir;
mod semantic;
mod scopy_ir;
mod byte_maker;
mod wasm_unit_tests;
mod include_build_in_modules;




fn main() {
    let all_modules = collect_js_files();


    let project = parse_modules(all_modules);
    let d = convert_to_lowering_ir(&project);
    println!("lowered semantic: {:?}", d);
    let wasm_bytes = build_modules(d);
    create_executable(&wasm_bytes, "scopy_app").expect("");
}

const RUNNER_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/wasm-runner-bin"));
fn create_executable(wasm_bytes: &[u8], output_exe: &str) -> Result<(), Box<dyn Error>> {
    let out = if cfg!(windows) {
        format!("{output_exe}.exe")
    } else {
        output_exe.to_string()
    };

    let mut file = fs::File::create(&out)?;
    file.write_all(RUNNER_BIN)?;                       // сам раннер
    file.write_all(wasm_bytes)?;                       // wasm
    file.write_all(&(wasm_bytes.len() as u64).to_le_bytes())?;  // длина wasm

    Ok(())
}



//
// fn generate_wasm_bytes() -> Vec<u8> {
//     let mut module = Module::new();
//
//     // type 0: fd_write(i32, i32, i32, i32) -> i32
//     // type 1: main() -> ()
//     let mut types = TypeSection::new();
//     types.ty().function(
//         [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
//         [ValType::I32],
//     );
//     types.ty().function([], []);
//     module.section(&types);
//
//     // import "wasi_snapshot_preview1" "fd_write" -> func 0
//     let mut imports = ImportSection::new();
//     imports.import(
//         "wasi_snapshot_preview1",
//         "fd_write",
//         EntityType::Function(0),
//     );
//     module.section(&imports);
//
//     // func 1 = main, тип 1
//     let mut functions = FunctionSection::new();
//     functions.function(1);
//     module.section(&functions);
//
//     // memory 1 page
//     let mut memories = MemorySection::new();
//     memories.memory(MemoryType {
//         minimum: 1,
//         maximum: None,
//         memory64: false,
//         shared: false,
//         page_size_log2: None,
//     });
//     module.section(&memories);
//
//     // export "main" (func 1), "memory" (mem 0)
//     let mut exports = ExportSection::new();
//     exports.export("main", ExportKind::Func, 1);
//     exports.export("memory", ExportKind::Memory, 0);
//     module.section(&exports);
//
//
//     let mut codes = CodeSection::new();
//     let mut f = Function::new([]);
//     f.instruction(&Instruction::I32Const(1));
//     f.instruction(&Instruction::I32Const(32));
//     f.instruction(&Instruction::I32Const(1));
//     f.instruction(&Instruction::I32Const(40));
//     f.instruction(&Instruction::Call(0));
//     f.instruction(&Instruction::Drop);
//     f.instruction(&Instruction::End);
//     codes.function(&f);
//     module.section(&codes);
//
//
//     let mut data = DataSection::new();
//
//     let msg = b"Hello from wasm!\n";
//     data.active(0, &ConstExpr::i32_const(0), msg.iter().copied());
//
//     let mut iovec = Vec::new();
//     iovec.extend_from_slice(&0u32.to_le_bytes());
//     iovec.extend_from_slice(&(msg.len() as u32).to_le_bytes());
//     data.active(0, &ConstExpr::i32_const(32), iovec.iter().copied());
//
//     module.section(&data);
//
//     module.finish()
// }

fn is_ignored(entry: &DirEntry) -> bool {
    let ignored_dirs = [
        "target",
        "node_modules",
        ".git",
        "ir_output",
        "build_in_modules"
    ];

    entry.path()
        .components()
        .any(|c| {
            let name = c.as_os_str().to_str();

            ignored_dirs.contains(&name.unwrap_or(""))
        })
}






pub fn dump_ir_html(ir: &ScopyModule, name: ObjectIdent) {



    // 1. сериализация
    let json = serde_json::to_string(ir).unwrap_or(String::new());

    if json.is_empty() {
        panic!("cannot serialize module for dumping in html");
    }


    // 2. путь к шаблону (рядом с Cargo.toml)
    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("viewer.template.html");

    let template = fs::read_to_string(template_path)
        .expect("failed to read viewer.template.html");

    // 3. вставка JSON
    let html = template.replace("__JSON__", &json);

    // 4. запись результата
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("ir_output/{name}_ir.html"));

    let write_res = fs::write(out_path, html);

    if write_res.is_err(){
        panic!("failed to write ir.html");
    }

}



pub fn collect_js_files() -> Vec<CodeModuleMetaData> {
    let cur_dir = std::env::current_dir();

    let res = match cur_dir {
        Ok(cwd) => {
            let res = WalkDir::new(&cwd)
                .into_iter()
                .filter_entry(|e| !is_ignored(e))
                .filter_map(std::result::Result::ok)
                .filter(|entry| entry.path().is_file())
                .filter(|entry| {
                    entry.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        == Some("js")
                })
                .map(|entry| {

                    let path_result = entry
                        .path()
                        .strip_prefix(&cwd.as_path());


                    match path_result {
                        Ok(entry) => entry.to_path_buf(),
                        Err(err) => core::panic!("Error while trying parse path {}", err)
                    }
                })
                .map(|f| {


                    fn read_path_buf(f: &PathBuf) -> String {
                        let read_result = fs::read_to_string(&f);
                        let file_content = match read_result {
                            Ok(result) => result,
                            Err(err) => {
                                core::panic!("Error while fetching {:}", err)
                            }
                        };
                        file_content
                    }

                    fn get_file_name(f: &PathBuf) -> String {
                        let read_file_name = f.file_name();
                        match read_file_name {
                            None => {
                                core::panic!("Error while trying reading file name")
                            }
                            Some(file_name) => {
                                match file_name.to_str() {
                                    None => {
                                        core::panic!("Error while trying reading file name and trying to str it")
                                    }
                                    Some(file_name) => {
                                        file_name.replace(['/', '\\'], "_")
                                    }
                                }
                            }
                        }
                    }


                    CodeModuleMetaData{
                        module_meta_type: CodeModuleSourceFileType::External,
                        code: read_path_buf(&f),
                        name: get_file_name(&f),
                    }
                })
                .collect();
            res
        }
        Err(err) => core::panic!("Could not get current directory. {:?}", err),
    };

    res

}
