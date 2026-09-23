use std::fs;
use std::path::PathBuf;
use crate::low_ir::{convert_to_lowering_ir, OperationUnit};
use crate::scopy_ir::parse_modules;
use crate::semantic::{ObjectIdent, ScopyModule};
use wasm_encoder::{CodeSection, ConstExpr, DataSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, ImportSection, Instruction, MemorySection, MemoryType, Module, TypeSection, ValType};
use std::path::Path;
use std::process::Command;
use wasmtime::{Config, Engine, Instance, Linker, Module as WasmTimeModule, Result, Store};
use wasmtime_wasi::WasiCtxBuilder;
use std::error::Error;
use std::io::Write;
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use crate::byte_maker::build_module;

mod unit_tests;
mod low_ir;
mod semantic;
mod scopy_ir;
mod byte_maker;
pub mod wasm_unit_tests;

fn main() {
    let project = parse_modules();
    // println!("semantic: {:#?}", project);
    let d = convert_to_lowering_ir(&project);
    // project.modules.into_iter().for_each(|module| {
    //     dump_ir_html(&module, module.name.clone());
    // });
    println!("lowered semantic: {:?}", d);
    //TODO:
    /*
        1) убрать синтаксис мутаций и сделать как апи в анализаторах и не более того
        2) убрать ...
        3) добавить поддержку ast дерево массивов как span
        4) почекать код импортов экспортов
        5) тесты на импорт экспорт
    */
    let wasm_bytes = build_module(d);
    



    // let wasm_bytes = generate_wasm_bytes();
    println!("wasm size: {} bytes", wasm_bytes.len());

    create_executable(&wasm_bytes, "add_app").expect("");
    println!("executable created: add_app");


}

// fn main() -> Result<(), Box<dyn Error>> {
//     let wasm_bytes = generate_wasm_bytes();
//     println!("wasm: {} bytes", wasm_bytes.len());
//
//     create_executable(&wasm_bytes, "hello_app")?;
//     println!("done: hello_app");
//
//     Ok(())
// }
//
// fn create_executable(wasm_bytes: &[u8], output_exe: &str) -> Result<(), Box<dyn Error>> {
//     let template_dir = "runner";
//     let work_dir = "build/runner";
//
//     // 1. чистим и копируем шаблон
//     let _ = fs::remove_dir_all(work_dir);
//     copy_dir(template_dir, work_dir)?;
//
//     // 2. подкладываем .wasm
//     fs::write(format!("{work_dir}/module.wasm"), wasm_bytes)?;
//
//     // 3. собираем
//     let status = Command::new("cargo")
//         .args(["build", "--release"])
//         .current_dir(work_dir)
//         .status()?;
//     if !status.success() {
//         return Err("cargo build failed".into());
//     }
//
//     // 4. копируем бинарник
//     let built = if cfg!(windows) {
//         format!("{work_dir}/target/release/wasm-runner.exe")
//     } else {
//         format!("{work_dir}/target/release/wasm-runner")
//     };
//     let out = if cfg!(windows) {
//         format!("{output_exe}.exe")
//     } else {
//         output_exe.to_string()
//     };
//     fs::copy(&built, &out)?;
//
//     Ok(())
// }

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

// fn append_wasm_to_exe(path: &str, wasm_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
//     let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
//     file.write_all(wasm_bytes)?;
//     file.write_all(&(wasm_bytes.len() as u64).to_le_bytes())?;
//     Ok(())
// }
//
// fn copy_dir(from: &str, to: &str) -> std::io::Result<()> {
//     fs::create_dir_all(to)?;
//     for entry in fs::read_dir(from)? {
//         let entry = entry?;
//         let name = entry.file_name();
//         let name_str = name.to_string_lossy();
//
//         // не тащим сборочный мусор
//         if name_str == "target" || name_str == ".git" {
//             continue;
//         }
//
//         let src = entry.path();
//         let dst = Path::new(to).join(&name);
//
//         if src.is_dir() {
//             copy_dir(src.to_str().unwrap(), dst.to_str().unwrap())?;
//         } else {
//             fs::copy(&src, &dst)?;
//         }
//     }
//     Ok(())
// }




fn generate_wasm_bytes() -> Vec<u8> {
    let mut module = Module::new();

    // type 0: fd_write(i32, i32, i32, i32) -> i32
    // type 1: main() -> ()
    let mut types = TypeSection::new();
    types.ty().function(
        [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
        [ValType::I32],
    );
    types.ty().function([], []);
    module.section(&types);

    // import "wasi_snapshot_preview1" "fd_write" -> func 0
    let mut imports = ImportSection::new();
    imports.import(
        "wasi_snapshot_preview1",
        "fd_write",
        EntityType::Function(0),
    );
    module.section(&imports);

    // func 1 = main, тип 1
    let mut functions = FunctionSection::new();
    functions.function(1);
    module.section(&functions);

    // memory 1 page
    let mut memories = MemorySection::new();
    memories.memory(MemoryType {
        minimum: 1,
        maximum: None,
        memory64: false,
        shared: false,
        page_size_log2: None,
    });
    module.section(&memories);

    // export "main" (func 1), "memory" (mem 0)
    let mut exports = ExportSection::new();
    exports.export("main", ExportKind::Func, 1);
    exports.export("memory", ExportKind::Memory, 0);
    module.section(&exports);

    // ─── CODE (до DATA!) ───
    // body main:
    //   i32.const 1    ; fd = 1 (stdout)
    //   i32.const 32   ; iovs ptr
    //   i32.const 1    ; iovs_len
    //   i32.const 40   ; nwritten ptr
    //   call 0         ; fd_write
    //   drop
    //   end
    let mut codes = CodeSection::new();
    let mut f = Function::new([]);
    f.instruction(&Instruction::I32Const(1));
    f.instruction(&Instruction::I32Const(32));
    f.instruction(&Instruction::I32Const(1));
    f.instruction(&Instruction::I32Const(40));
    f.instruction(&Instruction::Call(0));
    f.instruction(&Instruction::Drop);
    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    // ─── DATA (после CODE) ───
    //   offset 0  — сама строка
    //   offset 32 — iovec { buf: u32, len: u32 }
    //   offset 40 — nwritten
    let mut data = DataSection::new();

    let msg = b"Hello from wasm!\n";
    data.active(0, &ConstExpr::i32_const(0), msg.iter().copied());

    let mut iovec = Vec::new();
    iovec.extend_from_slice(&0u32.to_le_bytes());
    iovec.extend_from_slice(&(msg.len() as u32).to_le_bytes());
    data.active(0, &ConstExpr::i32_const(32), iovec.iter().copied());

    module.section(&data);

    module.finish()
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
