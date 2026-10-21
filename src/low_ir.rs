#![allow(unused)]


use std::collections::{BTreeMap, HashMap};
use serde::Serialize;
use swc_ecma_ast::op;
use wasm_encoder::{
    CodeSection,
    Function,
    FunctionSection,
    Instruction,
    Module,
    TypeSection,
    ValType,
};
use wasmparser::Data;
use wasmtime::component::__internal::wasmtime_environ::object::ReadCacheOps;
use wasmtime::component::ReadBuffer;
use crate::semantic::{AnotherObjectValue, AnotherObjectValuePath, BinaryObjectValue, BinaryOpType, ConditionStatement, ExportDataValue, FunctionCallResultValue, FunctionValue, ImportDataValue, LitValueBool, LitValueNum, LitValueString, LiteralValue, LoopStatement, ObjectData, ObjectDataValue, ObjectValue, ScopeStatement, ScopyModule, ScopyProject, Statement};

trait InsertMany<T> {
    fn insert_many<I: IntoIterator<Item = T>>(&mut self, iter: I);
}

impl<T> InsertMany<T> for Vec<T> {
    fn insert_many<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
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






pub fn convert_to_lowering_ir(scopy_project: &ScopyProject) -> Vec<ModuleWithOperations>{
    let mut current_ctx =  &mut CurrentOperationContext::default();
    let t = scopy_project.modules.iter().map(|module| {

        let start_module = OperationUnit::StartModule;

        current_ctx.last_operation.push(start_module.clone());
        let operation_units = module.statements.iter().flat_map(|stmt|{
            let res = convert_statement(&stmt, current_ctx);
            current_ctx.last_operation.insert_many(res.clone());
            res
        }).collect::<Vec<_>>();

        let end_module = OperationUnit::EndModule;


        let operation_units = [vec![start_module], operation_units, vec![end_module]].concat();

        ModuleWithOperations{
            name: module.name.clone(),
            operations: operation_units,
        }
    }).collect::<Vec<_>>();




    t

}





type AlignSize = i64;



pub type ObjectIdent = String;
pub type ObjectIdentFull = Vec<ObjectIdent>;

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveVal{
    pub data: AlignedData,
    pub ident: ObjectIdentFull
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveValNum{
    pub data: AlignedData,
    pub ident: ObjectIdentFull
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveValNull {
    pub data: AlignedData,
    pub ident: ObjectIdentFull
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveValBool{
    pub data: AlignedData,
    pub ident: ObjectIdentFull
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveValStr{
    pub data: Vec<AlignedData>,
    pub ident: ObjectIdentFull
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct AnotherObjectVal{
    pub ident: ObjectIdentFull,
    pub data: ObjectIdentFull
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct  ModuleWithOperations {
    pub name: ObjectIdent,
    pub operations: Vec<OperationUnit>,
}




#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum OperationUnit {
    CallChainStart,
    CallChainEnd,
    StartModule,
    FuncStart(FunctionPrimitiveVal),
    FuncResultEnd(FuncResultHeader),
    FuncInitArg(FunctionArgData),
    FuncCall(FunctionPrimitiveVal),
    AnotherObjectVal(AnotherObjectVal),
    Str(PrimitiveValStr),
    Num(PrimitiveValNum),

    Export(ObjectIdentFull),
    Import(ObjectIdentFull),

    IfScope,
    LoopScope,
    ElseScope,
    StartScope,
    EndScope,
    StartCond,
    EndCond,
    EmptyScope,
    EndModule,
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,
    /// `??`
    NullishCoalescing,


}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FuncCallData{

}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FuncResultHeader {
    pub size: i32,
    pub cur_obj: ObjectIdentFull
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionArgData{
    pub ident: ObjectIdentFull,
    pub number: u32,
    pub is_val: bool
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionPrimitiveVal{
    pub ident: ObjectIdentFull,
    pub params: Vec<FunctionArgData>,
    pub result_param:  FuncResultHeader
}






pub type ParamsCount = u64;






type AlignedData = i64;






#[derive(Clone, PartialEq, Debug, Serialize, Eq, Default)]
pub struct CurrentOperationContext {
    last_operation: Vec<OperationUnit>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum OperationExpr {
    NotFoundIdentExpr,
    NotFoundOwnerId
}

impl std::fmt::Display for OperationExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            OperationExpr::NotFoundIdentExpr => write!(f, "Object ident not found in operation context"),
            OperationExpr::NotFoundOwnerId => write!(f, "Owner id not found"),
        }
    }
}

fn convert_obj (obj: &ObjectData, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {

    let cur_mtx = &mut current_ctx.clone();
    let  cur_obj = vec![cur_obj.clone(), vec![obj.name.clone()]].concat() as ObjectIdentFull;


    let res = convert_object_data_value(&obj.value, cur_mtx, cur_obj.clone());


    res
}

fn convert_object_data_value(
    obj_data_val: &ObjectDataValue,
    current_context: &CurrentOperationContext,
    current_obj: ObjectIdentFull) -> Vec<OperationUnit>{



    match &obj_data_val{
        ObjectDataValue::Object(object_val) => {
            convert_obj_val(&object_val, current_context, current_obj)
        },
        ObjectDataValue::AnotherObject(another_obj) => {
            convert_another_object(another_obj, current_context, current_obj, false, false)
        },
        ObjectDataValue::Literal(literal) => {
            convert_literal(literal, current_obj)
        },
        ObjectDataValue::Function(func) => {
            parse_func_val(func, current_context, current_obj)
        }
        ObjectDataValue::FunctionCall(func_call) => {
            parse_func_call(func_call, current_context, current_obj)
        }
        ObjectDataValue::Binary(bin) => {
            convert_bin(bin, current_context, current_obj)
        }
        ObjectDataValue::Import(import) => {
            parse_imports(import, current_context, current_obj)
        }
        ObjectDataValue::Export(export) => {
            parse_exports(export, current_context, current_obj)
        }
    }
}


fn parse_exports(
    export: &ExportDataValue,
    current_context: &CurrentOperationContext,
    current_obj: ObjectIdentFull) -> Vec<OperationUnit> {

    let opers = export.exports.iter().flat_map(|m| {
        let objs = convert_another_object(m, current_context, current_obj.clone(), true, false);
        objs
    }).collect::<Vec<_>>();
    opers
}

fn parse_imports(
    export: &ImportDataValue,
    current_context: &CurrentOperationContext,
    current_obj: ObjectIdentFull) -> Vec<OperationUnit> {

    let opers = export.imports.iter().flat_map(|m| {
        let objs = convert_another_object(m, current_context, current_obj.clone(), false, true);
        objs
    }).collect::<Vec<_>>();
    opers
}

fn convert_bin(bin: &BinaryObjectValue, current_context: &CurrentOperationContext,  cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let cur_ctx = &mut current_context.clone();
    let v1 = convert_object_data_value(&bin.v1, cur_ctx, cur_obj.clone());
    cur_ctx.last_operation.insert_many(v1.clone());
    let v2 = convert_object_data_value(&bin.v2, cur_ctx, cur_obj.clone());
    cur_ctx.last_operation.insert_many(v2.clone());
    let operation_unit = convert_bin_oper(bin, cur_ctx);
    [v1, operation_unit,v2].concat()
}

fn convert_bin_oper(bin: &BinaryObjectValue, current_context: &CurrentOperationContext) -> Vec<OperationUnit> {
    let bin_oper = convert_binary_op_type(&bin.op);
    vec![bin_oper]
}

fn parse_func_call(func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext,  cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let current_context = &mut current_context.clone();

    let args_objs = parse_func_cal_param(func_call, current_context, cur_obj.clone());
    let args_operations = args_objs.iter().flat_map(|o| convert_obj(o, current_context, cur_obj.clone())).collect::<Vec<_>>();
    let params_for_header = args_objs.iter().enumerate().map(|(i, o)| {
        FunctionArgData{
            is_val: is_val(o),
            ident: cur_obj.clone(),
            number: i as u32
        }
    }).collect::<Vec<_>>();
    current_context.last_operation.insert_many(args_operations.clone());
    let res_header = FuncResultHeader{
        size: calc_result_size_value(&func_call.result),
        cur_obj: cur_obj.clone(),
    };

    let func_cal_oper = vec![OperationUnit::FuncCall(FunctionPrimitiveVal{
        result_param: res_header,
        ident: cur_obj.clone(),
        params: params_for_header
    })];


    [args_operations, func_cal_oper].concat()
}

fn parse_func_cal_param (func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<ObjectData> {
    let args_operations = func_call.args.iter().enumerate().map(|(i, m)| {
        let obj_name: ObjectIdent = format!("arg{}", i);
        let data = ObjectData{
            name: obj_name.clone(),
            value: m.clone(),
            is_mutable: false,
            attrs: vec![]
        };
        data
    }).collect::<Vec<_>>();
    args_operations
}




fn parse_func_call_result (func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let result_operation = convert_object_data_value(&func_call.result, current_context, cur_obj.clone());
    result_operation
}

fn parse_func_val(func: &FunctionValue, current_context: &CurrentOperationContext,  cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let current_context = &mut current_context.clone();

    let params = parse_func_params(func, current_context, cur_obj.clone());


    let func_oper = vec![OperationUnit::FuncStart(FunctionPrimitiveVal{
        ident: cur_obj.clone(),
        result_param: FuncResultHeader{
            cur_obj: vec![func.result.name.clone()],
            size: calc_func_result_size(&func.result),
        },
        params: params.clone(),
    })];


    let params = params.iter().map(|p| OperationUnit::FuncInitArg(p.clone())).collect::<Vec<_>>();

    current_context.last_operation.insert_many(params.clone());
    let scope_operations = convert_scope(&func.scope, current_context, cur_obj.clone());


    current_context.last_operation.insert_many(scope_operations.clone());
    let end_oper = parse_func_result(&func, current_context, cur_obj.clone());


    [func_oper, params, scope_operations, end_oper].concat()
}

fn calc_func_result_size(result: &ObjectData) -> i32{
    calc_result_size_value(&result.value)
}

fn calc_result_size_value(value: &ObjectDataValue) -> i32 {
    match &value {
        ObjectDataValue::Object(obj) => {
            let props = obj.props.iter().map(|p| calc_func_result_size(p)).sum::<i32>();
            props
        }
        ObjectDataValue::AnotherObject(an_obj) => {
            calc_func_result_size(&an_obj.obj)
        }
        ObjectDataValue::Literal(lit) => {
            match lit {
                LiteralValue::Str(str) => {
                    let vec = str_to_vec_i64(str).len() * size_of::<AlignedData>();
                    vec as i32
                }
                LiteralValue::Bool(bo   ) => {
                    size_of::<AlignedData>() as i32
                }
                LiteralValue::Null => {
                    size_of::<AlignedData>() as i32
                }
                LiteralValue::Num(_) => {
                    size_of::<AlignedData>() as i32
                }
            }
        }
        ObjectDataValue::Function(_) => {
            size_of::<AlignedData>() as i32
        }
        ObjectDataValue::FunctionCall(res) => {
            calc_result_size_value(&res.result)
        }
        ObjectDataValue::Binary(bin) => {
            size_of::<AlignedData>() as i32
        }
        ObjectDataValue::Import(imports) => {
            imports.imports.iter().map(|o| calc_func_result_size(&o.obj)).sum::<i32>()
        }
        ObjectDataValue::Export(exports) => {
            exports.exports.iter().map(|o| calc_func_result_size(&o.obj)).sum::<i32>()
        }
    }
}



fn parse_func_result(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    vec![OperationUnit::FuncResultEnd(FuncResultHeader{
        size: calc_func_result_size(&func.result),
        cur_obj: cur_obj.clone(),
    })]
}




fn parse_func_params(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<FunctionArgData> {
    let mut mut_arg_index = 0;
    let params = func.params.iter().map(|p| {
        let cur_obj = vec![p.name.clone()] as ObjectIdentFull;
        let arg = FunctionArgData{
            ident: cur_obj.clone(),
            number: mut_arg_index,
            is_val: is_val(p)
        };
        mut_arg_index += 1;
        arg
    }).collect::<Vec<_>>();
    params
}

fn is_val(p: &ObjectData) -> bool {
     is_val_obj_val(&p.value)
}

fn is_val_obj_val(p_val: &ObjectDataValue) -> bool {
    match p_val {
        ObjectDataValue::Object(_) => false,
        ObjectDataValue::AnotherObject(_) => false,
        ObjectDataValue::Literal(_) => true, 
        ObjectDataValue::Function(_) => false,
        ObjectDataValue::FunctionCall(func_call) => is_val_obj_val(&func_call.result),
        ObjectDataValue::Binary(_) => false,
        ObjectDataValue::Import(_) => false,
        ObjectDataValue::Export(_) => false
    }
}



fn convert_binary_op_type(
    binary_op_type: &BinaryOpType,
) -> OperationUnit {
    match binary_op_type {
        BinaryOpType::EqEq => OperationUnit::EqEq,
        BinaryOpType::NotEq => OperationUnit::NotEq,
        BinaryOpType::Lt => OperationUnit::Lt,
        BinaryOpType::LtEq => OperationUnit::LtEq,
        BinaryOpType::Gt => OperationUnit::Gt,
        BinaryOpType::GtEq => OperationUnit::GtEq,
        BinaryOpType::Add => OperationUnit::Add,
        BinaryOpType::Sub => OperationUnit::Sub,
        BinaryOpType::Mul => OperationUnit::Mul,
        BinaryOpType::Div => OperationUnit::Div,
        BinaryOpType::Mod => OperationUnit::Mod,
        BinaryOpType::LogicalOr => OperationUnit::LogicalOr,
        BinaryOpType::LogicalAnd => OperationUnit::LogicalAnd,
        BinaryOpType::NullishCoalescing => {
            OperationUnit::NullishCoalescing
        }
    }
}


fn convert_another_object(another_obj: &AnotherObjectValue, current_context: &CurrentOperationContext, curr_owner: ObjectIdentFull, is_export:bool, is_import: bool) -> Vec<OperationUnit> {

    let call_chain_start = OperationUnit::CallChainStart;

    let mut full_ident = &mut vec![];

    let current_context = &mut current_context.clone();

    let mut data = vec![];
    let mut opers = vec![];
    another_obj.path.iter().for_each(|m| {
        match m {
            AnotherObjectValuePath::Ident(ident) => {
                full_ident.push(ident.clone());
                data = full_ident.clone();
            }
            AnotherObjectValuePath::FunctionCall(func_call) => {
                let func_call = ObjectDataValue::FunctionCall(func_call.clone());
                opers = convert_object_data_value(&func_call, current_context, full_ident.clone());
                full_ident.push("result".to_string());
                data = full_ident.clone();
                full_ident.clear();
            }
        }
    });

    if is_export{
        vec![OperationUnit::Export(full_ident.clone())]
    }
    else if is_import {
        vec![OperationUnit::Import(full_ident.clone())]
    }
    else {
        [opers, vec![OperationUnit::AnotherObjectVal(AnotherObjectVal{
            data: data,
            ident: curr_owner,
        })]].concat()
    }

}
fn convert_obj_val(object_val: &ObjectValue, current_ctx: &CurrentOperationContext, current_owner_ident: ObjectIdentFull) -> Vec<OperationUnit> {
    let oper = object_val.props.iter().flat_map(|prop| {
        let current_context = &mut current_ctx.clone();

        let current_owner_ident = [current_owner_ident.clone(), vec![prop.name.clone()]].concat();


        let res = convert_object_data_value(&prop.value, current_context, current_owner_ident.clone());

        res
    }).collect::<Vec<_>>();
    oper
}




fn convert_literal(literal: &LiteralValue, current_id: ObjectIdentFull) -> Vec<OperationUnit> {
    match &literal {
        LiteralValue::Str(val) => {
            vec![OperationUnit::Str(PrimitiveValStr{
                data: str_to_vec_i64(val),
                ident: current_id.clone(),
            })]
        },
        LiteralValue::Bool(val) => {
            vec![]
        },
        LiteralValue::Null => {
            vec![]
        },
        LiteralValue::Num(val) => {
            vec![OperationUnit::Num(PrimitiveValNum{
                data: num_to_i64(val),
                ident: current_id.clone(),
            })]
        },
    }
}
fn num_to_i64(val: &LitValueNum) -> AlignedData{
    val.val as i64
}

fn null_as_i64() -> AlignedData {
    0i64
}

fn str_to_vec_i64(val: &LitValueString) -> Vec<AlignedData> {
    let bytes = align(val.val.clone().into_bytes());
    assert_eq!(bytes.len() % 8, 0, "len must be multiple of 8");

    bytes
        .chunks_exact(8)
        .map(|chunk| i64::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}





fn align(bytes: Vec<u8>) -> Vec<u8> {
    let padding = (8 - bytes.len() % 8) % 8;

    bytes
        .into_iter()
        .chain(std::iter::repeat_n(0, padding))
        .collect()
}




fn bool_to_i64(val: &LitValueBool) -> AlignedData {
    val.val as i64
}




fn convert_statement(stmt: &Statement, current_ctx: &CurrentOperationContext) -> Vec<OperationUnit> {
    match stmt {
        Statement::Object(obj) => {
            convert_obj_as_stmt(obj, current_ctx,  vec![])
        }
        Statement::ObjectValue(obj_val) => {
            convert_object_data_value(&obj_val, current_ctx, vec![])
        }
        Statement::Conditional(cond) => {
            convert_conditional(cond, current_ctx, vec![])
        }
        Statement::Loop(loop_scope) => {
            convert_loop(loop_scope, current_ctx, vec![])
        }
        Statement::Scope(scope) => {
            convert_scope(scope, current_ctx, vec![])
        }
    }
}

fn convert_obj_as_stmt(obj: &ObjectData, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let obj_res = convert_obj(&obj, current_ctx,  cur_obj.clone());

    if cur_obj == vec!["result"]{
        [obj_res].concat()
    }
    else {
        obj_res
    }
}

fn convert_conditional(cond: &ConditionStatement, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let mut scope_oper = OperationUnit::IfScope;
    let cond_operations = convert_cond(&cond.cond, current_ctx, cur_obj.clone());
    let true_scope_operations = convert_scope(&cond.true_scope, current_ctx, cur_obj.clone());
    let false_scope = match &cond.false_scope {
        None => vec![],
        Some(false_scope) => {
            let else_scope = OperationUnit::ElseScope;
            let false_scope_operations = convert_scope(&*false_scope, current_ctx, cur_obj.clone());
            [vec![OperationUnit::IfScope], false_scope_operations].concat()
        }
    };
    [vec![scope_oper], cond_operations, true_scope_operations, false_scope].concat()
}

fn convert_loop(loop_scope: &LoopStatement, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let loop_scope_oper = OperationUnit::LoopScope;
    let loop_cond = convert_cond(&loop_scope.cond, current_ctx, cur_obj.clone());
    let loop_scope_operations = convert_scope(&loop_scope.loop_scope, current_ctx, cur_obj.clone());
    [vec![loop_scope_oper], loop_cond, loop_scope_operations].concat()
}


fn convert_cond(cond: &ObjectDataValue, current_ctx: &CurrentOperationContext, owner_unit_id: ObjectIdentFull) -> Vec<OperationUnit> {
    let start_cond = OperationUnit::StartCond;


    let cond_operations = convert_object_data_value(cond, current_ctx, ObjectIdentFull::default());
    let end_cond = OperationUnit::EndCond;

    [vec![start_cond], cond_operations, vec![end_cond]].concat()
}

fn convert_scope(scope: &ScopeStatement, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let start_scope = OperationUnit::StartScope;
    let d =  scope.statements.iter().flat_map(|s| convert_statement(s, current_ctx)).collect::<Vec<_>>();
    let end_scope = OperationUnit::EndScope;
    [vec![start_scope], d, vec![end_scope]].concat()
}




fn calc_offset(scopy_project: &ScopyProject) -> Vec<FunctionLayout>{
    let ctx =  OffsetCalcCtx::default();
    let data = scopy_project.modules.iter().flat_map(|module| {
        let data = module.statements.iter().map(|statement| {
            let layout = calc_layout_statement(statement, &ctx);
            layout
        }).collect::<Vec<_>>();
        let layouts = data.iter().flat_map(|(obj, _)| {
            obj.clone()
        }).collect::<Vec<_>>();

        let funcs = data.iter().flat_map(|(_, fns)| {
            let funcs = fns.clone();
            funcs
        }).collect::<Vec<_>>();
        let int_func = FunctionLayout{

        };

        let data = [funcs, vec![int_func]].concat();
        data
    }).collect::<Vec<_>>();




    data

}

fn layout_to_operations(layouts: ObjectLayout){
    layouts.iter().flat_map(|data|{
        data
    });
    todo!()
}

fn calc_layout_statement(statement: &Statement, ctx: &OffsetCalcCtx) -> (ObjectLayout, ModuleLayout){
    match statement {
        Statement::Object(obj) => {
            let d = calc_offset_obj(obj, ctx);
            d
        }
        Statement::ObjectValue(val) => {
            let d = calc_offset_obj_val(&val, ctx);
            d
        }
        Statement::Scope(scope) => {
            let d = scope.statements.iter().flat_map(|s| {
                let d = calc_layout_statement(&s, ctx);
                d
            }).collect::<Vec<_>>();
            d
        }
        //todo: remove
        Statement::Conditional(_) => {}
        Statement::Loop(_) => {}
    }
}




fn calc_offset_obj(obj: &ObjectData, ctx: &OffsetCalcCtx) -> (ObjectLayout, ModuleLayout){
    let obj_ident = [ctx.cur_ident.clone(), vec![obj.name.clone()]].concat() as ObjectIdentFull;
    let ctx = &OffsetCalcCtx{
        cur_ident: obj_ident.clone(),
        ..ctx.clone()
    };
    let data_layout = calc_offset_obj_val(&obj.value, ctx);
    data_layout
}

fn calc_offset_obj_val(obj_val: &ObjectDataValue, ctx: &OffsetCalcCtx) -> (ObjectLayout, ModuleLayout){
    let data = match obj_val {
        ObjectDataValue::Object(obj) => {
            todo!()
        }
        ObjectDataValue::AnotherObject(_) => {todo!()}
        ObjectDataValue::Literal(lit) => {
            let base_data = match lit {
                LiteralValue::Str(str) => {
                    let str_len = str.val.as_bytes().len();
                    let num_size = size_of::<AlignSize>();
                    let data = str_to_vec_i64(str);
                    let str_layout = StrLayout{
                        offset: ctx.cur_offset + str_len,
                        str_data: data,
                        ident: ctx.cur_ident
                    };
                    BaseDataLayout::StrLayout(str_layout)
                }
                LiteralValue::Num(num) => {
                    let num_size = size_of::<AlignSize>();

                    let num_layout = NumLayout{
                        num_data: num.val as i64,
                        offset: ctx.cur_offset + num_size,
                        ident: ctx.cur_ident.clone()
                    };
                    BaseDataLayout:: NumLayout(num_layout)
                }
                //todo:remove
                LiteralValue::Bool(_) => {}
                LiteralValue::Null => {}
            };
            (vec![DataLayout::BaseData(base_data)], vec![])
        }
        ObjectDataValue::Function(func_val) => {
            func_val.scope.
            let func_layout = FunctionLayout{

            };
            
            (vec![DataLayout::BaseData(BaseDataLayout::FuncRef)], vec![])
        }
        ObjectDataValue::FunctionCall(func_call) => {
            let func_call_data_layout = FunctionResultLayout {
                ident: ctx.cur_ident,
                data
            };
            todo!()
        }
        ObjectDataValue::Import(_) => {}
        ObjectDataValue::Export(_) => {}

        //todo:remove
        ObjectDataValue::Binary(_) => {}
    };

    data
}





#[derive(Clone, Debug, Serialize)]
pub struct FunctionLayout{
    pub params_count: ObjectLayout,
    pub result: FunctionResultLayout,
    pub offset: usize,
    pub scope_body: ObjectLayout
}


pub type ModuleLayout = Vec<FunctionLayout>;
pub type ObjectLayout = Vec<DataLayout>;


#[derive(Clone, Debug, Serialize, Default)]
pub struct OffsetCalcCtx{
    pub cur_offset: usize,
    pub cur_ident: ObjectIdentFull,
    pub call_map: BTreeMap<ObjectIdentFull, usize>
}

#[derive(Clone, Debug, Serialize)]
pub struct FuncRefLayout{

}

#[derive(Clone, Debug, Serialize)]
pub enum BaseDataLayout{
    NumLayout(NumLayout),
    StrLayout(StrLayout),
    AnotherObjectLayout(AnotherObjectLayout),
    FuncRef
}

#[derive(Clone, Debug, Serialize)]
pub enum DataLayout{
    BaseData(BaseDataLayout),
    FunctionCall(FunctionResultLayout),
}



#[derive(Clone, Debug, Serialize)]
pub struct FunctionResultLayout {
    pub data: Vec<BaseDataLayout>,
    pub offset: usize,
    pub ident: ObjectIdentFull,
}


#[derive(Clone, Debug, Serialize)]
pub struct NumLayout{
    pub num_data: AlignedData,
    pub offset: usize,
    pub ident: ObjectIdentFull
}




#[derive(Clone, Debug, Serialize)]
pub struct StrLayout{
    pub str_data: Vec<AlignedData>,
    pub offset: usize,
    pub ident: ObjectIdentFull,
}

#[derive(Clone, Debug, Serialize)]
pub struct AnotherObjectLayout{
    pub obj_index_data: AlignedData,
    pub offset: usize,
    pub ident: ObjectIdentFull,
    pub another_obj_ident: ObjectIdentFull,
    pub another_obj_data_size: usize,
    pub another_obj_offset: usize,
    pub layout_type: AnotherObjectLayoutType
}


#[derive(Clone, Debug, Serialize)]
pub enum AnotherObjectLayoutType{
    NumLayout,
    StrLayout,
    ObjectLayout
}


