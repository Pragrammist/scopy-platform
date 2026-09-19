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
use crate::low_ir::ChainedTag::BinOperation;
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
fn main() {
    // let mut module = Module::new();
    //
    // // types
    // let mut types = TypeSection::new();
    //
    // types
    //     .ty()
    //     .function(
    //         [ValType::I32, ValType::I32],
    //         [ValType::I32],
    //     );
    //
    // module.section(&types);
    //
    // // functions
    // let mut functions = FunctionSection::new();
    // functions.function(0);
    //
    // module.section(&functions);
    //
    // // code
    // let mut code = CodeSection::new();
    //
    // let mut func = Function::new([]);
    //
    // func.instruction(&Instruction::LocalGet(0));
    // func.instruction(&Instruction::LocalGet(1));
    // func.instruction(&Instruction::I32Add);
    // func.instruction(&Instruction::End);
    //
    // code.function(&func);
    //
    // module.section(&code);
    //
    // // готовый .wasm
    // let wasm: Vec<u8> = module.finish();
    //
    // std::fs::write("test.wasm", wasm).unwrap();
}


pub fn none_data () -> AlignedData{
    0u64.to_le_bytes().to_vec()
}




pub fn convert_to_lowering_ir(scopy_project: &ScopyProject) -> Vec<OperationUnit>{
    let mut current_ctx =  &mut CurrentOperationContext::default();

    let mut start_owner_id = 0i64 as OwnerUnitId;
    let t =scopy_project.modules.iter().flat_map(|module| {



        let chained_tags = vec![ChainedTag::Ident(ChainedTagIdent::ModuleId(start_owner_id))];


        let operation_units = module.statements.iter().flat_map(|stmt|{
            let res = convert_statement(&stmt, current_ctx, chained_tags.clone(), start_owner_id);
            current_ctx.last_operation.insert_many(res.clone());
            res
        }).collect::<Vec<_>>();

        start_owner_id = start_owner_id + 1 as OwnerUnitId;



        operation_units

    }).collect::<Vec<_>>();
    t

}

impl OperationUnit {

    pub fn new(chained_tags: Vec<ChainedTag>, data: AlignedData, id: OwnerUnitId) -> Self {
        let aligned = align(data);
        OperationUnit {
            data: aligned.clone(),
            size: aligned.clone().len() as AlignSize,
            chained_tags: chained_tags,
            owner_id: id,
        }
    }
}


type AlignSize = i64;
pub type OwnerUnitId = Vec<ObjectIdent>;

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct OwnerUnitIdIdentMap{
    pub idents: Vec<ObjectIdent>,
    pub id: OwnerUnitId,
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum ChainedTag {
    CallChainStart,
    CallChainEnd,
    Ident(ChainedTagIdent),
    FuncInit,
    BinOperation(BinaryOpTypeOperation),
    LitOperation(LitOperationType),
    IfScope,
    LoopScope,
    ElseScope,
    StartScope,
    EndScope,
    StartCond,
    EndCond,
    EmptyScope,
    OwnerIdMap(OwnerUnitIdIdentMap)
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum ChainedTagIdent{
    Object(OwnerUnitId),
    Function(OwnerUnitId),
    ModuleId(OwnerUnitId),
    FuncResult(OwnerUnitId),
    Property(OwnerUnitId),
    FuncArg(OwnerUnitId),
    AnotherObject(OwnerUnitId),
    FunctionCall(OwnerUnitId),
}


type AlignedData = Vec<u8>;

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct OperationUnit {
    pub owner_id: OwnerUnitId,
    pub chained_tags: Vec<ChainedTag>,
    pub data: AlignedData,
    pub size: AlignSize
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum LitOperationType {
    Str,
    Bool,
    Null,
    Num
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "bin-op-type", content = "bin-op-type-data")]
pub enum BinaryOpTypeOperation {
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

fn convert_obj (obj: &ObjectData, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, cur_owner_id: OwnerUnitId) -> Vec<OperationUnit> {

    let cur_mtx = &mut current_ctx.clone();

    // let obj_id = cur_owner_id + 1;
    let obj_id = cur_owner_id;
    let chained_tags = [
        chained_tags,
        vec![ChainedTag::OwnerIdMap(OwnerUnitIdIdentMap{ id: obj_id, idents: vec![obj.name.clone()]
        }), ]].concat();
    let oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), obj_id)];

    cur_mtx.last_operation.insert_many(oper.clone());

    let res = convert_object_data_value(&obj.value, cur_mtx, chained_tags.clone(), obj_id);


    [oper, res.clone()].concat()
}

fn convert_object_data_value(
    obj_data_val: &ObjectDataValue,
    current_context: &CurrentOperationContext,
    chained_tags: Vec<ChainedTag>,
    current_id: OwnerUnitId) -> Vec<OperationUnit>{



    let d = match &obj_data_val{
        ObjectDataValue::Enum(_) => todo!(),
        ObjectDataValue::Object(object_val) => {
            convert_obj_val(&object_val, current_context, chained_tags,current_id)
        },
        ObjectDataValue::AnotherObject(another_obj) => {
            convert_another_object(another_obj, current_context, chained_tags, current_id)
        },
        ObjectDataValue::Literal(literal) => {
            convert_literal(literal, chained_tags, current_id)
        },
        ObjectDataValue::Function(func) => {
            parse_func_val(func, current_context, chained_tags, current_id)
        }
        ObjectDataValue::FunctionCall(func_call) => {
            parse_func_call(func_call, current_context, chained_tags, current_id)
        }
        ObjectDataValue::Binary(bin) => {
            convert_bin(bin, current_context, chained_tags, current_id)
        }
    };
    d
}

fn convert_bin(bin: &BinaryObjectValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let cur_ctx = &mut current_context.clone();
    let v1 = convert_object_data_value(&bin.v1, cur_ctx, chained_tags.clone(), current_id);
    cur_ctx.last_operation.insert_many(v1.clone());
    let v2 = convert_object_data_value(&bin.v2, cur_ctx, chained_tags.clone(), current_id);
    cur_ctx.last_operation.insert_many(v2.clone());
    let operation_unit = convert_bin_oper(bin, cur_ctx, chained_tags.clone(), current_id);
    let bin_oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), current_id)];
    [bin_oper, v1, operation_unit,v2].concat()
}

fn convert_bin_oper(bin: &BinaryObjectValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let bin_oper = convert_binary_op_type(&bin.op);
    let chained_tags = [chained_tags.clone(), vec![BinOperation(bin_oper)]].concat();
    let operation_unit = OperationUnit::new(
        chained_tags, none_data(), current_id
    );
    vec![operation_unit]
}

fn parse_func_call(func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let current_context = &mut current_context.clone();
    let call_func_oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), current_id.clone())];

    current_context.last_operation.insert_many(call_func_oper.clone());

    let args_operations = parse_func_cal_param(func_call, current_context, chained_tags.clone(), current_id);
    current_context.last_operation.insert_many(args_operations.clone());
    let result_operation = parse_func_call_result(&func_call, current_context, chained_tags.clone(), current_id);
    [call_func_oper, args_operations, result_operation].concat()
}

fn parse_func_cal_param (func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let mut cur_virtual_obj: i64 = current_id as i64;
    let args_operations = func_call.args.iter().flat_map(|m| {
        cur_virtual_obj += 1;
        let chained_tag = [chained_tags.clone(), vec![ChainedTag::Ident(ChainedTagIdent::FunctionCall(cur_virtual_obj))]].concat();
        let res = convert_object_data_value(m, current_context, chained_tags.clone(), cur_virtual_obj);
        res
    }).collect::<Vec<_>>();
    args_operations
}

fn parse_func_call_result (func_call: &FunctionCallResultValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let chained_tags = vec![
        vec![ChainedTag::Ident(ChainedTagIdent::FuncResult(current_id))],
        chained_tags.clone()
    ].concat();
    let result_operation = convert_object_data_value(&func_call.result, current_context, chained_tags.clone(), current_id);
    result_operation
}

fn parse_func_val(func: &FunctionValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let call_unit = vec![OperationUnit::new(chained_tags.clone(), none_data(), current_id.clone())];

    let current_context = &mut current_context.clone();

    current_context.last_operation.insert_many(call_unit.clone());
    let params = parse_func_params(func, current_context, chained_tags.clone(), current_id);


    current_context.last_operation.insert_many(params.clone());
    let result_unit = parse_func_result(&func, current_context, chained_tags.clone(), current_id);

    current_context.last_operation.insert_many(result_unit.clone());
    let scope_operations = convert_scope(&func.scope, current_context, chained_tags.clone(), current_id);


    let result_prop = fn_result_data(current_id, chained_tags.clone());

    [call_unit, result_unit, scope_operations, result_prop].concat()
}

fn fn_result_data(current_id: OwnerUnitId, chained_tags: Vec<ChainedTag>) -> Vec<OperationUnit> {
    let result_data = current_id.to_le_bytes().to_vec();
    let result_prop = OperationUnit::new(chained_tags.clone(), result_data, current_id);
    vec![result_prop]
}


fn parse_func_result(func: &FunctionValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let obj_id = current_id + 1 as OwnerUnitId;
    let chained_tags = [chained_tags.clone(), vec![ChainedTag::Ident(ChainedTagIdent::FuncResult(obj_id))]].concat();
    let oper_unit = convert_obj(&func.result, current_context, chained_tags.clone(), obj_id);
    oper_unit
}

fn parse_func_params(func: &FunctionValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    let obj_id = current_id + 1 as OwnerUnitId;
    let chained_tags = [chained_tags.clone(), vec![ChainedTag::Ident(ChainedTagIdent::FuncArg(obj_id))]].concat();
    let params = func.params.iter().flat_map(|p| convert_obj(p, current_context, chained_tags.clone(), obj_id)).collect::<Vec<_>>();
    params
}


fn convert_binary_op_type(
    binary_op_type: &BinaryOpType,
) -> BinaryOpTypeOperation {
    match binary_op_type {
        BinaryOpType::EqEq => BinaryOpTypeOperation::EqEq,
        BinaryOpType::NotEq => BinaryOpTypeOperation::NotEq,
        BinaryOpType::Lt => BinaryOpTypeOperation::Lt,
        BinaryOpType::LtEq => BinaryOpTypeOperation::LtEq,
        BinaryOpType::Gt => BinaryOpTypeOperation::Gt,
        BinaryOpType::GtEq => BinaryOpTypeOperation::GtEq,
        BinaryOpType::Add => BinaryOpTypeOperation::Add,
        BinaryOpType::Sub => BinaryOpTypeOperation::Sub,
        BinaryOpType::Mul => BinaryOpTypeOperation::Mul,
        BinaryOpType::Div => BinaryOpTypeOperation::Div,
        BinaryOpType::Mod => BinaryOpTypeOperation::Mod,
        BinaryOpType::LogicalOr => BinaryOpTypeOperation::LogicalOr,
        BinaryOpType::LogicalAnd => BinaryOpTypeOperation::LogicalAnd,
        BinaryOpType::NullishCoalescing => {
            BinaryOpTypeOperation::NullishCoalescing
        }
    }
}


fn convert_another_object(another_obj: &AnotherObjectValue, current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, curr_owner: OwnerUnitId) -> Vec<OperationUnit> {

    let chained_tags = [chained_tags, vec![ChainedTag::CallChainStart]].concat();

    let mut chained_path = &mut vec![];
    let obj_oper = vec![OperationUnit::new(chained_tags.clone(), none_data(), curr_owner)];

    let current_context = &mut current_context.clone();
    current_context.last_operation.insert_many(obj_oper.clone());


    let opers = another_obj.path.iter().flat_map(|m| {
        match m {
            AnotherObjectValuePath::Ident(ident) => {
                chained_path.push(ident.clone());

                let oper_id = convert_object_id_to_operation(current_context, chained_tags.clone(), chained_path.clone());
                vec![oper_id]
            }
            AnotherObjectValuePath::FunctionCall(func_call) => {
                let func_call = ObjectDataValue::FunctionCall(func_call.clone());
                let opers = convert_object_data_value(&func_call, current_context, chained_tags.clone(), curr_owner);
                opers
            }
        }
    }).collect::<Vec<_>>();

    let chained_tags = [chained_tags.clone(), vec![ChainedTag::CallChainEnd]].concat();


    [obj_oper, opers].concat()

}

fn convert_object_id_to_operation(current_context: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, ident_path: Vec<ObjectIdent>) -> OperationUnit{
    let id =  find_id_in_context(current_context, ident_path);
    let chained_tags = [chained_tags, vec![ChainedTag::Ident(ChainedTagIdent::AnotherObject(id.clone()))]].concat();
    OperationUnit::new(chained_tags, id.to_le_bytes().to_vec(), id)
}

fn find_id_in_context(current_context: &CurrentOperationContext, object_ident: Vec<ObjectIdent>) -> OwnerUnitId {

    let id = current_context.last_operation.iter().find_map(|o| {
        let tag = o.chained_tags.iter().find_map(|t| {
            match t {
                ChainedTag::OwnerIdMap(ident_map) => {
                    if ident_map.idents == object_ident{
                        Some(ident_map.id.clone())
                    } else {
                        None
                    }
                },
                _ => None
            }
        });
        tag
    }).unwrap_or_else(|| compiler_panic!(OperationExpr::NotFoundIdentExpr));
    id
}



fn new_owner_id(current_context: &CurrentOperationContext) -> OwnerUnitId {
    let ids = &mut current_context.last_operation.iter().flat_map(|o| {
        let tag = o.chained_tags.iter().filter_map(|t| {
            match t {
                ChainedTag::OwnerIdMap(ident_map) => Some(ident_map.id.clone()),
                _ => None
            }
        }).collect::<Vec<_>>();
        tag
    }).collect::<Vec<_>>();
    ids.sort();

    let last_id = ids.last().unwrap_or_else(|| compiler_panic!(OperationExpr::NotFoundOwnerId));

    let new_id = last_id + 1 as OwnerUnitId;

    new_id

}

fn find_id_in_context_by_id(current_context: &CurrentOperationContext, owner_unit_id: OwnerUnitId) -> Vec<ObjectIdent> {

    let id = current_context.last_operation.iter().find_map(|o| {
        let tag = o.chained_tags.iter().find_map(|t| {
            match t {
                ChainedTag::OwnerIdMap(ident_map) => {
                    if ident_map.id == owner_unit_id{
                        Some(ident_map.idents.clone())
                    } else {
                        None
                    }
                },
                _ => None
            }
        });
        tag
    }).unwrap_or_else(|| compiler_panic!(OperationExpr::NotFoundIdentExpr));



    id
}

fn convert_obj_val(object_val: &ObjectValue, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, current_owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    let oper = object_val.props.iter().flat_map(|prop| {
        let current_context = &mut current_ctx.clone();

        let current_owner_ident = find_id_in_context_by_id(current_context, current_owner_id);
        let cur_obj = [current_owner_ident.clone(), vec![prop.name.clone()]].concat();
        let new_ower_id = current_owner_id + 1 as OwnerUnitId;

        let cur_map = OwnerUnitIdIdentMap{
            idents: cur_obj,
            id: new_ower_id,
        };




        let d = vec![ChainedTag::Ident(ChainedTagIdent::Property(new_ower_id)), ChainedTag::OwnerIdMap(cur_map)];
        let chained_tags = [chained_tags.clone(), d].concat();


        let prop_opers = vec![OperationUnit::new(chained_tags.clone(), none_data(), new_ower_id)];
        current_context.last_operation.insert_many(prop_opers.clone());

        let res = convert_object_data_value(&prop.value, current_context, chained_tags.clone(), new_ower_id.clone());

        [prop_opers, res].concat()
    }).collect::<Vec<_>>();
    oper
}




fn convert_literal(literal: &LiteralValue, chained_tags: Vec<ChainedTag>, current_id: OwnerUnitId) -> Vec<OperationUnit> {
    match &literal {
        LiteralValue::Str(val) => {
            let chained_tags = [chained_tags.clone(), vec![ChainedTag::LitOperation(LitOperationType::Str)]].concat();
            vec![OperationUnit::new(chained_tags, val.val.clone().into_bytes(), current_id)]
        },
        LiteralValue::Bool(val) => {
            let chained_tags = [chained_tags.clone(), vec![ChainedTag::LitOperation(LitOperationType::Bool)]].concat();
            vec![OperationUnit::new(chained_tags, bool_to_bytes(val.val), current_id)]
        },
        LiteralValue::Null => {
            let chained_tags = [chained_tags.clone(), vec![ChainedTag::LitOperation(LitOperationType::Null)]].concat();
            vec![OperationUnit::new(chained_tags, AlignSize::default().to_le_bytes().to_vec(), current_id)]
        },
        LiteralValue::Num(val) => {
            let chained_tags = [chained_tags.clone(), vec![ChainedTag::LitOperation(LitOperationType::Num)]].concat();
            vec![OperationUnit::new(chained_tags, val.val.to_le_bytes().to_vec(), current_id)]
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




fn convert_statement(stmt: &Statement, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    match stmt {
        Statement::Object(obj) => {
            convert_obj_as_stmt(obj, current_ctx, chained_tags, owner_id)
        }
        Statement::ObjectValue(obj_val) => {
            convert_object_data_value(&obj_val, current_ctx, chained_tags, owner_id)
        }
        Statement::Conditional(cond) => {
            convert_conditional(cond, current_ctx, chained_tags, owner_id)
        }
        Statement::Loop(loop_scope) => {
            convert_loop(loop_scope, current_ctx, chained_tags, owner_id)
        }
        Statement::Scope(scope) => {
            convert_scope(scope, current_ctx, chained_tags, owner_id)
        }
    }
}

fn convert_obj_as_stmt(obj: &ObjectData, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    let obj_id = owner_id + 1 as OwnerUnitId;
    let chained_tags = [chained_tags.clone(), vec![ChainedTag::Ident(ChainedTagIdent::Object(obj_id))]].concat();
    convert_obj(&obj, current_ctx, chained_tags, owner_id)
}

fn convert_conditional(cond: &ConditionStatement, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    let chained_tags = [chained_tags.clone(), vec![ChainedTag::IfScope]].concat();
    let cond_operations = convert_cond(&cond.cond, chained_tags.clone(), current_ctx, owner_id);
    let true_scope_operations = convert_scope(&cond.true_scope, current_ctx, chained_tags.clone(), owner_id);
    let false_scope = match &cond.false_scope {
        None => vec![],
        Some(false_scope) => {
            let chained_tags = [chained_tags.clone(), vec![ChainedTag::ElseScope]].concat();
            let true_scope_operations = convert_scope(&*false_scope, current_ctx, chained_tags.clone(), owner_id);
            true_scope_operations
        }
    };
    [cond_operations, true_scope_operations, false_scope].concat()
}

fn convert_loop(loop_scope: &LoopStatement, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    let chained_tags = [chained_tags.clone(), vec![ChainedTag::LoopScope]].concat();
    let loop_cond = convert_cond(&loop_scope.cond, chained_tags.clone(), current_ctx, owner_id);
    let loop_scope_operations = convert_scope(&loop_scope.loop_scope, current_ctx, chained_tags.clone(), owner_id);
    [loop_cond, loop_scope_operations].concat()
}


fn convert_cond(cond: &ObjectDataValue, chained_tags: Vec<ChainedTag>, current_ctx: &CurrentOperationContext, owner_unit_id: OwnerUnitId) -> Vec<OperationUnit> {
    let chained_tags = [chained_tags, vec![ChainedTag::StartCond]].concat();


    let cond_operations = convert_object_data_value(cond, current_ctx, chained_tags.clone(), OwnerUnitId::default());
    let chained_tags = [chained_tags, vec![ChainedTag::EndCond]].concat();
    let s = OperationUnit::new(chained_tags,  none_data(), owner_unit_id);

    [cond_operations.clone(), vec![s]].concat()
}

fn convert_scope(scope: &ScopeStatement, current_ctx: &CurrentOperationContext, chained_tags: Vec<ChainedTag>, owner_id: OwnerUnitId) -> Vec<OperationUnit> {
    let chained_tags = [chained_tags, vec![ChainedTag::StartScope]].concat();
    let d =  scope.statements.iter().flat_map(|s| convert_statement(s, current_ctx, chained_tags.clone(), owner_id)).collect::<Vec<_>>();
    let chained_tags = [chained_tags, vec![ChainedTag::EndScope]].concat();
    let s = OperationUnit::new(chained_tags, none_data(), owner_id);
    [d, vec![s]].concat()
}

