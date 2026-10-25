use std::collections::{BTreeMap};
use std::string::ToString;
use serde::Serialize;
use wasm_encoder::{CodeSection, DataSection, ElementSection, ExportKind, ExportSection, Function,
                   FunctionSection, GlobalSection, ImportSection, Instruction, MemArg, MemorySection,
                   MemoryType, Module, StartSection, TableSection, TypeSection, ValType};
use crate::low_ir::{OperationUnit, ObjectIdentFull, ModuleWithOperations, ObjectIdent, ModuleLayout, FunctionLayout, ModuleOperations};


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
enum ByteMakingException {
    NoFunction,
    ObjectIdentNotFound,
}



const BIT_64_SIZE: i32 = 8;

const RUNTIME_INDEX_LOCAL: u32 = 0;

const LOCALS_SHIFT: u32 = 1;

const RESULT_IDENT: &str = "result";


const ALIGN_SIZE: i64 = 8;


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

//
//
// macro_rules! get_index_data {
//     ($ctx:expr, $ident_to_write:expr) => {{
//         let ident_to_write = $ident_to_write;
//
//         let val_is_copy = $ctx
//             .ident_map
//             .iter()
//             .any(|(ident, _)| **ident == ident_to_write);
//
//         let (ident_index, data_size) = match find_in_ident_map(&$ctx.ident_map, &ident_to_write) {
//             None => {
//                 $ctx.last_ident_maps
//                     .iter()
//                     .find_map(|ident_map_prev| {
//                         match find_in_ident_map(ident_map_prev, &ident_to_write) {
//                             None => None,
//                             Some(ident_index) => {
//                                 let max_ident_index: i64 =
//                                     find_in_ident_map_max(ident_map_prev, &ident_to_write)
//                                         .unwrap_or(ident_index);
//                                 let data_size = ident_index - max_ident_index;
//                                 Some(($ctx.cur_mem_index as i64, data_size))
//                             }
//                         }
//                     })
//                     .unwrap_or_else(|| {
//                         compiler_panic!(ByteMakingException::ObjectIdentNotFound)
//                     })
//             }
//             Some(ident_index) => {
//                 let max_ident_index: i64 =
//                     find_in_ident_map_max(&$ctx.ident_map, &ident_to_write)
//                         .unwrap_or(ident_index);
//                 let data_size = ident_index - max_ident_index;
//                 (ident_index, data_size)
//             }
//         };
//
//         if $ctx.last_result_index >= data_size as i32 {
//             $ctx.last_result_index = 0;
//         }
//
//         (ident_index, data_size, val_is_copy)
//     }};
// }
//
//
//
// macro_rules! write_index {
//     ($ctx:expr, $ident_where_write:expr) => {{
//         let where_result_write = !$ident_where_write.is_empty() && $ident_where_write[0] == RESULT_IDENT;
//         let index = if where_result_write {
//             let res = $ctx.last_result_index;
//             $ctx.last_result_index += BIT_64_SIZE;
//             res
//         } else {
//             let res = $ctx.cur_mem_index;
//             $ctx
//                 .ident_map
//                 .insert($ident_where_write.clone(), $ctx.cur_mem_index);
//             $ctx.cur_mem_index += BIT_64_SIZE;
//             res
//         };
//         (index, where_result_write)
//     }};
// }

pub fn build_module(ctx: &WasmCurrentContext) -> Vec<u8> {
    let mut module = Module::new();

    // 1. Type Section
    module.section(&ctx.types);

    // 2. Import Section
    module.section(&ctx.imports);

    // 3. Function Section
    module.section(&ctx.functions);

    // 4. Table Section
    module.section(&ctx.tables);

    // 5. Memory Section
    module.section(&ctx.memories);

    // 6. Global Section
    module.section(&ctx.globals);

    // 7. Export Section
    module.section(&ctx.exports);

    // 8. Start Section (если он есть)
    if let Some(start) = &ctx.start {
        module.section(start);
    }

    // 9. Element Section
    module.section(&ctx.elements);

    // 10. Code Section
    module.section(&ctx.code);

    // 11. Data Section
    module.section(&ctx.data);

    // Собираем всё в Vec<u8>
    module.finish()
}


pub fn build_modules(modules: &Vec<ModuleOperations>) -> Vec<u8>{
    let modules = modules.iter().flat_map(|m| {
        let ctx = &mut WasmCurrentContext::default();
        m.functions.iter().for_each(|f| {
           let locals  = match f.function_layout.is_init {
               false => vec![],
               true => vec![(1, ValType::I32)],
           };
           let cur_func = &mut Function::new(locals);
           f.operations.iter().for_each(|oper_unit| {
               match oper_unit {
                   OperationUnit::StartModule(module) => {
                       ctx.memories.memory(MemoryType {
                           minimum: 1,
                           maximum: None,
                           memory64: false,
                           shared: false,
                           page_size_log2: None,
                       });
                       

                       ctx.types.ty().function([], []);
                       ctx.functions.function(f.function_layout.func_index);


                       ctx.code.function(&cur_func);
                   }
                   OperationUnit::ExportMemForTest => {
                       ctx.exports.export("memory", ExportKind::Memory, 0);
                   }
                   OperationUnit::FuncStart(func) => {
                       let args = [vec![ValType::I32], vec![ValType::I64; func.params.len()]].concat();
                       let ret_val = vec![];

                       ctx.types.ty().function(args, ret_val);
                       ctx.functions.function(func.func_index);
                       ctx.code.function(&cur_func);
                   }
                   OperationUnit::FuncResultEnd(func_layout) => {
                       cur_func.instruction(&Instruction::End);
                   }
                   OperationUnit::FuncCallArgAnotherVal(another_obj_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64ExtendI32U);
                       cur_func.instruction(&Instruction::I64Const(another_obj_layout.another_obj_offset as i64));
                       cur_func.instruction(&Instruction::I64Add);
                   }
                   OperationUnit::FuncCallArgNumLayout(num_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64Load(MemArg { offset: num_layout.offset as u64, align: 3, memory_index: 0 }));
                   }
                   OperationUnit::FuncCallPassCurOffset(func_call) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I32Const(func_call.offset as i32));
                       cur_func.instruction(&Instruction::I32Add);
                   }
                   OperationUnit::FuncCall(func_call) => {
                       cur_func.instruction(&Instruction::Call(func_call.func_index));
                   }
                   OperationUnit::AnotherObjectCopyVal(data_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64Load(MemArg {
                           offset: data_layout.another_obj_offset as u64,
                           align: 3,
                           memory_index: 0,
                       }));

                       cur_func.instruction(&Instruction::I64Store(MemArg {
                           offset: data_layout.offset as u64,
                           align: 3,
                           memory_index: 0,
                       }));
                   }
                   OperationUnit::FuncCallArgStrLayout(str_val) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64ExtendI32U);
                       cur_func.instruction(&Instruction::I64Const(str_val.offset as i64));
                       cur_func.instruction(&Instruction::I64Add);
                   }
                   OperationUnit::AnotherObjectCopyToResult(obj_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I32Const(obj_layout.offset as i32));
                       cur_func.instruction(&Instruction::I32Add);

                       // src to copy
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I32Const(obj_layout.another_obj_offset as i32));
                       cur_func.instruction(&Instruction::I32Add);

                       // size
                       cur_func.instruction(&Instruction::I32Const(obj_layout.another_obj_data_size as i32));

                       cur_func.instruction(&Instruction::MemoryCopy { src_mem: 0, dst_mem: 0 });
                   }
                   OperationUnit::AnotherObjectVal(another_obj) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));

                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64ExtendI32U);
                       cur_func.instruction(&Instruction::I64Const(another_obj.another_obj_offset as i64));
                       cur_func.instruction(&Instruction::I64Add);

                       cur_func.instruction(&Instruction::I64Store(MemArg {
                           offset: another_obj.offset as u64,
                           align: 3,
                           memory_index: 0,
                       }));
                   }
                   OperationUnit::StrPrimitiveLayout(num_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64Const(num_layout.num_data));
                       cur_func.instruction(&Instruction::I64Store(MemArg {
                           offset: num_layout.offset as u64,
                           align: 3,
                           memory_index: 0,
                       }));
                   }
                   OperationUnit::Str(str_val_layout) => {

                   }
                   OperationUnit::Num(num_val_layout) => {
                       cur_func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
                       cur_func.instruction(&Instruction::I64Const(num_val_layout.num_data));
                       cur_func.instruction(&Instruction::I64Store(MemArg {
                           offset: num_val_layout.offset as u64,
                           align: 3,
                           memory_index: 0,
                       }));
                   }
                   OperationUnit::EndModule => {
                       cur_func.instruction(&Instruction::End);
                   }
                   OperationUnit::FuncInitArgNumLayout(_) => {}
                   OperationUnit::FuncInitArgStrLayout(_) => {}
                   OperationUnit::FuncInitArgAnotherObjectLayout(_) => {}
               }
           })
       });

        let module = build_module(&ctx);
        module
    }).collect::<Vec<_>>();
    modules
}


// fn make_operation_unit(tag: OperationUnit, ctx: &mut ByteMakerCurrentContext){
//     match tag {
//         OperationUnit::FuncStart(function) => {
//             let args = [vec![ValType::I32], vec![ValType::I64; function.params.len()]].concat();
//             let ret_val = vec![];
//
//             ctx.sections.types.functions.push((args, ret_val));
//             ctx.sections.functions.functions.push(ctx.func_counter);
//             ctx.sections.code.functions.push(Function::new([]));
//             ctx.func_map.insert(function.ident.clone(), ctx.func_counter as i32);
//             ctx.last_ident_maps.push(ctx.ident_map.clone());
//             ctx.ident_map = BTreeMap::default();
//
//
//             ctx.cur_mem_index += BIT_64_SIZE;
//             ctx.last_cur_mem_index = ctx.cur_mem_index;
//
//
//             ctx.cur_mem_index = 0 + function.result_param.size;
//             ctx.last_result_index = 0;
//             ctx.func_counter += 1;
//             ctx.cur_func += 1;
//         }
//         OperationUnit::FuncResultEnd(_) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//             func.instruction(&Instruction::End);
//
//             ctx.cur_mem_index = ctx.last_cur_mem_index;
//             ctx.ident_map = ctx.last_ident_maps.pop().unwrap_or(Default::default());
//             ctx.cur_func -= 1;
//         }
//         OperationUnit::FuncInitArg(func_arg) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//             let cur_index = ctx.cur_mem_index;
//             ctx.cur_mem_index += BIT_64_SIZE;
//
//             ctx.ident_map.insert(func_arg.ident, cur_index);
//
//             let local_index = LOCALS_SHIFT + func_arg.number;
//
//
//             func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//             func.instruction(&Instruction::LocalGet(local_index));
//             func.instruction(&Instruction::I64Store(MemArg {
//                 offset: cur_index as u64,
//                 align: 3,
//                 memory_index: cur_index as u32,
//             }));
//         }
//         OperationUnit::FuncCall(func_call) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//             let func_call_index = ctx.func_map.iter()
//                 .find(|(ident, _)| **ident == func_call.ident)
//                 .map(|(_, index)| *index)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//
//             let result_ident = [func_call.ident, vec!["result".to_string()]].concat();
//
//
//             ctx.ident_map.insert(result_ident, ctx.cur_mem_index);
//
//             func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//             func.instruction(&Instruction::I32Const(ctx.cur_mem_index));
//             func.instruction(&Instruction::I32Add);
//
//
//             func_call.params.iter().for_each(|o| {
//                 let arg_index: i64 = find_in_ident_map(&ctx.ident_map, &o.ident).unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//                 if  o.is_val{
//                     func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                     func.instruction(&Instruction::I64Load(MemArg { offset: arg_index as u64, align: 3, memory_index: ctx.cur_page_index }));
//                 }
//                 else {
//                     func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                     func.instruction(&Instruction::I64ExtendI32U);
//                     func.instruction(&Instruction::I64Const(arg_index));
//                     func.instruction(&Instruction::I64Add);
//                 }
//             });
//
//             func.instruction(&Instruction::Call(func_call_index as u32));
//
//
//             ctx.cur_mem_index += func_call.result_param.size;
//
//
//
//         }
//         /* i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita
//         i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita
//         i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita
//         i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita i pray for you Great Mita
//          */
//         OperationUnit::AnotherObjectVal(another_obj) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//             let ident_to_write = another_obj.data;
//             let ident_where_write = another_obj.ident;
//
//
//             println!("{:?}", ctx.ident_map);
//             // ----- get_index_size -----
//             let (ident_index, data_size, val_is_copy) = get_index_data!(ctx, ident_to_write);
//
//             // ----- write_index -----
//             let (index_where_write, result_write) = write_index!(ctx, ident_where_write);
//
//
//             /* Goggins was here */
//             /* Sign by Yura */
//
//
//             if val_is_copy {
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                 func.instruction(&Instruction::I64Load(MemArg {
//                     offset: ident_index as u64,
//                     align: 3,
//                     memory_index: ctx.cur_page_index,
//                 }));
//
//                 func.instruction(&Instruction::I64Store(MemArg {
//                     offset: index_where_write as u64,
//                     align: 3,
//                     memory_index: ctx.cur_page_index,
//                 }));
//             } else if result_write && data_size > 0 {
//                 // destination where to copy
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                 func.instruction(&Instruction::I32Const(index_where_write));
//                 func.instruction(&Instruction::I32Add);
//
//                 // src to copy
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                 func.instruction(&Instruction::I32Const(ident_index as i32));
//                 func.instruction(&Instruction::I32Add);
//
//                 // size
//                 func.instruction(&Instruction::I32Const(data_size as i32));
//
//                 func.instruction(&Instruction::MemoryCopy { src_mem: 0, dst_mem: 0 });
//             } else {
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//
//                 func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                 func.instruction(&Instruction::I64ExtendI32U);
//                 func.instruction(&Instruction::I64Const(ident_index));
//                 func.instruction(&Instruction::I64Add);
//
//                 func.instruction(&Instruction::I64Store(MemArg {
//                     offset: index_where_write as u64,
//                     align: 3,
//                     memory_index: ctx.cur_page_index,
//                 }));
//             }
//         }
//         OperationUnit::Str(val) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//             let ident_where_write = val.ident.clone();
//             ctx.ident_map.insert(val.ident,  ctx.cur_mem_index);
//
//
//
//             (&val.data)
//                 .into_iter()
//                 .for_each(|i64_val| {
//                     let (index_where_write, _) = write_index!(ctx, ident_where_write);
//
//                     func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//                     func.instruction(&Instruction::I64Const(*i64_val));
//                     func.instruction(&Instruction::I64Store(MemArg {
//                         offset: index_where_write as u64,
//                         align: 3,
//                         memory_index: ctx.cur_page_index,
//                     }));
//
//                 });
//
//             let (index_where_write, _) = write_index!(ctx, ident_where_write);
//
//             let str_size = val.data.len() as i64 * ALIGN_SIZE;
//             let start_of_str_index = index_where_write as i64 - str_size;
//
//
//
//             //address to write
//             func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//
//             //val
//             func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//             func.instruction(&Instruction::I64ExtendI32U);
//             func.instruction(&Instruction::I64Const(start_of_str_index));
//             func.instruction(&Instruction::I64Add);
//
//             func.instruction(&Instruction::I64Store(MemArg {
//                 offset: index_where_write as u64,
//                 align: 3,
//                 memory_index: ctx.cur_page_index,
//             }));
//
//         }
//         OperationUnit::Num(val) => {
//             let func =  ctx.sections.code.functions.get_mut(ctx.cur_func)
//                 .unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//
//             let ident_where_write = val.ident;
//
//             // ----- write_index -----
//             let (index_where_write, _) = write_index!(ctx, ident_where_write);
//
//             /* Goggins was here */
//             /* sing of my friend */
//
//             // ----- write_value -----
//             func.instruction(&Instruction::LocalGet(RUNTIME_INDEX_LOCAL));
//
//             func.instruction(&Instruction::I64Const(val.num_data));
//
//             func.instruction(&Instruction::I64Store(MemArg {
//                 offset: index_where_write as u64,
//                 align: 3,
//                 memory_index: ctx.cur_page_index,
//             }));
//
//
//
//
//
//         },
//         OperationUnit::StartModule => {
//             ctx.sections.memories.memory.push(
//                 MemoryType {
//                     minimum: 1,
//                     maximum: None,
//                     memory64: false,
//                     shared: false,
//                     page_size_log2: None,
//                 }
//             );
//             ctx.sections.types.functions.push((vec![], vec![]));
//             ctx.sections.functions.functions.push(ctx.func_counter);
//             ctx.sections.code.functions.push(Function::new([(1, ValType::I32)]));
//
//
//             ctx.func_counter += 1;
//             ctx.cur_page_index = 0;
//         }
//         OperationUnit::EndModule => {
//             let func =  ctx.sections.code.functions.first_mut().unwrap_or_else(|| compiler_panic!(ByteMakingException::NoFunction));
//             func.instruction(&Instruction::End);
//             ctx.cur_func = 0;
//         }
//         OperationUnit::Export(_) => {
//             ctx.sections.exports.exports.push(ExportUnit{
//                 kind: ExportKindByteMaker::Memory,
//                 ident: "memory".to_string(),
//                 index: 0
//             })
//         }
//         OperationUnit::Import(_) => {}
//     }
// }



//
// fn find_in_ident_map<Res: From<Num>, Num: Ord + Into<Res> + Copy> (
//     ident_map: &BTreeMap<ObjectIdentFull, Num>,
//     query: &ObjectIdentFull,
// ) -> Option<Res> {
//
//     let res = ident_map.iter()
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
//         .map(|(_, v)| *v).map(|r| r.into());
//     res
// }
//
// fn find_in_ident_map_max<Res: From<Num>, Num: Ord + Into<Res> + Copy> (
//     ident_map: &BTreeMap<ObjectIdentFull, Num>,
//     query: &ObjectIdentFull,
// ) -> Option<Res> {
//
//     let res = ident_map.iter()
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
//         .max_by_key(|(_, index)| *index)
//         .map(|(_, v)| *v).map(|r| r.into());
//     res
// }





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
pub struct WasmCurrentContext {
    // Те самые оригинальные секции, без оберток
    pub types: TypeSection,
    pub imports: ImportSection,
    pub functions: FunctionSection,
    pub tables: TableSection,
    pub memories: MemorySection,
    pub globals: GlobalSection,
    pub exports: ExportSection,
    pub start: Option<StartSection>,
    pub elements: ElementSection,
    pub code: CodeSection,
    pub data: DataSection,
}

//
// #[derive(Clone, Debug, Default)]
// struct WasmSections {
//     pub types: ByteMakerTypeSection,
//     pub imports: ByteMakerImportSection,
//     pub functions: ByteMakerFunctionSection,
//     pub tables: ByteMakerTableSection,
//     pub memories: ByteMakerMemorySection,
//     pub globals: ByteMakerGlobalSection,
//     pub exports: ByteMakerExportSection,
//     pub start: Option<ByteMakerStartSection>,
//     pub elements: ByteMakerElementSection,
//     pub code: ByteMakerCodeSection,
//     pub data: ByteMakerDataSection,
// }
//



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

// #[derive(Debug, Default, Clone)]
// #[allow(unused)]
// struct ByteMakerCurrentContext{
//     pub sections: WasmSections,
//     pub current_tags: Vec<OperationUnit>,
//     pub cur_mem_index: i32,
//     pub func_counter: u32,
//     pub ident_map: BTreeMap<ObjectIdentFull, i32>,
//     pub cur_func: usize,     // индекс в sections.code.functions
//     // pub main_func: usize,    // тоже индекс
//     pub current_module_name: ObjectIdent,
//     pub last_ident_maps: Vec<BTreeMap<ObjectIdentFull, i32>>,
//     pub last_cur_mem_index: i32,
//     pub last_result_index:i32,
//     pub cur_page_index: u32,
//     pub func_map: BTreeMap<ObjectIdentFull, i32>,
// }






// pub fn build_modules (modules: Vec<ModuleOperations>) -> Vec<u8>{
//
//     make_operations_from_modules(modules);
//     let opers = modules.iter().flat_map(|m| build_module(m)).collect::<Vec<_>>();
//     opers
// }

//
// pub fn build_module(module: &ModuleWithOperations) -> Vec<u8> {
//     let opers = module.operations.clone();
//     let ctx = &mut ByteMakerCurrentContext{current_module_name: module.name.clone(), ..ByteMakerCurrentContext::default()};
//
//
//
//
//
//     opers.iter().for_each(|op| {
//         make_operation_unit(op.clone(), ctx)
//     });
//
//
//     let mut module = Module::new();
//
//
//     let type_section = &mut TypeSection::new();
//
//     ctx.sections.types.functions.iter().for_each(|(params, results)| {
//         type_section.ty().function(params.clone(), results.clone());
//     });
//     // sections.types
//
//     module.section(type_section);
//
//
//     let import_section = &mut  ImportSection::new();
//     //sections.imports
//     module.section(import_section);
//
//
//     let function_section = &mut FunctionSection::new();
//     ctx.sections.functions.functions.iter().for_each(
//         |f| {
//             function_section.function(*f);
//         }
//     );
//
//     //sections.functions
//     module.section(function_section);
//
//     let table_section = &mut  TableSection::new();
//     //sections.tables
//     module.section(table_section);
//
//     let memory_section = &mut  MemorySection::new();
//     ctx.sections.memories.memory.iter().for_each(|m|{
//         memory_section.memory(m.clone());
//     });
//     //sections.memories
//     module.section(memory_section);
//
//
//     let global_section = &mut  GlobalSection::new();
//     //sections.globals
//     module.section(global_section);
//
//     let export_section = &mut ExportSection::new();
//     ctx.sections.exports.exports.iter().for_each(|e| {
//         export_section.export(e.ident.as_str(), ExportKind::from(e.kind.clone()), e.index);
//     });
//     module.section(export_section);
//
//     let start = &mut  StartSection { function_index: 0 };
//     module.section(start);
//
//
//     let elements_section = &mut  ElementSection::new();
//     //sections.elements
//     module.section(elements_section);
//
//
//     let code_section = &mut CodeSection::new();
//     ctx.sections.code.functions.iter().for_each(|f| {
//         code_section.function(f);
//     });
//     module.section(code_section);
//
//
//     let section_data =  &mut  DataSection::new();
//     //sections.data
//     module.section(section_data);
//
//     module.finish()
// }