mod unit_tests;
mod low_ir;
mod semantic;

use core::{panic};
use std::fs;
use std::path::{PathBuf};
use std::rc::Rc;
use swc_common::{SourceFile, Span, Spanned};
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_ast::*;
use swc_common::comments::{CommentKind, Comments, SingleThreadedComments};
use std::collections::{HashSet};
use swc_common::comments::Comment;
use walkdir::{DirEntry, WalkDir};
use std::collections::HashMap;
use crate::semantic::{AnotherObjectValue, AnotherObjectValuePath, AstCurrentContext, AstGlobalContext, Attribute, BinaryObjectValue, BinaryOpType, CodeModuleMetaData, CodeModuleSourceFileType, ConditionStatement, CurrentContext, CurrentContextType, ExportStatement, ExportObject, ExportObjectIdent, FunctionCallResultValue, FunctionValue, GlobalContext, LitValueBool, LitValueNum, LitValueString, LiteralValue, LoopStatement, ModuleContext, ObjectData, ObjectDataValue, ObjectIdent, ObjectValue, ScopyModule, Statement, ImportStatement, ScopeStatement};

macro_rules! compiler_panic {
    ($reason:expr) => {{
        let reason = $reason;
        eprintln!("Compiler panic: {reason}");
        std::panic::panic_any(reason);
    }};
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
        name:"default.js".to_string(),
        module_meta_type: CodeModuleSourceFileType::Internal,
    }
}



fn test_module_meta() -> CodeModuleMetaData{
    CodeModuleMetaData{
        code:  r#"
            import {string, bool, number, generic} from "default.js";

            export const va1 = {
                va21={
                    va31=(result={va41={}}) => {},
                },
                va22={
                    va31={}
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


fn reorder_modules_to_parse(ctxs: &Vec<AstCurrentContext>) -> Vec<ObjectIdent>{
    let mut modules_graph: HashMap<ObjectIdent, Vec<ObjectIdent>> = HashMap::new();


    ctxs.into_iter().for_each(|ctx| {
        let modules  = module_import_map(&ctx);
        let module_name = ctx.name.clone();
        modules_graph.insert(module_name, modules);
    });

    println!("modules: {:?}", modules_graph);





    let mut module_parse_queue : Vec<ObjectIdent> = Vec::new();
    let mut module_parse_stack: Vec<ObjectIdent> = vec!["main.js".to_string()];





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




fn dump_ir_html(ir: &ScopyModule, name: ObjectIdent) {


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


fn module_import_map(ast_context: &AstCurrentContext) -> Vec<ObjectIdent>{

    let mut module_stmts: HashSet<ObjectIdent> = HashSet::new();
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



fn import_module_decl_module_import_map(module_decl: &ModuleDecl) -> Option<ObjectIdent>{
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





fn parse_export_specifier(spec: &ExportSpecifier) -> ObjectIdent{
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

fn parse_export_named_specifier(spec: &ExportNamedSpecifier) -> ObjectIdent{
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

fn parse_export_module_name(spec_name: &ModuleExportName) ->ObjectIdent
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

            Statement::Import(ImportStatement{
                val: imports,
                src: src
            })
        },
        ModuleDecl::ExportDecl(export_decl) => {
            let obj_data = parse_export_decl(export_decl, ctx, glob_ctx, ast_global_context);
            Statement::Export(ExportStatement::ObjectExport(ExportObject{
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
                compiler_panic!(ModuleDeclPanic::NotAllowedExport)
            }


            Statement::Export(ExportStatement::ObjectIdentExport(ExportObjectIdent{
                val: named_spec,
                src: ctx.current_module_name.clone()
            }))
        },
        ModuleDecl::ExportDefaultDecl(_) => compiler_panic!(ModuleDeclPanic::DefaultExportNotAllowed),
        ModuleDecl::ExportDefaultExpr(_) => compiler_panic!(ModuleDeclPanic::DefaultExportNotAllowed),
        ModuleDecl::ExportAll(_) => compiler_panic!(ModuleDeclPanic::ExportAllNotAllowed),
        ModuleDecl::TsImportEquals(_) => compiler_panic!(ModuleDeclPanic::TsImportEqualsNotAllowed),
        ModuleDecl::TsExportAssignment(_) => compiler_panic!(ModuleDeclPanic::TsExportAssignmentNotAllowed),
        ModuleDecl::TsNamespaceExport(_) => compiler_panic!(ModuleDeclPanic::TsNamespaceExportNotAllowed),
    };

    
    stmt
    
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ModuleDeclPanic {
    DefaultExportNotAllowed,
    ExportAllNotAllowed,
    TsImportEqualsNotAllowed,
    TsExportAssignmentNotAllowed,
    TsNamespaceExportNotAllowed,
    NotAllowedExport,
    DefaultSpecifiersNotAllowed,
    ClassExportNotAllowed,
    UsingExportNotAllowed,
    FnExportNotAllowed,
    TsNotAllowed, // общий для любых TS-конструкций в экспорте
}

impl std::fmt::Display for ModuleDeclPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultExportNotAllowed => write!(f, "Default export is not allowed"),
            Self::ExportAllNotAllowed => write!(f, "Export all from another module is not allowed"),
            Self::TsImportEqualsNotAllowed => write!(f, "TypeScript import equals is not allowed"),
            Self::TsExportAssignmentNotAllowed => write!(f, "TypeScript export assignment is not allowed"),
            Self::TsNamespaceExportNotAllowed => write!(f, "TypeScript namespace export is not allowed"),
            Self::NotAllowedExport => write!(f, "From while export not allowed"),
            Self::DefaultSpecifiersNotAllowed => write!(f, "Default specifiers is not allowed"),
            Self::ClassExportNotAllowed => write!(f, "Exporting class declaration is not allowed"),
            Self::UsingExportNotAllowed => write!(f, "Exporting using declaration is not allowed"),
            Self::FnExportNotAllowed => write!(f, "Exporting function declaration is not allowed; use arrow functions instead"),
            Self::TsNotAllowed => write!(f, "TypeScript syntax is not allowed"),
        }
    }
}








fn parse_export_decl(export_decl: &ExportDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
    match &export_decl.decl {
        Decl::Var(var_decl) => parse_decl(var_decl, ctx, glob_ctx, ast_global_context),
        Decl::Class(_) => compiler_panic!(ModuleDeclPanic::ClassExportNotAllowed),
        Decl::Using(_) => compiler_panic!(ModuleDeclPanic::UsingExportNotAllowed),
        Decl::Fn(_) => compiler_panic!(ModuleDeclPanic::FnExportNotAllowed),
        Decl::TsInterface(_) => compiler_panic!(ModuleDeclPanic::TsNotAllowed),
        Decl::TsTypeAlias(_) => compiler_panic!(ModuleDeclPanic::TsNotAllowed),
        Decl::TsEnum(_) => compiler_panic!(ModuleDeclPanic::TsNotAllowed),
        Decl::TsModule(_) => compiler_panic!(ModuleDeclPanic::TsNotAllowed),
    }
}



fn create_context_in_global_context(global_ctx: &GlobalContext, module_name: &ObjectIdent) -> CurrentContext{
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





fn create_object_from_exports(ctx: &CurrentContext, glob_ctx: &GlobalContext, import_name: &ObjectIdent) -> ObjectData {

    let props = ctx.current_statements.iter().filter_map(|stmt|{
        if let Statement::Export(export_obj) = stmt{
            let obj = create_objects_from_export(export_obj, ctx, glob_ctx);
            Some(obj)
        }
        else { None }
    }).flatten().collect::<Vec<_>>();

    ObjectData {
        value: ObjectDataValue::Object(ObjectValue{
            props: props,
        }),
        attrs: vec![],
        is_mutable: false,
        name: import_name.clone(),
    }
}


fn create_objects_from_export(export: &ExportStatement, ctx: &CurrentContext, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let val = match export {
        ExportStatement::ObjectExport(exp_obj) => {
            let val = exp_obj.val.clone();
            vec![val]
        }
        ExportStatement::ObjectIdentExport(ident_export) => {
            let props = ident_exports_to_objs(ident_export.val.clone(), ctx, glob_ctx);
            props
        }
    };
    val
}


fn ident_exports_to_objs(ident_exports: Vec<ObjectIdent>, ctx: &CurrentContext, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let objs = ident_exports.iter().map(|export_obj| {
        let obj_val = find_in_context(ctx, export_obj.clone(), glob_ctx);
        let obj = ObjectData {
            is_mutable: false,
            name: export_obj.clone(),
            attrs: vec![],
            value: ObjectDataValue::AnotherObject(obj_val)
        };
        obj
    }).collect::<Vec<_>>();
    objs
}





fn get_import_decl_module_name(import_decl: &ImportDecl) -> ObjectIdent{
    import_decl.src.value.to_string_lossy().into_owned()
}



fn parse_import_decl(import_decl: &ImportDecl, glob_ctx: &GlobalContext) -> Vec<ObjectData>{
    let imports = import_decl.specifiers.iter().map(parse_import_specifier)
    .map(|(name, is_named)| {
        let import_name = get_import_decl_module_name(&import_decl);
        let module_context = create_context_in_global_context(glob_ctx, &import_name);
        if is_named {
            let another_obj_res = find_in_context(&module_context, name.clone(), glob_ctx);
            *another_obj_res.obj
        }
        else {
            let obj_d = create_object_from_exports(&module_context, glob_ctx, &import_name);
            obj_d
        }
    }).collect::<Vec<_>>();
    imports
}




fn parse_import_specifier(s: &ImportSpecifier) -> (ObjectIdent, bool){
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


fn parse_import_start_specifier(import_default_specifier: &ImportStarAsSpecifier) -> ObjectIdent {
    parse_ident(&import_default_specifier.local)
}



fn parse_named_specifier(import_named_specifier: &ImportNamedSpecifier) -> ObjectIdent{


    match &import_named_specifier.imported {
        None => parse_ident(&import_named_specifier.local),
        Some(exported) => parse_module_export_name(&exported),
    }


}


fn parse_module_export_name(imported: &ModuleExportName) -> ObjectIdent{
    let name = match imported{
        ModuleExportName::Ident(ident) => parse_ident(&ident),
        ModuleExportName::Str(_) => panic!("str export not allowed"),
    };
    name
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum StmtPanic {
    EmptyStatementNotAllowed,
    DebuggerNotAllowed,
    WithNotAllowed,
    ReturnNotAllowed,
    LabeledNotAllowed,
    BreakNotAllowed,
    ContinueNotAllowed,
    SwitchNotAllowed,
    ThrowNotAllowed,
    TryNotAllowed,
    DoWhileNotAllowed,
    ForNotAllowed,
    ForInNotAllowed,
    ForOfNotAllowed,
    ClassDeclarationNotAllowed,
    FunctionDeclarationNotAllowed,
    UsingNotAllowed,
    TsNotAllowed,
    NoDeclarations,
    MultipleDeclarationsNotAllowed,
    VarNotAllowed,
    LetNotAllowed,
    MissingInitializer,
    ObjectNotScope,
    ObjectValueNotScope,
    ConditionalNotScope,
    LoopNotScope,
    ImportNotScope,
    ExportNotScope,
}

impl std::fmt::Display for StmtPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyStatementNotAllowed => write!(f, "Empty statement is not allowed"),
            Self::DebuggerNotAllowed => write!(f, "Debugger statement is not allowed"),
            Self::WithNotAllowed => write!(f, "With statement is not allowed (TS/legacy)"),
            Self::ReturnNotAllowed => write!(f, "Return statement is not allowed; use 'const result = yourResult' instead"),
            Self::LabeledNotAllowed => write!(f, "Labeled statement is not allowed"),
            Self::BreakNotAllowed => write!(f, "Break statement is not allowed"),
            Self::ContinueNotAllowed => write!(f, "Continue statement is not allowed"),
            Self::SwitchNotAllowed => write!(f, "Switch statement is not allowed"),
            Self::ThrowNotAllowed => write!(f, "Throw statement is not allowed"),
            Self::TryNotAllowed => write!(f, "Try statement is not allowed"),
            Self::DoWhileNotAllowed => write!(f, "Do-while statement is not allowed"),
            Self::ForNotAllowed => write!(f, "For statement is not allowed"),
            Self::ForInNotAllowed => write!(f, "For-in statement is not allowed"),
            Self::ForOfNotAllowed => write!(f, "For-of statement is not allowed"),
            Self::ClassDeclarationNotAllowed => write!(f, "Class declaration is not allowed"),
            Self::FunctionDeclarationNotAllowed => write!(f, "Function declaration is not allowed; use arrow functions instead"),
            Self::UsingNotAllowed => write!(f, "Using declaration is not allowed"),
            Self::TsNotAllowed => write!(f, "TypeScript syntax is not allowed"),
            Self::NoDeclarations => write!(f, "No declaration found"),
            Self::MultipleDeclarationsNotAllowed => write!(f, "Multiple declarations in one statement are not allowed"),
            Self::VarNotAllowed => write!(f, "Var not allowed"),
            Self::LetNotAllowed => write!(f, "Let not allowed"),
            Self::MissingInitializer => write!(f, "Missing initializer error"),
            Self::ObjectNotScope => write!(f, "Object not scope"),
            Self::ObjectValueNotScope => write!(f, "Object value not scope"),
            Self::ConditionalNotScope => write!(f, "Conditional not scope"),
            Self::LoopNotScope => write!(f, "Loop not scope"),
            Self::ImportNotScope => write!(f, "Import not scope"),
            Self::ExportNotScope => write!(f, "Export not scope"),
        }
    }
}

fn parse_stmt(stmt: &Stmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement{
    let stmt = match stmt{
        Stmt::Block(block_stmt) => {
            let scope = parse_block(block_stmt, ctx, glob_ctx, ast_global_context);
            let stmt = Statement::Scope(scope);
            stmt
        },
        Stmt::If(if_stmt) => {
            let if_cond = parse_if(if_stmt, ctx, glob_ctx, ast_global_context);

            Statement::Conditional(if_cond)
        },
        Stmt::While(while_stmt) => {
            let loop_s = parse_while(while_stmt, ctx, glob_ctx, ast_global_context);
            let loop_stmt = Statement::Loop(loop_s);
            loop_stmt
        },
        Stmt::Decl(decl) => {
            match decl{
                Decl::Var(var_decl) => {
                    let decl = parse_decl(var_decl, ctx, glob_ctx, ast_global_context);
                    let stmt = Statement::Object(decl);
                    stmt
                },
                Decl::Class(_) => compiler_panic!(StmtPanic::ClassDeclarationNotAllowed),
                Decl::Fn(_) => compiler_panic!(StmtPanic::FunctionDeclarationNotAllowed),
                Decl::Using(_) => compiler_panic!(StmtPanic::UsingNotAllowed),
                Decl::TsInterface(_) => compiler_panic!(StmtPanic::TsNotAllowed),
                Decl::TsTypeAlias(_) => compiler_panic!(StmtPanic::TsNotAllowed),
                Decl::TsEnum(_) => compiler_panic!(StmtPanic::TsNotAllowed),
                Decl::TsModule(_) => compiler_panic!(StmtPanic::TsNotAllowed),
            }
        },
        Stmt::Expr(expr_stmt) => {
            let expr = parse_expr_stmt(expr_stmt, ctx, glob_ctx, ast_global_context);
            let stmt = Statement::ObjectValue(expr);
            stmt
        },
        Stmt::Empty(_) => compiler_panic!(StmtPanic::EmptyStatementNotAllowed),
        Stmt::Debugger(_) => compiler_panic!(StmtPanic::DebuggerNotAllowed),
        Stmt::With(_) => compiler_panic!(StmtPanic::WithNotAllowed),
        Stmt::Return(_) => compiler_panic!(StmtPanic::ReturnNotAllowed),
        Stmt::Labeled(_) => compiler_panic!(StmtPanic::LabeledNotAllowed),
        Stmt::Break(_) => compiler_panic!(StmtPanic::BreakNotAllowed),
        Stmt::Continue(_) => compiler_panic!(StmtPanic::ContinueNotAllowed),
        Stmt::Switch(_) => compiler_panic!(StmtPanic::SwitchNotAllowed),
        Stmt::Throw(_) => compiler_panic!(StmtPanic::ThrowNotAllowed),
        Stmt::Try(_) => compiler_panic!(StmtPanic::TryNotAllowed),
        Stmt::DoWhile(_) => compiler_panic!(StmtPanic::DoWhileNotAllowed),
        Stmt::For(_) => compiler_panic!(StmtPanic::ForNotAllowed),
        Stmt::ForIn(_) => compiler_panic!(StmtPanic::ForInNotAllowed),
        Stmt::ForOf(_) => compiler_panic!(StmtPanic::ForOfNotAllowed),
    };
    stmt
}







fn parse_expr_stmt(expr_stmt: &ExprStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
    let expr = parse_expr(&expr_stmt.expr, ctx, glob_ctx, ast_global_context);
    expr
}








fn parse_decl(var_decl: &VarDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
    // Проверка количества деклараторов
    let decls_len = var_decl.decls.len();
    if decls_len > 1 {
        compiler_panic!(StmtPanic::MultipleDeclarationsNotAllowed);
    }

    // Проверка вида объявления
    match var_decl.kind {
        VarDeclKind::Var => compiler_panic!(StmtPanic::VarNotAllowed),
        VarDeclKind::Let => compiler_panic!(StmtPanic::LetNotAllowed),
        VarDeclKind::Const => { /* разрешено */ }
    }

    let decl = var_decl.decls.first().unwrap_or_else(||{
        compiler_panic!(StmtPanic::NoDeclarations);
    });

    // Инициализатор должен быть
    let init = decl.init.as_ref().unwrap_or_else(|| {
        compiler_panic!(StmtPanic::MissingInitializer);
    });

    let val = parse_expr(init, ctx, glob_ctx, ast_global_context);
    let name = parse_pat(&decl.name); // предполагается, что она существует

    ObjectData {
        attrs: get_attributes(&decl.span, ast_global_context),
        is_mutable: false, // пока всегда false
        name,
        value: val,
    }
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


fn find_in_context(ctx: &CurrentContext, name: ObjectIdent, glob_ctx: &GlobalContext) -> AnotherObjectValue {
    let mut current = Some(ctx);

    while let Some(c) = current {
        if let Some(obj) = c.current_statements
            .iter()
            .find_map(|stmt| find_in_statements(stmt, ctx, &name, glob_ctx)) {
                return AnotherObjectValue { obj: Box::new(obj), path: vec![AnotherObjectValuePath::Ident(name)] };
            }
        current = c.prev.as_deref();
    }

    compiler_panic!(ExprPanic::ObjectNotFoundByName)
}






fn check_obj_data(object_data: &ObjectData, name: &ObjectIdent) -> Option<ObjectData>{
    if object_data.name == *name{
        Some(object_data.clone())
    }
    else { None }
}

fn find_obj_from_import(import: &ImportStatement, name: &ObjectIdent) -> Option<ObjectData>{
    let f_obj = import.val.iter().find(|val| {val.name == *name}).cloned();
    f_obj
}

fn check_export_obj(obj_exp: &ExportObject, name: &ObjectIdent) -> Option<ObjectData>{
    if obj_exp.val.name == *name{
        Some(obj_exp.val.clone())
    }
    else { None }
}



fn find_in_scopy_module(module: &ScopyModule, name: &ObjectIdent, glob_ctx: &GlobalContext, ctx: &CurrentContext) -> Option<ObjectData> {
    let find_result = module.statements.iter().find_map(|stmt| {
        let obj_data = find_in_statements(&stmt, ctx,  &name, glob_ctx);
        obj_data
    });
    find_result
}



fn find_global_ctx(name: &ObjectIdent, glob_ctx: &GlobalContext,  ctx: &CurrentContext) -> Option<ObjectData> {
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


fn check_ident_export(ident_export: &ExportObjectIdent, name: &ObjectIdent, ctx: &CurrentContext,  glob_ctx: &GlobalContext) -> Option<ObjectData> {
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
                ExportStatement::ObjectExport (obj_exp) => {
                    check_export_obj(obj_exp, name)
                },
                ExportStatement:: ObjectIdentExport(ident_export) => {
                    check_ident_export(ident_export, name, ctx, glob_ctx)
                }
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum ExprPanic {
    ThisNotAllowed,
    ArrayNotAllowed,
    FnNotAllowed,
    UnaryExpressionNotAllowed,
    UpdateExpressionNotAllowed,
    AssignExpressionNotAllowed,
    SuperPropNotAllowed,
    CondExpressionNotAllowed,
    NewExpressionNotAllowed,
    SeqExpressionNotAllowed,
    TsNotAllowed,
    YieldAllowed,
    MetaPropertyNotAllowed,
    AwaitNotAllowed,
    ClassNotAllowed,
    InvalidExpression,
    ShorthandPropNotAllowed,
    GetterPropNotAllowed,
    SetterPropNotAllowed,
    MethodPropNotAllowed,
    BinaryOperationNotSupported,
    PrivateNamePropNotAllowed,
    ComputedPropNotAllowed,
    SuperCalleeNotSupported,
    ImportCalleeNotSupported,
    EnumValueNotFunction,
    ObjectValueNotFunction,
    LiteralValueNotFunction,
    NestedFunctionCallNotAllowed,
    BinaryValueNotFunction,
    BigIntLiteralNotAllowed,
    RegexLiteralNotAllowed,
    IdentPatternNotSupported,
    ArrayPatternNotSupported,
    RestPatternNotSupported,
    ObjectPatternNotSupported,
    InvalidPatternNotSupported,
    ExpressionPatternNotSupported,
    AssignPatternNotSupported,
    ExpressionNotAllowedAsStatement,
    ResultNotFoundInParams,
    InvalidMemberExpressionMemberNotFound,
    ObjectNotFoundByName,
    EnumValueCannotBeSpread,
    ObjectValueCannotBeSpread,
    LiteralValueCannotBeSpread,
    FunctionValueCannotBeSpread,
    BinaryValueCannotBeSpread,
    KeyValuePropNotSupported,
    NotAnotherObjectValueBeforeFunctionCall,
    NotFunctionCallAfterAnotherObjectValue,
}

impl std::fmt::Display for ExprPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyValuePropNotSupported => write!(f, "Key value property not supported"),
            Self::ThisNotAllowed => write!(f, "this not allowed"),
            Self::ArrayNotAllowed => write!(f, "array not allowed"),
            Self::FnNotAllowed => write!(f, "fn not allowed"),
            Self::UnaryExpressionNotAllowed => write!(f, "unary expression not allowed"),
            Self::UpdateExpressionNotAllowed => write!(f, "update expression not allowed"),
            Self::AssignExpressionNotAllowed => write!(f, "assign expression not allowed"),
            Self::SuperPropNotAllowed => write!(f, "super prop not allowed"),
            Self::CondExpressionNotAllowed => write!(f, "cond expression not allowed"),
            Self::NewExpressionNotAllowed => write!(f, "new expression not allowed"),
            Self::SeqExpressionNotAllowed => write!(f, "seq expression not allowed"),
            Self::TsNotAllowed => write!(f, "ts not allowed"),
            Self::YieldAllowed => write!(f, "yield allowed"),
            Self::MetaPropertyNotAllowed => write!(f, "meta property not allowed"),
            Self::AwaitNotAllowed => write!(f, "await not allowed"),
            Self::ClassNotAllowed => write!(f, "class not allowed"),
            Self::InvalidExpression => write!(f, "invalid expression"),
            Self::ShorthandPropNotAllowed => write!(f, "Shorthand properties are not allowed"),
            Self::GetterPropNotAllowed => write!(f, "Getter properties are not allowed"),
            Self::SetterPropNotAllowed => write!(f, "Setter properties are not allowed"),
            Self::MethodPropNotAllowed => write!(f, "Method properties are not allowed"),
            Self::BinaryOperationNotSupported => write!(f, "Binary operation is not supported"),
            Self::PrivateNamePropNotAllowed => write!(f, "Private name in properties is not allowed"),
            Self::ComputedPropNotAllowed => write!(f, "Computed properties are not allowed"),
            Self::SuperCalleeNotSupported => write!(f, "Super callee is not supported"),
            Self::ImportCalleeNotSupported => write!(f, "Import is not supported as callee"),
            Self::EnumValueNotFunction => write!(f, "Enum value is not a function"),
            Self::ObjectValueNotFunction => write!(f, "Object value is not a function"),
            Self::LiteralValueNotFunction => write!(f, "Literal value is not a function"),
            Self::NestedFunctionCallNotAllowed => write!(f, "Nested function calls like foo_call()() are not allowed because they are hard to read"),
            Self::BinaryValueNotFunction => write!(f, "Binary value is not a function"),
            Self::BigIntLiteralNotAllowed => write!(f, "BigInt literals are not allowed"),
            Self::RegexLiteralNotAllowed => write!(f, "Regular expression literals are not allowed"),
            Self::IdentPatternNotSupported => write!(f, "Identifier patterns are not supported"),
            Self::ArrayPatternNotSupported => write!(f, "Array patterns are not supported"),
            Self::RestPatternNotSupported => write!(f, "Rest patterns are not supported"),
            Self::ObjectPatternNotSupported => write!(f, "Object patterns are not supported"),
            Self::InvalidPatternNotSupported => write!(f, "Invalid patterns are not supported"),
            Self::ExpressionPatternNotSupported => write!(f, "Expression patterns are not supported"),
            Self::AssignPatternNotSupported => write!(f, "Assign patterns are not supported"),
            Self::ExpressionNotAllowedAsStatement => write!(f, "Expressions are not allowed as statements"),
            Self::ResultNotFoundInParams => write!(f, "Cannot find result in params"),
            Self::InvalidMemberExpressionMemberNotFound => write!(f, "Invalid member expression. Member not found."),
            Self::ObjectNotFoundByName => write!(f, "Cannot find object with name"),
            Self::EnumValueCannotBeSpread => write!(f, "Enum values cannot be spread"),
            Self::ObjectValueCannotBeSpread => write!(f, "Object values cannot be spread"),
            Self::LiteralValueCannotBeSpread => write!(f, "Literal values cannot be spread"),
            Self::FunctionValueCannotBeSpread => write!(f, "Function values cannot be spread"),
            Self::BinaryValueCannotBeSpread => write!(f, "Binary values cannot be spread"),
            Self::NotAnotherObjectValueBeforeFunctionCall => write!(f, "Cannot call non-existent function"),
            Self::NotFunctionCallAfterAnotherObjectValue => write!(f, "AnotherValue cannot be after AnotherValue after function call")
        }
    }
}




fn parse_expr(expr:&Expr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
    match expr {
        Expr::Object(object_lit) => {
            let object_val = parse_obj_lit(object_lit, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Object(object_val)
        },
        Expr::Bin(bin_expr) => {
            let bin_val = parse_bin_expr(bin_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Binary(bin_val)
        },
        Expr::Member(member_expr) => {
            let member_expr = parse_member_expr(member_expr, &ctx, glob_ctx, ast_global_context);

            ObjectDataValue::AnotherObject(member_expr)
        },
        Expr::Lit(lit) => {
            let lit_expr = parse_lit_expr(lit);
            lit_expr
        },
        Expr::Ident(ident) => {
            let name = parse_ident(ident);
            let another_obj = find_in_context(ctx, name, glob_ctx);

            ObjectDataValue::AnotherObject(another_obj)

        },
        Expr::Paren(paren_expr) => {
            let res = parse_paren_expr(paren_expr, ctx, glob_ctx, ast_global_context);
            res
        },
        Expr::Arrow(arrow_expr) => {
            let function = parse_arrow_expr(arrow_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::Function(function)
        },
        Expr::Call(call_expr) => {
            let call_expr = parse_call_expr(call_expr, ctx, glob_ctx, ast_global_context);
            ObjectDataValue::AnotherObject(call_expr)
        },


        //todo
        Expr::Array(_) => compiler_panic!(ExprPanic::ArrayNotAllowed),


        Expr::This(_) => compiler_panic!(ExprPanic::ThisNotAllowed),
        Expr::Fn(_) => compiler_panic!(ExprPanic::FnNotAllowed),
        Expr::Unary(_) => compiler_panic!(ExprPanic::UnaryExpressionNotAllowed),
        Expr::Update(_) => compiler_panic!(ExprPanic::UpdateExpressionNotAllowed),
        Expr::Assign(_) => compiler_panic!(ExprPanic::AssignExpressionNotAllowed),
        Expr::SuperProp(_) => compiler_panic!(ExprPanic::SuperPropNotAllowed),
        Expr::New(_) => compiler_panic!(ExprPanic::NewExpressionNotAllowed),
        Expr::Seq(_) => compiler_panic!(ExprPanic::SeqExpressionNotAllowed),
        Expr::Cond(_) => compiler_panic!(ExprPanic::CondExpressionNotAllowed),
        //todo
        Expr::Tpl(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        //todo
        Expr::TaggedTpl(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::Class(_) => compiler_panic!(ExprPanic::ClassNotAllowed),
        Expr::Yield(_) => compiler_panic!(ExprPanic::YieldAllowed),
        Expr::MetaProp(_) => compiler_panic!(ExprPanic::MetaPropertyNotAllowed),
        Expr::Await(_) => compiler_panic!(ExprPanic::AwaitNotAllowed),
        Expr::TsTypeAssertion(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::TsConstAssertion(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::TsNonNull(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::TsAs(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::TsInstantiation(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::TsSatisfies(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::PrivateName(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::OptChain(_) => compiler_panic!(ExprPanic::TsNotAllowed),
        Expr::Invalid(_) => compiler_panic!(ExprPanic::InvalidExpression),
        Expr::JSXMember(_) => todo!(),
        Expr::JSXNamespacedName(_) => todo!(),
        Expr::JSXEmpty(_) => todo!(),
        Expr::JSXElement(_) => todo!(),
        Expr::JSXFragment(_) => todo!(),
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

    unpack_data_value_with_check(&val)
}


fn unpack_data_value_with_check(val: &ObjectDataValue) ->Vec<ObjectData>{
    match val {
        ObjectDataValue::AnotherObject(_) => unpack_data_value(val),
        ObjectDataValue::FunctionCall(_) => unpack_data_value(val),
        ObjectDataValue::Enum(_) => compiler_panic!(ExprPanic::EnumValueCannotBeSpread),
        ObjectDataValue::Object(_) => compiler_panic!(ExprPanic::ObjectValueCannotBeSpread),
        ObjectDataValue::Literal(_) => compiler_panic!(ExprPanic::LiteralValueCannotBeSpread),
        ObjectDataValue::Function(_) => compiler_panic!(ExprPanic::FunctionValueCannotBeSpread),
        ObjectDataValue::Binary(_) => compiler_panic!(ExprPanic::BinaryValueCannotBeSpread),
    }
}


fn unpack_data_value(val: &ObjectDataValue) -> Vec<ObjectData>{
    match val {
        ObjectDataValue::Enum(enum_object_value) => enum_object_value.props.clone(),
        ObjectDataValue::Object(object_value) => object_value.props.clone(),
        ObjectDataValue::AnotherObject(another_object_value) =>
            unpack_data_value(&another_object_value.obj.value),
        ObjectDataValue::Literal(_) => Vec::new(),
        ObjectDataValue::Function(_) => Vec::new(),
        ObjectDataValue::FunctionCall(function_meta_data) => unpack_data_value(&function_meta_data.result),
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



fn another_obj_from_member(val: &ObjectDataValue, member_name: &ObjectIdent, ctx: &CurrentContext) -> Option<AnotherObjectValue>{
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


            let another_obj = another_obj_from_member(&another_object_value.obj.value, &member_name, ctx);
            
            let another_obj = another_obj.map(|o| {
                let path = [another_object_value.path.clone(), o.path].concat();
                AnotherObjectValue { obj: o.obj, path: path }
            });
            another_obj
        },
        ObjectDataValue::FunctionCall(function_meta_data) => {
            let another_obj = another_obj_from_member(&function_meta_data.result, member_name, ctx);
            let another_obj = another_obj.map(|o| {
                AnotherObjectValue { obj: o.obj, path: [o.path.clone()].concat() }
            });
            another_obj
        },
        ObjectDataValue::Literal(_) => None,
        ObjectDataValue::Function(_) => None,
        ObjectDataValue::Binary(_) => None,
    }
}






fn find_member_in_props(props: &Vec<ObjectData>, prop_name: &ObjectIdent) -> Option<ObjectData>{
    props.iter().find(|p| p.name == *prop_name).cloned()
}


fn parse_prop(prop: &Prop, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
    match prop {
        Prop::KeyValue(_) => compiler_panic!(ExprPanic::KeyValuePropNotSupported),
        Prop::Shorthand(_) => compiler_panic!(ExprPanic::ShorthandPropNotAllowed),
        Prop::Assign(prop) => parse_assign_prop(&prop, ctx, glob_ctx, ast_global_context),
        Prop::Getter(_) => compiler_panic!(ExprPanic::GetterPropNotAllowed),
        Prop::Setter(_) => compiler_panic!(ExprPanic::SetterPropNotAllowed),
        Prop::Method(_) => compiler_panic!(ExprPanic::MethodPropNotAllowed),
    }
}

fn parse_assign_prop(prop: &AssignProp, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {

    let current_object_name = parse_ident(&prop.key);
    let value = parse_expr(&prop.value, ctx, glob_ctx, ast_global_context);

    let res = ObjectData {
        name: current_object_name,
        value,
        is_mutable: false,
        attrs: get_attributes(&prop.span(), ast_global_context)
    };
    res
}





fn parse_ident_name(ident_name:&IdentName) -> ObjectIdent{
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
        _ => compiler_panic!(ExprPanic::BinaryOperationNotSupported),
    };
    let res = BinaryObjectValue{
        op:op,
        v1: Box::new(left_expr),
        v2: Box::new(right_expr)
    };
    res
}





fn parse_binding_ident(binding_ident: &BindingIdent) -> ObjectIdent{
    let ident = parse_ident(&binding_ident.id);
    ident
}




fn parse_pat(pat: &Pat) -> ObjectIdent{
    let ident = match &pat{
        Pat::Ident(binding_ident) => parse_binding_ident(binding_ident),
        Pat::Rest(_) => compiler_panic!(ExprPanic::RestPatternNotSupported),
        Pat::Object(_) => compiler_panic!(ExprPanic::ObjectPatternNotSupported),
        Pat::Assign(_) => compiler_panic!(ExprPanic::AssignPatternNotSupported),
        Pat::Array(_) => compiler_panic!(ExprPanic::ArrayPatternNotSupported),
        Pat::Invalid(_) => compiler_panic!(ExprPanic::InvalidPatternNotSupported),
        Pat::Expr(_) => compiler_panic!(ExprPanic::ExpressionPatternNotSupported),
    };
    ident
}














fn parse_pat_as_assign_func_init(pat: &Pat, ctx:&CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
        match &pat{
        Pat::Assign(assign_pat) => {
            let name = parse_pat(&*assign_pat.left);
            let assign_res = parse_expr(&*assign_pat.right, ctx, glob_ctx, ast_global_context);
            ObjectData {
                value: assign_res,
                name: name,
                is_mutable: false,
                attrs: get_attributes(&assign_pat.span(), ast_global_context)
            }
        },
        Pat::Ident(_) => compiler_panic!(ExprPanic::IdentPatternNotSupported),
        Pat::Rest(_) => compiler_panic!(ExprPanic::RestPatternNotSupported),
        Pat::Object(_) => compiler_panic!(ExprPanic::ObjectPatternNotSupported),
        Pat::Array(_) => compiler_panic!(ExprPanic::ArrayPatternNotSupported),
        Pat::Invalid(_) => compiler_panic!(ExprPanic::InvalidPatternNotSupported),
        Pat::Expr(_) => compiler_panic!(ExprPanic::ExpressionPatternNotSupported),
    }
}








fn parse_paren_expr(paren_expr: &ParenExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
    parse_expr(&paren_expr.expr, ctx, glob_ctx, ast_global_context)
}

fn parse_arrow_expr(arrow_expr: &ArrowExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> FunctionValue {

    let params = arrow_expr.params
        .iter()
        .map(|pat| {
            let obj_data = parse_pat_as_assign_func_init(pat, ctx, glob_ctx, ast_global_context);
            obj_data
        })
        .collect::<Vec<_>>();    



    
    //

    let result_opt = find_result_in_params(&params);



    if  let Some(result) = result_opt.clone() {
        let scope = parse_block_stmt_or_expr(&arrow_expr.body, ctx, glob_ctx, ast_global_context);


        FunctionValue {
            scope: Box::new(scope),
            params: params,
            result:Box::new(result)
        }
    }
    else {
        compiler_panic!(ExprPanic::ResultNotFoundInParams);
    }

}


fn find_result_in_params(params: &Vec<ObjectData>) -> Option<ObjectData>{
    params.iter().find(|p| p.name == "result").cloned()
}






fn parse_block_stmt_or_expr(block_stmt_or_expr: &BlockStmtOrExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext ) -> ScopeStatement {
    match &block_stmt_or_expr{
        BlockStmtOrExpr::BlockStmt(block_stmt) => {
            let block_scope = parse_block(block_stmt, ctx, glob_ctx, ast_global_context);
            block_scope
        },
        BlockStmtOrExpr::Expr(_) => compiler_panic!(ExprPanic::ExpressionNotAllowedAsStatement),
    }
}




fn parse_lit_expr(lit_expr: &Lit) -> ObjectDataValue {
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
            compiler_panic!(ExprPanic::BigIntLiteralNotAllowed)
        },
        Lit::Regex(_regex) => {
            compiler_panic!(ExprPanic::RegexLiteralNotAllowed)
        },
        Lit::JSXText(_jsxtext) => {
            todo!()
        },
    }
}

fn parse_ident(ident_expr: &Ident) -> ObjectIdent{
    let name = ident_expr.sym.to_string();
    name
}


fn parse_call_expr(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> AnotherObjectValue {

    let expr = get_expr_from_arg(call_expr);
    let args = get_func_args_call(call_expr, ctx, glob_ctx, ast_global_context);

    let val_call = parse_expr(&expr, ctx, glob_ctx, ast_global_context);


    let func_call = create_func_result(val_call, args);

    func_call

}

fn create_func_result(val_call: ObjectDataValue, args: Vec<ObjectDataValue>) -> AnotherObjectValue {
    let func_call = match val_call {
        ObjectDataValue::Function(_) => compiler_panic!(ExprPanic::NotAnotherObjectValueBeforeFunctionCall),
        ObjectDataValue::AnotherObject(another_object_value) => func_call_result(&another_object_value, args),
        ObjectDataValue::Enum(_) => compiler_panic!(ExprPanic::EnumValueNotFunction),
        ObjectDataValue::Object(_) => compiler_panic!(ExprPanic::ObjectValueNotFunction),
        ObjectDataValue::Literal(_) => compiler_panic!(ExprPanic::LiteralValueNotFunction),
        ObjectDataValue::FunctionCall(_) => compiler_panic!(ExprPanic::NestedFunctionCallNotAllowed),
        ObjectDataValue::Binary(_) => compiler_panic!(ExprPanic::BinaryValueNotFunction),
    };
    func_call
}




fn get_expr_from_arg(call_expr: &CallExpr) -> Box<Expr>{
    match &call_expr.callee {
        Callee::Super(_) => compiler_panic!(ExprPanic::SuperCalleeNotSupported),
        Callee::Import(_) => compiler_panic!(ExprPanic::ImportCalleeNotSupported),
        Callee::Expr(exr) => Box::new(*exr.clone())
    }
}

fn get_func_args_call(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectDataValue>{
    let args = call_expr.args
        .iter()
        .map(|arg| {
            let obj_val = parse_expr_or_spread(arg, ctx, glob_ctx, ast_global_context);
            obj_val
        })
        .flatten()
        .collect::<Vec<_>>();

    args
}




fn func_call_result(another_val: &AnotherObjectValue, args: Vec<ObjectDataValue>) -> AnotherObjectValue {
    match &another_val.obj.value {
        ObjectDataValue::Function(func_data) => {
            let func_call = FunctionCallResultValue {
                args: args,
                result: Box::new(func_data.result.value.clone()),
            };
            AnotherObjectValue{
                obj: Box::new(ObjectData {
                    name: another_val.obj.name.clone(),
                    is_mutable: false,
                    attrs: another_val.obj.attrs.clone(),
                    value: ObjectDataValue::FunctionCall(func_call.clone())
                }),
                path: [another_val.path.clone(), vec![AnotherObjectValuePath::FunctionCall(func_call.clone())]].concat(),
            }
        },
        ObjectDataValue::AnotherObject(_) => compiler_panic!(ExprPanic::NotFunctionCallAfterAnotherObjectValue),
        ObjectDataValue::Enum(_) => compiler_panic!(ExprPanic::EnumValueNotFunction),
        ObjectDataValue::Object(_) => compiler_panic!(ExprPanic::ObjectValueNotFunction),
        ObjectDataValue::Literal(_) => compiler_panic!(ExprPanic::LiteralValueNotFunction),
        ObjectDataValue::FunctionCall(_) => compiler_panic!(ExprPanic::NestedFunctionCallNotAllowed),
        ObjectDataValue::Binary(_) => compiler_panic!(ExprPanic::BinaryValueNotFunction),
    }
}





fn parse_expr_or_spread(expr_or_spread: &ExprOrSpread, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectDataValue>{
    let arg = parse_expr(&expr_or_spread.expr,ctx, glob_ctx, ast_global_context);


    let is_spread = expr_or_spread.spread.is_some();

    if is_spread {
        let val = unpack_data_value_with_check(&arg).iter().map(|v|{
            ObjectDataValue::AnotherObject(
                AnotherObjectValue{
                    obj: Box::from(v.clone()),
                    path: vec![AnotherObjectValuePath::Ident(v.name.clone())]
                }
            )
        }).collect::<Vec<_>>();
        val
    }
    else {
        vec![arg]
    }
    
}





fn parse_member_expr(member: &MemberExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> AnotherObjectValue{
    let member_prop = parse_member_prop(&member.prop);
    let parse_member_val = parse_expr(&member.obj, ctx, glob_ctx, ast_global_context);


    let object_stmts_opt = another_obj_from_member(&parse_member_val, &member_prop.clone(), &ctx);


    match object_stmts_opt {
        None => {
            compiler_panic!(ExprPanic::InvalidMemberExpressionMemberNotFound);
        }
        Some(object_stmts) => object_stmts
    }


}

fn parse_member_prop(member_prop: &MemberProp) -> ObjectIdent {
    match &member_prop{
        MemberProp::Ident(ident_name) => parse_ident_name(ident_name),
        MemberProp::PrivateName(_) => compiler_panic!(ExprPanic::PrivateNamePropNotAllowed),
        MemberProp::Computed(_) => compiler_panic!(ExprPanic::ComputedPropNotAllowed),
    }
}




fn parse_block(block_stmt: &BlockStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ScopeStatement{
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
    let scope = ScopeStatement{
        statements: stmts
    };
    scope
}


fn parse_if(if_stmt: &IfStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ConditionStatement {

    let cons_stmt = parse_stmt_as_scope(parse_stmt(&if_stmt.cons, ctx, glob_ctx, ast_global_context));

    let test_expr = parse_expr(&if_stmt.test, ctx, glob_ctx, ast_global_context);



    let else_stmt = match &if_stmt.alt{
        Some(r) =>  Some(Box::new(parse_stmt_as_scope(parse_stmt(&r, ctx, glob_ctx, ast_global_context)))),
        None => None,
    };
    
    let cond = ConditionStatement {
        cond: test_expr,
        false_scope: else_stmt,
        true_scope: Box::new(cons_stmt)
    };
    cond
}

fn parse_stmt_as_scope(statement: Statement) -> ScopeStatement {
    match statement {
        Statement::Scope(scope) => scope,
        Statement::Object(_) => compiler_panic!(StmtPanic::ObjectNotScope),
        Statement::ObjectValue(_) => compiler_panic!(StmtPanic::ObjectValueNotScope),
        Statement::Conditional(_) => compiler_panic!(StmtPanic::ConditionalNotScope),
        Statement::Loop(_) => compiler_panic!(StmtPanic::LoopNotScope),
        Statement::Import(_) => compiler_panic!(StmtPanic::ImportNotScope),
        Statement::Export(_) => compiler_panic!(StmtPanic::ExportNotScope),
    }
}



fn parse_while(while_stm: &WhileStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> LoopStatement {
    let loop_stmt = parse_stmt(&while_stm.body, ctx, glob_ctx, ast_global_context);
    let cond_val = parse_expr(&while_stm.test, ctx, glob_ctx, ast_global_context);
    let loop_st = LoopStatement {
        cond: cond_val,
        loop_scope: Box::new(loop_stmt)
    };
    loop_st
}
