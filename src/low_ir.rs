#![allow(unused)]


use std::collections::HashMap;
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
    FuncCall(ObjectIdentFull),
    FuncParam,
    AnotherObject(ObjectIdentFull),
    AnotherObjectVal(AnotherObjectVal),
    Str(PrimitiveValStr),
    Bool(PrimitiveValBool),
    Null(PrimitiveValNull),
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
pub struct FuncResultHeader {
    pub semantic_data: ObjectData,
    pub cur_obj: ObjectIdentFull
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionArgData{
    pub ident: ObjectIdentFull,
    pub number: u32,
    pub semantic_data: ObjectData
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionPrimitiveVal{
    pub ident: ObjectIdentFull,
    pub params_count: ParamsCount,
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
        ObjectDataValue::Enum(_) => todo!(),
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

    let args_operations = parse_func_cal_param(func_call, current_context, cur_obj.clone());
    current_context.last_operation.insert_many(args_operations.clone());
    let result_operation = parse_func_call_result(&func_call, current_context, cur_obj.clone());

    [args_operations, result_operation].concat()
}

fn parse_func_cal_param (func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let args_operations = func_call.args.iter().flat_map(|m| {
        let oper = OperationUnit::FuncParam;
        let res = convert_object_data_value(m, current_context, cur_obj.clone());
        [vec![oper], res].concat()
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
        params_count: func.params.len() as ParamsCount,
        result_param: FuncResultHeader{
            cur_obj: cur_obj.clone(),
            semantic_data: *func.result.clone(),
        }
    })];

    let scope_operations = convert_scope(&func.scope, current_context, cur_obj.clone());

    current_context.last_operation.insert_many(params.clone());
    let end_oper = parse_func_result(&func, current_context, cur_obj.clone());


    [func_oper, params, scope_operations, end_oper].concat()
}




fn parse_func_result(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    vec![OperationUnit::FuncResultEnd(FuncResultHeader{
        semantic_data: *func.result.clone(),
        cur_obj: cur_obj.clone(),
    })]
}




fn parse_func_params(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let mut mut_arg_index = 0;
    let params = func.params.iter().flat_map(|p| {
        let cur_obj = vec![p.name.clone()] as ObjectIdentFull;
        let oper_unit = OperationUnit::FuncInitArg(FunctionArgData{
            ident: cur_obj.clone(),
            number: mut_arg_index,
            semantic_data: p.clone()
        });
        mut_arg_index += 1;
        vec![oper_unit]
    }).collect::<Vec<_>>();
    params
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


    let opers = another_obj.path.iter().for_each(|m| {
        match m {
            AnotherObjectValuePath::Ident(ident) => {
                full_ident.push(ident.clone());

            }
            AnotherObjectValuePath::FunctionCall(func_call) => {
                let func_call = ObjectDataValue::FunctionCall(func_call.clone());
                let opers = convert_object_data_value(&func_call, current_context, curr_owner.clone());

            }
        }
    });

    if is_export{
        vec![OperationUnit::Export(full_ident.clone())]
    }
    else if is_import {
        vec![OperationUnit::Import(full_ident.clone())]
    }
    else if curr_owner.is_empty(){
        vec![OperationUnit::AnotherObject(full_ident.clone())]
    }
    else {
        vec![OperationUnit::AnotherObjectVal(AnotherObjectVal{
            data: full_ident.clone(),
            ident: curr_owner,
        })]
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
            vec![OperationUnit::Bool(PrimitiveValBool{
                data: bool_to_i64(val),
                ident: current_id.clone(),
            })]
        },
        LiteralValue::Null => {
            vec![OperationUnit::Null(PrimitiveValNull {
                data: null_as_i64(),
                ident: current_id.clone(),
            })]
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

