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
use crate::semantic::{AnotherObjectValue, AnotherObjectValuePath, BinaryObjectValue, BinaryOpType, ConditionStatement, FunctionCallResultValue, FunctionValue, LiteralValue, LoopStatement, ObjectData, ObjectDataValue, ObjectIdent, ObjectValue, ScopeStatement, ScopyModule, ScopyProject, Statement};

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



pub fn none_data () -> AlignedData{
    0u64.to_le_bytes().to_vec()
}




pub fn convert_to_lowering_ir(scopy_project: &ScopyProject) -> Vec<OperationUnit>{
    let mut current_ctx =  &mut CurrentOperationContext::default();
    let start_program = OperationUnit::StartProgram;
    current_ctx.last_operation.push(start_program.clone());
    let t =scopy_project.modules.iter().flat_map(|module| {

        let module_ident = vec![module.name.clone()] as ObjectIdentFull;

        let module_id = OperationUnit::ModuleId(module_ident.clone());

        current_ctx.last_operation.push(module_id.clone());
        let operation_units = module.statements.iter().flat_map(|stmt|{
            let res = convert_statement(&stmt, current_ctx);
            current_ctx.last_operation.insert_many(res.clone());
            res
        }).collect::<Vec<_>>();




        [vec![module_id], operation_units].concat()

    }).collect::<Vec<_>>();
    
    let end_program = OperationUnit::EndProgram;
    
    
    [vec![start_program], t, vec![end_program]].concat()

}

// impl OperationUnit {
//
//     pub fn new(op_type: OperationUnit, id: ObjectIdentFull) -> Self {
//         OperationUnit {
//             operation_type: op_type,
//             owner_id: id,
//         }
//     }
// }


type AlignSize = i64;
pub type ObjectIdentFull = Vec<ObjectIdent>;

// #[derive(Clone, PartialEq, Debug, Serialize, Eq)]
// pub struct OwnerUnitIdIdentMap{
//     pub idents: Vec<ObjectIdent>,
//     pub id: OwnerUnitId,
// }

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct PrimitiveVal{
    pub data: Vec<u8>,
    pub ident: ObjectIdentFull
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct AnotherObjectVal{
    pub ident: ObjectIdentFull,
    pub data: ObjectIdentFull
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum OperationUnit {
    CallChainStart,
    CallChainEnd,
    Function(ObjectIdentFull),
    ModuleId(ObjectIdentFull),
    FuncResult(ObjectIdentFull),
    FuncArg(ObjectIdentFull),
    FunctionCall(ObjectIdentFull),
    FuncInit,
    FuncParam,
    AnotherObject(ObjectIdentFull),
    AnotherObjectVal(AnotherObjectVal),
    Str(PrimitiveVal),
    Bool(PrimitiveVal),
    Null(PrimitiveVal),
    Num(PrimitiveVal),
    IfScope,
    LoopScope,
    ElseScope,
    StartScope,
    EndScope,
    StartCond,
    EndCond,
    EmptyScope,
    EndModule,

    StartProgram,
    EndProgram,

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
pub enum ChainedTagIdent{
    Object(ObjectIdentFull),
    Function(ObjectIdentFull),
    ModuleId(ObjectIdentFull),
    FuncResult(ObjectIdentFull),
    FuncArg(ObjectIdentFull),
    AnotherObject(ObjectIdentFull),
    FunctionCall(ObjectIdentFull),
}


type AlignedData = Vec<u8>;

// #[derive(Clone, PartialEq, Debug, Serialize, Eq)]
// pub struct OperationUnit {
//     pub operation_type: OperationUnit
// }



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum LitOperationType {
    Str(Vec<u8>),
    Bool(Vec<u8>),
    Null(Vec<u8>),
    Num(Vec<u8>)
}




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
    // let obj_id = cur_owner_id + 1;


    // let oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), cur_obj.clone())];

    // cur_mtx.last_operation.insert_many(oper.clone());

    let res = convert_object_data_value(&obj.value, cur_mtx, cur_obj.clone());


    // [oper, res.clone()].concat()
    res
}

fn convert_object_data_value(
    obj_data_val: &ObjectDataValue,
    current_context: &CurrentOperationContext,
    current_obj: ObjectIdentFull) -> Vec<OperationUnit>{



    let d = match &obj_data_val{
        ObjectDataValue::Enum(_) => todo!(),
        ObjectDataValue::Object(object_val) => {
            convert_obj_val(&object_val, current_context, current_obj)
        },
        ObjectDataValue::AnotherObject(another_obj) => {
            convert_another_object(another_obj, current_context, current_obj)
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
    };
    d
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
    // let call_func_oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), cur_obj.clone())];
    //
    // current_context.last_operation.insert_many(call_func_oper.clone());

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
    // let call_unit = vec![OperationUnit::new(chained_tags.clone(), none_data(), cur_obj.clone())];

    let current_context = &mut current_context.clone();

    // current_context.last_operation.insert_many(call_unit.clone());
    let params = parse_func_params(func, current_context, cur_obj.clone());


    current_context.last_operation.insert_many(params.clone());
    let result_unit = parse_func_result(&func, current_context, cur_obj.clone());

    current_context.last_operation.insert_many(result_unit.clone());
    let scope_operations = convert_scope(&func.scope, current_context, cur_obj.clone());




    [params, scope_operations, result_unit].concat()
}




fn parse_func_result(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let oper = OperationUnit::FuncResult(cur_obj.clone());
    let oper_unit = convert_obj(&func.result, current_context,  cur_obj.clone());
    [vec![oper], oper_unit].concat()
}

fn parse_func_params(func: &FunctionValue, current_context: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let params = func.params.iter().flat_map(|p| {
        let cur_obj = [cur_obj.clone(), vec![p.name.clone()]].concat();
        let oper_unit = OperationUnit::FuncArg(cur_obj.clone());
        let obj = convert_obj(p, current_context, cur_obj.clone());
        [vec![oper_unit], obj].concat()
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


fn convert_another_object(another_obj: &AnotherObjectValue, current_context: &CurrentOperationContext, curr_owner: ObjectIdentFull) -> Vec<OperationUnit> {

    let call_chain_start = OperationUnit::CallChainStart;

    let mut chained_path = &mut vec![]  ;
    // let mut func_cal = &mut vec![];

    let current_context = &mut current_context.clone();


    let opers = another_obj.path.iter().for_each(|m| {
        match m {
            AnotherObjectValuePath::Ident(ident) => {
                chained_path.push(ident.clone());

            }
            AnotherObjectValuePath::FunctionCall(func_call) => {
                let func_call = ObjectDataValue::FunctionCall(func_call.clone());
                let opers = convert_object_data_value(&func_call, current_context, curr_owner.clone());

            }
        }
    });


    if curr_owner.is_empty(){
        vec![OperationUnit::AnotherObject(chained_path.clone())]
    }
    else {
        vec![OperationUnit::AnotherObjectVal(AnotherObjectVal{
            data: chained_path.clone(),
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
            vec![OperationUnit::Str(PrimitiveVal{
                data: val.val.clone().into_bytes(),
                ident: current_id.clone(),
            })]
        },
        LiteralValue::Bool(val) => {
            vec![OperationUnit::Bool(PrimitiveVal{
               data: bool_to_bytes(val.val.clone()),
                ident: current_id.clone(),
            })]
        },
        LiteralValue::Null => {
            vec![OperationUnit::Null(PrimitiveVal{
                data: AlignSize::default().to_le_bytes().to_vec(),
                ident: current_id.clone(),
            })]
        },
        LiteralValue::Num(val) => {
            vec![OperationUnit::Num(PrimitiveVal{
                data: val.val.to_le_bytes().to_vec(),
                ident: current_id.clone(),
            })]
        },
    }
}




fn align(bytes: Vec<u8>) -> AlignedData {
    let padding = (8 - bytes.len() % 8) % 8;

    bytes
        .into_iter()
        .chain(std::iter::repeat_n(0, padding))
        .collect()
}




fn bool_to_bytes(value: bool) -> AlignedData {
    (value as AlignSize).to_le_bytes().to_vec()
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
    obj_res
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
    // let s = OperationUnit::new(chained_tags,  none_data(), owner_unit_id);

    // [cond_operations.clone(), vec![s]].concat()
    [vec![start_cond], cond_operations, vec![end_cond]].concat()
}

fn convert_scope(scope: &ScopeStatement, current_ctx: &CurrentOperationContext, cur_obj: ObjectIdentFull) -> Vec<OperationUnit> {
    let start_scope = OperationUnit::StartScope;
    let d =  scope.statements.iter().flat_map(|s| convert_statement(s, current_ctx)).collect::<Vec<_>>();
    let end_scope = OperationUnit::EndScope;
    [vec![start_scope], d, vec![end_scope]].concat()
}

