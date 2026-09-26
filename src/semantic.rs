use std::collections::HashMap;
use serde::Serialize;
use swc_common::comments::SingleThreadedComments;
use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_ast::Module;


pub type ObjectIdent = String;



#[derive(Debug, Clone, Default)]
pub struct GlobalContext{
    pub parsed_modules: HashMap<ObjectIdent, ScopyModule>
}

#[derive(Debug, Clone, Default)]
pub struct CurrentContext {
    pub prev: Option<Box<CurrentContext>>,
    pub context_type: CurrentContextType,
    pub current_statements: Vec<Statement>,
    pub current_module_name: ObjectIdent
}


#[derive(Debug, Clone)]
pub struct AstCurrentContext{
    pub module:Module,
    pub name: ObjectIdent
}






#[derive(Clone, Default)]
pub struct AstGlobalContext{
    pub comments:SingleThreadedComments,
    pub cm: Lrc<SourceMap>,
    pub ast_modules: HashMap<ObjectIdent, Module>
}







#[derive(Clone, Debug)]

#[allow(unused)]
pub enum  CurrentContextType{
    ModuleContext(ModuleContext),
    FunctionContext(FunctionContext),
    LoopContext(LoopContext),
    ConditionContext(ConditionContext)
}

impl Default for CurrentContextType {
    fn default() -> Self { CurrentContextType::ModuleContext(ModuleContext::default()) }
}


#[derive(Default, Debug, Clone)]

#[allow(unused)]
pub struct ModuleContext{
    pub name: ObjectIdent,
}



#[derive(Clone, Debug)]
#[allow(unused)]
pub struct FunctionContext{
    pub name: ObjectIdent,
}


#[derive(Clone, Debug)]
#[allow(unused)]
pub struct LoopContext{
    pub condition: ObjectDataValue
}



#[derive(Clone, Debug)]
#[allow(unused)]
pub struct ConditionContext{
    pub condition: ObjectDataValue,
    pub true_context: ConditionTrueContext,
    pub false_context: Option<ConditionFalseContext>
}




#[derive(Clone, Debug)]
pub struct  ConditionTrueContext  {

}


#[derive(Clone, Debug)]
pub struct ConditionFalseContext{

}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ObjectData {
    pub is_mutable: bool,
    pub name: ObjectIdent,
    pub attrs: Vec<Attribute>,
    pub value: ObjectDataValue,
}






#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "val", content = "val_data")]
pub enum ObjectDataValue {
    Enum (EnumObjectValue),
    Object (ObjectValue),
    AnotherObject(AnotherObjectValue),
    Literal (LiteralValue),
    Function (FunctionValue),
    FunctionCall(FunctionCallResultValue),
    Binary (BinaryObjectValue),
    Import (ImportDataValue),
    Export (ExportDataValue)
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "import-val")]
pub struct ImportDataValue {
    pub imports: Vec<AnotherObjectValue>
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "export-val")]
pub struct ExportDataValue {
    pub exports: Vec<AnotherObjectValue>
}




#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct BinaryObjectValue{
    pub op: BinaryOpType,
    pub v1: Box<ObjectDataValue>,
    pub v2: Box<ObjectDataValue>
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct EnumObjectValue{
    pub props: Vec<ObjectData>,
    pub variants: Vec<ObjectData>
}




#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ObjectValue{
    pub props: Vec<ObjectData>
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct AnotherObjectValue{
    pub obj: Box<ObjectData>,
    pub path: Vec<AnotherObjectValuePath>
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum AnotherObjectValuePath{
    Ident(ObjectIdent),
    FunctionCall(FunctionCallResultValue),
}





#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "lit-val", content = "lit-val-data")]
pub enum LiteralValue {
    Str (LitValueString),
    Bool (LitValueBool),
    Null,
    Num (LitValueNum)
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct LitValueString{
    pub val: ObjectIdent,
}


#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct LitValueNum{
    pub val: f64
}

impl Eq for LitValueNum {

}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct LitValueBool{
    pub val: bool
}






#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "bin-op-type", content = "bin-op-type-data")]
pub enum BinaryOpType {
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
pub struct Attribute{
    pub name: ObjectIdent,
    pub vals: Vec<ObjectIdent>
}






#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionCallResultValue {
    pub args:Vec<ObjectDataValue>,
    pub result: Box<ObjectDataValue>
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct FunctionValue {
    pub scope: Box<ScopeStatement>,
    pub params: Vec<ObjectData>,
    pub result: Box<ObjectData>
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
#[serde(tag = "stmt", content = "stmt-data")]
pub enum Statement {
    Object(ObjectData),
    ObjectValue(ObjectDataValue),
    Conditional(ConditionStatement),
    Loop(LoopStatement),
    Scope(ScopeStatement),
}






#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct  ScopyModule{
    pub name: ObjectIdent,
    pub statements: Vec<Statement>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]

pub struct ScopyProject{
    pub modules: Vec<ScopyModule>,
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct LoopStatement {
    pub cond: ObjectDataValue,
    pub loop_scope: Box<ScopeStatement>
}



#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ConditionStatement {
    pub cond: ObjectDataValue,
    pub false_scope: Option<Box<ScopeStatement>>,
    pub true_scope: Box<ScopeStatement>
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct ScopeStatement {
    pub statements: Vec<Statement>
}


#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub enum CodeModuleSourceFileType{
    Internal,
    External,
}

#[derive(Clone, PartialEq, Debug, Serialize, Eq)]
pub struct CodeModuleMetaData{
    pub code: String,
    pub name: ObjectIdent,
    pub module_meta_type: CodeModuleSourceFileType,
}


