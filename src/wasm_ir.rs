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
use crate::semantic::{AnotherObjectValue, CurrentContext, LiteralValue, ObjectData, ObjectDataValue, ObjectIdent, ObjectValue};
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

type NameIdentWasm = String;

pub type ObjectId = i32;

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ObjectIdentWasm{
    pub ident: NameIdentWasm,
    pub id: ObjectId
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum ObjectPathWasm{
    Ident(ObjectIdentWasm),
    Property(NameIdentWasm),
    FuncCall
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct  ObjectPropWasm {
    pub path: Vec<ObjectPathWasm>,
    pub data: ObjectValueWasm,

}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ObjectValueWasm{
    pub data: Vec<u8>,
    pub size: i64,
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ObjectDataWasm{
    pub id: ObjectId,
    pub data: Vec<ObjectPropWasm>,
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct WasmCurrentContext{
    obj_table: HashMap<ObjectIdent, ObjectId>
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum WasmExpr{
    NotFoundIdentExpr
}

impl std::fmt::Display for WasmExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            WasmExpr::NotFoundIdentExpr => write!(f, "Object ident not found in wasm context"),
        }
    }
}

fn convert_obj (obj: ObjectData, current_ctx: &WasmCurrentContext, path: &Vec<ObjectPathWasm>) ->  ObjectPropWasm{
    let d = convert_object_data_value(&obj.value, current_ctx, path);
    todo!()
}

fn convert_object_data_value(
        obj_data_val: &ObjectDataValue,
        current_context: &WasmCurrentContext,
        path: Vec<ObjectPathWasm>,
        result: &mut Vec<ObjectPropWasm>){



    let d = match &obj_data_val{
        ObjectDataValue::Enum(_) => todo!(),
        ObjectDataValue::Object(object_val) => {
            convert_obj_val(&object_val, current_context, path, result);
        },
        ObjectDataValue::AnotherObject(another_obj) => {
            convert_another_object()
            todo!()
        },
        ObjectDataValue::Literal(literal) => {
            let obj_data = convert_literal(literal);
            let data = ObjectValueWasm{
                data: obj_data.clone(),
                size: obj_data.len() as i64
            };
            let d = ObjectPropWasm{
                data: data,
                path: path
            };

            result.push(d)
        },
        ObjectDataValue::Function(_) => {}
        ObjectDataValue::FunctionCall(_) => {}
        ObjectDataValue::Binary(_) => {}
    };
    d
}




fn convert_another_object(another_obj: &AnotherObjectValue, current_context: &WasmCurrentContext) -> Vec<u8> {
    let id =  find_id_in_context(current_context, another_obj.obj.name);
    align(id.to_be_bytes().to_vec())
}

fn find_id_in_context(current_context: &WasmCurrentContext, object_ident: ObjectIdent) -> ObjectId {
    let obj = current_context.obj_table.get(&object_ident);


    if let Some(obj_id) = obj {
        *obj_id
    }
    else {
        compiler_panic!(WasmExpr::NotFoundIdentExpr);
    }
}

fn convert_obj_val(object_val: &ObjectValue, current_ctx: &WasmCurrentContext, path: Vec<ObjectPathWasm>, result: &mut Vec<ObjectPropWasm>) {
    object_val.props.iter().for_each(|prop| {
        let d = vec![ObjectPathWasm::Property(prop.name.clone())];
        let path = [path.clone(), d].concat();
        convert_object_data_value(&prop.value, current_ctx, path, result);
    });
}

fn convert_literal(literal: &LiteralValue) -> Vec<u8> {
    match &literal {
        LiteralValue::Str(val) => align(val.val.clone().into_bytes()),
        LiteralValue::Bool(val) => bool_to_bytes(val.val),
        LiteralValue::Null => 0i64.to_le_bytes().to_vec(),
        LiteralValue::Num(val) => val.val.to_le_bytes().to_vec(),
    }
}




fn align(bytes: Vec<u8>) -> Vec<u8> {
    let padding = (8 - bytes.len() % 8) % 8;

    bytes
        .into_iter()
        .chain(std::iter::repeat_n(0, padding))
        .collect()
}



fn bool_to_bytes(value: bool) -> Vec<u8> {
    (value as i64).to_le_bytes().to_vec()
}




