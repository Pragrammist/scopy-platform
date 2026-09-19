use serde::Serialize;
use wasm_encoder::{CodeSection, ConstExpr, CustomSection, DataSection, ElementSection, EntityType, ExportKind, ExportSection, Function, FunctionSection, GlobalSection, ImportSection, Instruction, MemArg, MemorySection, MemoryType, Module, StartSection, TableSection, TypeSection, ValType};
use crate::low_ir::{ChainedTag, ChainedTagIdent, OperationExpr, OperationUnit, OwnerUnitId};

fn make_bytes(operations: Vec<OperationUnit>) -> Vec<u8> {
    operations.into_iter().map(|op| {
        todo!()
    }).collect()
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
enum ByteMakingException {
    NoFunction
}

impl std::fmt::Display for ByteMakingException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ByteMakingException::NoFunction => write!(f, "No function at WASM module."),
        }
    }
}

macro_rules! compiler_panic {
    ($reason:expr) => {{
        let reason = $reason;
        eprintln!("Compiler panic: {reason}");
        std::panic::panic_any(reason);
    }};
}


fn add_first_byte_init(func: &mut Function, id: OwnerUnitId){
    func.instruction(&Instruction::I32Const(id as i32));
    func.instruction(&Instruction::I64Const(0i64));
    func.instruction(&Instruction::I64Store(MemArg {
        offset: 0,
        align: 3,           // выравнивание 8 байт — log2(8)=3
        memory_index: 0,
    }));
}

fn make_chained_tag(tag: ChainedTag, ctx: ByteMakerCurrentContext){
    match tag {
        ChainedTag::CallChainStart => {}
        ChainedTag::CallChainEnd => {}
        ChainedTag::Ident(id) => {
            let functions = &mut ctx.sections.code.functions.clone();
            let func = &mut functions.pop().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction)).clone();


            match id {
                ChainedTagIdent::Object(id) => {
                    func.instruction(&Instruction::I32Const(id as i32));
                }
                ChainedTagIdent::Function(id) => {}
                ChainedTagIdent::ModuleId(id) => {}
                ChainedTagIdent::FuncResult(id) => {}
                ChainedTagIdent::Property(id) => {
                    func.instruction(&Instruction::I32Const(id as i32));
                }
                ChainedTagIdent::FuncArg(id) => {}
                ChainedTagIdent::AnotherObject(id) => {

                }
                ChainedTagIdent::FunctionCall(id) => {}
            }
        }
        ChainedTag::FuncInit => {}
        ChainedTag::BinOperation(_) => {}
        ChainedTag::LitOperation(lit_operation) => {

        }
        ChainedTag::IfScope => {}
        ChainedTag::LoopScope => {}
        ChainedTag::ElseScope => {}
        ChainedTag::StartScope => {}
        ChainedTag::EndScope => {}
        ChainedTag::StartCond => {}
        ChainedTag::EndCond => {}
        ChainedTag::EmptyScope => {}
        ChainedTag::OwnerIdMap(map) => {}
    }
}

// ─────────────────────────────────────────────
// СЕКЦИИ
// ─────────────────────────────────────────────

// type 0: fd_write(i32, i32, i32, i32) -> i32
// type 1: main() -> ()
fn build_types() -> TypeSection {
    let mut types = TypeSection::new();

    // types.ty(). function(
    //     [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
    //     [ValType::I32],
    // );
    // types.ty().function([], []);

    types
}

// import "wasi_snapshot_preview1" "fd_write" -> func 0
fn build_imports() -> ImportSection {
    let mut imports = ImportSection::new();

    // imports.import(
    //     "wasi_snapshot_preview1",
    //     "fd_write",
    //     EntityType::Function(0),
    // );

    imports
}

// func 1 = main, тип 1
fn build_functions() -> FunctionSection {
    let mut functions = FunctionSection::new();
    //
    // functions.function(1);

    functions
}

// memory 1 page
fn build_memory() -> MemorySection {
    let mut memories = MemorySection::new();

    // memories.memory(MemoryType {
    //     minimum: 1,
    //     maximum: None,
    //     memory64: false,
    //     shared: false,
    //     page_size_log2: None,
    // });

    memories
}

// export "main" (func 1), "memory" (mem 0)
fn build_exports() -> ExportSection {
    let mut exports = ExportSection::new();
    //
    // exports.export("main", ExportKind::Func, 1);
    // exports.export("memory", ExportKind::Memory, 0);

    exports
}

// code: тело функции main
//
//   i32.const 1    ; fd = 1 (stdout)
//   i32.const 32   ; iovs ptr
//   i32.const 1    ; iovs_len
//   i32.const 40   ; nwritten ptr
//   call 0         ; fd_write
//   drop
//   end
fn build_code() -> CodeSection {
    let mut codes = CodeSection::new();
    // let mut f = Function::new([]);
    //
    // f.instruction(&Instruction::I32Const(1));
    // f.instruction(&Instruction::I32Const(32));
    // f.instruction(&Instruction::I32Const(1));
    // f.instruction(&Instruction::I32Const(40));
    // f.instruction(&Instruction::Call(0));
    // f.instruction(&Instruction::Drop);
    // f.instruction(&Instruction::End);
    //
    // codes.function(&f);

    codes
}

// data:
//   offset 0  — строка
//   offset 32 — iovec { buf: u32, len: u32 }
fn build_data() -> DataSection {
    let mut data = DataSection::new();

    // let msg = b"Hello from wasm!\n";
    // data.active(0, &ConstExpr::i32_const(0), msg.iter().copied());
    //
    //
    // let mut iovec = Vec::new();
    // iovec.extend_from_slice(&0u32.to_le_bytes());
    // iovec.extend_from_slice(&(msg.len() as u32).to_le_bytes());
    // data.active(0, &ConstExpr::i32_const(32), iovec.iter().copied());

    data
}

// ─────────────────────────────────────────────
// СБОРКА
// ─────────────────────────────────────────────

// Порядок секций в wasm фиксирован спецификацией:
// type → import → function → memory → export → code → data



struct WasmSections {
    pub types: TypeSection,
    pub imports: ImportSection,
    pub functions: FunctionSection,
    pub tables: TableSection,
    pub memories: MemorySection,
    pub globals: GlobalSection,
    pub exports: ExportSection,
    pub start: Option<StartSection>,
    pub elements: ElementSection,
    pub code: ByteMakerCodeSection,
    pub data: DataSection,
}

struct ByteMakerCodeSection{
    pub functions: Vec<Function>,
}

struct ByteMakerCurrentContext{
    pub sections: WasmSections,
    pub current_tags: Vec<ChainedTag>,
}



fn build_module(sections: WasmSections) -> Vec<u8> {
    let mut module = Module::new();

    module.section(&sections.types);
    module.section(&sections.imports);
    module.section(&sections.functions);
    module.section(&sections.tables);
    module.section(&sections.memories);
    module.section(&sections.globals);
    module.section(&sections.exports);

    if let Some(start) = &sections.start {
        module.section(start);
    }

    module.section(&sections.elements);
    let code_section = &mut  CodeSection::new();
    sections.code.functions.iter().for_each(|f| {
        code_section.function(f);
    });
    module.section(code_section);
    module.section(&sections.data);

    module.finish()
}