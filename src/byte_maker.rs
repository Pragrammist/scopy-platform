use std::collections::{BTreeMap};
use serde::Serialize;
use wasm_encoder::{CodeSection, DataSection, ElementSection, ExportKind, ExportSection, Function, FunctionSection, GlobalSection, ImportSection, Instruction, MemArg, MemorySection, MemoryType, Module, StartSection, TableSection, TypeSection, ValType};
use crate::low_ir::{OperationUnit, ObjectIdentFull, ModuleWithOperations, ObjectIdent};


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
enum ByteMakingException {
    NoFunction,
    ObjectIdentNotFound,
}



const BIT_64_SIZE: i32 = 8;

const RUNTIME_INDEX_LOCAL: u32 = 0;

const LOCALS_SHIFT: u32 = 1;



impl std::fmt::Display for ByteMakingException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ByteMakingException::NoFunction => write!(f, "No function at WASM module."),
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


macro_rules! generate_compute_index {
    ($func:expr, $ctx:expr) => {{
        $func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
        $func.instruction(&Instruction::I32Const($ctx.cur_mem_index));
        $func.instruction(&Instruction::I32Add);
    }};
}

macro_rules! add_func_to_counter {
    ($ctx:expr) => {{
        $ctx.cur_func_index += 1;
        $ctx.cur_func_index
    }};
}
macro_rules! incr_compile_time_index {
    ($ctx:expr) => {{
        $ctx.cur_mem_index += BIT_64_SIZE;
    }};
}

macro_rules! store_i64_to_cur_index {
    ($func:expr, $val:expr) => {{
        $func.instruction(&Instruction::I64Const($val));
        $func.instruction(&Instruction::I64Store(MemArg {
            offset: 0,
            align: 3,
            memory_index: 0,
        }));
    }};
}



fn make_operation_unit(tag: OperationUnit, ctx: &mut ByteMakerCurrentContext){
    match tag {
        OperationUnit::CallChainStart => {}
        OperationUnit::CallChainEnd => {}
        OperationUnit::FuncStart(function) => {
            let args = [vec![ValType::I32], vec![ValType::I32; function.params_count as usize]].concat();
            let ret_val = vec![ValType::I32];

            ctx.sections.types.functions.push((args, ret_val));
            ctx.sections.functions.functions.push(ctx.cur_func_index);
            ctx.sections.code.functions.push(Function::new([]));


            add_func_to_counter!(ctx);

            ctx.last_cur_mem_index = ctx.cur_mem_index;
        }
        OperationUnit::FuncResultEnd(_) => {
            ctx.cur_mem_index = ctx.last_cur_mem_index;
        }
        OperationUnit::FuncInitArg(func_arg) => {
            let func =  ctx.sections.code.functions
                .last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));

            ctx.ident_map.insert(func_arg.ident, ctx.cur_mem_index);

            let local_index = LOCALS_SHIFT + func_arg.number;


            generate_compute_index!(func, ctx);



            func.instruction(&Instruction::LocalGet(local_index));
            func.instruction(&Instruction::I64ExtendI32U);


            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,
                memory_index: 0,
            }));


            incr_compile_time_index!(ctx);


        }
        OperationUnit::AnotherObjectVal(another_obj) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));

            let ident_to_write = another_obj.data;
            let ident_where_write = another_obj.ident;
            ctx.ident_map.insert(ident_where_write.clone(),  ctx.cur_mem_index);
            let ident_index = find_in_ident_map(&ctx.ident_map, &ident_to_write).unwrap_or_else(|| compiler_panic!(ByteMakingException::ObjectIdentNotFound));

            generate_compute_index!(func, ctx);


            func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
            func.instruction(&Instruction::I64ExtendI32U);

            //нужно чтобы записывался первоначальный индекс объекта, а не это вот
            func.instruction(&Instruction::I64Const(ident_index));


            func.instruction(&Instruction::I64Add);


            incr_compile_time_index!(ctx);
        }
        OperationUnit::FuncCall(_) => {}
        OperationUnit::FuncParam => {}
        OperationUnit::Str(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));

            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);


            val.data
                .into_iter()
                .for_each(|i64_val| {

                    generate_compute_index!(func, ctx);

                    store_i64_to_cur_index!(func, i64_val);

                    incr_compile_time_index!(ctx);
                });

        }
        OperationUnit::Bool(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);

            generate_compute_index!(func, ctx);

            store_i64_to_cur_index!(func, val.data);

            incr_compile_time_index!(ctx);
        }
        OperationUnit::Null(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);

            generate_compute_index!(func, ctx);

            store_i64_to_cur_index!(func, val.data);

            incr_compile_time_index!(ctx);
        }
        OperationUnit::Num(val) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);


            generate_compute_index!(func, ctx);

            store_i64_to_cur_index!(func, val.data);

            incr_compile_time_index!(ctx);
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
            ctx.sections.code.functions.push(Function::new([(0, ValType::I32)]));


            add_func_to_counter!(ctx);
        }
        OperationUnit::EndModule => {
            let func =  ctx.sections.code.functions.first_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
            func.instruction(&Instruction::End);
        }
        OperationUnit::AnotherObject(obj) => {
            let func =  ctx.sections.code.functions.last_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));

            let ident_to_write = obj;

            let ident_index = find_in_ident_map(&ctx.ident_map, &ident_to_write).unwrap_or_else(|| compiler_panic!(ByteMakingException::ObjectIdentNotFound));



            generate_compute_index!(func, ctx);

            func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
            func.instruction(&Instruction::I64ExtendI32U);
            func.instruction(&Instruction::I64Const(ident_index));
            func.instruction(&Instruction::I64Add);


            func.instruction(&Instruction::I64Store(MemArg {
                offset: 0,
                align: 3,
                memory_index: 0,
            }));

            incr_compile_time_index!(ctx);
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


// fn find_in_ident_map(
//     ident_map: &BTreeMap<ObjectIdentFull, i32>,
//     query: &ObjectIdentFull,
// ) -> i64 {
//
//     ident_map.iter()
//         .filter(|(path, _)| {
//             let query_len = query.len();
//
//             let path = path.iter()
//                 .take(query_len)
//                 .map(|m| m.clone())
//                 .collect::<Vec<_>>() as ObjectIdentFull;
//
//             path == *query
//         })
//         .min_by_key(|(_, index)| *index)
//         .map(|(_, v)| *v).unwrap_or_else(|| compiler_panic!(ByteMakingException::ObjectIdentNotFound)) as i64
// }

fn find_in_ident_map<Res: From<Num>, Num: Ord + Into<Res> + Copy> (
    ident_map: &BTreeMap<ObjectIdentFull, Num>,
    query: &ObjectIdentFull,
) -> Option<Res> {

    let res = ident_map.iter()
        .filter(|(path, _)| {
            let query_len = query.len();

            let path = path.iter()
                .take(query_len)
                .map(|m| m.clone())
                .collect::<Vec<_>>() as ObjectIdentFull;

            path == *query
        })
        .min_by_key(|(_, index)| *index)
        .map(|(_, v)| *v).map(|r| r.into());
    res
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
    pub current_module_name: ObjectIdent,
    pub last_cur_mem_index: i32,


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