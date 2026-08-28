use swc_atoms::Atom;
use swc_common::{Span, SyntaxContext, DUMMY_SP};
use swc_common::comments::{Comment, CommentKind};
use swc_ecma_ast::{BindingIdent, Expr, Ident, IdentName, KeyValueProp, ObjectLit, Pat, Prop, PropName, PropOrSpread, SpreadElement};
use crate::{AstGlobalContext, CurrentContext, GlobalContext};

#[allow(unused)]
pub fn create_test_fake_span() -> Span {
    DUMMY_SP
}

#[allow(unused)]
pub fn create_test_comment_with_attribute(attributes: String) -> Comment{
    Comment{
        kind:CommentKind::Line,
        span: create_test_fake_span(),
        text: Atom::from(attributes),
    }
}

#[allow(unused)]
pub fn create_test_ident(name: String) -> Ident{
    let ident = Ident{
        span: create_test_fake_span(),
        ctxt: SyntaxContext::empty(),
        sym: Atom::from(name),
        optional: false,
    };
    ident
}
#[allow(unused)]
pub fn create_test_ast_context () -> AstGlobalContext {
    AstGlobalContext::default()
}

#[allow(unused)]
pub fn create_test_pat_as_ident(pat_name: String) -> Pat{
    let pat = Pat::Ident(BindingIdent{
        id: create_test_ident(pat_name),
        type_ann: None,
    });
    pat
}

#[allow(unused)]
pub fn create_test_object_lit(props: Vec<PropOrSpread>) -> Expr{
    Expr::Object(ObjectLit{
        span: create_test_fake_span(),
        props: props
    })
}

#[allow(unused)]
pub fn create_test_current_context() -> CurrentContext{
    CurrentContext::default()
}

#[allow(unused)]
pub fn create_test_global_context () -> GlobalContext {GlobalContext::default()}


#[allow(unused)]
pub fn create_test_prop(prop_name: String, value: Expr) -> PropOrSpread {
    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
        key: PropName::Ident(IdentName{
            span:create_test_fake_span(),
            sym:Atom::from(prop_name),
        }),
        value: Box::new(value),
    })))
}

#[allow(unused)]
pub fn create_test_spread(value: Expr) -> PropOrSpread {
    PropOrSpread::Spread(SpreadElement{
        dot3_token: create_test_fake_span(),
        expr: Box::new(value),
    })
}



