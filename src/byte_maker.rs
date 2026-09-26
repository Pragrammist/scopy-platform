use std::collections::{BTreeMap};
use serde::Serialize;
use wasm_encoder::{CodeSection, DataSection, ElementSection, ExportKind, ExportSection, Function, FunctionSection, GlobalSection, ImportSection, Instruction, MemArg, MemorySection, MemoryType, Module, StartSection, TableSection, TypeSection, ValType};
use crate::low_ir::{OperationUnit, ObjectIdentFull, ModuleWithOperations, ObjectIdent};


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
enum ByteMakingException {
    NoFunction,
    CannotUnwrapVecU8AsI64,
    ObjectIdentNotFound,
}



const BIT_64_SIZE: i32 = 8;

impl std::fmt::Display for ByteMakingException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ByteMakingException::NoFunction => write!(f, "No function at WASM module."),
            ByteMakingException::CannotUnwrapVecU8AsI64 => write!(f, "Can't unwrap on bool literal."),
            ByteMakingException::ObjectIdentNotFound => write!(f, "Object ident not found."),
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




fn make_operation_unit(tag: OperationUnit, ctx: &mut ByteMakerCurrentContext){
    match tag {
        OperationUnit::CallChainStart => {}
        OperationUnit::CallChainEnd => {}
        OperationUnit::Function(function) => {

        }
        OperationUnit::FuncResult(_) => {}
        OperationUnit::FuncArg(_) => {}
        OperationUnit::AnotherObjectVal(another_obj) => {
            let ident_to_write = another_obj.data;
            let ident_where_write = another_obj.ident;
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(ident_where_write.clone(),  ctx.cur_mem_index);
            println!("ident_map: {:?}", ctx.ident_map.clone());
            let ident_index = find_in_ident_map(&ctx.ident_map, &ident_to_write);

            func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
            func.instruction(&Instruction::I64Const(ident_index));
            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,        // log2(8) = 3, выровнено на 8 байт
                memory_index: 0,
            }));
            ctx.cur_mem_index += BIT_64_SIZE;
        }
        OperationUnit::FunctionCall(_) => {}
        OperationUnit::FuncParam => {}
        OperationUnit::FuncInit => {

        }
        OperationUnit::Str(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));

            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);


            val.data
                .into_iter()
                .map(|x| x as i32)
                .for_each(|i32_val| {
                    func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
                    func.instruction(&Instruction::I32Const(i32_val));
                    func.instruction(&Instruction::I32Store8(MemArg {
                        offset: 0,
                        align: 0,        // log2(8) = 3, выровнено на 8 байт
                        memory_index: 0,
                    }));
                    ctx.cur_mem_index += 1;
                });

        }
        OperationUnit::Bool(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);
            func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
            func.instruction(&Instruction::I64Const(i64::from_le_bytes(val.data.try_into().unwrap_or_else(|_| compiler_panic!(ByteMakingException::CannotUnwrapVecU8AsI64)))));
            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,        // log2(8) = 3, выровнено на 8 байт
                memory_index: 0,
            }));
            ctx.cur_mem_index += BIT_64_SIZE;
        }
        OperationUnit::Null(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);
            func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
            func.instruction(&Instruction::I64Const(i64::from_le_bytes(val.data.try_into().unwrap_or_else(|_| compiler_panic!(ByteMakingException::CannotUnwrapVecU8AsI64)))));
            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,        // log2(8) = 3, выровнено на 8 байт
                memory_index: 0,
            }));
            ctx.cur_mem_index += BIT_64_SIZE;
        }
        OperationUnit::Num(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);
            func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
            func.instruction(&Instruction::I64Const(i64::from_le_bytes(val.data.try_into().unwrap_or_else(|_| compiler_panic!(ByteMakingException::CannotUnwrapVecU8AsI64)))));
            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,        // log2(8) = 3, выровнено на 8 байт
                memory_index: 0,
            }));
            ctx.cur_mem_index += BIT_64_SIZE;
        },
        OperationUnit::IfScope => {}
        OperationUnit::LoopScope => {}
        OperationUnit::ElseScope => {}
        OperationUnit::StartScope => {}
        OperationUnit::EndScope => {}
        OperationUnit::StartCond => {}
        OperationUnit::EndCond => {}
        OperationUnit::EmptyScope => {}
        OperationUnit::EqEq => {}
        OperationUnit::NotEq => {}
        OperationUnit::Lt => {}
        OperationUnit::LtEq => {}
        OperationUnit::Gt => {}
        OperationUnit::GtEq => {}
        OperationUnit::Add => {}
        OperationUnit::Sub => {}
        OperationUnit::Mul => {}
        OperationUnit::Div => {}
        OperationUnit::Mod => {}
        OperationUnit::LogicalOr => {}
        OperationUnit::LogicalAnd => {}
        OperationUnit::NullishCoalescing => {}
        OperationUnit::StartModule => {
            ctx.sections.memories.memory.push(
                MemoryType {
                    minimum: 1,
                    maximum: None,
                    memory64: false,
                    shared: false,
                    page_size_log2: None,
                }
            );
            ctx.sections.types.functions.push((vec![], vec![]));
            ctx.sections.functions.functions.push(ctx.cur_func_index);
            ctx.sections.code.functions.push(Function::new([]));
        }
        OperationUnit::EndModule => {
            let func =  ctx.sections.code.functions.first_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            func.instruction(&Instruction::End);
        }
        OperationUnit::AnotherObject(obj) => {

        }
        OperationUnit::Export(_) => {
            ctx.sections.exports.exports.push(ExportUnit{
                kind: ExportKindByteMaker::Memory,
                ident: "memory".to_string(),
                index: 0
            })
        }
        OperationUnit::Import(_) => {}
    }
}


fn find_in_ident_map(
    ident_map: &BTreeMap<ObjectIdentFull, i32>,
    query: &ObjectIdentFull,
) -> i64 {




    ident_map.iter()
        .filter(|(path, _)| {
            let query_len = query.len();

            let path = path.iter()
                .take(query_len)
                .map(|m| m.clone())
                .collect::<Vec<_>>() as ObjectIdentFull;

            path == *query
        })
        .min_by_key(|(_, index)| *index)
        .map(|(_, v)| *v).unwrap_or_else(|| compiler_panic!(ByteMakingException::ObjectIdentNotFound)) as i64
}


// ─────────────────────────────────────────────
// СЕКЦИИ
// ─────────────────────────────────────────────

// type 0: fd_write(i32, i32, i32, i32) -> i32
// type 1: main() -> ()
// fn build_types() -> TypeSection {
//     let mut types = TypeSection::new();
//
//     // types.ty(). function(
//     //     [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
//     //     [ValType::I32],
//     // );
//     // types.ty().function([], []);
//
//     types
// }

// import "wasi_snapshot_preview1" "fd_write" -> func 0
// fn build_imports() -> ImportSection {
//     let mut imports = ImportSection::new();
//
//     // imports.import(
//     //     "wasi_snapshot_preview1",
//     //     "fd_write",
//     //     EntityType::Function(0),
//     // );
//
//     imports
// }

// func 1 = main, тип 1
// fn build_functions() -> FunctionSection {
//     let mut functions = FunctionSection::new();
//     //
//     // functions.function(1);
//
//     functions
// }

// memory 1 page
// fn build_memory() -> MemorySection {
//     let mut memories = MemorySection::new();
//
//     // memories.memory(MemoryType {
//     //     minimum: 1,
//     //     maximum: None,
//     //     memory64: false,
//     //     shared: false,
//     //     page_size_log2: None,
//     // });
//
//     memories
// }

// export "main" (func 1), "memory" (mem 0)
// fn build_exports() -> ExportSection {
//     let mut exports = ExportSection::new();
//     //
//     // exports.export("main", ExportKind::Func, 1);
//     // exports.export("memory", ExportKind::Memory, 0);
//
//     exports
// }

// code: тело функции main
//
//   i32.const 1    ; fd = 1 (stdout)
//   i32.const 32   ; iovs ptr
//   i32.const 1    ; iovs_len
//   i32.const 40   ; nwritten ptr
//   call 0         ; fd_write
//   drop
//   end
// fn build_code() -> CodeSection {
//     let mut codes = CodeSection::new();
//     // let mut f = Function::new([]);
//     //
//     // f.instruction(&Instruction::I32Const(1));
//     // f.instruction(&Instruction::I32Const(32));
//     // f.instruction(&Instruction::I32Const(1));
//     // f.instruction(&Instruction::I32Const(40));
//     // f.instruction(&Instruction::Call(0));
//     // f.instruction(&Instruction::Drop);
//     // f.instruction(&Instruction::End);
//     //
//     // codes.function(&f);
//
//     codes
// }

// data:
//   offset 0  — строка
//   offset 32 — iovec { buf: u32, len: u32 }
// fn build_data() -> DataSection {
//     let mut data = DataSection::new();
//
//     let msg = b"Hello from wasm!\n";
//     data.active(0, &ConstExpr::i32_const(0), msg.iter().copied());
//
//
//     let mut iovec = Vec::new();
//     iovec.extend_from_slice(&0u32.to_le_bytes());
//     iovec.extend_from_slice(&(msg.len() as u32).to_le_bytes());
//     data.active(0, &ConstExpr::i32_const(32), iovec.iter().copied());
//
//     data
// }

// ─────────────────────────────────────────────
// СБОРКА
// ─────────────────────────────────────────────

// Порядок секций в wasm фиксирован спецификацией:
// type → import → function → memory → export → code → data


#[derive(Clone, Debug, Default)]
#[allow(unused)]
struct WasmSections {
    pub types: ByteMakerTypeSection,
    pub imports: ByteMakerImportSection,
    pub functions: ByteMakerFunctionSection,
    pub tables: ByteMakerTableSection,
    pub memories: ByteMakerMemorySection,
    pub globals: ByteMakerGlobalSection,
    pub exports: ByteMakerExportSection,
    pub start: Option<ByteMakerStartSection>,
    pub elements: ByteMakerElementSection,
    pub code: ByteMakerCodeSection,
    pub data: ByteMakerDataSection,
}




#[derive(Clone, Debug, Default)]
pub struct ByteMakerTypeSection {
    pub functions: Vec<(Vec<ValType>, Vec<ValType>)>,
}



#[derive(Clone, Debug,  Default)]
pub struct ByteMakerImportSection {

}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerFunctionSection {
     pub functions: Vec<u32>
}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerTableSection {

}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerMemorySection {
    pub memory: Vec<MemoryType>
}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerGlobalSection {

}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerExportSection {
    pub exports: Vec<ExportUnit>
}

#[derive(Clone, Debug, Default)]
pub struct ExportUnit{
    pub ident: String,
    pub kind: ExportKindByteMaker,
    pub index: u32,
}

#[derive(Clone, Debug, Default)]
pub enum ExportKindByteMaker {
    /// The export is a function.

    Func,
    /// The export is a table.
    Table,
    #[default]
    /// The export is a memory.
    Memory,
    /// The export is a global.
    Global,
    /// The export is a tag.
    Tag,
}

impl From<ExportKindByteMaker> for ExportKind {
    fn from(kind: ExportKindByteMaker) -> Self {
        match kind {
            ExportKindByteMaker::Func   => ExportKind::Func,
            ExportKindByteMaker::Table  => ExportKind::Table,
            ExportKindByteMaker::Memory => ExportKind::Memory,
            ExportKindByteMaker::Global => ExportKind::Global,
            ExportKindByteMaker::Tag    => ExportKind::Tag,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ByteMakerStartSection {

}

#[derive(Clone, Debug, Default)]
pub struct ByteMakerElementSection {

}


#[derive(Clone, Debug, Default)]
pub struct ByteMakerDataSection {

}

#[derive(Clone, Debug, Default)]
struct ByteMakerCodeSection{
    pub functions: Vec<Function>,
}

#[derive(Debug, Default, Clone)]
#[allow(unused)]
struct ByteMakerCurrentContext{
    pub sections: WasmSections,
    pub current_tags: Vec<OperationUnit>,
    pub cur_mem_index: i32,
    pub cur_func_index: u32,
    pub ident_map: BTreeMap<ObjectIdentFull, i32>,
    pub cur_func: usize,     // индекс в sections.code.functions
    pub main_func: usize,    // тоже индекс
    pub  current_module_name: ObjectIdent

}


pub fn build_modules (modules: Vec<ModuleWithOperations>) -> Vec<u8>{
    let opers = modules.iter().flat_map(|m| build_module(m)).collect::<Vec<_>>();
    opers
}

pub fn build_module(module: &ModuleWithOperations) -> Vec<u8> {
    let opers = module.operations.clone();
    let ctx = &mut ByteMakerCurrentContext{current_module_name: module.name.clone(), ..ByteMakerCurrentContext::default()};





    opers.iter().for_each(|op| {
        make_operation_unit(op.clone(), ctx)
    });


    let mut module = Module::new();


    let type_section = &mut TypeSection::new();

    ctx.sections.types.functions.iter().for_each(|(params, results)| {
        type_section.ty().function(params.clone(), results.clone());
    });
    // sections.types

    module.section(type_section);


    let import_section = &mut  ImportSection::new();
    //sections.imports
    module.section(import_section);


    let function_section = &mut FunctionSection::new();
    ctx.sections.functions.functions.iter().for_each(
        |f| {
            function_section.function(*f);
        }
    );

    //sections.functions
    module.section(function_section);

    let table_section = &mut  TableSection::new();
    //sections.tables
    module.section(table_section);

    let memory_section = &mut  MemorySection::new();
    ctx.sections.memories.memory.iter().for_each(|m|{
        memory_section.memory(m.clone());
    });
    //sections.memories
    module.section(memory_section);


    let global_section = &mut  GlobalSection::new();
    //sections.globals
    module.section(global_section);

    let export_section = &mut ExportSection::new();
    ctx.sections.exports.exports.iter().for_each(|e| {
        export_section.export(e.ident.as_str(), ExportKind::from(e.kind.clone()), e.index);
    });
    module.section(export_section);

    let start = &mut  StartSection { function_index: 0 };
    module.section(start);


    let elements_section = &mut  ElementSection::new();
    //sections.elements
    module.section(elements_section);


    let code_section = &mut CodeSection::new();
    ctx.sections.code.functions.iter().for_each(|f| {
        code_section.function(f);
    });
    module.section(code_section);


    let section_data =  &mut  DataSection::new();
    //sections.data
    module.section(section_data);

    module.finish()
}