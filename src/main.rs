use core::{panic};
use std::fs;
use std::path::PathBuf;


use serde::Serialize;
use swc_common::{Span, Spanned};
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_ast::*;
use swc_common::comments::{Comments, SingleThreadedComments};




type ObjectIdent = String;


#[derive(Clone)]
#[derive(Debug)]
struct CurrentContext{
    prev: Option<Box<CurrentContext>>,
    context_type: CurrentContextType,
    current_statements: Vec<Statement>
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
enum  CurrentContextType{
    ModuleContext(ModuleContext),
    FunctionContext(FunctionContext),
    LoopContext(LoopContext),
    ConditionContext(ConditionContext)
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
struct ModuleContext{
    name: String,
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
struct FunctionContext{
    name: String,
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
struct LoopContext{
    condition: ObjectDataValue
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
struct ConditionContext{
    condition: ObjectDataValue,
    true_context: ConditionTrueContext,
    false_context: Option<ConditionFalseContext>
}



#[derive(Clone)]
#[derive(Debug)]
struct  ConditionTrueContext  {
    
}


#[derive(Clone)]
#[derive(Debug)]
struct ConditionFalseContext{

} 



#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
struct ObjectData{
    is_mutable: bool, 
    name: ObjectIdent,
    value: ObjectDataValue,
    object_type: ObjectDataType,
    attrs: Vec<Attribute>
}




#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
#[serde(tag = "val", content = "val_data")]
enum ObjectDataValue {
    Enum (EnumObjectValue),
    Object (ObjectValue),
    AnotherObject(AnotherObjectValue),
    Literal (LiteralValue),
    Function (Function),
    FunctionCall(FunctionCallResult),
    Binary (BinaryObjectValue)
}



#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct BinaryObjectValue{
    op: BinaryOpType,
    v1: Box<ObjectDataValue>,
    v2: Box<ObjectDataValue>
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct EnumObjectValue{
    props: Vec<ObjectData>,
    variants: Vec<ObjectData>
}




#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct ObjectValue{
    props: Vec<ObjectData>
}



#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct AnotherObjectValue{
    obj: Box<ObjectData>,
    path: Vec<ObjectIdent>
}




#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
#[serde(tag = "lit-val", content = "lit-val-data")]
pub enum LiteralValue {
    Str,
    Bool{val: bool},
    Null,
    Num {val: f64}
}








#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
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



#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct ObjectDataType{
    name: String,
    hash_type: String,
    object_kind: ObjectDataTypeKind
}



#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
#[serde(tag = "obj-kind", content = "data")]
enum ObjectDataTypeKind{
    Enum (EnumObjectDataType),
    Object (ObjectObjectDataType),
    Function (FunctionMetaDataType),
    AnotherType(AnotherTypeObjectDataType)
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct  AnotherTypeObjectDataType{
    obj: Box<ObjectDataType>
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct EnumObjectDataType{
    default_props: Vec<ObjectDataType>,
    variants: Vec<ObjectDataType>
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct Attribute{
    name: String,
    vals: Vec<String>
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct FunctionMetaDataType{
    meta_data: Box<FunctionMetaData>
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Clone)]
struct ObjectObjectDataType{
    props: Vec<ObjectDataType>,
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct FunctionCallResult{
    function_meta: FunctionMetaData,
    args:Vec<ObjectDataValue>,
    result: Box<ObjectDataValue>,
    path: Vec<ObjectIdent>
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct FunctionMetaData{
    result: Box<ObjectDataType>, 
    args: Vec<ObjectDataType>,
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct Function{
    meta:Box<FunctionMetaData>,
    statment: Box<Statement>,
    params: Vec<ObjectData>,
    result: Box<ObjectData>
}

#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
#[serde(tag = "stmt", content = "stmt-data")]
enum Statement {
    Object(ObjectData),
    ObjectValue(ObjectDataValue),
    Conditional(Condition),
    Loop(Loop),
    Scope(Scope)
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Serialize)]
#[derive(Clone)]
struct Loop{
    cond: ObjectDataValue,
    loop_scope: Box<Statement>
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct Condition{
    cond: ObjectDataValue,
    false_scope: Option<Box<Statement>>,
    true_scope: Box<Statement>
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct Scope{
    name: Option<String>,
    statements: Vec<Statement>
}


#[allow(unused)]
struct CodeModuleMetaData{
    code: String,
    name: String
}

#[allow(unused)]
fn get_modules(cm: &Lrc<SourceMap>){


    let default_module  = CodeModuleMetaData{
        code:  r#"
            export const string = {};

            export const bool = {};

            export const number = {};

            export const generic = {};
        "#.to_string(),
        name: "default.js".to_string()
    };

   
    let internal_modules = 
        vec![default_module]
        .iter()
        .map(|f| {
            cm.new_source_file(FileName::Internal(f.name.clone().into()).into(), f.code.clone())
        })
        .collect::<Vec::<_>>();
    

    
    




    
}


fn main() {
    let code = r#"
        export const va1 = {
            va21:{
                va31:(result={va41:{}}) => {},
            },
            va22: {
                va31:{}
            }
        };


        
        va1.va21.va31().va41
        
    "#;

    let cm: Lrc<SourceMap> = Default::default();

    
    let fm = cm.new_source_file(
        FileName::Custom("input.js".into()).into(),
        code
    );

    let comments = SingleThreadedComments::default();
    // --- ВАЖНО: новый способ через Lexer ---
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().expect("failed to parse");
    
    



    // println!("==== AST ====");
    // println!("{:#?}", module);

    let ctx = CurrentContext{
        context_type: CurrentContextType::ModuleContext(ModuleContext { name: "input".to_string() }),
        prev: None,
        current_statements: Vec::new()
    };
    let parsed_module = module_parse(module, "input".to_string(), &ctx, &comments);


    // println!("==== IR ====");
    // println!("{:#?}", parsed_module);

    dump_ir_html(&parsed_module)
}



fn dump_ir_html(ir: &Statement) {
    // 1. сериализация
    let json = serde_json::to_string(ir).unwrap();

    // 2. путь к шаблону (рядом с Cargo.toml)
    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("viewer.template.html");

    let template = fs::read_to_string(template_path)
        .expect("failed to read viewer.template.html");

    // 3. вставка JSON
    let html = template.replace("__JSON__", &json);

    // 4. запись результата
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ir_output/ir.html");

    fs::write(out_path, html).unwrap();
}

fn module_parse(module: Module, module_name: String, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Statement{
    let mut ctx_statements: Vec<Statement> = ctx.current_statements.clone();
    
    
    let module_stmts = module.body.iter().map(|el| {
        let context = CurrentContext{
            context_type: CurrentContextType::ModuleContext(ModuleContext  {
                name: module_name.clone()
            }),
            prev: Some(Box::new(ctx.clone())),
            current_statements: ctx_statements.clone()
        };
        let res_el = match el {
            ModuleItem::ModuleDecl(module_decl) => {
                let import_stmt = parse_module_decl(module_decl, &context, comments);
                import_stmt
            },
            ModuleItem::Stmt(stmt) => {
                let p_stmt = parse_stmt(stmt, &context, comments);
                p_stmt
            },
        };
        ctx_statements.push(res_el.clone());
        res_el
    }).collect::<Vec<_>>();
    Statement::Scope(Scope { name: None, statements: module_stmts })
}


fn parse_module_decl(module_decl: &ModuleDecl, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Statement{
    let stmt = match module_decl{
        ModuleDecl::Import(import_decl) => {
            let imports = parse_import_decl(import_decl, ctx,comments);
            let obj_val = ObjectDataValue::Object(ObjectValue { props: imports });
            Statement::ObjectValue(obj_val)
        },
        ModuleDecl::ExportDecl(export_decl) => {
            let obj_data = parse_export_decl(export_decl, ctx, comments);
            Statement::Object(obj_data)
        },
        ModuleDecl::ExportNamed(_) => todo!(),
        ModuleDecl::ExportDefaultDecl(_) => todo!(),
        ModuleDecl::ExportDefaultExpr(_) => todo!(),
        ModuleDecl::ExportAll(_) => todo!(),
        ModuleDecl::TsImportEquals(_) => todo!(),
        ModuleDecl::TsExportAssignment(_) => todo!(),
        ModuleDecl::TsNamespaceExport(_) => todo!(),
    };

    
    stmt
    
}




fn parse_export_decl(export_decl: &ExportDecl, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    match &export_decl.decl {
        Decl::Class(_) => todo!(),
        Decl::Fn(_) => todo!(),
        Decl::Var(var_decl) => {
            let p = parse_decl(var_decl, ctx, comments);
            p
        },
        Decl::Using(_) => todo!(),
        Decl::TsInterface(_) => todo!(),
        Decl::TsTypeAlias(_) => todo!(),
        Decl::TsEnum(_) => todo!(),
        Decl::TsModule(_) => todo!(),
    }
}

fn parse_import_decl(import_decl: &ImportDecl, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Vec<ObjectData>{
    let imports = import_decl.specifiers.iter().map(|s| {
        let spec = parse_specifier(s);
        spec
    })
    .map(|name| {
        let attrs = get_attributes(comments, &import_decl.span);
        let another_obj = find_in_context(ctx, name.clone()).unwrap();
        let obj_val = ObjectDataValue::AnotherObject(another_obj.clone());
        let obj_type_kind = object_data_val_to_type(&obj_val, ctx);
        let obj_data_type = ObjectDataType{
            hash_type: String::new(),
            name: name.clone(),
            object_kind: obj_type_kind
        };
        let obj = ObjectData{
            is_mutable: false,
            name: name,
            object_type: obj_data_type,
            value: ObjectDataValue::AnotherObject(another_obj),
            attrs: attrs
        };
        obj
    }).collect::<Vec<_>>();
    imports
}

fn parse_specifier(s: &ImportSpecifier) -> String{
    match s {
        ImportSpecifier::Named(import_named_specifier) => {
            let name = parse_named_specifier(import_named_specifier);
            name
        },
        ImportSpecifier::Default(import_default_specifier) => {
            let name = parse_default_specifier(import_default_specifier);
            name
        },
        ImportSpecifier::Namespace(_) => todo!(),
    }
}

fn parse_default_specifier(import_default_specifier: &ImportDefaultSpecifier) -> String{
    parse_ident(&import_default_specifier.local)
}

fn parse_named_specifier(import_named_specifier: &ImportNamedSpecifier) -> String{
    let imported = import_named_specifier.imported.clone().unwrap();

    let name = match imported{
        ModuleExportName::Ident(ident) => parse_ident(&ident),
        ModuleExportName::Str(_) => panic!("str export not allowed"),
    };
    name
}

fn parse_stmt(stmt: &Stmt, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Statement{
    let stmt = match stmt{
        Stmt::Block(block_stmt) => {
            let scope = parse_block(block_stmt, ctx, comments);
            let stmt = Statement::Scope(scope);
            stmt
        },
        Stmt::Empty(_empty_stmt) => todo!(),
        Stmt::Debugger(_debugger_stmt) => todo!(),
        Stmt::With(_) => todo!(),
        Stmt::Return(_return_stmt) => todo!(),
        Stmt::Labeled(_labeled_stmt) => todo!(),
        Stmt::Break(_break_stmt) => todo!(),
        Stmt::Continue(_continue_stmt) => todo!(),
        Stmt::If(if_stmt) => {
            let if_cond = parse_if(if_stmt, ctx, comments);

            Statement::Conditional(if_cond)
        },
        Stmt::Switch(_switch_stmt) => todo!(),
        Stmt::Throw(_throw_stmt) => todo!(),
        Stmt::Try(_try_stmt) => todo!(),
        Stmt::While(while_stmt) => {
            let loop_s = parse_while(while_stmt, ctx, comments);
            let loop_stmt = Statement::Loop(loop_s);
            loop_stmt
        },
        Stmt::DoWhile(_do_while_stmt) => todo!(),
        Stmt::For(_for_stmt) => todo!(),
        Stmt::ForIn(_for_in_stmt) => todo!(),
        Stmt::ForOf(_for_of_stmt) => todo!(),
        Stmt::Decl(decl) => {
            match decl{
                Decl::Class(_class_decl) => todo!(),
                Decl::Fn(_fn_decl) => todo!(),
                Decl::Var(var_decl) => {
                    let decl = parse_decl(var_decl, ctx, comments);
                    let stmt = Statement::Object(decl);
                    stmt
                },
                Decl::Using(_using_decl) => todo!(),
                Decl::TsInterface(_ts_interface_decl) => todo!(),
                Decl::TsTypeAlias(_ts_type_alias_decl) => todo!(),
                Decl::TsEnum(_ts_enum_decl) => todo!(),
                Decl::TsModule(_ts_module_decl) => todo!(),
            }
        },
        Stmt::Expr(expr_stmt) => {
            let expr = parse_expr_stmt(expr_stmt, ctx, comments);
            let stmt = Statement::ObjectValue(expr);
            stmt
        },
    };
    stmt
}







fn parse_expr_stmt(expr_stmt: &ExprStmt, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectDataValue{
    let expr = parse_expr(&expr_stmt.expr, ctx, comments);
    expr
}


fn object_data_to_type(obj_data: &ObjectData, ctx: &CurrentContext) -> ObjectDataType{
    let obj_type_kind = object_data_val_to_type(&obj_data.value, ctx);
    let obj_type = ObjectDataType{
        hash_type: String::default(),
        name: obj_data.name.clone(),
        object_kind: obj_type_kind
    };
    obj_type
}

fn object_value_to_type(val: &ObjectValue, ctx: &CurrentContext) -> ObjectObjectDataType{
    let obj_props = obj_prop_to_type_props(&val.props, ctx);
    let obj = ObjectObjectDataType{
        props: obj_props
    };
    obj
}

fn obj_prop_to_type_props(props: &Vec<ObjectData>, ctx: &CurrentContext) -> Vec<ObjectDataType>{
    let obj_props = props.iter().map(|p| {
        object_data_to_type(p, ctx)
    }).collect::<Vec<_>>();
    obj_props
}


fn object_enum_to_type(enum_object_value: &EnumObjectValue, ctx: &CurrentContext) -> EnumObjectDataType{
    let default_props = obj_prop_to_type_props(&enum_object_value.props, ctx);

    let variants = obj_prop_to_type_props(&enum_object_value.variants, ctx);

    let enum_obj_val_type = EnumObjectDataType{
        default_props: default_props,
        variants: variants
    };
    enum_obj_val_type
}

fn literal_value_to_type(val: &LiteralValue, ctx: &CurrentContext) -> ObjectDataTypeKind{
    let str_obj = find_in_context_internal(ctx, "string".to_string());
    let bool_obj = find_in_context_internal(ctx, "bool".to_string());
    let generic_obj = find_in_context_internal(ctx, "generic".to_string());
    let num_obj = find_in_context_internal(ctx, "number".to_string());

    
    let str_type = another_object_value_to_type(&str_obj, ctx);
    let bool_type = another_object_value_to_type(&bool_obj, ctx);
    let generic_obj = another_object_value_to_type(&generic_obj, ctx);
    let num_obj = another_object_value_to_type(&num_obj, ctx);


    let val_obj_t = match &val{
        LiteralValue::Str => ObjectDataTypeKind::AnotherType(str_type),
        LiteralValue::Bool { val: _ } => ObjectDataTypeKind::AnotherType(bool_type),
        LiteralValue::Null => ObjectDataTypeKind::AnotherType(generic_obj),
        LiteralValue::Num { val: _ } => ObjectDataTypeKind::AnotherType(num_obj)
    };
    val_obj_t
}


fn another_object_value_to_type(val: &AnotherObjectValue, ctx: &CurrentContext) -> AnotherTypeObjectDataType{
    let type_obj = object_data_to_type(&val.obj, ctx);
    AnotherTypeObjectDataType {
        obj: Box::new(type_obj)
    }
}

fn function_to_type(func: &Function) -> FunctionMetaDataType{
    let func_type = FunctionMetaDataType{
        meta_data: func.meta.clone()
    };
    func_type
}


fn function_call_to_type(func_call: &FunctionCallResult, ctx: &CurrentContext) -> ObjectDataTypeKind{
    object_data_val_to_type(&func_call.result,ctx)
}

fn binary_to_type(binary_object_value: &BinaryObjectValue, ctx: &CurrentContext) -> ObjectDataTypeKind{
    let type_v1 = object_data_val_to_type(&binary_object_value.v1, ctx);
    let _type_v2 = object_data_val_to_type(&binary_object_value.v2, ctx);

    type_v1
}


fn object_data_val_to_type(val:&ObjectDataValue, ctx: &CurrentContext) -> ObjectDataTypeKind{
    let object_kind = match val{
        ObjectDataValue::Enum(enum_object_value) => ObjectDataTypeKind::Enum(object_enum_to_type(enum_object_value, ctx)),
        ObjectDataValue::Object(object_value) => ObjectDataTypeKind::Object(object_value_to_type(object_value, ctx)),
        ObjectDataValue::Function(function) => ObjectDataTypeKind::Function(function_to_type(function)),
        ObjectDataValue::AnotherObject(another_object_value) => ObjectDataTypeKind::AnotherType(another_object_value_to_type(another_object_value, ctx)),
        ObjectDataValue::Literal(literal_value) => literal_value_to_type(literal_value, ctx),
        ObjectDataValue::FunctionCall(function_call_result) => function_call_to_type(function_call_result, ctx),
        ObjectDataValue::Binary(binary_object_value) => binary_to_type(binary_object_value, ctx)
    };

    object_kind
}


fn parse_decl(var_decl: &VarDecl, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    let delc_len = var_decl.decls.len();

    if delc_len == 0{
        panic!("No decl!")
    }
    if delc_len > 1 {
        panic!("Multiple declarations is not allowed!");
    }
    let decl = var_decl.decls.first().unwrap();
    let var_name = parse_pat_as_ident(&decl.name);
    let init = &decl.init.clone().unwrap();
    let val = parse_expr(init, ctx, comments);




    let is_mutable = match var_decl.kind {
        VarDeclKind::Var => panic!("Var is not allowed"),
        VarDeclKind::Let => true,
        VarDeclKind::Const => false,
    };

    let attrs = get_attributes(comments, &var_decl.span);
    

    let obj_type_kind = object_data_val_to_type(&val, ctx);
    let obj_data_type = ObjectDataType{
        name: var_name.clone(),
        hash_type: String::default(),
        object_kind: obj_type_kind
    };
    

    let obj = ObjectData { 
        is_mutable: is_mutable, 
        name: var_name, 
        value: val,     
        object_type: obj_data_type,
        attrs: attrs
    };

    obj


}


#[allow(unused)]
fn has_attribute(
    attrs: Vec<Attribute>,
    name: &str,
    args: Option<&[&str]>,
) -> bool {
    attrs.into_iter().any(|attr| {
        if attr.name != name {
            return false;
        }

        match args {
            None => true,
            Some(expected) => {
                if attr.vals.len() != expected.len() {
                    return false;
                }

                attr.vals.iter().zip(expected.iter()).all(|(a, b)| a == b)
            }
        }
    })
}


fn get_attributes(comments: &SingleThreadedComments, span: &Span) -> Vec<Attribute>{
    let comment_opt = comments.get_leading(span.lo());
    if let Some(vec_comments) = comment_opt{
        let attrs = parse_attributes_from_comments(vec_comments);
        attrs
    }
    else{
        Vec::new()
    }
}

use swc_common::comments::Comment;
fn parse_attributes_from_comments(comments: Vec<Comment>) -> Vec<Attribute> {
    let mut result = Vec::new();

    for comment in comments {
        // comment.text — это уже без // или /* */
        let attrs = parse_attributes(&comment.text);
        result.extend(attrs);
    }

    result
}

fn parse_attributes(input: &str) -> Vec<Attribute> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        if *c != '@' {
            chars.next();
            continue;
        }

        chars.next(); // skip '@'

        // --- parse name ---
        let mut name = String::new();
        while let Some(c) = chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                name.push(*c);
                chars.next();
            } else {
                break;
            }
        }

        if chars.peek() != Some(&'(') {
            continue;
        }
        chars.next(); // skip '('

        let mut vals = Vec::new();
        let mut current = String::new();

        let mut in_string = false;
        let mut escape = false;

        while let Some(c) = chars.next() {
            if in_string {
                if escape {
                    // обрабатываем escape-последовательности
                    let real = match c {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        other => other, // неизвестные просто пропускаем как есть
                    };
                    current.push(real);
                    escape = false;
                    continue;
                }

                match c {
                    '\\' => {
                        escape = true;
                    }
                    '"' => {
                        in_string = false;
                    }
                    _ => {
                        current.push(c);
                    }
                }
                continue;
            }

            match c {
                '"' => {
                    in_string = true;
                }
                ',' => {
                    let val = current.trim().to_string();
                    if !val.is_empty() {
                        vals.push(val);
                    }
                    current.clear();
                }
                ')' => {
                    let val = current.trim().to_string();
                    if !val.is_empty() {
                        vals.push(val);
                    }
                    break;
                }
                _ => {
                    current.push(c);
                }
            }
        }

        result.push(Attribute { name, vals });
    }

    result
}


fn find_in_context(ctx: &CurrentContext, name: String) -> Option<AnotherObjectValue> {
    let mut current = Some(ctx);

    while let Some(c) = current {
        if let Some(obj) = c.current_statements
            .iter()
            .find_map(|stmt| find_in_statements(stmt, &name)) {
                return Some(AnotherObjectValue { obj: Box::new(obj), path: vec![name] });
            }
        current = c.prev.as_deref();
    }

    None
}



fn find_in_context_internal(ctx: &CurrentContext, name: String) -> AnotherObjectValue {
    let mut current = Some(ctx);

    while let Some(c) = current {
        if let Some(obj) = c.current_statements
            .iter()
            .find_map(|stmt| find_in_statements(stmt, &name)) {
                return AnotherObjectValue { 
                    obj: Box::new(obj), 
                    path: vec![name] }
            }
        current = c.prev.as_deref();
    }

    panic!("Cannot find internal obj {:#}", name)
}





fn find_in_statements(stmt: &Statement, name: &String) -> Option<ObjectData>{
    match stmt {
        Statement::Object(object_data) => {
            if object_data.name == *name{
                Some(object_data.clone())
            }
            else {
                None
            }
        },
        Statement::ObjectValue(_) => None,
        Statement::Conditional(_) => None,
        Statement::Loop(_) => None,
        Statement::Scope(_) => None,
    }
}

fn parse_expr(expr:&Expr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectDataValue{
    match expr {
        Expr::This(_this_expr) => todo!(),
        Expr::Array(_array_lit) => todo!(),
        Expr::Object(object_lit) => {
            let object_val = parse_obj_lit(object_lit, ctx, comments);
            ObjectDataValue::Object(object_val)
        },
        Expr::Fn(_fn_expr) => todo!(),
        Expr::Unary(_unary_expr) => todo!(),
        Expr::Update(_update_expr) => todo!(),
        Expr::Bin(bin_expr) => {
            let bin_val = parse_bin_expr(bin_expr, ctx, comments);
            ObjectDataValue::Binary(bin_val)
        },
        Expr::Assign(_assign_expr) => todo!(),
        Expr::Member(member_expr) => {
            let memb_expr = parse_member_expr(member_expr, ctx, comments);
            ObjectDataValue::AnotherObject(memb_expr)
        },
        Expr::SuperProp(_super_prop_expr) => todo!(),
        Expr::Cond(_cond_expr) => todo!(),
        Expr::Call(call_expr) => {
            let call_expr = parse_call_expr(call_expr, ctx, comments);
            ObjectDataValue::FunctionCall(call_expr)
        },
        Expr::New(_new_expr) => todo!(),
        Expr::Seq(_seq_expr) => todo!(),
        Expr::Ident(ident) => {
            let name = parse_ident(ident);
            let another_obj = find_in_context(ctx, name).unwrap();
            let res = ObjectDataValue::AnotherObject(another_obj);
            res
        },
        Expr::Lit(lit) => {
            let lit_expr = parse_lit_expr(lit);
            lit_expr
        },
        Expr::Tpl(_) => todo!(),
        Expr::TaggedTpl(_) => todo!(),
        Expr::Arrow(arrow_expr) => {
            let function = parse_arrow_expr(arrow_expr, ctx, comments);
            ObjectDataValue::Function(function)
        },
        Expr::Class(_class_expr) => todo!(),
        Expr::Yield(_yield_expr) => todo!(),
        Expr::MetaProp(_meta_prop_expr) => todo!(),
        Expr::Await(_await_expr) => todo!(),
        Expr::Paren(paren_expr) => {
            let res = parse_paren_expr(paren_expr, ctx, comments);
            res
        },
        Expr::JSXMember(_jsxmember_expr) => todo!(),
        Expr::JSXNamespacedName(_jsxnamespaced_name) => todo!(),
        Expr::JSXEmpty(_jsxempty_expr) => todo!(),
        Expr::JSXElement(_jsxelement) => todo!(),
        Expr::JSXFragment(_jsxfragment) => todo!(),
        Expr::TsTypeAssertion(_ts_type_assertion) => todo!(),
        Expr::TsConstAssertion(_ts_const_assertion) => todo!(),
        Expr::TsNonNull(_ts_non_null_expr) => todo!(),
        Expr::TsAs(_ts_as_expr) => todo!(),
        Expr::TsInstantiation(_ts_instantiation) => todo!(),
        Expr::TsSatisfies(_ts_satisfies_expr) => todo!(),
        Expr::PrivateName(_private_name) => todo!(),
        Expr::OptChain(_opt_chain_expr) => todo!(),
        Expr::Invalid(_invalid) => {
            panic!("")
        },
    }
}

fn parse_obj_lit(object_lit:&ObjectLit, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectValue{
    let props = object_lit.props
        .iter()
        .flat_map(|p| parse_prop_or_spread(p, ctx, comments))
        .collect::<Vec<_>>();


    return ObjectValue{ props: props };
}

fn parse_prop_or_spread(p: &PropOrSpread, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Vec<ObjectData>{
    match p{
        PropOrSpread::Spread(spread_element) => parse_spread_el(spread_element, ctx, comments),
        PropOrSpread::Prop(prop) =>  vec![parse_prop(prop, ctx, comments)] ,
    }
}


fn parse_spread_el(spread_element: &SpreadElement, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Vec<ObjectData>{
    let val: ObjectDataValue = parse_expr(&spread_element.expr, ctx, comments);
    let unpack_value = unpapck_data_value(&val);
    

    unpack_value
}



fn unpapck_data_value(val: &ObjectDataValue) -> Vec<ObjectData>{
    match val {
        ObjectDataValue::Enum(enum_object_value) => enum_object_value.props.clone(),
        ObjectDataValue::Object(object_value) => object_value.props.clone(),
        ObjectDataValue::AnotherObject(another_object_value) => 
            unpapck_data_value(&another_object_value.obj.value),
        ObjectDataValue::Literal(_) => Vec::new(),
        ObjectDataValue::Function(_) => Vec::new(),
        ObjectDataValue::FunctionCall(function_meta_data) => unpapck_data_value(&function_meta_data.result),
        ObjectDataValue::Binary(_) => Vec::new()
    }
}


fn another_obj_from_member(val: &ObjectDataValue, member_name: &ObjectIdent) -> Option<AnotherObjectValue>{
    match val {
        ObjectDataValue::Enum(enum_object_value) => {
            let prop = find_member_in_props(&enum_object_value.props, &member_name);
            let another_obj = prop.map(|o| {
                AnotherObjectValue{
                    obj: Box::new(o.clone()),
                    path: vec![member_name.clone()]
                }
            });
            another_obj
        },
        ObjectDataValue::Object(object_value) => {
            let prop = find_member_in_props(&object_value.props, &member_name);
            let another_obj = prop.map(|o| {
                AnotherObjectValue{
                    obj: Box::new(o.clone()),
                    path: vec![member_name.clone()]
                }
            });
            another_obj
        },
        ObjectDataValue::AnotherObject(another_object_value) => {
            let another_obj = another_obj_from_member(&another_object_value.obj.value, &member_name);
            
            let another_obj = another_obj.map(|o| {
                let path = [another_object_value.path.clone(), o.path].concat();
                AnotherObjectValue { obj: o.obj, path: path }
            });
            another_obj
        }
        ObjectDataValue::Literal(_) => None,
        ObjectDataValue::Function(_) => None,
        ObjectDataValue::FunctionCall(function_meta_data) => {
            
            let another_obj = another_obj_from_member(&function_meta_data.result, member_name);
            

            // let another_obj = another_obj.map(|o| {
                
            //     AnotherObjectValue { obj: o.obj, path: o.path }
            // });
            another_obj
        },
        ObjectDataValue::Binary(_) => None
    }
}



fn find_member_in_props(props: &Vec<ObjectData>, prop_name: &ObjectIdent) -> Option<ObjectData>{
    props.iter().find(|p| p.name == *prop_name).cloned()
}


fn parse_prop(prop: &Prop, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    match prop {
        Prop::Shorthand(_ident) => panic!("Short hand ident"),
        Prop::KeyValue(key_value_prop) => parse_key_value_prop(key_value_prop, ctx, comments),
        Prop::Assign(_assign_prop) => panic!("Asign prop"),
        Prop::Getter(_getter_prop) => panic!("Getter prop"),
        Prop::Setter(_setter_prop) => panic!("Setter prop"),
        Prop::Method(_method_prop) => panic!("Method prop"),
    }
}



fn parse_key_value_prop(key_value_prop: &KeyValueProp, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    let current_object_name = parse_prop_name(&key_value_prop.key);
    let value = parse_expr(&key_value_prop.value, ctx, comments);

    let val_type_kind = object_data_val_to_type(&value, ctx);
    let type_val = ObjectDataType{
        name: current_object_name.clone(),
        hash_type: String::default(),
        object_kind: val_type_kind,
    };
    let res = ObjectData { 
        name: current_object_name, 
        value, 
        object_type: type_val,
        is_mutable: false,
        attrs: get_attributes(comments, &key_value_prop.span())
    };
    res
}

 
fn parse_prop_name(prop_name:&PropName) -> String{
    match &prop_name{
        PropName::Ident(ident_name) => {
            parse_ident_name(&ident_name)
        },
        PropName::Str(_) => todo!(),
        PropName::Num(_number) => todo!(),
        PropName::Computed(_computed_prop_name) => todo!(),
        PropName::BigInt(_big_int) => todo!(),
    }
}

fn parse_ident_name(ident_name:&IdentName) -> String{
    ident_name.sym.to_string()
}

fn parse_bin_expr(bin_expr: &BinExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> BinaryObjectValue {
    let left_expr = parse_expr(&bin_expr.left, ctx, comments);
    let right_expr = parse_expr(&bin_expr.right, ctx, comments);
    let op = match bin_expr.op {
        BinaryOp::EqEq => BinaryOpType::EqEq,
        BinaryOp::NotEq => BinaryOpType::NotEq,
        BinaryOp::Lt => BinaryOpType::Lt,
        BinaryOp::LtEq => BinaryOpType::LtEq,
        BinaryOp::Gt => BinaryOpType::Gt,
        BinaryOp::GtEq => BinaryOpType::GtEq,
        BinaryOp::Add => BinaryOpType::Add,
        BinaryOp::Sub => BinaryOpType::Sub,
        BinaryOp::Mul => BinaryOpType::Mul,
        BinaryOp::Div => BinaryOpType::Div,
        BinaryOp::Mod => BinaryOpType::Mod,
        BinaryOp::LogicalAnd => BinaryOpType::LogicalAnd,
        BinaryOp::LogicalOr => BinaryOpType::LogicalOr,
        BinaryOp::NullishCoalescing => BinaryOpType::NullishCoalescing,
        _ => panic!("Not supported binary"),
    };
    let res = BinaryObjectValue{
        op:op,
        v1: Box::new(left_expr),
        v2: Box::new(right_expr)
    };
    res
}





fn parse_binding_ident(binding_ident: &BindingIdent) -> String{
    return parse_ident(&binding_ident.id);
}



fn parse_pat_as_ident(pat: &Pat) -> String{
    let ident = match &pat{
        Pat::Ident(binding_ident) => parse_binding_ident(binding_ident),
        Pat::Array(array_pat) => panic!("{:#?}", array_pat),
        Pat::Rest(rest_pat) => panic!("{:#?}", rest_pat),
        Pat::Object(object_pat) => panic!("{:#?}", object_pat),
        Pat::Assign(assign_pat) => panic!("{:#?}", assign_pat),
        Pat::Invalid(invalid) => panic!("{:#?}", invalid),
        Pat::Expr(expr) => panic!("{:#?}", expr),
    };

   return ident;
}


fn parse_pat_as_assign(pat: &Pat, ctx:&CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    let ident = match &pat{
        Pat::Ident(binding_ident) => panic!("{:#?}",binding_ident),
        Pat::Array(array_pat) => panic!("{:#?}", array_pat),
        Pat::Rest(rest_pat) => panic!("{:#?}", rest_pat),
        Pat::Object(object_pat) => panic!("{:#?}", object_pat),
        Pat::Assign(assign_pat) => {
            parse_assign_pat(assign_pat, ctx, comments)
        },
        Pat::Invalid(invalid) => panic!("{:#?}", invalid),
        Pat::Expr(expr) => panic!("{:#?}", expr),
    };

   return ident;
}


fn parse_assign_pat(assign_pat: &AssignPat, ctx:&CurrentContext, comments: &SingleThreadedComments) -> ObjectData{
    let param_name = parse_pat_as_ident(&assign_pat.left);
    let obj_val = parse_expr(&assign_pat.right, ctx, comments);


    let obj_data_type_kind = object_data_val_to_type(&obj_val, ctx);
    let obj_data_type = ObjectDataType{
        name: param_name.clone(),
        hash_type: String::new(),
        object_kind: obj_data_type_kind,
    };
    let obj = ObjectData{
        attrs: get_attributes(comments, &assign_pat.span),
        is_mutable: false,
        name: param_name,
        value: obj_val,
        object_type: obj_data_type,
    };
    obj
}


fn parse_paren_expr(paren_expr: &ParenExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectDataValue{
    parse_expr(&paren_expr.expr, ctx, comments)
}

fn parse_arrow_expr(arrow_expr: &ArrowExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Function {

    let params = arrow_expr.params
        .iter()
        .map(|pat| {
            let obj_data = parse_pat_as_assign(pat, ctx, comments);
            obj_data
        })
        .collect::<Vec<_>>();    


    let params_type = params.clone().iter().map(|param|{
        let obj_data_type = object_data_to_type(param, ctx);
        obj_data_type
    }).collect::<Vec<_>>();   
    
    
    let scope = parse_block_stmt_or_expr(&arrow_expr.body, ctx, comments);
    let result_type = find_result_type_in_params(&params_type).unwrap();
    let stmt_res = Statement::Scope(scope);
    

    let meta = Box::new(FunctionMetaData { result: Box::new(result_type), args: params_type });
    let result = find_result_in_params(&params).unwrap();

    Function { 
        meta: meta, 
        statment: Box::new(stmt_res), 
        params: params, 
        result:Box::new(result) 
    }
}


fn find_result_in_params(params: &Vec<ObjectData>) -> Option<ObjectData>{
    params.iter().find(|p| p.name == "result").cloned()
}


fn find_result_type_in_params(params: &Vec<ObjectDataType>) -> Option<ObjectDataType>{
    let result = params
        .iter()
        .find(|p| p.name == "result");
    result.map(|r| r.clone())
}





fn parse_block_stmt_or_expr(block_stmt_or_expr: &BlockStmtOrExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Scope {
    match &block_stmt_or_expr{
        BlockStmtOrExpr::BlockStmt(block_stmt) => {
            let block_scope = parse_block(block_stmt, ctx, comments);
            block_scope
        },
        BlockStmtOrExpr::Expr(_) => todo!()
    }
}



fn parse_lit_expr(lit_expr: &Lit) -> ObjectDataValue{
    match &lit_expr {
        Lit::Str(_) => {
            ObjectDataValue::Literal(LiteralValue::Str)
        },
        Lit::Bool(val) => {
            ObjectDataValue::Literal(LiteralValue::Bool {val: val.value})
        },
        Lit::Null(_null) => {
            ObjectDataValue::Literal(LiteralValue::Null)
        },
        Lit::Num(number) => {
            ObjectDataValue::Literal(LiteralValue::Num { val: number.value})
        },
        Lit::BigInt(_big_int) => {
            panic!("big_int not allowed")
        },
        Lit::Regex(_regex) => {
            panic!("Regex not allowed");
        },
        Lit::JSXText(_jsxtext) => {
            todo!()
        },
    }
}

fn parse_ident(ident_expr: &Ident) -> String{
    let name = ident_expr.sym.to_string();
    return name;
}


fn parse_call_expr(call_expr: &CallExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> FunctionCallResult{
    let expr = call_expr.callee.as_expr().unwrap();
    let args = call_expr.args
        .iter()
        .map(|arg| {
            let obj_val = parse_expr_or_spred(arg, ctx, comments);
            obj_val
        })
        .collect::<Vec<_>>();
    
    
   
    let val_call = parse_expr(expr, ctx, comments);
    
    let func_data = val_call_parse(&val_call);


    let meta = *func_data.meta.clone();


    
    
    let func_call = FunctionCallResult{
        function_meta: meta,
        args: args,
        result: Box::new(func_data.result.value.clone()),
        path: todo!()
    };
    
    func_call
}





fn val_call_parse(val_call: &ObjectDataValue) -> &Function{
    match val_call {
        ObjectDataValue::Enum(_) => panic!("not a function"),
        ObjectDataValue::Object(_) => panic!("not a function"),
        ObjectDataValue::AnotherObject(another_object_value) => val_call_parse(&another_object_value.obj.value),
        ObjectDataValue::Literal(_) => panic!("not a function"),
        ObjectDataValue::Function(function) => function,
        ObjectDataValue::FunctionCall(_) => panic!("dont't do foo_call()() it's hard to read"),
        ObjectDataValue::Binary(_) => panic!("not a function"),
    }
}





fn parse_expr_or_spred(expr_or_spread: &ExprOrSpread, ctx: &CurrentContext, comments: &SingleThreadedComments) -> ObjectDataValue{
    let is_spread = expr_or_spread.spread.is_some();

    if is_spread
    {
        panic!("Spread is not allowed in function call");
    }
        

    let arg = parse_expr(&expr_or_spread.expr,ctx, comments);
    
    arg
    
}

fn parse_member_expr(member: &MemberExpr, ctx: &CurrentContext, comments: &SingleThreadedComments) -> AnotherObjectValue{
    let parse_member_val = parse_expr(&member.obj,ctx, comments);
    let member_prop = parse_member_prop(&member.prop);


    

    let object_stmts = another_obj_from_member(&parse_member_val, &member_prop).unwrap();

    object_stmts
}

fn parse_member_prop(member_prop: &MemberProp) -> String {
    match &member_prop{
        MemberProp::Ident(ident_name) => parse_ident_name(ident_name),
        MemberProp::PrivateName(_private_name) => todo!(),
        MemberProp::Computed(_computed_prop_name) => todo!(),
    }
}




fn parse_block(block_stmt: &BlockStmt, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Scope{
    let mut ctx_statements: Vec<Statement> = ctx.current_statements.clone();
    let stmts = block_stmt.stmts.iter().map(|s| {
        let context = CurrentContext{
            context_type: ctx.context_type.clone(),
            prev: Some(Box::new(ctx.clone())),
            current_statements: ctx_statements.clone()
        };
        let ps = parse_stmt(s, &context, comments);
        ctx_statements.push(ps.clone());
        ps
    }).collect::<Vec<_>>();
    let scope = Scope{
        name: None,
        statements: stmts
    };
    scope
}


fn parse_if(if_stmt: &IfStmt, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Condition{

    let cons_stmt = parse_stmt(&if_stmt.cons, ctx,comments);

    let test_expr = parse_expr(&if_stmt.test, ctx, comments);



    let else_stmt = match &if_stmt.alt{
        Some(r) =>  Some(Box::new(parse_stmt(&r, ctx,comments))),
        None => None,
    };
    
    let cond = Condition {
        cond: test_expr,
        false_scope: else_stmt,
        true_scope: Box::new(cons_stmt)
    };
    cond
}



fn parse_while(while_stm: &WhileStmt, ctx: &CurrentContext, comments: &SingleThreadedComments) -> Loop{
    let loop_stmt = parse_stmt(&while_stm.body, ctx,comments);
    let cond_val = parse_expr(&while_stm.test, ctx, comments);
    let loop_st = Loop{
        cond: cond_val,
        loop_scope: Box::new(loop_stmt)
    };
    loop_st
}
