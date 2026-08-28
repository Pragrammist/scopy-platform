mod test_unit_helpers;
mod unit_tests;

use core::{panic};
use std::fs;
use std::path::{PathBuf};
use std::rc::Rc;
use serde::Serialize;
use swc_common::{SourceFile, Span, Spanned};
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_ast::*;
use swc_common::comments::{CommentKind, Comments, SingleThreadedComments};
use std::collections::{HashMap, HashSet};
use swc_common::comments::Comment;
use walkdir::{DirEntry, WalkDir};
type ObjectIdent = String;



#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
struct GlobalContext{
    parsed_modules: HashMap<String, ScopyModule>
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
struct CurrentContext {
    prev: Option<Box<CurrentContext>>,
    context_type: CurrentContextType,
    current_statements: Vec<Statement>,
    current_module_name: String
}


#[derive(Debug)]
#[derive(Clone)]
struct AstCurrentContext{
    module:Module,
    name: String
}



#[derive(Clone)]
#[derive(Default)]
struct AstGlobalContext{
    comments:SingleThreadedComments,
    cm: Lrc<SourceMap>,
    ast_modules: HashMap<String, Module>
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

impl Default for CurrentContextType {
    fn default() -> Self { CurrentContextType::ModuleContext(ModuleContext::default()) }
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
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
    // object_type: ObjectDataType,
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
    path: Vec<AnotherObjectValuePath>
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
enum AnotherObjectValuePath{
    Ident(ObjectIdent),
    FunctionCall {name: ObjectIdent, func_call: FunctionCallResult},
}




#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
#[serde(tag = "lit-val", content = "lit-val-data")]
enum LiteralValue {
    Str (LitValueString),
    Bool (LitValueBool),
    Null,
    Num (LitValueNum)
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct LitValueString{
    val: String,
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct LitValueNum{
    val: f64
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct LitValueBool{
    val: bool
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
#[derive(Clone)]
#[derive(Serialize)]
struct FunctionCallResult{
    function_meta: FunctionMetaData,
    args:Vec<ObjectDataValue>,
    result: Box<ObjectDataValue>,
    path: Vec<AnotherObjectValuePath>
}


#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct FunctionMetaData{
    result: Box<ObjectData>,
    args: Vec<ObjectData>,
}

#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize)]
struct Function{
    meta:Box<FunctionMetaData>,
    statement: Box<Statement>,
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
    Scope(Scope),
    Import (Import),
    Export (Export),
}

#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
struct Import {
    val: Vec<ObjectData>,
    src: String,
}



#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
enum Export {
    ObjectExport(ExportObject),
    ObjectIdentExport(ExportObjectIdent)
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
struct ExportObject{
    val: ObjectData,
    src: String
}


#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
struct ExportObjectIdent{
    src: String,
    val: Vec<ObjectIdent>
}






#[allow(unused)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
struct  ScopyModule{
    name: String,
    statements: Vec<Statement>,
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



enum CodeModuleSourceFileType{
    Internal,
    External,
}

struct CodeModuleMetaData{
    code: String,
    name: String,
    module_meta_type: CodeModuleSourceFileType,
}





fn default_module_meta() -> CodeModuleMetaData{
    CodeModuleMetaData{
        code:  r#"
            const string = {};

            const bool = {};

            const number = {};

            const generic = {};

            export {string, bool, number, generic};
        "#.to_string(),
        name: "default.js".to_string(),
        module_meta_type: CodeModuleSourceFileType::Internal,
    }
}



fn test_module_meta() -> CodeModuleMetaData{
    CodeModuleMetaData{
        code:  r#"
            import {string, bool, number, generic} from "default.js";

            export const va1 = {
                va21:{
                    va31:(result={va41:{}}) => {},
                },
                va22: {
                    va31:{}
                }
            };



            va1.va21.va31().va41

        "#.to_string(),
        name: "main.js".to_string(),
        module_meta_type: CodeModuleSourceFileType::Internal,
    }
}




fn create_source_file(cm: &Lrc<SourceMap>, meta: &CodeModuleMetaData) -> Rc<SourceFile>{

    match meta.module_meta_type{
        CodeModuleSourceFileType::Internal =>
            cm.new_source_file(FileName::Internal(meta.name.clone().into()).into(), meta.code.clone()),
        CodeModuleSourceFileType::External =>
            cm.new_source_file(FileName::Real(meta.name.clone().into()).into(), meta.code.clone()),
    }

}







fn get_ast_from_src(meta: &CodeModuleMetaData, glob_ast_ctx: &AstGlobalContext) -> AstCurrentContext{

    let module_name = meta.name.clone();


    let source_file = create_source_file(&glob_ast_ctx.cm, &meta);
    // let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*source_file),
        Some(&glob_ast_ctx.comments),
    );



    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().expect("failed to parse");

    AstCurrentContext{
        module: module,
        name: module_name
    }
}


fn get_ctxs(ast_global_context: &AstGlobalContext) -> Vec<AstCurrentContext>{
    let internal = collect_internal_modules(ast_global_context);
    let external = collect_js_files(ast_global_context);
    let ctxs = [internal, external].concat();

    let module_parse_queue = reorder_modules_to_parse(&ctxs);

    let mut vec_res: Vec<AstCurrentContext> = Vec::new();

    module_parse_queue.into_iter().for_each(|module_name| {
        let ctx = ctxs.iter().find(|ctx_i| ctx_i.name == module_name);

        match ctx{
            None => panic!("There is no module {:?} while trying iter from reorder", module_name),
            Some(ctx) => vec_res.push(ctx.clone())
        }

    });
    vec_res
}

fn is_ignored(entry: &DirEntry) -> bool {
    let ignored_dirs = [
        "target",
        "node_modules",
        ".git",
        "ir_output"
    ];

    entry.path()
        .components()
        .any(|c| {
            let name = c.as_os_str().to_str();

            ignored_dirs.contains(&name.unwrap_or(""))
        })
}



fn collect_internal_modules(ast_global_context: &AstGlobalContext) -> Vec<AstCurrentContext>{
    let internal_modules = vec![test_module_meta(), default_module_meta()];
    internal_modules.into_iter().map(|meta| get_ast_from_src(&meta, ast_global_context)).collect()
}

fn collect_js_files(ast_global_context: &AstGlobalContext) -> Vec<AstCurrentContext> {
    let cur_dir = std::env::current_dir();

    let res = match cur_dir {
        Ok(cwd) => {
            let res = WalkDir::new(&cwd)
                .into_iter()
                .filter_entry(|e| !is_ignored(e))
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .filter(|entry| {
                    entry.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        == Some("js")
                })
                .map(|entry| {

                    let path_result = entry
                        .path()
                        .strip_prefix(&cwd.as_path());


                    match path_result {
                        Ok(entry) => entry.to_path_buf(),
                        Err(err) => panic!("Error while trying parse path {}", err)
                    }
                })
                .map(|f| {


                    fn read_path_buf(f: &PathBuf) -> String {
                        let read_result = fs::read_to_string(&f);
                        let file_content = match read_result {
                            Ok(result) => result,
                            Err(err) => {
                                panic!("Error while fetching {:}", err)
                            }
                        };
                        file_content
                    }

                    fn get_file_name(f: &PathBuf) -> String {
                        let read_file_name = f.file_name();
                        match read_file_name {
                            None => {
                                panic!("Error while trying reading file name")
                            }
                            Some(file_name) => {
                                match file_name.to_str() {
                                    None => {
                                        panic!("Error while trying reading file name and trying to str it")
                                    }
                                    Some(file_name) => {
                                        file_name.replace(['/', '\\'], "_")
                                    }
                                }
                            }
                        }
                    }


                    CodeModuleMetaData{
                        module_meta_type: CodeModuleSourceFileType::External,
                        code: read_path_buf(&f),
                        name: get_file_name(&f),
                    }
                })
                .map(|f| get_ast_from_src(&f, ast_global_context))
                .collect();
            res
        }
        Err(err) => panic!("Could not get current directory. {:?}", err),
    };

    res

}




fn parse_modules(){


    let mut global_ast_context = AstGlobalContext{
        ast_modules: HashMap::new(),
        comments: SingleThreadedComments::default(),
        cm: Default::default(),
    };
    let ctxs = get_ctxs(&global_ast_context);

    let mut glob_ctx = GlobalContext{
        parsed_modules: HashMap::new(),
    };


    ctxs.into_iter().for_each(|ctx| {

        global_ast_context.ast_modules.insert(ctx.name.clone(), ctx.module.clone());

        let parsed_module = module_parse(&ctx, &global_ast_context, &glob_ctx);
        glob_ctx.parsed_modules.insert(ctx.name.clone(), parsed_module.clone());
        dump_ir_html(&parsed_module, ctx.name.clone())
    });


}


fn reorder_modules_to_parse(ctxs: &Vec<AstCurrentContext>) -> Vec<String>{
    let mut modules_graph: HashMap<String, Vec<String>> = HashMap::new();


    ctxs.into_iter().for_each(|ctx| {
        let modules  = module_import_map(&ctx);
        let module_name = ctx.name.clone();
        modules_graph.insert(module_name, modules);
    });

    println!("modules: {:?}", modules_graph);





    let mut module_parse_queue : Vec<String> = Vec::new();
    let mut module_parse_stack: Vec<String> = vec!["main.js".to_string()];





    while module_parse_stack.len() != 0 {
        let current_module_name = module_parse_stack.pop().unwrap_or("".to_string());


        if current_module_name.is_empty() {
            break;
        }


        let inner_modules = modules_graph.entry(current_module_name.clone()).or_default();


        let inner_modules_len = inner_modules.len();


        if inner_modules_len == 0 && !module_parse_queue.contains(&current_module_name) {


            module_parse_queue.push(current_module_name.clone());



        }

        if inner_modules_len > 0 && !module_parse_queue.contains(&current_module_name){
            let inner_module_result = inner_modules.iter()
                .find(|inner_module| !module_parse_queue.contains(&inner_module.to_string()));

            if let Some(inner_module) = inner_module_result {
                module_parse_stack.push(current_module_name.to_string());
                module_parse_stack.push(inner_module.clone());
            }
            else{
                module_parse_queue.push(current_module_name.to_string());
            }

        }
    }

    module_parse_queue
}



fn main() {
    parse_modules();
}




fn dump_ir_html(ir: &ScopyModule, name: String) {


    // 1. сериализация
    let json = serde_json::to_string(ir).unwrap_or(String::new());

    if json.is_empty() {
        panic!("cannot serialize module for dumping in html");
    }


    // 2. путь к шаблону (рядом с Cargo.toml)
    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("viewer.template.html");

    let template = fs::read_to_string(template_path)
        .expect("failed to read viewer.template.html");

    // 3. вставка JSON
    let html = template.replace("__JSON__", &json);

    // 4. запись результата
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("ir_output/{name}_ir.html"));

    let write_res = fs::write(out_path, html);

    if write_res.is_err(){
        panic!("failed to write ir.html");
    }

}

fn module_parse(ast_current_context: &AstCurrentContext, ast_global_context: &AstGlobalContext, glob_ctx: &GlobalContext) -> ScopyModule{
    let mut ctx_statements: Vec<Statement> = Vec::new();
    
    let module_stmts = ast_current_context.module.body.iter().map(|el| {
        let context = CurrentContext{
            context_type: CurrentContextType::ModuleContext(ModuleContext  {
                name: ast_current_context.name.clone()
            }),
            current_module_name: ast_current_context.name.clone(),
            prev: None,
            current_statements: ctx_statements.clone(),
        };
        let res_el = parse_module_item(el, &context, glob_ctx, ast_global_context);
        ctx_statements.push(res_el.clone());
        res_el
    }).collect::<Vec<_>>();
    ScopyModule { name: ast_current_context.name.clone(), statements: module_stmts }
}





fn parse_module_item(el: &ModuleItem, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement {
    let res_el = match el {
        ModuleItem::ModuleDecl(module_decl) => {
            let import_stmt = parse_module_decl(module_decl, ctx, glob_ctx, ast_global_context);
            import_stmt
        },
        ModuleItem::Stmt(stmt) => {
            let p_stmt = parse_stmt(stmt, ctx, glob_ctx, ast_global_context);
            p_stmt
        },
    };
    res_el
}


fn module_import_map(ast_context: &AstCurrentContext) -> Vec<String>{

    let mut module_stmts: HashSet<String> = HashSet::new();
    ast_context.module.body.iter()
        .for_each(|el| {
            match el {
                ModuleItem::ModuleDecl(import_name) => {
                    match import_module_decl_module_import_map(import_name) {
                        None => {}
                        Some(module_name) => {module_stmts.insert(module_name);}
                    }
                },
                ModuleItem::Stmt(_) => {},
            }
        });

    module_stmts.into_iter().collect()

}



fn import_module_decl_module_import_map(module_decl: &ModuleDecl) -> Option<String>{
    let module_name = match module_decl{
        ModuleDecl::Import(import_decl) => {
            let module_name = get_import_decl_module_name(import_decl);
            Some(module_name)
        },
        ModuleDecl::ExportDecl(_) => None,
        ModuleDecl::ExportNamed(_) => None,
        ModuleDecl::ExportDefaultDecl(_) => None,
        ModuleDecl::ExportDefaultExpr(_) => None,
        ModuleDecl::ExportAll(_) => None,
        ModuleDecl::TsImportEquals(_) => None,
        ModuleDecl::TsExportAssignment(_) => None,
        ModuleDecl::TsNamespaceExport(_) => None,
    };

    module_name
}





fn parse_export_specifier(spec: &ExportSpecifier) -> String{
    match spec {
        ExportSpecifier::Namespace(_) => {
            panic!("Namespace export not supported");
        }
        ExportSpecifier::Default(_) => {
            panic!("Default export not supported");
        }
        ExportSpecifier::Named(named_spec) => {
            parse_export_named_specifier(named_spec)
        }
    }
}

fn parse_export_named_specifier(spec: &ExportNamedSpecifier) -> String{
    match &spec.exported {
        None => {
            let d = parse_export_module_name(&spec.orig);
            d
        }
        Some(spec_name) => {
            let d = parse_export_module_name(spec_name);
            d
        },
    }
}

fn parse_export_module_name(spec_name: &ModuleExportName) ->String
{
    let d = match spec_name {
        ModuleExportName::Ident(ident) => parse_ident(&ident),
        ModuleExportName::Str(str_name) => str_name
            .value
            .as_str()
            .unwrap()
            .to_string(),
    };

    d
}


fn parse_module_decl(module_decl: &ModuleDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement{
    let stmt = match module_decl{
        ModuleDecl::Import(import_decl) => {
            let src = get_import_decl_module_name(import_decl);
            let imports = parse_import_decl(import_decl, glob_ctx);

            Statement::Import(Import{
                val: imports,
                src: src
            })
        },
        ModuleDecl::ExportDecl(export_decl) => {
            let obj_data = parse_export_decl(export_decl, ctx, glob_ctx, ast_global_context);
            Statement::Export(Export::ObjectExport(ExportObject{
                val: obj_data,
                src: ctx.current_module_name.clone()
            }))
        },
        ModuleDecl::ExportNamed(named_export) => {
            let named_spec = named_export.specifiers.iter().map(|spec| {
                let parse_spec = parse_export_specifier(spec);
                parse_spec as ObjectIdent
            }).collect::<Vec<_>>();

            if named_export.src != None
            {
                panic!("from while export not allowed");
            }


            Statement::Export(Export::ObjectIdentExport(ExportObjectIdent{
                val: named_spec,
                src: ctx.current_module_name.clone()
            }))
        },
        ModuleDecl::ExportDefaultDecl(_) => panic!("default export not allowed"),
        ModuleDecl::ExportDefaultExpr(_) => panic!("default export not allowed"),
        ModuleDecl::ExportAll(_) => panic!("export another module not allowed"),
        ModuleDecl::TsImportEquals(_) => panic!("ts not allowed"),
        ModuleDecl::TsExportAssignment(_) => panic!("ts not allowed"),
        ModuleDecl::TsNamespaceExport(_) => panic!("ts not allowed"),
    };

    
    stmt
    
}









fn parse_export_decl(export_decl: &ExportDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
    match &export_decl.decl {
        Decl::Class(_) => panic!("class not allowed"),
        Decl::Fn(_) => panic!("use arrow fn instead function"),
        Decl::Var(var_decl) => {
            let p = parse_decl(var_decl, ctx, glob_ctx, ast_global_context);
            p
        },
        Decl::Using(_) => panic!("using not allowed"),
        Decl::TsInterface(_) => panic!("ts not allowed"),
        Decl::TsTypeAlias(_) => panic!("ts not allowed"),
        Decl::TsEnum(_) => panic!("ts not allowed"),
        Decl::TsModule(_) => panic!("ts not allowed"),
    }
}



fn create_context_in_global_context(global_ctx: &GlobalContext, module_name: &String) -> CurrentContext{
    let res = global_ctx.parsed_modules.iter().find_map(|(name, m)| {
        if *name == *module_name{
            let context = CurrentContext{
                context_type: CurrentContextType::ModuleContext(ModuleContext  {
                    name: module_name.clone()
                }),
                current_module_name: module_name.clone(),
                prev: None,
                current_statements: m.statements.clone(),
            };
            Some(context)
        }
        else{
            None
        }
    });

    if res.is_none(){
        panic!("Module not found {:?}", module_name);
    }
    else {
        res.unwrap()
    }
}





fn create_object_from_exports(ctx: &CurrentContext, glob_ctx: &GlobalContext, import_name: &String) -> ObjectData{

    let props = ctx.current_statements.iter().filter_map(|stmt|{
        if let Statement::Export(export_obj) = stmt{
            let obj = create_objects_from_export(export_obj, ctx, glob_ctx);
            Some(obj)
        }
        else { None }
    }).flatten().collect::<Vec<_>>();

    ObjectData{
        value: ObjectDataValue::Object(ObjectValue{
            props: props,
        }),
        attrs: vec![],
        is_mutable: false,
        name: import_name.clone(),
    }
}


fn create_objects_from_export(export: &Export, ctx: &CurrentContext, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let val = match export {
        Export::ObjectExport(exp_obj) => {
            let val = exp_obj.val.clone();
            vec![val]
        }
        Export::ObjectIdentExport(ident_export) => {
            let props = ident_exports_to_objs(ident_export.val.clone(), ctx, glob_ctx);
            props
        }
    };
    val
}


fn ident_exports_to_objs(ident_exports: Vec<ObjectIdent>, ctx: &CurrentContext, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let objs = ident_exports.iter().map(|export_obj| {
        let obj_opt = find_in_context(ctx, export_obj.clone(), glob_ctx);
        if let Some(obj_val) = obj_opt {
            let obj = ObjectData{
                is_mutable: false,
                name: export_obj.clone(),
                attrs: vec![],
                value: ObjectDataValue::AnotherObject(obj_val)
            };
            obj
        }
        else {
            panic!("Could not find object export {:?}", export_obj);
        }
    }).collect::<Vec<_>>();
    objs
}





fn get_import_decl_module_name(import_decl: &ImportDecl) -> String{
    import_decl.src.value.to_string_lossy().into_owned()
}



fn parse_import_decl(import_decl: &ImportDecl, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let imports = import_decl.specifiers.iter().map(parse_import_specifier)
    .map(|(name, is_named)| {
        let import_name = get_import_decl_module_name(&import_decl);
        let module_context = create_context_in_global_context(glob_ctx, &import_name);
        let another_obj_res = find_in_context(&module_context, name.clone(), glob_ctx);


        match another_obj_res {
            None => {
                if is_named {
                    panic!("{:?} is not found", name.clone());
                }
                let obj_d = create_object_from_exports(&module_context, glob_ctx, &import_name);
                obj_d
            }
            Some(another_obj) => {
                *another_obj.obj
            }
        }




    }).collect::<Vec<_>>();
    imports
}

fn parse_import_specifier(s: &ImportSpecifier) -> (String, bool){
    match s {
        ImportSpecifier::Named(import_named_specifier) => {
            let name = parse_named_specifier(import_named_specifier);
            (name, true)
        },
        ImportSpecifier::Default(import_default_specifier) => {
            // let name = parse_default_specifier(import_default_specifier);
            // name
            panic!("Dont use default specifier {:?}", import_default_specifier);
        },
        ImportSpecifier::Namespace(import_start_specifier) => {
            let name = parse_import_start_specifier(import_start_specifier);
            (name, false)
        },
    }
}


fn parse_import_start_specifier(import_default_specifier: &ImportStarAsSpecifier) -> String {
    parse_ident(&import_default_specifier.local)
}



fn parse_named_specifier(import_named_specifier: &ImportNamedSpecifier) -> String{


    match &import_named_specifier.imported {
        None => parse_ident(&import_named_specifier.local),
        Some(exported) => parse_module_export_name(&exported),
    }


}


fn parse_module_export_name(imported: &ModuleExportName) -> String{
    let name = match imported{
        ModuleExportName::Ident(ident) => parse_ident(&ident),
        ModuleExportName::Str(_) => panic!("str export not allowed"),
    };
    name
}

fn parse_stmt(stmt: &Stmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement{
    let stmt = match stmt{
        Stmt::Block(block_stmt) => {
            let scope = parse_block(block_stmt, ctx, glob_ctx, ast_global_context);
            let stmt = Statement::Scope(scope);
            stmt
        },
        Stmt::Empty(_empty_stmt) => panic!("empty statement not allowed"),
        Stmt::Debugger(_debugger_stmt) => panic!("debugger not allowed"),
        Stmt::With(_) => panic!("ts not allowed"),
        Stmt::Return(_return_stmt) => panic!("const result = yourResult use instead"),
        Stmt::Labeled(_labeled_stmt) => panic!("labeled statement not allowed"),
        Stmt::Break(_break_stmt) => panic!("break statement not allowed"),
        Stmt::Continue(_continue_stmt) => panic!("continue statement not allowed"),
        Stmt::If(if_stmt) => {
            let if_cond = parse_if(if_stmt, ctx, glob_ctx, ast_global_context);

            Statement::Conditional(if_cond)
        },
        Stmt::Switch(_switch_stmt) => panic!("switch statement not allowed"),
        Stmt::Throw(_throw_stmt) => panic!("throw statement not allowed"),
        Stmt::Try(_try_stmt) => panic!("try statement not allowed"),
        Stmt::While(while_stmt) => {
            let loop_s = parse_while(while_stmt, ctx, glob_ctx, ast_global_context);
            let loop_stmt = Statement::Loop(loop_s);
            loop_stmt
        },
        Stmt::DoWhile(_do_while_stmt) => panic!("do while not allowed"),
        Stmt::For(_for_stmt) => panic!("for not allowed"),
        Stmt::ForIn(_for_in_stmt) => panic!("for in not allowed"),
        Stmt::ForOf(_for_of_stmt) => panic!("for of not allowed"),
        Stmt::Decl(decl) => {
            match decl{
                Decl::Class(_class_decl) => panic!("class declaration not allowed"),
                Decl::Fn(_fn_decl) => panic!("function not allowed"),
                Decl::Var(var_decl) => {
                    let decl = parse_decl(var_decl, ctx, glob_ctx, ast_global_context);
                    let stmt = Statement::Object(decl);
                    stmt
                },
                Decl::Using(_using_decl) => panic!("using not allowed"),
                Decl::TsInterface(_ts_interface_decl) => panic!("ts not allowed"),
                Decl::TsTypeAlias(_ts_type_alias_decl) => panic!("ts not allowed"),
                Decl::TsEnum(_ts_enum_decl) => panic!("ts not allowed"),
                Decl::TsModule(_ts_module_decl) => panic!("ts not allowed"),
            }
        },
        Stmt::Expr(expr_stmt) => {
            let expr = parse_expr_stmt(expr_stmt, ctx, glob_ctx, ast_global_context);
            let stmt = Statement::ObjectValue(expr);
            stmt
        },
    };
    stmt
}







fn parse_expr_stmt(expr_stmt: &ExprStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue{
    let expr = parse_expr(&expr_stmt.expr, ctx, glob_ctx, ast_global_context);
    expr
}








fn parse_decl(var_decl: &VarDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
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
    let val = parse_expr(init, ctx, glob_ctx, ast_global_context);




    let is_mutable = match var_decl.kind {
        VarDeclKind::Var => panic!("Var is not allowed"),
        VarDeclKind::Let => true,
        VarDeclKind::Const => false,
    };

    let attrs = get_attributes(&var_decl.span, ast_global_context);

    let obj = ObjectData { 
        is_mutable: is_mutable,
        name: var_name, 
        value: val,
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


fn get_attributes(span: &Span, ast_global_context: &AstGlobalContext) -> Vec<Attribute>{
    let comment_opt = ast_global_context.comments.get_leading(span.lo());
    if let Some(vec_comments) = comment_opt{
        let attrs = parse_attributes_from_comments(vec_comments);
        attrs
    }
    else{
        Vec::new()
    }
}







fn parse_attributes_from_comments(comments: Vec<Comment>) -> Vec<Attribute> {
    let mut result = Vec::new();

    for comment in comments {
        if comment.kind == CommentKind::Block {
            continue;
        }
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


fn find_in_context(ctx: &CurrentContext, name: ObjectIdent, glob_ctx: &GlobalContext) -> Option<AnotherObjectValue> {
    let mut current = Some(ctx);

    while let Some(c) = current {
        if let Some(obj) = c.current_statements
            .iter()
            .find_map(|stmt| find_in_statements(stmt, ctx, &name, glob_ctx)) {
                return Some(AnotherObjectValue { obj: Box::new(obj), path: vec![AnotherObjectValuePath::Ident(name)] });
            }
        current = c.prev.as_deref();
    }

    None
}






fn check_obj_data(object_data: &ObjectData, name: &String) -> Option<ObjectData>{
    if object_data.name == *name{
        Some(object_data.clone())
    }
    else { None }
}

fn find_obj_from_import(import: &Import, name: &String) -> Option<ObjectData>{
    let f_obj = import.val.iter().find(|val| {val.name == *name}).cloned();
    f_obj
}

fn check_export_obj(obj_exp: &ExportObject, name: &String) -> Option<ObjectData>{
    if obj_exp.val.name == *name{
        Some(obj_exp.val.clone())
    }
    else { None }
}



fn find_in_scopy_module(module: &ScopyModule, name: &String, glob_ctx: &GlobalContext, ctx: &CurrentContext) -> Option<ObjectData> {
    let find_result = module.statements.iter().find_map(|stmt| {
        let obj_data = find_in_statements(&stmt, ctx,  &name, glob_ctx);
        obj_data
    });
    find_result
}



fn find_global_ctx(name: &String, glob_ctx: &GlobalContext,  ctx: &CurrentContext) -> Option<ObjectData> {
    let obj_val_opt = glob_ctx
        .parsed_modules
        .iter()
        .find_map(|(module_name, module)| {
            if *module_name == ctx.current_module_name {
                find_in_scopy_module(module, name, glob_ctx, ctx)
            }
            else { None }
        });

    obj_val_opt
}


fn check_ident_export(ident_export: &ExportObjectIdent, name: &String, ctx: &CurrentContext,  glob_ctx: &GlobalContext) -> Option<ObjectData> {
    let d = ident_export.val.iter().find_map(|val| {
        if val == name {
            let obj_val_opt = find_global_ctx(name, &glob_ctx, ctx);
            obj_val_opt
        }
        else {
            None
        }

    });
    d
}


fn find_in_statements(stmt: &Statement, ctx: &CurrentContext, name: &ObjectIdent, glob_ctx: &GlobalContext) -> Option<ObjectData>{
    match stmt {
        Statement::Object(object_data) => {
            check_obj_data(object_data, name)
        },
        Statement::ObjectValue(_) => None,
        Statement::Conditional(_) => None,
        Statement::Loop(_) => None,
        Statement::Scope(_) => None,
        Statement::Import(import) => {
            find_obj_from_import(import, name)
        }
        Statement::Export(export) => {
            match export {
                Export::ObjectExport (obj_exp) => {
                    check_export_obj(obj_exp, name)
                },
                Export:: ObjectIdentExport(ident_export) => {
                    check_ident_export(ident_export, name, ctx, glob_ctx)
                }
            }
        }
    }
}

fn parse_expr(expr:&Expr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue{
    match expr {
        Expr::This(_this_expr) => panic!("this not allowed"),
        Expr::Array(_array_lit) => panic!("array not allowed"),
        Expr::Object(object_lit) => {
            let object_val = parse_obj_lit(object_lit, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Object(object_val)
        },
        Expr::Fn(_fn_expr) => panic!("fn not allowed"),
        Expr::Unary(_unary_expr) => panic!("unary expression not allowed"),
        Expr::Update(_update_expr) => panic!("update expression  not allowed"),
        Expr::Bin(bin_expr) => {
            let bin_val = parse_bin_expr(bin_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Binary(bin_val)
        },
        Expr::Assign(_assign_expr) => panic!("assign expression not allowed"),
        Expr::Member(member_expr) => {
            let member_expr = parse_member_expr(member_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::AnotherObject(member_expr)
        },
        Expr::SuperProp(_super_prop_expr) => panic!("super prop not allowed"),
        Expr::Cond(_cond_expr) => panic!("cond expression not allowed"),
        Expr::Call(call_expr) => {
            let call_expr = parse_call_expr(call_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::FunctionCall(call_expr)
        },
        Expr::New(_new_expr) => panic!("new expression not allowed"),
        Expr::Seq(_seq_expr) => panic!("seq expression not allowed"),
        Expr::Ident(ident) => {
            let name = parse_ident(ident);
            let another_obj = find_in_context(ctx, name, glob_ctx).unwrap();
            let res = ObjectDataValue::AnotherObject(another_obj);
            res
        },
        Expr::Lit(lit) => {
            let lit_expr = parse_lit_expr(lit);
            lit_expr
        },
        Expr::Tpl(_) => panic!("ts not allowed"),
        Expr::TaggedTpl(_) => panic!("ts not allowed"),
        Expr::Arrow(arrow_expr) => {
            let function = parse_arrow_expr(arrow_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Function(function)
        },
        Expr::Class(_class_expr) => panic!("class not allowed"),
        Expr::Yield(_yield_expr) => panic!("yield allowed"),
        Expr::MetaProp(_meta_prop_expr) => panic!("meta property not allowed"),
        Expr::Await(_await_expr) => panic!("await not allowed"),
        Expr::Paren(paren_expr) => {
            let res = parse_paren_expr(paren_expr, ctx, glob_ctx, ast_global_context);
            res
        },
        Expr::JSXMember(_jsxmember_expr) => todo!(),
        Expr::JSXNamespacedName(_jsxnamespaced_name) => todo!(),
        Expr::JSXEmpty(_jsxempty_expr) => todo!(),
        Expr::JSXElement(_jsxelement) => todo!(),
        Expr::JSXFragment(_jsxfragment) => todo!(),
        Expr::TsTypeAssertion(_ts_type_assertion) => panic!("ts not allowed"),
        Expr::TsConstAssertion(_ts_const_assertion) => panic!("ts not allowed"),
        Expr::TsNonNull(_ts_non_null_expr) => panic!("ts not allowed"),
        Expr::TsAs(_ts_as_expr) => panic!("ts not allowed"),
        Expr::TsInstantiation(_ts_instantiation) => panic!("ts not allowed"),
        Expr::TsSatisfies(_ts_satisfies_expr) => panic!("ts not allowed"),
        Expr::PrivateName(_private_name) => panic!("ts not allowed"),
        Expr::OptChain(_opt_chain_expr) => panic!("ts not allowed"),
        Expr::Invalid(_invalid) => {
            panic!("invalid expression")
        },
    }
}

fn parse_obj_lit(object_lit:&ObjectLit, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectValue{
    let props = object_lit.props
        .iter()
        .flat_map(|p| parse_prop_or_spread(p, ctx, glob_ctx, ast_global_context))
        .collect::<Vec<_>>();


     ObjectValue{ props: props }
}

fn parse_prop_or_spread(p: &PropOrSpread, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectData>{
    match p{
        PropOrSpread::Spread(spread_element) => parse_spread_el(spread_element, ctx, glob_ctx, ast_global_context),
        PropOrSpread::Prop(prop) =>  vec![parse_prop(prop, ctx, glob_ctx, ast_global_context)] ,
    }
}


fn parse_spread_el(spread_element: &SpreadElement, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectData>{
    let val: ObjectDataValue = parse_expr(&spread_element.expr, ctx, glob_ctx, ast_global_context);
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


fn map_prop_obj_data(prop: Option<ObjectData>, member_name: &ObjectIdent) -> Option<AnotherObjectValue>{
    let another_obj = prop.map(|o| {
        AnotherObjectValue{
            obj: Box::new(o.clone()),
            path: vec![AnotherObjectValuePath::Ident(member_name.clone())]
        }
    });
    another_obj
}

fn another_obj_from_member(val: &ObjectDataValue, member_name: &ObjectIdent) -> Option<AnotherObjectValue>{
    match val {
        ObjectDataValue::Enum(enum_object_value) => {
            let prop = find_member_in_props(&enum_object_value.props, &member_name);
            let another_obj = map_prop_obj_data(prop, member_name);
            another_obj
        },
        ObjectDataValue::Object(object_value) => {
            let prop = find_member_in_props(&object_value.props, &member_name);
            let another_obj = map_prop_obj_data(prop, member_name);
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
            

            let another_obj = another_obj.map(|o| {
                AnotherObjectValue { obj: o.obj, path: [function_meta_data.path.clone(), o.path.clone()].concat() }
            });
            another_obj
        },
        ObjectDataValue::Binary(_) => None
    }
}






fn find_member_in_props(props: &Vec<ObjectData>, prop_name: &ObjectIdent) -> Option<ObjectData>{
    props.iter().find(|p| p.name == *prop_name).cloned()
}


fn parse_prop(prop: &Prop, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
    match prop {
        Prop::Shorthand(_ident) => panic!("Short hand ident"),
        Prop::KeyValue(key_value_prop) => parse_key_value_prop(key_value_prop, ctx, glob_ctx, ast_global_context),
        Prop::Assign(_assign_prop) => panic!("Assign prop"),
        Prop::Getter(_getter_prop) => panic!("Getter prop"),
        Prop::Setter(_setter_prop) => panic!("Setter prop"),
        Prop::Method(_method_prop) => panic!("Method prop"),
    }
}



fn parse_key_value_prop(key_value_prop: &KeyValueProp, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
    let current_object_name = parse_prop_name(&key_value_prop.key);
    let value = parse_expr(&key_value_prop.value, ctx, glob_ctx, ast_global_context);

    let res = ObjectData { 
        name: current_object_name, 
        value,
        is_mutable: false,
        attrs: get_attributes(&key_value_prop.span(), ast_global_context)
    };
    res
}

 
fn parse_prop_name(prop_name:&PropName) -> String{
    match &prop_name{
        PropName::Ident(ident_name) => {
            parse_ident_name(&ident_name)
        },
        PropName::Str(_) => panic!("str as name not allowed"),
        PropName::Num(_number) => panic!("num as name not allowed"),
        PropName::Computed(_computed_prop_name) => panic!("computed prop not allowed"),
        PropName::BigInt(_big_int) => panic!("big int not allowed"),
    }
}

fn parse_ident_name(ident_name:&IdentName) -> String{
    ident_name.sym.to_string()
}

fn parse_bin_expr(bin_expr: &BinExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> BinaryObjectValue {
    let left_expr = parse_expr(&bin_expr.left, ctx, glob_ctx, ast_global_context);
    let right_expr = parse_expr(&bin_expr.right, ctx, glob_ctx, ast_global_context);
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
    parse_ident(&binding_ident.id)
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
    ident
}












fn parse_pat_as_assign(pat: &Pat, ctx:&CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
    let ident = match &pat{
        Pat::Ident(binding_ident) => panic!("{:#?}",binding_ident),
        Pat::Array(array_pat) => panic!("{:#?}", array_pat),
        Pat::Rest(rest_pat) => panic!("{:#?}", rest_pat),
        Pat::Object(object_pat) => panic!("{:#?}", object_pat),
        Pat::Assign(assign_pat) => {
            parse_assign_pat(assign_pat, ctx, glob_ctx, ast_global_context)
        },
        Pat::Invalid(invalid) => panic!("{:#?}", invalid),
        Pat::Expr(expr) => panic!("{:#?}", expr),
    };

   ident
}


fn parse_assign_pat(assign_pat: &AssignPat, ctx:&CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData{
    let param_name = parse_pat_as_ident(&assign_pat.left);
    let obj_val = parse_expr(&assign_pat.right, ctx, glob_ctx, ast_global_context);

    let obj = ObjectData{
        attrs: get_attributes(&assign_pat.span, ast_global_context),
        is_mutable: false,
        name: param_name,
        value: obj_val,
    };
    obj
}


fn parse_paren_expr(paren_expr: &ParenExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue{
    parse_expr(&paren_expr.expr, ctx, glob_ctx, ast_global_context)
}

fn parse_arrow_expr(arrow_expr: &ArrowExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Function {

    let params = arrow_expr.params
        .iter()
        .map(|pat| {
            let obj_data = parse_pat_as_assign(pat, ctx, glob_ctx, ast_global_context);
            obj_data
        })
        .collect::<Vec<_>>();    



    
    
    let scope = parse_block_stmt_or_expr(&arrow_expr.body, ctx, glob_ctx, ast_global_context);
    let result_type = find_result_type_in_params(&params).unwrap();
    let stmt_res = Statement::Scope(scope);
    

    let meta = Box::new(FunctionMetaData { result: Box::new(result_type), args: params.clone() });
    let result = find_result_in_params(&params).unwrap();

    Function { 
        meta: meta,
        statement: Box::new(stmt_res),
        params: params,
        result:Box::new(result) 
    }
}


fn find_result_in_params(params: &Vec<ObjectData>) -> Option<ObjectData>{
    params.iter().find(|p| p.name == "result").cloned()
}


fn find_result_type_in_params(params: &Vec<ObjectData>) -> Option<ObjectData>{
    let result = params
        .iter()
        .find(|p| p.name == "result");
    result.map(|r| r.clone())
}





fn parse_block_stmt_or_expr(block_stmt_or_expr: &BlockStmtOrExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext ) -> Scope {
    match &block_stmt_or_expr{
        BlockStmtOrExpr::BlockStmt(block_stmt) => {
            let block_scope = parse_block(block_stmt, ctx, glob_ctx, ast_global_context);
            block_scope
        },
        BlockStmtOrExpr::Expr(_) => panic!("block stmt or expr not allowed"),
    }
}



fn parse_lit_expr(lit_expr: &Lit) -> ObjectDataValue{
    match &lit_expr {
        Lit::Str(val) => {
            ObjectDataValue::Literal(LiteralValue::Str (LitValueString{
                val: val.value.to_string_lossy().to_string()
            }))
        },
        Lit::Bool(val) => {
            ObjectDataValue::Literal(LiteralValue::Bool (LitValueBool{
                val: val.value
            }))
        },
        Lit::Null(_null) => {
            ObjectDataValue::Literal(LiteralValue::Null)
        },
        Lit::Num(number) => {
            ObjectDataValue::Literal(LiteralValue::Num (LitValueNum{
                val: number.value
            }))
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
    name
}


fn parse_call_expr(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> FunctionCallResult{

    let expr = get_expr_from_arg(call_expr);
    let args = get_func_args_call(call_expr, ctx, glob_ctx, ast_global_context);
    let val_call = parse_expr(expr, ctx, glob_ctx, ast_global_context);
    let func_call = func_call_result(&val_call, args, vec![]);
    detail_func_call_path(&func_call)
}


fn detail_func_call_path(func_call: &FunctionCallResult) -> FunctionCallResult{
    let path = &func_call.path;
    let len = path.len();
    let new_path = path.iter()
        .enumerate()
        .map(|(i, o)| {
            if i == len - 1 && let AnotherObjectValuePath::Ident(name) = o {
                AnotherObjectValuePath::FunctionCall {func_call: func_call.clone(), name: name.clone()}
            } else {
                o.clone()
            }

        })
        .collect::<Vec<_>>();

    FunctionCallResult{
        function_meta: func_call.function_meta.clone(),
        args: func_call.args.clone(),
        result: func_call.result.clone(),
        path: new_path
    }
}


fn get_expr_from_arg(call_expr: &CallExpr) -> &Box<Expr>{
    let expr = call_expr.callee.as_expr().unwrap();
    expr
}

fn get_func_args_call(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectDataValue>{
    let args = call_expr.args
        .iter()
        .map(|arg| {
            let obj_val = parse_expr_or_spread(arg, ctx, glob_ctx, ast_global_context);
            obj_val
        })
        .collect::<Vec<_>>();

    args
}




fn func_call_result(val_call: &ObjectDataValue, args: Vec<ObjectDataValue>, path: Vec<AnotherObjectValuePath>) -> FunctionCallResult{
    match val_call {
        ObjectDataValue::Enum(_) => panic!("not a function"),
        ObjectDataValue::Object(_) => panic!("not a function"),
        ObjectDataValue::AnotherObject(another_object_value) => func_call_result(&another_object_value.obj.value, args, [another_object_value.path.clone(), path].concat()),
        ObjectDataValue::Literal(_) => panic!("not a function"),
        ObjectDataValue::Function(func_data) => {

            let func_call = FunctionCallResult{
                function_meta: *func_data.meta.clone(),
                args: args,
                result: Box::new(func_data.result.value.clone()),
                path: path
            };
            func_call
        },
        ObjectDataValue::FunctionCall(_) => panic!("don't do foo_call()() it's hard to read"),
        ObjectDataValue::Binary(_) => panic!("not a function"),
    }
}





fn parse_expr_or_spread(expr_or_spread: &ExprOrSpread, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue{
    let is_spread = expr_or_spread.spread.is_some();

    if is_spread
    {
        panic!("Spread is not allowed in function call");
    }
        

    let arg = parse_expr(&expr_or_spread.expr,ctx, glob_ctx, ast_global_context);
    
    arg
    
}

fn parse_member_expr(member: &MemberExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> AnotherObjectValue{
    let parse_member_val = parse_expr(&member.obj,ctx, glob_ctx, ast_global_context);
    let member_prop = parse_member_prop(&member.prop);


    

    let object_stmts = another_obj_from_member(&parse_member_val, &member_prop).unwrap();

    object_stmts
}

fn parse_member_prop(member_prop: &MemberProp) -> String {
    match &member_prop{
        MemberProp::Ident(ident_name) => parse_ident_name(ident_name),
        MemberProp::PrivateName(_private_name) => panic!("private name in props not allowed"),
        MemberProp::Computed(_computed_prop_name) => panic!("computed props not allowed"),
    }
}




fn parse_block(block_stmt: &BlockStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Scope{
    let mut ctx_statements: Vec<Statement> = ctx.current_statements.clone();
    let stmts = block_stmt.stmts.iter().map(|s| {
        let context = CurrentContext{
            context_type: ctx.context_type.clone(),
            prev: Some(Box::new(ctx.clone())),
            current_statements: ctx_statements.clone(),
            current_module_name: ctx.current_module_name.clone(),
        };
        let ps = parse_stmt(s, &context, glob_ctx, ast_global_context);
        ctx_statements.push(ps.clone());
        ps
    }).collect::<Vec<_>>();
    let scope = Scope{
        name: None,
        statements: stmts
    };
    scope
}


fn parse_if(if_stmt: &IfStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Condition{

    let cons_stmt = parse_stmt(&if_stmt.cons, ctx, glob_ctx, ast_global_context);

    let test_expr = parse_expr(&if_stmt.test, ctx, glob_ctx, ast_global_context);



    let else_stmt = match &if_stmt.alt{
        Some(r) =>  Some(Box::new(parse_stmt(&r, ctx, glob_ctx, ast_global_context))),
        None => None,
    };
    
    let cond = Condition {
        cond: test_expr,
        false_scope: else_stmt,
        true_scope: Box::new(cons_stmt)
    };
    cond
}



fn parse_while(while_stm: &WhileStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Loop{
    let loop_stmt = parse_stmt(&while_stm.body, ctx, glob_ctx, ast_global_context);
    let cond_val = parse_expr(&while_stm.test, ctx, glob_ctx, ast_global_context);
    let loop_st = Loop{
        cond: cond_val,
        loop_scope: Box::new(loop_stmt)
    };
    loop_st
}
