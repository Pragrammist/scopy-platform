use std::collections::HashMap;
use serde::Serialize;
use wasm_encoder::{
    CodeSection,
    Function,
    FunctionSection,
    Instruction,
    Module,
    TypeSection,
    ValType,
};
use crate::low_ir::OperationType::BinOperation;
use crate::semantic::{AnotherObjectValue, BinaryOpType, LiteralValue, ObjectData, ObjectDataValue, ObjectIdent, ObjectValue, Statement};
use crate::StmtPanic;

macro_rules! compiler_panic {
    ($reason:expr) => {{
        let reason = $reason;
        eprintln!("Compiler panic: {reason}");
        std::panic::panic_any(reason);
    }};
}
fn main() {
    let mut module = Module::new();

    // types
    let mut types = TypeSection::new();

    types
        .ty()
        .function(
            [ValType::I32, ValType::I32],
            [ValType::I32],
        );

    module.section(&types);

    // functions
    let mut functions = FunctionSection::new();
    functions.function(0);

    module.section(&functions);

    // code
    let mut code = CodeSection::new();

    let mut func = Function::new([]);

    func.instruction(&Instruction::LocalGet(0));
    func.instruction(&Instruction::LocalGet(1));
    func.instruction(&Instruction::I32Add);
    func.instruction(&Instruction::End);

    code.function(&func);

    module.section(&code);

    // готовый .wasm
    let wasm: Vec<u8> = module.finish();

    std::fs::write("test.wasm", wasm).unwrap();
}







fn te () {
    let mut module = Module::new();



}


type AlignSize = i64;
pub type OwnerUnitId = i64;


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum OperationType {
    Ident(OwnerUnitId),
    Property(OwnerUnitId),
    FuncCall,
    FuncArg,
    FuncResult,
    FuncInit,
    BinOperation(BinaryOpTypeOperation),
    LitOperation(LitOperationType)
}


type AlignedData = Vec<u8>;

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct OperationUnit {
    pub owner_id: OwnerUnitId,
    pub operation_types: Vec<OperationType>,
    pub data: AlignedData,
    pub size: AlignSize
}


impl OperationUnit {

    pub fn new(operation_types: Vec<OperationType>, data: AlignedData, id: OwnerUnitId) -> Self {
        let aligned = align(data);
        OperationUnit {
            data: aligned.clone(),
            size: aligned.clone().len() as AlignSize,
            operation_types: operation_types,
            owner_id: id,
        }
    }
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



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct CurrentOperationContext {
    obj_table: HashMap<ObjectIdent, OwnerUnitId>
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum OperationExpr {
    NotFoundIdentExpr
}

impl std::fmt::Display for OperationExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            OperationExpr::NotFoundIdentExpr => write!(f, "Object ident not found in operation context"),
        }
    }
}

fn convert_obj (obj: &ObjectData, current_ctx: &CurrentOperationContext, operation_types: Vec<OperationType>) -> Vec<OperationUnit> {
    let obj_id = find_id_in_context(current_ctx, obj.name.clone());

    let operation_types = [operation_types, vec![OperationType::Ident(obj_id)]].concat();

    let res = convert_object_data_value(&obj.value, current_ctx, operation_types, obj_id);


    res
}

fn convert_object_data_value(
    obj_data_val: &ObjectDataValue,
    current_context: &CurrentOperationContext,
    operation_types: Vec<OperationType>,
    current_id: OwnerUnitId) -> Vec<OperationUnit>{



    let d = match &obj_data_val{
        ObjectDataValue::Enum(_) => todo!(),
        ObjectDataValue::Object(object_val) => {
            convert_obj_val(&object_val, current_context, operation_types);
            todo!()
        },
        ObjectDataValue::AnotherObject(another_obj) => {
            let d = convert_another_object(another_obj, current_context, operation_types);
            vec![d]
        },
        ObjectDataValue::Literal(literal) => {
            let d = convert_literal(literal, operation_types, current_id);
            vec![d]
        },
        ObjectDataValue::Function(func) => {
            let operation_types_for_params = [operation_types.clone(), vec![OperationType::FuncArg]].concat();



            let result_operation_types = vec![OperationType::Ident(current_id)];
            let result_data = current_id.to_le_bytes().to_vec();
            let result_prop = OperationUnit::new(result_operation_types, result_data, current_id);

            let oper_unit = convert_obj(&func.result, current_context, operation_types_for_params);
            [vec![result_prop],oper_unit].concat()
        }
        ObjectDataValue::FunctionCall(func_call) => {
            let operation_types = vec![
                vec![OperationType::FuncCall, OperationType::FuncArg],
                operation_types.clone()
            ].concat();

            let args_operations = func_call.args.iter().flat_map(|m| {
                let res = convert_object_data_value(m, current_context, operation_types.clone(), current_id);
                res
            }).collect::<Vec<_>>();


            let operation_types = vec![
                vec![OperationType::FuncResult],
                operation_types.clone()
            ].concat();
            let result_operation = convert_object_data_value(&func_call.result, current_context, operation_types.clone(), current_id);
            [args_operations, result_operation].concat()
        }
        ObjectDataValue::Binary(bin) => {
            let bin_oper = convert_binary_op_type(&bin.op);
            let operation_types = [operation_types.clone(), vec![BinOperation(bin_oper)]].concat();
            let v1 = convert_object_data_value(&bin.v1, current_context, operation_types.clone(), current_id);
            let v2 = convert_object_data_value(&bin.v2, current_context, operation_types.clone(), current_id);
            let operation_unit = OperationUnit::new(
                operation_types, AlignedData::default(), current_id
            );
            [v1,vec![operation_unit],v2].concat()
        }
    };
    d
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


fn convert_another_object(another_obj: &AnotherObjectValue, current_context: &CurrentOperationContext, operation_types: Vec<OperationType>) -> OperationUnit {
    let id =  find_id_in_context(current_context, another_obj.obj.name.clone());
    let operation_types = [operation_types, vec![OperationType::Ident(id.clone())]].concat();
    OperationUnit::new(operation_types, id.to_be_bytes().to_vec(), id)
}

fn find_id_in_context(current_context: &CurrentOperationContext, object_ident: ObjectIdent) -> OwnerUnitId {
    let obj = current_context.obj_table.get(&object_ident);


    if let Some(obj_id) = obj {
        *obj_id
    }
    else {
        compiler_panic!(OperationExpr::NotFoundIdentExpr);
    }
}

fn convert_obj_val(object_val: &ObjectValue, current_ctx: &CurrentOperationContext, operation_types: Vec<OperationType>) {
    object_val.props.iter().for_each(|prop| {
        let owner_id = find_id_in_context(current_ctx, prop.name.clone());
        let d = vec![OperationType::Property(owner_id)];
        let operation_types = [operation_types.clone(), d].concat();
        convert_object_data_value(&prop.value, current_ctx, operation_types, owner_id);
    });
}




fn convert_literal(literal: &LiteralValue, operation_types: Vec<OperationType>, current_id: OwnerUnitId) -> OperationUnit {
    match &literal {
        LiteralValue::Str(val) => {
            let operation_types = [operation_types.clone(), vec![OperationType::LitOperation(LitOperationType::Str)]].concat();
            OperationUnit::new(operation_types, val.val.clone().into_bytes(), current_id)
        },
        LiteralValue::Bool(val) => {
            let operation_types = [operation_types.clone(), vec![OperationType::LitOperation(LitOperationType::Bool)]].concat();
            OperationUnit::new(operation_types, bool_to_bytes(val.val), current_id)
        },
        LiteralValue::Null => {
            let operation_types = [operation_types.clone(), vec![OperationType::LitOperation(LitOperationType::Null)]].concat();
            OperationUnit::new(operation_types, AlignSize::default().to_le_bytes().to_vec(), current_id)
        },
        LiteralValue::Num(val) => {
            let operation_types = [operation_types.clone(), vec![OperationType::LitOperation(LitOperationType::Num)]].concat();
            OperationUnit::new(operation_types, val.val.to_le_bytes().to_vec(), current_id)
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




fn convert_statements(stmt: &Statement, current_ctx: &CurrentOperationContext) -> Vec<OperationUnit> {
    match stmt {
        Statement::Object(obj) => {
            convert_obj(&obj, current_ctx, vec![])
        }
        Statement::ObjectValue(obj_val) => {
            convert_object_data_value(&obj_val, current_ctx, vec![], OwnerUnitId::default())
        }
        Statement::Conditional(cond) => {
            todo!()
        }
        Statement::Loop(_) => {
            todo!()
        }
        Statement::Scope(scope) => {
            scope.statements.iter().flat_map(|s| convert_statements(s, current_ctx)).collect()
        }
        Statement::Import(import) => {
            todo!()
        }
        Statement::Export(_) => {
            todo!()
        }
    }
}

