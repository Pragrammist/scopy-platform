use std::fs;
use std::path::PathBuf;
use crate::low_ir::convert_to_lowering_ir;
use crate::scopy_ir::parse_modules;
use crate::semantic::{ObjectIdent, ScopyModule};
use wasm_encoder::{CodeSection, ConstExpr, DataSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, ImportSection, Instruction, MemorySection, MemoryType, Module, TypeSection, ValType};
use std::path::Path;
use std::process::Command;
use wasmtime::{Config, Engine, Instance, Linker, Module as WasmTimeModule, Result, Store};
use wasmtime_wasi::WasiCtxBuilder;
use std::error::Error;
use wasmtime_wasi::p1::{self, WasiP1Ctx};


mod unit_tests;
mod low_ir;
mod semantic;
mod scopy_ir;








// fn main() {
//     // let project = parse_modules();
//     // println!("semantic: {:#?}", project);
//     // let d = convert_to_lowering_ir(&project);
//     // // project.modules.into_iter().for_each(|module| {
//     // //     dump_ir_html(&module, module.name.clone());
//     // // })
//     // println!("lowered semantic: {:?}", d);
//
//     //TODO:
//     /*
//         1) убрать синтаксис мутаций
//         2) убрать ...
//         3) добавить поддержку ast дерево массивов как span
//         4) почекать код импортов экспортов
//         5) тесты на импорт экспорт
//     */
//
//
//     let wasm_bytes = generate_wasm_bytes();
//     println!("wasm size: {} bytes", wasm_bytes.len());
//
//     create_executable(&wasm_bytes, "add_app").expect("");
//     println!("executable created: add_app");
//
//
// }

fn main() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = generate_wasm_bytes();
    println!("wasm: {} bytes", wasm_bytes.len());

    create_executable(&wasm_bytes, "hello_app")?;
    println!("done: hello_app");

    Ok(())
}

fn create_executable(wasm_bytes: &[u8], output_exe: &str) -> Result<(), Box<dyn Error>> {
    let work_dir = "target/runner";
    fs::create_dir_all(format!("{work_dir}/src"))?;

    // 1. кладём .wasm рядом с будущим раннером
    fs::write(format!("{work_dir}/module.wasm"), wasm_bytes)?;

    // 2. генерируем Cargo.toml раннера
    let cargo_toml = r#"
[package]
name = "wasm-runner"
version = "0.1.0"
edition = "2021"

[dependencies]
wasmtime = { version = "=48.0.2", features = ["cranelift"] }
wasmtime-wasi = "=48.0.2"

[[bin]]
name = "wasm-runner"
path = "src/main.rs"
"#;
    fs::write(format!("{work_dir}/Cargo.toml"), cargo_toml)?;

    // 3. генерируем код раннера
    let main_rs = r#"
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_multi_value(true);
    let engine = Engine::new(&config)?;

    // путь относительно src/main.rs -> корень крейта
    let wasm_bytes = include_bytes!("../module.wasm");
    let module = Module::new(&engine, wasm_bytes)?;

    let wasi: WasiP1Ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build_p1();

    let mut store = Store::new(&engine, wasi);
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    p1::add_to_linker_sync(&mut linker, |cx| cx)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let main_fn = instance.get_typed_func::<(), ()>(&mut store, "main")?;
    main_fn.call(&mut store, ())?;

    Ok(())
}
"#;
    fs::write(format!("{work_dir}/src/main.rs"), main_rs)?;

    // 4. собираем раннер
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(work_dir)
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    // 5. копируем бинарник туда, куда попросили
    let built = if cfg!(windows) {
        format!("{work_dir}/target/release/wasm-runner.exe")
    } else {
        format!("{work_dir}/target/release/wasm-runner")
    };
    let out = if cfg!(windows) {
        format!("{output_exe}.exe")
    } else {
        output_exe.to_string()
    };
    fs::copy(&built, &out)?;

    Ok(())
}




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
