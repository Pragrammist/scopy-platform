use std::rc::Rc;
use swc_common::{SourceFile, Span, Spanned};
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_ast::*;
use swc_common::comments::{CommentKind, Comments, SingleThreadedComments};
use std::collections::{HashSet};
use swc_common::comments::Comment;
use std::collections::HashMap;
use crate::semantic::{AnotherObjectValue, AnotherObjectValuePath, AstCurrentContext, AstGlobalContext,
                      Attribute, BinaryObjectValue, BinaryOpType, CodeModuleMetaData, CodeModuleSourceFileType,
                      ConditionStatement, CurrentContext, CurrentContextType, FunctionCallResultValue, FunctionValue,
                      GlobalContext, LitValueBool, LitValueNum, LitValueString, LiteralValue, LoopStatement, ModuleContext, ObjectData,
                      ObjectDataValue, ObjectIdent, ObjectValue, ScopyModule, Statement, ScopeStatement,
                      ScopyProject, ImportDataValue, ExportDataValue};
use crate::include_build_in_modules::builtin_modules;

macro_rules! compiler_panic {
    ($reason:expr) => {{
        let reason = $reason;
        eprintln!("Compiler panic: {reason}");
        std::panic::panic_any(reason);
    }};
}



pub fn create_source_file(cm: &Lrc<SourceMap>, meta: &CodeModuleMetaData) -> Rc<SourceFile>{

    match meta.module_meta_type{
        CodeModuleSourceFileType::Internal =>
            cm.new_source_file(FileName::Internal(meta.name.clone().into()).into(), meta.code.clone()),
        CodeModuleSourceFileType::External =>
            cm.new_source_file(FileName::Real(meta.name.clone().into()).into(), meta.code.clone()),
    }

}







pub fn get_ast_from_src(meta: &CodeModuleMetaData, glob_ast_ctx: &AstGlobalContext) -> AstCurrentContext{

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


pub fn get_ctxs(ast_global_context: &AstGlobalContext, module_to_compile: Vec<CodeModuleMetaData>) -> Vec<AstCurrentContext>{
    let ctxs = module_to_compile.into_iter().map(|meta| get_ast_from_src(&meta, ast_global_context)).collect();

    let module_parse_queue = reorder_modules_to_parse(&ctxs);

    let mut vec_res: Vec<AstCurrentContext> = Vec::new();

    module_parse_queue.into_iter().for_each(|module_name| {
        let ctx = ctxs.iter().find(|ctx_i| ctx_i.name == module_name);

        match ctx{
            None => compiler_panic!(ModuleDeclPanic::ModuleNotFound),
            Some(ctx) => vec_res.push(ctx.clone())
        }

    });
    vec_res
}





pub fn parse_modules(modules_to_compile: Vec<CodeModuleMetaData>) -> ScopyProject{


    let mut global_ast_context = AstGlobalContext{
        ast_modules: HashMap::new(),
        comments: SingleThreadedComments::default(),
        cm: Default::default(),
    };
    let modules_to_compile = [builtin_modules(), modules_to_compile].concat();
    let ctxs = get_ctxs(&global_ast_context, modules_to_compile);

    let mut glob_ctx = GlobalContext{
        parsed_modules: HashMap::new(),
    };


    let modules = ctxs.into_iter().map(|ctx| {

        global_ast_context.ast_modules.insert(ctx.name.clone(), ctx.module.clone());

        let parsed_module = module_parse(&ctx, &global_ast_context, &glob_ctx);
        glob_ctx.parsed_modules.insert(ctx.name.clone(), parsed_module.clone());
        parsed_module
    }).collect::<Vec<_>>();


    ScopyProject{
        modules: modules
    }

}


pub fn reorder_modules_to_parse(ctxs: &Vec<AstCurrentContext>) -> Vec<ObjectIdent>{
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









pub fn module_parse(ast_current_context: &AstCurrentContext, ast_global_context: &AstGlobalContext, glob_ctx: &GlobalContext) -> ScopyModule{
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





pub fn parse_module_item(el: &ModuleItem, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement {
    let res_el = match el {
        ModuleItem::ModuleDecl(module_decl) => {
            let import_stmt = parse_module_decl(module_decl, ctx, glob_ctx);
            Statement::Object(import_stmt)
        },
        ModuleItem::Stmt(stmt) => {
            let p_stmt = parse_stmt(stmt, ctx, glob_ctx, ast_global_context);
            p_stmt
        },
    };
    res_el
}


pub fn module_import_map(ast_context: &AstCurrentContext) -> Vec<ObjectIdent>{

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



pub fn import_module_decl_module_import_map(module_decl: &ModuleDecl) -> Option<ObjectIdent>{
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





pub fn parse_export_specifier(spec: &ExportSpecifier) -> ObjectIdent{
    match spec {
        ExportSpecifier::Namespace(_) => compiler_panic!(ModuleDeclPanic::NameSpaceExportNotAllowed),
        ExportSpecifier::Default(_) => compiler_panic!(ModuleDeclPanic::DefaultExportNotAllowed),
        ExportSpecifier::Named(named_spec) => {
            parse_export_named_specifier(named_spec)
        }
    }
}

pub fn parse_export_named_specifier(spec: &ExportNamedSpecifier) -> ObjectIdent{
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

pub fn parse_export_module_name(spec_name: &ModuleExportName) ->ObjectIdent
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


pub fn parse_module_decl(module_decl: &ModuleDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext) -> ObjectData{
    let stmt = match module_decl{
        ModuleDecl::Import(import_decl) => {
            let src = get_import_decl_module_name(import_decl);
            let imports = parse_import_decl(import_decl, glob_ctx);

            ObjectData{
                is_mutable: false,
                name: src,
                attrs: vec![],
                value: ObjectDataValue::Import(ImportDataValue{
                    imports: imports
                })
            }
        },
        ModuleDecl::ExportNamed(named_export) => {
            if named_export.src != None
            {
                compiler_panic!(ModuleDeclPanic::NotAllowedExport)
            }

            let imports = named_export.specifiers.iter().map(parse_export_specifier).map(|obj_id| {
                find_in_context(ctx, obj_id, glob_ctx)
            }).collect::<Vec<_>>();



            ObjectData{
                value: ObjectDataValue::Export(ExportDataValue{
                    exports: imports
                }),
                attrs: vec![],
                is_mutable: false,
                name: ctx.current_module_name.clone(),
            }
        },
        ModuleDecl::ExportDecl(_) => compiler_panic!(ModuleDeclPanic::NotNamedExport),
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

    ModuleNotFound,
    DefaultImportNotAllowed,
    NotNamedImportNotAllowed,
    ExportAllNotAllowed,
    TsImportEqualsNotAllowed,
    TsExportAssignmentNotAllowed,
    TsNamespaceExportNotAllowed,
    NotAllowedExport,
    NotNamedExport,
    StrExportNotAllowed,
    NameSpaceExportNotAllowed,
}

impl std::fmt::Display for ModuleDeclPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleDeclPanic::DefaultExportNotAllowed => write!(f, "Default export is not allowed"),
            ModuleDeclPanic::ExportAllNotAllowed => write!(f, "Export all from another module is not allowed"),
            ModuleDeclPanic::TsImportEqualsNotAllowed => write!(f, "TypeScript import equals is not allowed"),
            ModuleDeclPanic::TsExportAssignmentNotAllowed => write!(f, "TypeScript export assignment is not allowed"),
            ModuleDeclPanic::TsNamespaceExportNotAllowed => write!(f, "TypeScript namespace export is not allowed"),
            ModuleDeclPanic::NotAllowedExport => write!(f, "From while export not allowed"),
            ModuleDeclPanic::NotNamedExport => write!(f, "NOt named export not allowed"),
            ModuleDeclPanic::DefaultImportNotAllowed => write!(f, "Default import not allowed"),
            ModuleDeclPanic::NotNamedImportNotAllowed => write!(f, "Not named import not allowed"),
            ModuleDeclPanic::StrExportNotAllowed => write!(f, "Str export not allowed"),
            ModuleDeclPanic::ModuleNotFound => write!(f, "Module not found"),
            ModuleDeclPanic::NameSpaceExportNotAllowed => write!(f, "Namespace export not supported")
        }
    }
}










pub fn create_context_in_global_context(global_ctx: &GlobalContext, module_name: &ObjectIdent) -> CurrentContext{
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

    res.unwrap_or_else(|| compiler_panic!(ModuleDeclPanic::ModuleNotFound))
}







pub fn get_import_decl_module_name(import_decl: &ImportDecl) -> ObjectIdent{
    import_decl.src.value.to_string_lossy().into_owned()
}



pub fn parse_import_decl(import_decl: &ImportDecl, glob_ctx: &GlobalContext) -> Vec<AnotherObjectValue>{
    let imports = import_decl.specifiers.iter().map(parse_import_specifier)
        .map(|name| {
            let import_name = get_import_decl_module_name(&import_decl);
            let module_context = create_context_in_global_context(glob_ctx, &import_name);
            let export_obj = find_in_context(&module_context, name.clone(), glob_ctx);
            export_obj
        }).collect::<Vec<_>>();
    imports
}







pub fn parse_import_specifier(s: &ImportSpecifier) -> ObjectIdent{
    match s {
        ImportSpecifier::Named(import_named_specifier) => parse_named_specifier(import_named_specifier),
        ImportSpecifier::Default(_) => compiler_panic!(ModuleDeclPanic::DefaultImportNotAllowed),
        ImportSpecifier::Namespace(_) => compiler_panic!(ModuleDeclPanic::NotNamedImportNotAllowed),
    }
}



pub fn parse_named_specifier(import_named_specifier: &ImportNamedSpecifier) -> ObjectIdent{


    match &import_named_specifier.imported {
        None => parse_ident(&import_named_specifier.local),
        Some(exported) => parse_module_export_name(&exported),
    }


}


pub fn parse_module_export_name(imported: &ModuleExportName) -> ObjectIdent{
    let name = match imported{
        ModuleExportName::Ident(ident) => parse_ident(&ident),
        ModuleExportName::Str(_) => compiler_panic!(ModuleDeclPanic::StrExportNotAllowed),
    };
    name
}




#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StmtPanic {
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
    LoopNotScope
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
            Self::LoopNotScope => write!(f, "Loop not scope")
        }
    }
}

pub fn parse_stmt(stmt: &Stmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Statement{
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







pub fn parse_expr_stmt(expr_stmt: &ExprStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
    let expr = parse_expr(&expr_stmt.expr, ctx, glob_ctx, ast_global_context);
    expr
}








pub fn parse_decl(var_decl: &VarDecl, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
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
        name:name,
        value: val
    }
}





#[allow(unused)]
pub fn has_attribute(
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


pub fn get_attributes(span: &Span, ast_global_context: &AstGlobalContext) -> Vec<Attribute>{
    let comment_opt = ast_global_context.comments.get_leading(span.lo());
    if let Some(vec_comments) = comment_opt{
        let attrs = parse_attributes_from_comments(vec_comments);
        attrs
    }
    else{
        Vec::new()
    }
}







pub fn parse_attributes_from_comments(comments: Vec<Comment>) -> Vec<Attribute> {
    let mut result = Vec::new();

    for comment in comments {
        if comment.kind == CommentKind::Block {
            continue;
        }
        let attrs = parse_attributes(&comment.text);
        result.extend(attrs);
    }

    result
}





pub fn parse_attributes(input: &str) -> Vec<Attribute> {
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


pub fn find_in_context(ctx: &CurrentContext, name: ObjectIdent, glob_ctx: &GlobalContext) -> AnotherObjectValue {
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






pub fn check_obj_data(object_data: &ObjectData, name: &ObjectIdent) -> Option<ObjectData>{
    if object_data.name == *name{
        Some(object_data.clone())
    }
    else if let ObjectDataValue::Import(import) = object_data.value.clone() {
        import.imports.iter().find(|import| import.obj.name == name.clone()).map(|import| *import.obj.clone())
    }
    else if let ObjectDataValue::Export(import) = object_data.value.clone() {
        import.exports.iter().find(|export| export.obj.name == name.clone()).map(|export| *export.obj.clone())
    }
    else { None }
}

pub fn find_in_statements(stmt: &Statement, _: &CurrentContext, name: &ObjectIdent, _: &GlobalContext) -> Option<ObjectData>{
    match stmt {
        Statement::Object(object_data) => {
            check_obj_data(object_data, name)
        },
        Statement::ObjectValue(_) => None,
        Statement::Conditional(_) => None,
        Statement::Loop(_) => None,
        Statement::Scope(_) => None,
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExprPanic {
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
    SpreadNotAllowed,
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
    ImportNotFunction,
    ExportNotFunction,
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
    KeyValuePropNotSupported,
    NotAnotherObjectValueBeforeFunctionCall,
    NotFunctionCallAfterAnotherObjectValue,
}

impl std::fmt::Display for ExprPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprPanic::KeyValuePropNotSupported => write!(f, "Key value property not supported"),
            ExprPanic::ThisNotAllowed => write!(f, "this not allowed"),
            ExprPanic::ArrayNotAllowed => write!(f, "array not allowed"),
            ExprPanic::FnNotAllowed => write!(f, "fn not allowed"),
            ExprPanic::UnaryExpressionNotAllowed => write!(f, "unary expression not allowed"),
            ExprPanic::UpdateExpressionNotAllowed => write!(f, "update expression not allowed"),
            ExprPanic::AssignExpressionNotAllowed => write!(f, "assign expression not allowed"),
            ExprPanic::SuperPropNotAllowed => write!(f, "super prop not allowed"),
            ExprPanic::CondExpressionNotAllowed => write!(f, "cond expression not allowed"),
            ExprPanic::NewExpressionNotAllowed => write!(f, "new expression not allowed"),
            ExprPanic::SeqExpressionNotAllowed => write!(f, "seq expression not allowed"),
            ExprPanic::TsNotAllowed => write!(f, "ts not allowed"),
            ExprPanic::YieldAllowed => write!(f, "yield allowed"),
            ExprPanic::MetaPropertyNotAllowed => write!(f, "meta property not allowed"),
            ExprPanic::AwaitNotAllowed => write!(f, "await not allowed"),
            ExprPanic::ClassNotAllowed => write!(f, "class not allowed"),
            ExprPanic::InvalidExpression => write!(f, "invalid expression"),
            ExprPanic::ShorthandPropNotAllowed => write!(f, "Shorthand properties are not allowed"),
            ExprPanic::GetterPropNotAllowed => write!(f, "Getter properties are not allowed"),
            ExprPanic::SetterPropNotAllowed => write!(f, "Setter properties are not allowed"),
            ExprPanic::MethodPropNotAllowed => write!(f, "Method properties are not allowed"),
            ExprPanic::BinaryOperationNotSupported => write!(f, "Binary operation is not supported"),
            ExprPanic::PrivateNamePropNotAllowed => write!(f, "Private name in properties is not allowed"),
            ExprPanic::ComputedPropNotAllowed => write!(f, "Computed properties are not allowed"),
            ExprPanic::SuperCalleeNotSupported => write!(f, "Super callee is not supported"),
            ExprPanic::ImportCalleeNotSupported => write!(f, "Import is not supported as callee"),
            ExprPanic::EnumValueNotFunction => write!(f, "Enum value is not a function"),
            ExprPanic::SpreadNotAllowed => write!(f, "Spread is not allowed"),
            ExprPanic::ObjectValueNotFunction => write!(f, "Object value is not a function"),
            ExprPanic::LiteralValueNotFunction => write!(f, "Literal value is not a function"),
            ExprPanic::NestedFunctionCallNotAllowed => write!(f, "Nested function calls like foo_call()() are not allowed because they are hard to read"),
            ExprPanic::BinaryValueNotFunction => write!(f, "Binary value is not a function"),
            ExprPanic::BigIntLiteralNotAllowed => write!(f, "BigInt literals are not allowed"),
            ExprPanic::RegexLiteralNotAllowed => write!(f, "Regular expression literals are not allowed"),
            ExprPanic::IdentPatternNotSupported => write!(f, "Identifier patterns are not supported"),
            ExprPanic::ArrayPatternNotSupported => write!(f, "Array patterns are not supported"),
            ExprPanic::RestPatternNotSupported => write!(f, "Rest patterns are not supported"),
            ExprPanic::ObjectPatternNotSupported => write!(f, "Object patterns are not supported"),
            ExprPanic::InvalidPatternNotSupported => write!(f, "Invalid patterns are not supported"),
            ExprPanic::ExpressionPatternNotSupported => write!(f, "Expression patterns are not supported"),
            ExprPanic::AssignPatternNotSupported => write!(f, "Assign patterns are not supported"),
            ExprPanic::ExpressionNotAllowedAsStatement => write!(f, "Expressions are not allowed as statements"),
            ExprPanic::ResultNotFoundInParams => write!(f, "Cannot find result in params"),
            ExprPanic::InvalidMemberExpressionMemberNotFound => write!(f, "Invalid member expression. Member not found."),
            ExprPanic::ObjectNotFoundByName => write!(f, "Cannot find object with name"),
            ExprPanic::NotAnotherObjectValueBeforeFunctionCall => write!(f, "Cannot call non-existent function"),
            ExprPanic::NotFunctionCallAfterAnotherObjectValue => write!(f, "AnotherValue cannot be after AnotherValue after function call"),
            ExprPanic::ImportNotFunction => write!(f, "Export not a function"),
            ExprPanic::ExportNotFunction => write!(f, "Import not a function"),
        }
    }
}




pub fn parse_expr(expr:&Expr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
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

pub fn parse_obj_lit(object_lit:&ObjectLit, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectValue{
    let props = object_lit.props
        .iter()
        .flat_map(|p| parse_prop_or_spread(p, ctx, glob_ctx, ast_global_context))
        .collect::<Vec<_>>();


    ObjectValue{ props: props }
}

pub fn parse_prop_or_spread(p: &PropOrSpread, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectData>{
    match p{
        PropOrSpread::Spread(_) => compiler_panic!(ExprPanic::SpreadNotAllowed),
        PropOrSpread::Prop(prop) =>  vec![parse_prop(prop, ctx, glob_ctx, ast_global_context)] ,
    }
}



pub fn map_prop_obj_data(prop: Option<ObjectData>, member_name: &ObjectIdent) -> Option<AnotherObjectValue>{
    let another_obj = prop.map(|o| {
        AnotherObjectValue{
            obj: Box::new(o.clone()),
            path: vec![AnotherObjectValuePath::Ident(member_name.clone())]
        }
    });
    another_obj
}



pub fn another_obj_from_member(val: &ObjectDataValue, member_name: &ObjectIdent, ctx: &CurrentContext) -> Option<AnotherObjectValue>{
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
        ObjectDataValue::Import(import) => {
            import.imports.iter().find_map(|m| another_obj_from_member(&m.obj.value, member_name, ctx))
        }
        ObjectDataValue::Export(export) => {
            export.exports.iter().find_map(|m| another_obj_from_member(&m.obj.value, member_name, ctx))
        }
    }
}






pub fn find_member_in_props(props: &Vec<ObjectData>, prop_name: &ObjectIdent) -> Option<ObjectData>{
    props.iter().find(|p| p.name == *prop_name).cloned()
}


pub fn parse_prop(prop: &Prop, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
    match prop {
        Prop::KeyValue(_) => compiler_panic!(ExprPanic::KeyValuePropNotSupported),
        Prop::Shorthand(_) => compiler_panic!(ExprPanic::ShorthandPropNotAllowed),
        Prop::Assign(prop) => parse_assign_prop(&prop, ctx, glob_ctx, ast_global_context),
        Prop::Getter(_) => compiler_panic!(ExprPanic::GetterPropNotAllowed),
        Prop::Setter(_) => compiler_panic!(ExprPanic::SetterPropNotAllowed),
        Prop::Method(_) => compiler_panic!(ExprPanic::MethodPropNotAllowed),
    }
}

pub fn parse_assign_prop(prop: &AssignProp, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {

    let current_object_name = parse_ident(&prop.key);
    let value = parse_expr(&prop.value, ctx, glob_ctx, ast_global_context);

    let res = ObjectData {
        name: current_object_name,
        value:value,
        is_mutable: false,
        attrs: get_attributes(&prop.span(), ast_global_context)
    };
    res
}





pub fn parse_ident_name(ident_name:&IdentName) -> ObjectIdent{
    ident_name.sym.to_string()
}

pub fn parse_bin_expr(bin_expr: &BinExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> BinaryObjectValue {
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





pub fn parse_binding_ident(binding_ident: &BindingIdent) -> ObjectIdent{
    let ident = parse_ident(&binding_ident.id);
    ident
}




pub fn parse_pat(pat: &Pat) -> ObjectIdent{
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














pub fn parse_pat_as_assign_func_init(pat: &Pat, ctx:&CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectData {
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








pub fn parse_paren_expr(paren_expr: &ParenExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ObjectDataValue {
    parse_expr(&paren_expr.expr, ctx, glob_ctx, ast_global_context)
}

pub fn parse_arrow_expr(arrow_expr: &ArrowExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> FunctionValue {

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


pub fn find_result_in_params(params: &Vec<ObjectData>) -> Option<ObjectData>{
    params.iter().find(|p| p.name == "result").cloned()
}






pub fn parse_block_stmt_or_expr(block_stmt_or_expr: &BlockStmtOrExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext ) -> ScopeStatement {
    match &block_stmt_or_expr{
        BlockStmtOrExpr::BlockStmt(block_stmt) => {
            let block_scope = parse_block(block_stmt, ctx, glob_ctx, ast_global_context);
            block_scope
        },
        BlockStmtOrExpr::Expr(_) => compiler_panic!(ExprPanic::ExpressionNotAllowedAsStatement),
    }
}




pub fn parse_lit_expr(lit_expr: &Lit) -> ObjectDataValue {
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

pub fn parse_ident(ident_expr: &Ident) -> ObjectIdent{
    let name = ident_expr.sym.to_string();
    name
}


pub fn parse_call_expr(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> AnotherObjectValue {

    let expr = get_expr_from_arg(call_expr);
    let args = get_func_args_call(call_expr, ctx, glob_ctx, ast_global_context);

    let val_call = parse_expr(&expr, ctx, glob_ctx, ast_global_context);


    let func_call = create_func_result(val_call, args);

    func_call

}

pub fn create_func_result(val_call: ObjectDataValue, args: Vec<ObjectDataValue>) -> AnotherObjectValue {
    let func_call = match val_call {
        ObjectDataValue::Function(_) => compiler_panic!(ExprPanic::NotAnotherObjectValueBeforeFunctionCall),
        ObjectDataValue::AnotherObject(another_object_value) => func_call_result(&another_object_value, args),
        ObjectDataValue::Enum(_) => compiler_panic!(ExprPanic::EnumValueNotFunction),
        ObjectDataValue::Object(_) => compiler_panic!(ExprPanic::ObjectValueNotFunction),
        ObjectDataValue::Literal(_) => compiler_panic!(ExprPanic::LiteralValueNotFunction),
        ObjectDataValue::FunctionCall(_) => compiler_panic!(ExprPanic::NestedFunctionCallNotAllowed),
        ObjectDataValue::Binary(_) => compiler_panic!(ExprPanic::BinaryValueNotFunction),

        ObjectDataValue::Import(_) => compiler_panic!(ExprPanic::ImportNotFunction),
        ObjectDataValue::Export(_) => compiler_panic!(ExprPanic::ExportNotFunction),
    };
    func_call
}




pub fn get_expr_from_arg(call_expr: &CallExpr) -> Box<Expr>{
    match &call_expr.callee {
        Callee::Super(_) => compiler_panic!(ExprPanic::SuperCalleeNotSupported),
        Callee::Import(_) => compiler_panic!(ExprPanic::ImportCalleeNotSupported),
        Callee::Expr(exr) => Box::new(*exr.clone())
    }
}

pub fn get_func_args_call(call_expr: &CallExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectDataValue>{
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




pub fn func_call_result(another_val: &AnotherObjectValue, args: Vec<ObjectDataValue>) -> AnotherObjectValue {
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

        ObjectDataValue::Import(_) => compiler_panic!(ExprPanic::ImportNotFunction),
        ObjectDataValue::Export(_) => compiler_panic!(ExprPanic::ExportNotFunction),
    }
}





pub fn parse_expr_or_spread(expr_or_spread: &ExprOrSpread, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> Vec<ObjectDataValue>{
    let arg = parse_expr(&expr_or_spread.expr,ctx, glob_ctx, ast_global_context);


    let is_spread = expr_or_spread.spread.is_some();

    if is_spread {
        // let val = unpack_data_value_with_check(&arg).iter().map(|v|{
        //     ObjectDataValue::AnotherObject(
        //         AnotherObjectValue{
        //             obj: Box::from(v.clone()),
        //             path: vec![AnotherObjectValuePath::Ident(v.name.clone())]
        //         }
        //     )
        // }).collect::<Vec<_>>();
        // val
        compiler_panic!(ExprPanic::SpreadNotAllowed)
    }
    else {
        vec![arg]
    }

}





pub fn parse_member_expr(member: &MemberExpr, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> AnotherObjectValue{
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

pub fn parse_member_prop(member_prop: &MemberProp) -> ObjectIdent {
    match &member_prop{
        MemberProp::Ident(ident_name) => parse_ident_name(ident_name),
        MemberProp::PrivateName(_) => compiler_panic!(ExprPanic::PrivateNamePropNotAllowed),
        MemberProp::Computed(_) => compiler_panic!(ExprPanic::ComputedPropNotAllowed),
    }
}




pub fn parse_block(block_stmt: &BlockStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ScopeStatement{
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


pub fn parse_if(if_stmt: &IfStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> ConditionStatement {

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

pub fn parse_stmt_as_scope(statement: Statement) -> ScopeStatement {
    match statement {
        Statement::Scope(scope) => scope,
        Statement::Object(_) => compiler_panic!(StmtPanic::ObjectNotScope),
        Statement::ObjectValue(_) => compiler_panic!(StmtPanic::ObjectValueNotScope),
        Statement::Conditional(_) => compiler_panic!(StmtPanic::ConditionalNotScope),
        Statement::Loop(_) => compiler_panic!(StmtPanic::LoopNotScope),
        // Statement::Import(_) => compiler_panic!(StmtPanic::ImportNotScope),
        // Statement::Export(_) => compiler_panic!(StmtPanic::ExportNotScope),
    }
}



pub fn parse_while(while_stm: &WhileStmt, ctx: &CurrentContext, glob_ctx: &GlobalContext, ast_global_context: &AstGlobalContext) -> LoopStatement {
    let loop_stmt = parse_stmt_as_scope(parse_stmt(&while_stm.body, ctx, glob_ctx, ast_global_context));
    let cond_val = parse_expr(&while_stm.test, ctx, glob_ctx, ast_global_context);
    let loop_st = LoopStatement {
        cond: cond_val,
        loop_scope: Box::new(loop_stmt)
    };
    loop_st
}
