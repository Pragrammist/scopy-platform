use std::panic::{catch_unwind, AssertUnwindSafe};

#[allow(unused)]
pub fn assert_compiler_panic<E, F, R>(f: F, expected: E)
where
    E: PartialEq + std::fmt::Debug + 'static,
    F: FnOnce() -> R,
    R: std::fmt::Debug,
{
    let result = catch_unwind(AssertUnwindSafe(f));
    let panic = result.expect_err("Expected compiler panic");
    let reason = panic
        .downcast_ref::<E>()
        .expect("Expected panic payload of type E");
    assert_eq!(reason, &expected);
}



#[cfg(test)]
mod get_attributes_tests {
    use swc_atoms::Atom;
    use swc_common::comments::{Comment, CommentKind, Comments};
    use swc_common::DUMMY_SP;
    use crate::scopy_ir::{get_attributes, parse_attributes, parse_attributes_from_comments};
    use crate::semantic::AstGlobalContext;

    #[test]
    fn test_get_attributes(){



        /*
            parsing attributes
            example usage:

            //@testAttribute("test", test2) <--- must be js comment. all input attr is reading as str.
            const d = {};
        */

        let attribute = r#"@testAttribute("test", test2)"#.to_string();
        let ctx = create_global_ctx_with_comments(attribute);
        let span = DUMMY_SP;
        let attributes = get_attributes(&span, &ctx);
        let res = attributes.iter().find(|attr|{attr.name == "testAttribute"});
        assert!(res.is_some(), "Cannot find attribute 'testAttribute'");
    }

    fn create_global_ctx_with_comments(attributes: String) -> AstGlobalContext{
        let ast_global_ctx = AstGlobalContext::default();

        let comment_test = Comment{
            kind:CommentKind::Line,
            span: DUMMY_SP,
            text: Atom::from(attributes),
        };
        ast_global_ctx.comments.add_leading(DUMMY_SP.lo, comment_test);

        ast_global_ctx

    }

    #[test]
    fn test_get_attributes_no_comments() {
        /*
            js code:
            const d = {};
            // no attributes
        */

        let ast_global_ctx = AstGlobalContext::default();
        let span = DUMMY_SP;

        let attributes = get_attributes(&span, &ast_global_ctx);

        assert!(attributes.is_empty(), "Attributes should be empty");
    }

    #[test]
    fn test_get_attributes_multiple_attributes() {
        /*
            parsing attributes
            example usage:

            //@first("one")
            //@second("two")
            const d = {};
        */

        let attribute = r#"@first("one") @second("two")"#.to_string();
        let ctx = create_global_ctx_with_comments(attribute);
        let span = DUMMY_SP;

        let attributes = get_attributes(&span, &ctx);

        assert_eq!(attributes.len(), 2);

        assert!(attributes.iter().any(|attr| attr.name == "first"));
        assert!(attributes.iter().any(|attr| attr.name == "second"));
    }

    #[test]
    fn test_parse_attributes_from_block_comment() {
        /*
            js code:
            /*
                @testAttribute("test")
            */
            const d = {};
            // block comments must be ignored
        */

        let comment = Comment {
            kind: CommentKind::Block,
            span: DUMMY_SP,
            text: Atom::from(r#"@testAttribute("test")"#),
        };

        let comments = vec![comment];

        let attributes = parse_attributes_from_comments(comments);

        assert!(
            attributes.is_empty(),
            "Attributes from block comments should be ignored"
        );
    }

    #[test]
    fn test_parse_attributes_from_multiple_line_comments() {
        /*
            js code:
            //@first("one")
            //@second("two")
            const d = {};
        */

        let comment1 = Comment {
            kind: CommentKind::Line,
            span: DUMMY_SP,
            text: Atom::from(r#"@first("one")"#),
        };

        let comment2 = Comment {
            kind: CommentKind::Line,
            span: DUMMY_SP,
            text: Atom::from(r#"@second("two")"#),
        };

        let attributes = parse_attributes_from_comments(vec![comment1, comment2]);

        assert_eq!(attributes.len(), 2);
        assert_eq!(attributes[0].name, "first");
        assert_eq!(attributes[1].name, "second");
    }

    #[test]
    fn test_parse_attributes_name_and_values() {
        /*
            js code:
            //@testAttribute("test", test2)
        */

        let input = r#"@testAttribute("test", test2)"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");
        assert_eq!(
            attributes[0].vals,
            vec!["test".to_string(), "test2".to_string()]
        );
    }

    #[test]
    fn test_parse_attributes_multiple() {
        /*
            js code:
            //@first("one") @second("two")
        */

        let input = r#"@first("one") @second("two")"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 2);

        assert_eq!(attributes[0].name, "first");
        assert_eq!(attributes[0].vals, vec!["one".to_string()]);

        assert_eq!(attributes[1].name, "second");
        assert_eq!(attributes[1].vals, vec!["two".to_string()]);
    }

    #[test]
    fn test_parse_attributes_without_values() {
        /*
            js code:
            //@testAttribute()
        */

        let input = "@testAttribute()";

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");
        assert!(attributes[0].vals.is_empty());
    }

    #[test]
    fn test_parse_attributes_with_spaces() {
        /*
            js code:
            //@testAttribute(  "test"  ,  test2  )
        */

        let input = r#"@testAttribute(  "test"  ,  test2  )"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");
        assert_eq!(
            attributes[0].vals,
            vec!["test".to_string(), "test2".to_string()]
        );
    }

    #[test]
    fn test_parse_attributes_escaped_characters() {
        /*
            js code:
            //@testAttribute("line\ntext", "tab\ttext", "quote\"text")
        */

        let input = r#"@testAttribute("line\ntext", "tab\ttext", "quote\"text")"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");

        assert_eq!(
            attributes[0].vals,
            vec![
                "line\ntext".to_string(),
                "tab\ttext".to_string(),
                "quote\"text".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_attributes_escape_carriage_return_and_backslash() {
        /*
            js code:
            //@testAttribute("line\rtext", "path\\test")
        */

        let input = r#"@testAttribute("line\rtext", "path\\test")"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");

        assert_eq!(
            attributes[0].vals,
            vec![
                "line\rtext".to_string(),
                "path\\test".to_string(),
            ]
        );
    }

    #[test]
    fn test_parse_attributes_ignores_text_before_attribute() {
        /*
            js code:
            // some random text @testAttribute("test")
        */

        let input = r#"some random text @testAttribute("test")"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");
        assert_eq!(attributes[0].vals, vec!["test".to_string()]);
    }

    #[test]
    fn test_parse_attributes_without_opening_parenthesis() {
        /*
            js code:
            //@testAttribute
        */

        let input = "@testAttribute";

        let attributes = parse_attributes(input);

        assert!(
            attributes.is_empty(),
            "Attribute without '(' should be ignored"
        );
    }

    #[test]
    fn test_parse_attributes_unknown_escape() {
        /*
            js code:
            //@testAttribute("test\xvalue")
        */

        let input = r#"@testAttribute("test\xvalue")"#;

        let attributes = parse_attributes(input);

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].name, "testAttribute");
        assert_eq!(
            attributes[0].vals,
            vec!["testxvalue".to_string()]
        );
    }

}





#[cfg(test)]
mod parse_ident_tests {

    use crate::unit_tests::assert_compiler_panic;
    use swc_atoms::Atom;
    use swc_common::{SyntaxContext, DUMMY_SP};
    use swc_ecma_ast::{ArrayPat, AssignPat, Expr, Ident, Invalid, Lit, Null, ObjectPat, RestPat};
    use swc_ecma_ast::{BindingIdent, Pat};
    use crate::scopy_ir::{parse_ident, parse_pat, ExprPanic};

    #[test]
    fn test_parse_ident_as_ident(){

        let test_name = "test_name".to_string();

        /* js example ident:

            import * as f from "./f.js"
            // f is ident
            const d = {}
            // d is ident
        */
        let ident = Ident{
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            sym: Atom::from(test_name.clone()),
            optional: false,
        };

        let name = parse_ident(&ident);

        assert_eq!(name, test_name.clone(), "cannot read test_name in ident");
    }

    #[test]
    fn test_parse_pat_as_ident(){

        let test_name = "test_name".to_string();


        /*
            js code:

            const d = {};

            ...d - <- that pat is ident.

            const f = {...d}
        */
        let pat = Pat::Ident(BindingIdent{
            id: Ident{
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                sym: Atom::from(test_name.clone()),
                optional: false,
            },
            type_ann: None,
        });

        let name = parse_pat(&pat);

        assert_eq!(name, test_name.clone(), "wrong name of pat ident");
    }


    #[test]
    fn test_parse_pat_as_array(){


        /*
            js code:
            const f = {...[]}
            //not supported
        */
        let pat = Pat::Array(ArrayPat{
            span: DUMMY_SP,
            elems: vec![],
            optional: false,
            type_ann: None
        });


        assert_compiler_panic(|| parse_pat(&pat), ExprPanic::ArrayPatternNotSupported);
    }

    #[test]
    fn test_parse_pat_as_rest() {
        /*
            js code:
            const f = {...rest}
            // not supported
        */
        let pat = Pat::Rest(RestPat {
            span: DUMMY_SP,
            dot3_token: DUMMY_SP,
            arg: Box::new(Pat::Ident(BindingIdent {
                id: Ident::new_no_ctxt("rest".into(), DUMMY_SP),
                type_ann: None,
            })),
            type_ann: None,
        });

        assert_compiler_panic(
            || parse_pat(&pat),
            ExprPanic::RestPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_pat_as_object() {
        /*
            js code:
            const f = {...{}}
            // not supported
        */
        let pat = Pat::Object(ObjectPat {
            span: DUMMY_SP,
            props: vec![],
            optional: false,
            type_ann: None,
        });

        assert_compiler_panic(
            || parse_pat(&pat),
            ExprPanic::ObjectPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_pat_as_assign() {
        /*
            js code:
            const f = {...(x = value)}
            // not supported
        */
        let pat = Pat::Assign(AssignPat {
            span: DUMMY_SP,
            left: Box::new(Pat::Ident(BindingIdent {
                id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                type_ann: None,
            })),
            right: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
        });

        assert_compiler_panic(
            || parse_pat(&pat),
            ExprPanic::AssignPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_pat_as_invalid() {
        /*
            js code:
            const f = {...invalid}
            // not supported
        */
        let pat = Pat::Invalid(Invalid {
            span: DUMMY_SP,
        });

        assert_compiler_panic(
            || parse_pat(&pat),
            ExprPanic::InvalidPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_pat_as_expr() {
        /*
            js code:
            const f = {...(foo)}
            // not supported
        */
        let pat = Pat::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
            "foo".into(),
            DUMMY_SP,
        ))));

        assert_compiler_panic(
            || parse_pat(&pat),
            ExprPanic::ExpressionPatternNotSupported,
        );
    }


}


#[cfg(test)]
mod parse_expr_tests {
    use crate::unit_tests::assert_compiler_panic;
    use swc_atoms::{Atom, Wtf8Atom};
    use swc_common::{SyntaxContext, DUMMY_SP};
    use swc_ecma_ast::{ArrayLit, ArrayPat, ArrowExpr, AssignExpr, AssignOp, AssignPat, AssignProp, AssignTarget, AwaitExpr, BigInt, BinExpr, BinaryOp, BindingIdent, BlockStmt, BlockStmtOrExpr, Bool, CallExpr, Callee, ClassExpr, ComputedPropName, CondExpr, Expr, ExprOrSpread, FnExpr, Function, GetterProp, Ident, IdentName, Import, Invalid, JSXMemberExpr, JSXObject, KeyValueProp, Lit, MemberExpr, MemberProp, MetaPropExpr, MetaPropKind, MethodProp, NewExpr, Null, Number, ObjectLit, ObjectPat, OptChainBase, OptChainExpr, ParenExpr, Pat, PrivateName, Prop, PropName, PropOrSpread, Regex, RestPat, SeqExpr, SetterProp, SpreadElement, Str, Super, SuperProp, SuperPropExpr, TaggedTpl, ThisExpr, Tpl, TsAsExpr, TsConstAssertion, TsInstantiation, TsKeywordType, TsKeywordTypeKind, TsNonNullExpr, TsSatisfiesExpr, TsType, TsTypeAssertion, TsTypeParamInstantiation, UnaryExpr, UnaryOp, UpdateExpr, UpdateOp, YieldExpr};
    use crate::scopy_ir::{parse_expr, ExprPanic};
    use crate::semantic::{AnotherObjectValue, AnotherObjectValuePath, AstGlobalContext, BinaryOpType, CurrentContext, FunctionCallResultValue, FunctionValue, GlobalContext, LitValueBool, LitValueNum, LitValueString, LiteralValue, ObjectData, ObjectDataValue,  ObjectValue, ScopeStatement, Statement};

    #[test]
    fn parse_object_lit_key_value_test(){
        let prop_name = "test1".to_string();
        let ident_prop_val = "test2".to_string();



        /* code for testing
            {
                test1: "test2"
            };
        */


        let expr_obj_lit_props = Expr::Object(ObjectLit{
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
                    key: PropName::Ident(IdentName{
                        span:DUMMY_SP,
                        sym:Atom::from(prop_name.clone()),
                    }),
                    value: Box::new(
                        Expr::Lit(Lit::Str(Str{
                            span: DUMMY_SP,
                            value: Wtf8Atom::from(ident_prop_val.clone()),
                            raw: None,
                        }))
                    ),
                }))),
            ]
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(|| parse_expr(&expr_obj_lit_props, &test_context, &test_global_ctx, &test_ast_context), ExprPanic::KeyValuePropNotSupported);
    }


    //noinspection DuplicatedCode
    #[test]
    fn parse_object_lit_spread_test(){
        let ident_name = "test1".to_string();
        let prop_name = "test2".to_string();
        let module_name = "main".to_string();


        let test_context = CurrentContext{
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        value: ObjectDataValue::Object(ObjectValue{
                            props: vec![
                                ObjectData {
                                    is_mutable: false,
                                    name: prop_name.clone(),
                                    attrs: vec![],
                                    value: ObjectDataValue::Literal(LiteralValue::Null),
                                    
                                }
                            ]
                        }),
                        name: ident_name.clone(),
                        is_mutable: false,
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: module_name.clone(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();



        /* code for testing

            const test1 = {
                test2:null
            };


            {
                ...test1
            };
        */
        let obj_lit = Expr::Object(ObjectLit{
            span: DUMMY_SP,
            props: vec![PropOrSpread::Spread(SpreadElement{
                dot3_token: DUMMY_SP,
                expr: Box::new(Expr::Ident(Ident{
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    sym: Atom::from(ident_name.clone()),
                    optional: false,
                })),
            })]
        });

        //
        // let obj_parsed_value = parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context);

        assert_compiler_panic(
            || parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context),
            ExprPanic::SpreadNotAllowed,
        );

        // if let ObjectDataValue::Object(obj_parsed) = obj_parsed_value {
        //     let d = obj_parsed.props.iter().find(|p| p.name == prop_name.clone()).is_some();
        //     assert!(d, "There is no value for {:?}", prop_name)
        // } else {
        //     assert!(false, "Wrong object value")
        // }
    }


    #[test]
    #[ignore]
    fn parse_object_lit_spread_enum_test_todo(){
        //todo: make enum and tests for enum expr spreading
        assert!(false, "enum not impl yet");
    }

    #[test]
    fn parse_object_lit_spread_obj_test(){

        /* js code:
            {
                ...{
                    value=null
                }
            }
        */
        let obj_lit = Expr::Object(ObjectLit{
            span: DUMMY_SP,
            props: vec![PropOrSpread::Spread(SpreadElement{
                dot3_token: DUMMY_SP,
                expr: Box::new(Expr::Object(ObjectLit{
                    span: DUMMY_SP,
                    props: vec![PropOrSpread::Prop(Box::new(Prop::Assign(AssignProp{
                        key: Ident{
                            span: DUMMY_SP,
                            ctxt: Default::default(),
                            sym: Atom::from("value"),
                            optional: false,
                        },
                        span: DUMMY_SP,
                        value: Box::new(Expr::Lit(Lit::Null(Null {
                            span: DUMMY_SP,
                        }))),
                    })))]
                })),
            })]
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context),
            ExprPanic::SpreadNotAllowed,
        );
    }


    #[test]
    //noinspection DuplicatedCode
    fn parse_object_lit_spread_lit_test(){

        /* js code:
            {
                ...(result = null) => {}
            }
        */
        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Spread(SpreadElement {
                    dot3_token: DUMMY_SP,
                    expr: Box::new(Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        ctxt: Default::default(),
                        params: vec![
                            Pat::Assign(AssignPat {
                                span: DUMMY_SP,
                                left: Box::new(Pat::Ident(BindingIdent {
                                    id: Ident::new_no_ctxt("result".into(), DUMMY_SP),
                                    type_ann: None,
                                })),
                                right: Box::new(Expr::Lit(Lit::Null(Null {
                                    span: DUMMY_SP,
                                }))),
                            }),
                        ],
                        body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: vec![],
                        })),
                        is_async: false,
                        is_generator: false,
                        type_params: None,
                        return_type: None,
                    })),
                }),
            ],
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context),
            ExprPanic::SpreadNotAllowed,
        );
    }


    #[test]
    //noinspection DuplicatedCode
    fn parse_object_lit_spread_bin_expr_test(){

        /* js code:
            {
                ...Null==Null
            }
        */
        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Spread(SpreadElement {
                    dot3_token: DUMMY_SP,
                    expr: Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: BinaryOp::EqEq,
                        left: Box::new(Expr::Lit(Lit::Null(Null {
                            span: DUMMY_SP,
                        }))),
                        right: Box::new(Expr::Lit(Lit::Null(Null {
                            span: DUMMY_SP,
                        }))),
                    })),
                }),
            ],
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context),
            ExprPanic::SpreadNotAllowed,
        );
    }















    //noinspection DuplicatedCode
    #[test]
    fn parse_object_lit_prop_test(){
        let str_val = "str val".to_string();
        let prop_name = "test2".to_string();


        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();




        /*
            {
                test2="str val"
            };
        */

        let expr = Expr::Object(ObjectLit{
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::Assign(AssignProp{
                key: Ident{
                    span:DUMMY_SP,
                    ctxt: Default::default(),
                    sym:Atom::from(prop_name.clone()),
                    optional: false,
                },
                span: DUMMY_SP,
                value: Box::new(Expr::Lit(Lit::Str(Str{
                    span: DUMMY_SP,
                    value: Wtf8Atom::from(str_val),
                    raw: None,
                }))),
            })))]
        });


        let obj_parsed = parse_expr(&expr, &test_context, &test_global_ctx, &test_ast_context);


        if let ObjectDataValue::Object(obj_parsed) = obj_parsed {
            let d = obj_parsed.props.iter().find(|p| p.name == prop_name.clone()).is_some();
            assert!(d, "There is no value for {:?}", prop_name)
        } else {
            assert!(false, "Wrong object value")
        }




    }

    #[test]
    fn test_parse_expr_object_shorthand_prop_test() {
        /*
            js code:
            const obj = {
                value
            }
            // not supported
        */

        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    Prop::Shorthand(Ident::new_no_ctxt(
                        "value".into(),
                        DUMMY_SP,
                    )),
                )),
            ],
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &obj_lit,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context
                );
            },
            ExprPanic::ShorthandPropNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_object_assign_prop_test() {
        let prop_name = "value".to_string();
        /*
            js code:
            const obj = {
                value = null
            }
            // not supported
        */

        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    Prop::Assign(AssignProp {
                        span: DUMMY_SP,
                        key: Ident {
                            span: DUMMY_SP,
                            ctxt: Default::default(),
                            sym: Atom::from(prop_name.clone()),
                            optional: false,
                        },
                        value: Box::new(Expr::Lit(Lit::Null(Null {
                            span: DUMMY_SP,
                        }))),
                    }),
                )),
            ],
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let obj = parse_expr(
            &obj_lit,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        match obj
        {
            ObjectDataValue::Object(obj_val) => {
                let el = obj_val.props.iter().find(|p|  p.name == prop_name.clone());
                assert!(el.is_some(), "Cannot find object with prop_name {}", prop_name.clone());
            },
            _ => {
                assert!(false, "Failed to parse object literal. it's not an Object in ObjectDataValue");
            }
        }
    }


    #[test]
    fn test_parse_expr_object_getter_prop_test() {
        /*
            js code:
            const obj = {
                get value() {}
            }
            // not supported
        */

        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    Prop::Getter(GetterProp {
                        span: DUMMY_SP,
                        key: PropName::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: Atom::from("value"),
                        }),
                        type_ann: None,
                        body: Some(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: vec![],
                        }),
                    }),
                )),
            ],
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &obj_lit,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::GetterPropNotAllowed,
        );
    }


    #[test]
    fn test_parse_expr_object_setter_prop_test() {
        /*
            js code:
            const obj = {
                set value(value) {}
            }
            // not supported
        */

        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    Prop::Setter(SetterProp {
                        span: DUMMY_SP,
                        key: PropName::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: Atom::from("value"),
                        }),
                        this_param: None,
                        param: Box::from(Pat::Ident(BindingIdent {
                            id: Ident::new_no_ctxt(
                                "value".into(),
                                DUMMY_SP,
                            ),
                            type_ann: None,
                        })),
                        body: Some(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: vec![],
                        }),
                    }),
                )),
            ],
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &obj_lit,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SetterPropNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_object_method_prop_test() {
        /*
            js code:
            const obj = {
                value() {}
            }
            // not supported
        */

        let obj_lit = Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    Prop::Method(MethodProp {
                        key: PropName::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: Atom::from("value"),
                        }),
                        function: Box::new(Function {
                            params: vec![],
                            decorators: vec![],
                            span: DUMMY_SP,
                            ctxt: Default::default(),
                            body: Some(BlockStmt {
                                span: DUMMY_SP,
                                ctxt: SyntaxContext::empty(),
                                stmts: vec![],
                            }),
                            is_generator: false,
                            is_async: false,
                            type_params: None,
                            return_type: None,
                        }),
                    }),
                )),
            ],
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &obj_lit,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::MethodPropNotAllowed,
        );
    }









    #[test]
    fn test_binary_operations() {
        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let cases = [
            //==
            (BinaryOp::EqEq, BinaryOpType::EqEq),
            // !=
            (BinaryOp::NotEq, BinaryOpType::NotEq),
            // <
            (BinaryOp::Lt, BinaryOpType::Lt),
            // <=
            (BinaryOp::LtEq, BinaryOpType::LtEq),
            // >
            (BinaryOp::Gt, BinaryOpType::Gt),
            // >=
            (BinaryOp::GtEq, BinaryOpType::GtEq),
            // +
            (BinaryOp::Add, BinaryOpType::Add),
            // -
            (BinaryOp::Sub, BinaryOpType::Sub),
            // *
            (BinaryOp::Mul, BinaryOpType::Mul),
            // /
            (BinaryOp::Div, BinaryOpType::Div),
            // %
            (BinaryOp::Mod, BinaryOpType::Mod),
            // &
            (BinaryOp::LogicalAnd, BinaryOpType::LogicalAnd),
            // |
            (BinaryOp::LogicalOr, BinaryOpType::LogicalOr),
            // ??
            (BinaryOp::NullishCoalescing, BinaryOpType::NullishCoalescing),
        ];

        for (op, expected) in cases {
            assert_binary_op(
                op,
                expected,
                &test_context,
                &test_global_ctx,
                &test_ast_context,
            );
        }
    }

    #[test]
    fn test_parse_expr_private_name_member_prop() {
        /*
            js code:
                object.#private
            // not supported
        */

        let member_expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            prop: MemberProp::PrivateName(PrivateName {
                span: DUMMY_SP,
                name: Atom::from("private_name"),
            }),
        });

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &member_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::PrivateNamePropNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_computed_member_prop() {
        /*
            js code:
                object[null]
            // not supported
        */

        let member_expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            prop: MemberProp::Computed(ComputedPropName {
                span: DUMMY_SP,
                expr: Box::new(Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))),
            }),
        });

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &member_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ComputedPropNotAllowed,
        );
    }
    #[test]
    fn test_parse_expr_unsupported_binary_operations() {
        /*
            js code examples:
            null === null
            null !== null
            null << null
            null >> null
            null >>> null
            null | null
            null ^ null
            null & null
            null in null
            null instanceof null
            null ** null

            // not supported
        */

        let unsupported_ops = [
            BinaryOp::EqEqEq,
            BinaryOp::NotEqEq,
            BinaryOp::LShift,
            BinaryOp::RShift,
            BinaryOp::ZeroFillRShift,
            BinaryOp::BitOr,
            BinaryOp::BitXor,
            BinaryOp::BitAnd,
            BinaryOp::In,
            BinaryOp::InstanceOf,
            BinaryOp::Exp,
        ];

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        for op in unsupported_ops {
            let binary_expr = Expr::Bin(BinExpr {
                span: DUMMY_SP,
                op,
                left: Box::new(Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))),
                right: Box::new(Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))),
            });

            assert_compiler_panic(
                || {
                    parse_expr(
                        &binary_expr,
                        &test_context,
                        &test_global_ctx,
                        &test_ast_context,
                    );
                },
                ExprPanic::BinaryOperationNotSupported,
            );
        }
    }

    fn assert_binary_op(
        op: BinaryOp,
        expected: BinaryOpType,
        test_context: &CurrentContext,
        test_global_ctx: &GlobalContext,
        test_ast_context: &AstGlobalContext,
    ) {
        let left_val = "left val".to_string();
        let right_val = "right val".to_string();



        /*
            "left val" == "right val"
            "left val" != "right val"
            //etc.
        */


        let expr = Expr::Bin(BinExpr{
            op: op,
            span: DUMMY_SP,
            left: Box::from(Expr::Lit(Lit::Str(Str{
                span: DUMMY_SP,
                value: Wtf8Atom::from(left_val),
                raw: None,
            }))),
            right: Box::from(Expr::Lit(Lit::Str(Str{
                span: DUMMY_SP,
                value: Wtf8Atom::from(right_val),
                raw: None,
            }))),
        });

        let obj_val = parse_expr(
            &expr,
            test_context,
            test_global_ctx,
            test_ast_context,
        );

        match obj_val {
            ObjectDataValue::Binary(bin_val) => {
                assert_eq!(
                    bin_val.op,
                    expected,
                    "binary operation {:?} was parsed incorrectly",
                    op
                );
            }
            _ => {
                panic!("binary operation {:?} did not produce Binary", op);
            }
        }
    }


    #[test]
    #[ignore]
    fn create_parse_member_expr_enum(){
        assert!(false,"enum not done yet");
    }



    #[test]
    fn create_parse_member_expr_obj(){
        /* js code
            { prop = null }.prop
        */
        let prop_name = "prop".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![
                    PropOrSpread::Prop(Box::new(
                        Prop::Assign(AssignProp {
                            span: DUMMY_SP,
                            key: Ident::from(prop_name.clone()),
                            value: Box::new(Expr::Lit(Lit::Null(Null {
                                span: DUMMY_SP,
                            }))),
                        })
                    ))
                ],
            })),
            prop: MemberProp::Ident(IdentName::new(prop_name.clone().into(), DUMMY_SP)),
        });

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();


        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );


        let assert_res = res == ObjectDataValue::AnotherObject(AnotherObjectValue{
            obj: Box::new(ObjectData {
                is_mutable: false,
                name: prop_name.clone(),
                value: ObjectDataValue::Literal(LiteralValue::Null),
                attrs: vec![],
                
            }),
            path: vec![AnotherObjectValuePath::Ident(prop_name.clone())],
        });

        assert!(assert_res, "output not eq to example");

    }


    #[test]
    //noinspection DuplicatedCode
    fn create_parse_member_expr_another_obj(){
        /* js code
            obj.prop
        */
        let prop_name = "prop".to_string();
        let obj_name = "obj".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new_no_ctxt(
                obj_name.clone().into(),
                DUMMY_SP,
            ))),
            prop: MemberProp::Ident(IdentName::new(
                prop_name.clone().into(),
                DUMMY_SP,
            )),
        });

        let test_context = CurrentContext{
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: obj_name.clone().to_string(),
                        value: ObjectDataValue::Object(ObjectValue{
                            props: vec![
                                ObjectData {
                                    is_mutable: false,
                                    name: prop_name.clone(),
                                    value: ObjectDataValue::Literal(LiteralValue::Null),
                                    attrs: vec![],
                                    
                                }
                            ],
                        }),
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();


        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );





        let assert_res = res == ObjectDataValue::AnotherObject(
            AnotherObjectValue {
                obj: Box::new(ObjectData {
                    is_mutable: false,
                    name: prop_name.clone(),
                    value: ObjectDataValue::Literal(LiteralValue::Null),
                    attrs: vec![],
                    
                }),
                path: vec![
                    AnotherObjectValuePath::Ident(obj_name.clone()),
                    AnotherObjectValuePath::Ident(prop_name.clone()),
                ],
            }
        );

        assert!(assert_res, "output not eq to example");

    }

    #[test]
    //noinspection DuplicatedCode
    fn test_parse_member_expr_with_function_call() {
        /*
            js code:
            const va1 = {
                va21: {
                    va31: (result = { va41: null }) => {}
                }
            };
            va1.va21.va31().va41
        */

        let va1_name = "va1".to_string();
        let va21_name = "va21".to_string();
        let va31_name = "va31".to_string();
        let va41_name = "va41".to_string();

        // Создаём объект для параметра result: { va41: null }
        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Object(ObjectValue {
                props: vec![
                    ObjectData {
                        name: va41_name.clone(),
                        value: ObjectDataValue::Literal(LiteralValue::Null),
                        attrs: vec![],
                        is_mutable: false,
                        
                    }
                ],
            }),
            attrs: vec![],
            is_mutable: false,
            
        };

        // Контекст с объявлением va1
        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: va1_name.clone(),
                        
                        value: ObjectDataValue::Object(ObjectValue {
                            props: vec![
                                ObjectData {
                                    is_mutable: false,
                                    name: va21_name.clone(),
                                    value: ObjectDataValue::Object(ObjectValue {
                                        props: vec![
                                            ObjectData {
                                                is_mutable: false,
                                                name: va31_name.clone(),
                                                value: ObjectDataValue::Function(FunctionValue {
                                                    scope: Box::new(ScopeStatement { statements: vec![] }),
                                                    params: vec![result_obj.clone()],
                                                    result: Box::new(result_obj),
                                                }),
                                                attrs: vec![],
                                                
                                            }
                                        ],
                                    }),
                                    attrs: vec![],
                                    
                                },
                                // va22 – для полноты (не влияет на тест)
                                ObjectData {
                                    is_mutable: false,
                                    name: "va22".to_string(),
                                    value: ObjectDataValue::Object(ObjectValue {
                                        props: vec![
                                            ObjectData {
                                                is_mutable: false,
                                                name: "va31".to_string(),
                                                value: ObjectDataValue::Object(ObjectValue {
                                                    props: vec![],
                                                }),
                                                attrs: vec![],
                                                
                                            }
                                        ],
                                    }),
                                    attrs: vec![],
                                    
                                }
                            ],
                        }),
                        attrs: vec![],
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Строим выражение: va1.va21.va31().va41
        // 1) va1.va21
        let member_va21 = MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new_no_ctxt(
                va1_name.clone().into(),
                DUMMY_SP,
            ))),
            prop: MemberProp::Ident(IdentName::new(
                va21_name.clone().into(),
                DUMMY_SP,
            )),
        };

        // 2) va1.va21.va31
        let member_va31 = MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Member(member_va21)),
            prop: MemberProp::Ident(IdentName::new(
                va31_name.clone().into(),
                DUMMY_SP,
            )),
        };

        // 3) va1.va21.va31()
        let call_va31 = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Member(member_va31))),
            args: vec![],
            type_args: None,
        };

        // 4) (va1.va21.va31()).va41
        let final_expr = MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Call(call_va31)),
            prop: MemberProp::Ident(IdentName::new(
                va41_name.clone().into(),
                DUMMY_SP,
            )),
        };

        let res = parse_expr(
            &Expr::Member(final_expr),
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        // Ожидаемый результат – путь включает Ident("va31") перед FunctionCall
        let expected = ObjectDataValue::AnotherObject(
            AnotherObjectValue {
                obj: Box::new(ObjectData {
                    is_mutable: false,
                    name: va41_name.clone(),
                    value: ObjectDataValue::Literal(LiteralValue::Null),
                    attrs: vec![],
                    
                }),
                path: vec![
                    AnotherObjectValuePath::Ident(va1_name.clone()),
                    AnotherObjectValuePath::Ident(va21_name.clone()),
                    AnotherObjectValuePath::Ident(va31_name.clone()),
                    AnotherObjectValuePath::FunctionCall(
                        FunctionCallResultValue {
                            args: vec![],
                            result: Box::new(
                                ObjectDataValue::Object(ObjectValue {
                                    props: vec![
                                        ObjectData {
                                            is_mutable: false,
                                            name: va41_name.clone(),
                                            value: ObjectDataValue::Literal(LiteralValue::Null),
                                            attrs: vec![],
                                            
                                        }
                                    ],
                                })
                            ),
                        }
                    ),
                    AnotherObjectValuePath::Ident(va41_name.clone()),
                ],
            }
        );

        assert_eq!(res, expected, "Expression va1.va21.va31().va41 was parsed incorrectly");
    }

    #[test]
    fn parse_member_expr_literal_val(){
        let prop_name = "prop".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Lit(Lit::Null(Null{
                span: DUMMY_SP,
            }))),
            prop: MemberProp::Ident(IdentName::new(
                prop_name.clone().into(),
                DUMMY_SP,
            )),
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();



        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::InvalidMemberExpressionMemberNotFound)
    }


    #[test]
    //noinspection DuplicatedCode
    fn parse_member_expr_function_val(){
        // Arrow function: (result = null) => {}
        let prop_name = "prop".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(
                Expr::Arrow(ArrowExpr {
                    span: DUMMY_SP,
                    params: vec![
                        Pat::Assign(AssignPat {
                            span: DUMMY_SP,
                            left: Box::new(Pat::Ident(BindingIdent {
                                id: Ident::new_no_ctxt("result".into(), DUMMY_SP),
                                type_ann: None,
                            })),
                            right: Box::new(Expr::Lit(Lit::Null(Null {
                                span: DUMMY_SP,
                            }))),
                        }),
                    ],
                    body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt{
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        stmts: vec![],
                    })),
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                    ..Default::default()
                })
            ),
            prop: MemberProp::Ident(IdentName::new(
                prop_name.clone().into(),
                DUMMY_SP,
            )),
        });



        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();



        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::InvalidMemberExpressionMemberNotFound)
    }



    #[test]
    //noinspection DuplicatedCode
    fn parse_member_expr_with_not_field(){
        /* js code
                { prop = null }.prop
            */
        let prop_name = "prop".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![],
            })),
            prop: MemberProp::Ident(IdentName::new(prop_name.clone().into(), DUMMY_SP)),
        });

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();




        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::InvalidMemberExpressionMemberNotFound)

    }



    #[test]
    //noinspection DuplicatedCode
    fn parse_member_expr_bin_val(){
        let prop_name = "prop".to_string();
        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Bin(BinExpr{
                span: DUMMY_SP,
                op: BinaryOp::EqEq,
                left: Box::new(Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))),
                right: Box::new(Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))),
            })),
            prop: MemberProp::Ident(IdentName::new(
                prop_name.clone().into(),
                DUMMY_SP,
            )),
        });


        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();



        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::InvalidMemberExpressionMemberNotFound)
    }




    #[test]
    //noinspection DuplicatedCode
    fn parse_member_expr_func_call(){
        let prop_name = "prop".to_string();
        let func_name = "func".to_string();



        let expr = Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                callee: Callee::Expr(Box::new(Expr::Ident(
                    Ident::new_no_ctxt(func_name.clone().into(), DUMMY_SP)
                ))),
                args: vec![],
                type_args: None,
            })),
            prop: MemberProp::Ident(IdentName::new(
                prop_name.clone().into(),
                DUMMY_SP,
            )),
        });


        let test_context = CurrentContext{
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(ObjectData {
                    is_mutable: false,
                    name: func_name.clone().into(),
                    
                    value: ObjectDataValue::Function(FunctionValue{
                        scope: Box::new(ScopeStatement{
                            statements: vec![],
                        }),
                        params: vec![],
                        result: Box::new(ObjectData {
                            is_mutable: false,
                            name: "result".to_string(),
                            
                            value: ObjectDataValue::Object(ObjectValue{
                                props: vec![
                                    ObjectData {
                                        is_mutable: false,
                                        name: prop_name.clone().into(),
                                        value: ObjectDataValue::Literal(LiteralValue::Null),
                                        attrs: vec![],
                                        
                                    }
                                ]
                            }),
                            attrs: vec![],
                        }),
                    }),
                    attrs: vec![],
                })
            ],
            current_module_name: "main".to_string(),
        };
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();


        let call_res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let value = ObjectDataValue::AnotherObject(AnotherObjectValue {
            obj: Box::new(ObjectData {
                is_mutable: false,
                name: "prop".to_string(),
                value: ObjectDataValue::Literal(LiteralValue::Null),
                attrs: vec![],
                
            }),

            path: vec![
                AnotherObjectValuePath::Ident(func_name.clone()),

                AnotherObjectValuePath::FunctionCall(
                    FunctionCallResultValue {
                        args: vec![],
                        result: Box::new(
                            ObjectDataValue::Object(ObjectValue {
                                props: vec![
                                    ObjectData {
                                        is_mutable: false,
                                        name: prop_name.clone(),
                                        value: ObjectDataValue::Literal(LiteralValue::Null),
                                        attrs: vec![],
                                        
                                    }
                                ],
                            })
                        ),
                    }
                ),

                AnotherObjectValuePath::Ident(prop_name.clone()),
            ],
        });

        assert_eq!(value, call_res);
    }





    #[test]
    fn parse_expr_literal_string() {
        let value = "hello".to_string();

        let expr = Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: value.clone().into(),
            raw: None,
        }));

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let assert_res = res == ObjectDataValue::Literal(
            LiteralValue::Str(
                LitValueString {
                    val: value,
                }
            )
        );

        assert!(assert_res, "output not eq to example");
    }





    #[test]
    fn parse_expr_literal_bool() {
        let value = true;

        let expr = Expr::Lit(Lit::Bool(Bool {
            span: DUMMY_SP,
            value,
        }));

        let test_context = CurrentContext ::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let assert_res = res == ObjectDataValue::Literal(
            LiteralValue::Bool(
                LitValueBool {
                    val: value,
                }
            )
        );

        assert!(assert_res, "output not eq to example");
    }


    #[test]
    fn parse_expr_literal_null() {
        let expr = Expr::Lit(Lit::Null(Null {
            span: DUMMY_SP,
        }));

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let assert_res = res == ObjectDataValue::Literal(
            LiteralValue::Null
        );

        assert!(assert_res, "output not eq to example");
    }




    #[test]
    fn parse_expr_literal_num() {
        let value = 123.456_f64;

        let expr = Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value,
            raw: None,
        }));

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let assert_res = res == ObjectDataValue::Literal(
            LiteralValue::Num(
                LitValueNum {
                    val: value,
                }
            )
        );

        assert!(assert_res, "output not eq to example");
    }



    #[test]
    fn parse_expr_literal_bigint_not_allowed() {
        let expr = Expr::Lit(Lit::BigInt(BigInt {
            span: DUMMY_SP,
            value: Box::new(123_u32.into()),
            raw: None,
        }));

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();



        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::BigIntLiteralNotAllowed);
    }


    #[test]
    fn parse_expr_literal_regex_not_allowed() {
        let expr = Expr::Lit(Lit::Regex(Regex {
            span: DUMMY_SP,
            exp: "abc".into(),
            flags: "".into(),
        }));

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();


        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::RegexLiteralNotAllowed);
    }


    #[test]
    fn parse_paren_expr_literal_null() {
        /*
            js code:
                (null)
        */

        let expr = Expr::Paren(ParenExpr {
            span: DUMMY_SP,
            expr: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
        });

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let res = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let assert_res = res == ObjectDataValue::Literal(
            LiteralValue::Null
        );

        assert!(assert_res, "output not eq to example");
    }



    #[test]
    fn parse_expr_call_super_callee_not_supported() {
        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Super(Super {
                span: DUMMY_SP,
            }),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext ::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::SuperCalleeNotSupported);
    }




    #[test]
    fn parse_expr_call_import_callee_not_supported() {
        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Import(Import {
                span: DUMMY_SP,
                phase: Default::default(),
            }),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext ::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();


        assert_compiler_panic(|| parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        ), ExprPanic::ImportCalleeNotSupported);


    }


    #[test]
    fn parse_expr_call_object_value_not_function() {
        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(
                Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: vec![],
                })
            )),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(
                &expr,
                &test_context,
                &test_global_ctx,
                &test_ast_context,
            ),
            ExprPanic::ObjectValueNotFunction,
        );
    }


    #[test]
    fn parse_expr_call_literal_value_not_function() {
        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(
                Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))
            )),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(
                &expr,
                &test_context,
                &test_global_ctx,
                &test_ast_context,
            ),
            ExprPanic::LiteralValueNotFunction,
        );
    }



    #[test]
    //noinspection DuplicatedCode
    fn parse_expr_call_nested_function_call_not_allowed() {
        let func_name = "func".to_string();
        let inner_call = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(
                Expr::Ident(Ident::new_no_ctxt(
                    func_name.clone().into(),
                    DUMMY_SP,
                ))
            )),
            args: vec![],
            type_args: None,
        });

        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(inner_call)),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext{
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(ObjectData {
                    is_mutable: false,
                    
                    name: func_name.clone(),
                    value: ObjectDataValue::Function(FunctionValue{
                        scope: Box::new(ScopeStatement {
                            statements:vec![]
                        }),
                        params: vec![],
                        result: Box::new(ObjectData {
                            name: "result".to_string(),
                            value: ObjectDataValue::Literal(LiteralValue::Null),
                            attrs: vec![],
                            is_mutable: false,
                            
                        }),
                    }),
                    attrs: vec![],
                })
            ],
            current_module_name: "".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(
                &expr,
                &test_context,
                &test_global_ctx,
                &test_ast_context,
            ),
            ExprPanic::NestedFunctionCallNotAllowed,
        );
    }




    #[test]
    fn parse_expr_call_binary_value_not_function() {
        let expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(
                Expr::Bin(BinExpr {
                    span: DUMMY_SP,
                    op: BinaryOp::Add,
                    left: Box::new(
                        Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: 1.0,
                            raw: None,
                        }))
                    ),
                    right: Box::new(
                        Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: 2.0,
                            raw: None,
                        }))
                    ),
                })
            )),
            args: vec![],
            type_args: None,
        });

        let test_context = CurrentContext::default();

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || parse_expr(
                &expr,
                &test_context,
                &test_global_ctx,
                &test_ast_context,
            ),
            ExprPanic::BinaryValueNotFunction,
        );
    }


    



    #[test]
    #[ignore]
    fn parse_expr_call_enum_not_function   () {
        //todo:
        assert_compiler_panic(
            || (),
            ExprPanic::EnumValueNotFunction,
        );
    }



    #[test]
    //noinspection DuplicatedCode
    fn test_call_arrow_function_directly_panic() {
        /*
            js code:
            (() => {})()
            // Should panic with NotAnotherObjectValueBeforeFunctionCall
        */

        // Строим AST для стрелочной функции: (result = null) => {}
        let arrow_fn = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Assign(AssignPat {
                    span: DUMMY_SP,
                    left: Box::new(Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("result".into(), DUMMY_SP),
                        type_ann: None,
                    })),
                    right: Box::new(Expr::Lit(Lit::Null(Null {
                        span: DUMMY_SP,
                    }))),
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        // Выражение вызова: (() => {})()
        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Arrow(arrow_fn))),
            args: vec![],
            type_args: None,
        };

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::NotAnotherObjectValueBeforeFunctionCall,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_another_object_as_function_panic() {
        /*
            js code:
            const inner = {};
            const a = {
                b: inner
            };
            a.b()
            // Should panic with NotFunctionCallAfterAnotherObjectValue
        */

        let a_name = "a".to_string();
        let b_name = "b".to_string();

        // Создаём внутренний объект inner (простой объект без свойств)
        let inner_obj = ObjectData {
            name: "inner".to_string(),
            value: ObjectDataValue::Object(ObjectValue {
                props: vec![],
            }),
            attrs: vec![],
            is_mutable: false,
            
        };

        // Создаём a: { b: inner }
        let a_obj = ObjectData {
            is_mutable: false,
            name: a_name.clone(),
            
            value: ObjectDataValue::Object(ObjectValue {
                props: vec![
                    ObjectData {
                        is_mutable: false,
                        name: b_name.clone(),
                        // b ссылается на inner как на AnotherObject
                        value: ObjectDataValue::AnotherObject(
                            AnotherObjectValue {
                                obj: Box::new(inner_obj),
                                path: vec![AnotherObjectValuePath::Ident("inner".to_string())],
                            }
                        ),
                        attrs: vec![],
                        
                    }
                ],
            }),
            attrs: vec![],
        };

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(a_obj),
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Выражение: a.b()
        let member_expr = MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new_no_ctxt(
                a_name.clone().into(),
                DUMMY_SP,
            ))),
            prop: MemberProp::Ident(IdentName::new(
                b_name.clone().into(),
                DUMMY_SP,
            )),
        };

        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Member(member_expr))),
            args: vec![],
            type_args: None,
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::NotFunctionCallAfterAnotherObjectValue,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_no_args() {
        /*
            js code:
            const f = (result = null) => {};
            f()
        */

        let f_name = "f".to_string();

        // Создаём функцию
        let func_val = FunctionValue {
            scope: Box::new(ScopeStatement { statements: vec![] }),
            params: vec![
                ObjectData {
                    name: "result".to_string(),
                    value: ObjectDataValue::Literal(LiteralValue::Null),
                    attrs: vec![],
                    is_mutable: false,
                    
                }
            ],
            result: Box::new(ObjectData {
                name: "result".to_string(),
                value: ObjectDataValue::Literal(LiteralValue::Null),
                attrs: vec![],
                is_mutable: false,
                
            }),
        };

        // Контекст с объявлением f
        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: f_name.clone(),
                        value: ObjectDataValue::Function(func_val),
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Выражение: f()
        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                f_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![],
            type_args: None,
        };

        let res = parse_expr(
            &Expr::Call(call_expr),
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let expected = ObjectDataValue::AnotherObject(
            AnotherObjectValue {
                obj: Box::new(ObjectData {
                    is_mutable: false,
                    name: f_name.clone(),
                    value: ObjectDataValue::FunctionCall(
                        FunctionCallResultValue {
                            args: vec![],
                            result: Box::new(ObjectDataValue::Literal(LiteralValue::Null)),
                        }
                    ),
                    attrs: vec![],
                    
                }),
                path: vec![
                    AnotherObjectValuePath::Ident(f_name.clone()),
                    AnotherObjectValuePath::FunctionCall(
                        FunctionCallResultValue {
                            args: vec![],
                            result: Box::new(ObjectDataValue::Literal(LiteralValue::Null)),
                        }
                    ),
                ],
            }
        );

        assert_eq!(res, expected, "Function call without args failed");
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_with_arg() {
        /*
            js code:
            const func = (result = null, arg1 = null) => {};
            func(null)
        */

        let func_name = "func".to_string();

        // Параметры: result и arg1
        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };
        let arg1_obj = ObjectData {
            name: "arg1".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        // Контекст с объявлением func
        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: func_name.clone(),
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![result_obj.clone(), arg1_obj.clone()],
                            result: Box::new(result_obj),
                        }),
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Аргумент: null
        let arg_expr = Expr::Lit(Lit::Null(Null { span: DUMMY_SP }));
        let arg = ExprOrSpread {
            spread: None,
            expr: Box::new(arg_expr),
        };

        // Выражение: func(null)
        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![arg],
            type_args: None,
        };

        let res = parse_expr(
            &Expr::Call(call_expr),
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        // Ожидаемый результат: AnotherObject с FunctionCall, аргументы [Literal::Null], результат – null
        let expected = ObjectDataValue::AnotherObject(
            AnotherObjectValue {
                obj: Box::new(ObjectData {
                    is_mutable: false,
                    name: func_name.clone(),
                    value: ObjectDataValue::FunctionCall(
                        FunctionCallResultValue {
                            args: vec![ObjectDataValue::Literal(LiteralValue::Null)],
                            result: Box::new(ObjectDataValue::Literal(LiteralValue::Null)),
                        }
                    ),
                    attrs: vec![],
                    
                }),
                path: vec![
                    AnotherObjectValuePath::Ident(func_name.clone()),
                    AnotherObjectValuePath::FunctionCall(
                        FunctionCallResultValue {
                            args: vec![ObjectDataValue::Literal(LiteralValue::Null)],
                            result: Box::new(ObjectDataValue::Literal(LiteralValue::Null)),
                        }
                    ),
                ],
            }
        );

        assert_eq!(res, expected, "Function call with one arg parsed incorrectly");
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_with_spread_arg() {
        /*
            js code:
            const func = (arg1 = null, arg2 = null) => {};
            const obj = { arg1: null, arg2: null };
            func(...obj)
        */

        let func_name = "func".to_string();
        let obj_name = "obj".to_string();

        // Параметры функции: arg1 и arg2
        let arg1_obj = ObjectData {
            name: "arg1".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };
        let arg2_obj = ObjectData {
            name: "arg2".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        // Контекст с объявлением func и obj
        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: func_name.clone(),
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![arg1_obj.clone(), arg2_obj.clone()],
                            result: Box::new(ObjectData {
                                name: "result".to_string(),
                                value: ObjectDataValue::Literal(LiteralValue::Null),
                                attrs: vec![],
                                is_mutable: false,
                                
                            }),
                        }),
                        
                        attrs: vec![],
                    }
                ),
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: obj_name.clone(),
                        value: ObjectDataValue::Object(ObjectValue {
                            props: vec![
                                arg1_obj.clone(),
                                arg2_obj.clone(),
                            ],
                        }),
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Аргумент: ...obj
        let spread_arg = ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt(
                obj_name.clone().into(),
                DUMMY_SP,
            ))),
        };

        // Выражение: func(...obj)
        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![spread_arg],
            type_args: None,
        };



        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SpreadNotAllowed,
        );
    }
    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_spread_object_panic() {
        /*
            js code:
            const func = (result = null) => {};
            func(...{})
            // Should panic with ObjectValueCannotBeSpread
        */

        let func_name = "func".to_string();

        // Создаём функцию (просто чтобы был контекст)
        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: func_name.clone(),
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![result_obj.clone()],
                            result: Box::new(result_obj),
                        }),
                        attrs: vec![],
                        
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Аргумент: ...{} (spread пустого объекта)
        let spread_arg = ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![],
            })),
        };

        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![spread_arg],
            type_args: None,
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SpreadNotAllowed,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_spread_literal_panic() {
        /*
            js code:
            const func = (result = null) => {};
            func(...123)
            // Should panic with LiteralValueCannotBeSpread
        */

        let func_name = "func".to_string();

        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        
                        name: func_name.clone(),
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![result_obj.clone()],
                            result: Box::new(result_obj),
                        }),
                        attrs: vec![],
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let spread_arg = ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(Expr::Lit(Lit::Num(Number {
                span: DUMMY_SP,
                value: 123.0,
                raw: None,
            }))),
        };

        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![spread_arg],
            type_args: None,
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SpreadNotAllowed,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_spread_function_panic() {
        /*
            js code:
            const func = (result = null) => {};
            func(...(() => {}))
            // Should panic with FunctionValueCannotBeSpread
        */

        let func_name = "func".to_string();

        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: func_name.clone(),
                        
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![result_obj.clone()],
                            result: Box::new(result_obj),
                        }),
                        attrs: vec![],
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_fn = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Assign(AssignPat {
                    span: DUMMY_SP,
                    left: Box::new(Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("result".into(), DUMMY_SP),
                        type_ann: None,
                    })),
                    right: Box::new(Expr::Lit(Lit::Null(Null {
                        span: DUMMY_SP,
                    }))),
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        let spread_arg = ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(Expr::Arrow(arrow_fn)),
        };

        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![spread_arg],
            type_args: None,
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SpreadNotAllowed,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_call_function_spread_binary_panic() {
        /*
            js code:
            const func = (result = null) => {};
            func(...(null == null))
            // Should panic with BinaryValueCannotBeSpread
        */

        let func_name = "func".to_string();

        let result_obj = ObjectData {
            name: "result".to_string(),
            value: ObjectDataValue::Literal(LiteralValue::Null),
            attrs: vec![],
            is_mutable: false,
            
        };

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: func_name.clone(),
                        value: ObjectDataValue::Function(FunctionValue {
                            scope: Box::new(ScopeStatement { statements: vec![] }),
                            params: vec![result_obj.clone()],
                            result: Box::new(result_obj),
                        }),
                        
                        attrs: vec![],
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let bin_expr = BinExpr {
            span: DUMMY_SP,
            op: BinaryOp::EqEq,
            left: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
            right: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
        };

        let spread_arg = ExprOrSpread {
            spread: Some(DUMMY_SP),
            expr: Box::new(Expr::Bin(bin_expr)),
        };

        let call_expr = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                func_name.clone().into(),
                DUMMY_SP,
            )))),
            args: vec![spread_arg],
            type_args: None,
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Call(call_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SpreadNotAllowed,
        );
    }

    #[test]
    //noinspection DuplicatedCode
    fn test_parse_ident_success() {
        /*
            js code:
            const obj = { prop: null };
            obj
            // Should return AnotherObject with obj
        */

        let obj_name = "obj".to_string();

        let test_context = CurrentContext {
            prev: None,
            context_type: Default::default(),
            current_statements: vec![
                Statement::Object(
                    ObjectData {
                        is_mutable: false,
                        name: obj_name.clone(),
                        
                        value: ObjectDataValue::Object(ObjectValue {
                            props: vec![
                                ObjectData {
                                    is_mutable: false,
                                    name: "prop".to_string(),
                                    value: ObjectDataValue::Literal(LiteralValue::Null),
                                    attrs: vec![],
                                    
                                }
                            ],
                        }),
                        attrs: vec![],
                    }
                )
            ],
            current_module_name: "main".to_string(),
        };

        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ident_expr = Expr::Ident(Ident::new_no_ctxt(
            obj_name.clone().into(),
            DUMMY_SP,
        ));

        let res = parse_expr(
            &ident_expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let expected = ObjectDataValue::AnotherObject(
            AnotherObjectValue {
                obj: Box::new(ObjectData {
                    is_mutable: false,
                    name: obj_name.clone(),
                    
                    value: ObjectDataValue::Object(ObjectValue {
                        props: vec![
                            ObjectData {
                                is_mutable: false,
                                name: "prop".to_string(),
                                value: ObjectDataValue::Literal(LiteralValue::Null),
                                attrs: vec![],
                                
                            }
                        ],
                    }),
                    attrs: vec![],
                }),
                path: vec![AnotherObjectValuePath::Ident(obj_name.clone())],
            }
        );

        assert_eq!(res, expected, "Identifier was parsed incorrectly");
    }

    #[test]
    fn test_parse_ident_not_found_panic() {
        /*
            js code:
            unknown // but unknown is not declared
            // Should panic with ObjectNotFoundByName
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ident_expr = Expr::Ident(Ident::new_no_ctxt(
            "unknown".into(),
            DUMMY_SP,
        ));

        assert_compiler_panic(
            || {
                parse_expr(
                    &ident_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ObjectNotFoundByName,
        );
    }

    #[test]
    fn test_parse_paren_expr() {
        /*
            js code:
            (null)
            // Should return Literal(Null)
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let paren_expr = Expr::Paren(ParenExpr {
            span: DUMMY_SP,
            expr: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
        });

        let res = parse_expr(
            &paren_expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        let expected = ObjectDataValue::Literal(LiteralValue::Null);

        assert_eq!(res, expected, "Parenthesized expression was parsed incorrectly");
    }

    #[test]
    fn test_parse_arrow_expr_success() {
        /*
            js code:
            (result = null) => {}
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Assign(AssignPat {
                    span: DUMMY_SP,
                    left: Box::new(Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("result".into(), DUMMY_SP),
                        type_ann: None,
                    })),
                    right: Box::new(Expr::Lit(Lit::Null(Null {
                        span: DUMMY_SP,
                    }))),
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        let res = parse_expr(
            &Expr::Arrow(arrow_expr),
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        match res {
            ObjectDataValue::Function(func_val) => {
                assert_eq!(func_val.params.len(), 1);
                assert_eq!(func_val.params[0].name, "result");
                match &func_val.params[0].value {
                    ObjectDataValue::Literal(LiteralValue::Null) => {}
                    _ => panic!("Expected Literal(Null) as default value"),
                }
                match &*func_val.result {
                    ObjectData { name, value, .. } => {
                        assert_eq!(name, "result");
                        match value {
                            ObjectDataValue::Literal(LiteralValue::Null) => {}
                            _ => panic!("Expected Literal(Null) as result value"),
                        }
                    }
                }
            }
            _ => panic!("Expected FunctionValue"),
        }
    }

    #[test]
    fn test_parse_arrow_expr_no_result_panic() {
        /*
            js code:
            (x = null) => {}
            // Should panic with ResultNotFoundInParams
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Assign(AssignPat {
                    span: DUMMY_SP,
                    left: Box::new(Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    })),
                    right: Box::new(Expr::Lit(Lit::Null(Null {
                        span: DUMMY_SP,
                    }))),
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ResultNotFoundInParams,
        );
    }

    #[test]
    fn test_parse_arrow_expr_ident_pattern_panic() {
        /*
            js code:
            (x) => {}
            // Should panic with IdentPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Ident(BindingIdent {
                    id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                    type_ann: None,
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::IdentPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_arrow_expr_array_pattern_panic() {
        /*
            js code:
            ([]) => {}
            // Should panic with ArrayPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Array(ArrayPat {
                    span: DUMMY_SP,
                    elems: vec![],
                    optional: false,
                    type_ann: None,
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ArrayPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_arrow_expr_rest_pattern_panic() {
        /*
            js code:
            (...rest) => {}
            // Should panic with RestPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Rest(RestPat {
                    span: DUMMY_SP,
                    dot3_token: DUMMY_SP,
                    arg: Box::new(Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("rest".into(), DUMMY_SP),
                        type_ann: None,
                    })),
                    type_ann: None,
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::RestPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_arrow_expr_object_pattern_panic() {
        /*
            js code:
            ({}) => {}
            // Should panic with ObjectPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Object(ObjectPat {
                    span: DUMMY_SP,
                    props: vec![],
                    optional: false,
                    type_ann: None,
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ObjectPatternNotSupported,
        );
    }
    #[test]
    fn test_parse_arrow_expr_assign_with_object_left_panic() {
        /*
            js code:
            ({} = null) => {}
            // Should panic with ObjectPatternNotSupported (because left side is not Ident)
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Assign(AssignPat {
                    span: DUMMY_SP,
                    left: Box::new(Pat::Object(ObjectPat {
                        span: DUMMY_SP,
                        props: vec![],
                        optional: false,
                        type_ann: None,
                    })),
                    right: Box::new(Expr::Lit(Lit::Null(Null {
                        span: DUMMY_SP,
                    }))),
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ObjectPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_arrow_expr_invalid_pattern_panic() {
        /*
            js code:
            (invalid) => {}
            // Should panic with InvalidPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Invalid(Invalid {
                    span: DUMMY_SP,
                }),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::InvalidPatternNotSupported,
        );
    }

    #[test]
    fn test_parse_arrow_expr_expr_pattern_panic() {
        /*
            js code:
            (foo) => {}  // where foo is an expression (Pat::Expr wraps an Expr)
            // Should panic with ExpressionPatternNotSupported
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let arrow_expr = ArrowExpr {
            span: DUMMY_SP,
            params: vec![
                Pat::Expr(Box::new(Expr::Ident(Ident::new_no_ctxt(
                    "foo".into(),
                    DUMMY_SP,
                )))),
            ],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
            ..Default::default()
        };

        assert_compiler_panic(
            || {
                parse_expr(
                    &Expr::Arrow(arrow_expr),
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ExpressionPatternNotSupported,
        );
    }


    #[test]
    fn test_parse_expr_array_panic() {
        /*
            js code:
            []
            // Should panic with ArrayNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let array_expr = Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: vec![],
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &array_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ArrayNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_this_panic() {
        /*
            js code:
            this
            // Should panic with ThisNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let this_expr = Expr::This(ThisExpr { span: DUMMY_SP });

        assert_compiler_panic(
            || {
                parse_expr(
                    &this_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ThisNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_fn_panic() {
        /*
            js code:
            function() {}
            // Should panic with FnNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let fn_expr = Expr::Fn(FnExpr {
            ident: None,
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                }),
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &fn_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::FnNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_unary_panic() {
        /*
            js code:
            !null
            // Should panic with UnaryExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let unary_expr = Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::Bang,
            arg: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &unary_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::UnaryExpressionNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_update_panic() {
        /*
            js code:
            x++
            // Should panic with UpdateExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let update_expr = Expr::Update(UpdateExpr {
            span: DUMMY_SP,
            op: UpdateOp::PlusPlus,
            prefix: false,
            arg: Box::new(Expr::Ident(Ident::new_no_ctxt("x".into(), DUMMY_SP))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &update_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::UpdateExpressionNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_assign_panic() {
        /*
            js code:
            x = 1
            // Should panic with AssignExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let assign_expr = Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            left: AssignTarget::default(),
            op: AssignOp::Assign,
            right: Box::default(),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &assign_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::AssignExpressionNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_super_prop_panic() {
        /*
            js code:
            super.prop
            // Should panic with SuperPropNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let super_prop_expr = Expr::SuperProp(SuperPropExpr {
            span: DUMMY_SP,
            obj: Super { span: DUMMY_SP },
            prop: SuperProp::Ident(IdentName::new("prop".into(), DUMMY_SP)),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &super_prop_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SuperPropNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_new_panic() {
        /*
            js code:
            new Date()
            // Should panic with NewExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let new_expr = Expr::New(NewExpr {
            span: DUMMY_SP,
            ctxt: Default::default(),
            callee: Box::new(Expr::Ident(Ident::new_no_ctxt("Date".into(), DUMMY_SP))),
            args: Some(vec![]),
            type_args: None,
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &new_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::NewExpressionNotAllowed,
        );
    }


    #[test]
    fn test_parse_expr_seq_panic() {
        /*
            js code:
            1, 2
            // Should panic with SeqExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let seq_expr = Expr::Seq(SeqExpr {
            span: DUMMY_SP,
            exprs: vec![
                Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None }))),
                Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 2.0, raw: None }))),
            ],
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &seq_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::SeqExpressionNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_cond_panic() {
        /*
            js code:
            true ? 1 : 2
            // Should panic with CondExpressionNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let cond_expr = Expr::Cond(CondExpr {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Bool(Bool { span: DUMMY_SP, value: true }))),
            cons: Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None }))),
            alt: Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 2.0, raw: None }))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &cond_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::CondExpressionNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_tpl_panic() {
        /*
            js code:
            `hello`
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let tpl_expr = Expr::Tpl(Tpl {
            span: DUMMY_SP,
            quasis: vec![],
            exprs: vec![],
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &tpl_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_tagged_tpl_panic() {
        /*
            js code:
            tag`hello`
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let tagged_tpl_expr = Expr::TaggedTpl(TaggedTpl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            tag: Box::new(Expr::Ident(Ident::new_no_ctxt("tag".into(), DUMMY_SP))),
            type_params: None,
            tpl: Box::from(Tpl {
                span: DUMMY_SP,
                quasis: vec![],
                exprs: vec![],
            }),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &tagged_tpl_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_class_panic() {
        /*
            js code:
            class {}
            // Should panic with ClassNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let class_expr = Expr::Class(ClassExpr {
            ident: None,
            class: Default::default(),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &class_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::ClassNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_yield_panic() {
        /*
            js code:
            yield 1
            // Should panic with YieldAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let yield_expr = Expr::Yield(YieldExpr {
            span: DUMMY_SP,
            arg: Some(Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None })))),
            delegate: false,
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &yield_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::YieldAllowed,
        );
    }

    #[test]
    fn test_parse_expr_meta_prop_panic() {
        /*
            js code:
            import.meta
            // Should panic with MetaPropertyNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let meta_prop_expr = Expr::MetaProp(MetaPropExpr {
            span: DUMMY_SP,
            kind: MetaPropKind::NewTarget,
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &meta_prop_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::MetaPropertyNotAllowed,
        );
    }


    #[test]
    fn test_parse_expr_await_panic() {
        /*
            js code:
            await 1
            // Should panic with AwaitNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let await_expr = Expr::Await(AwaitExpr {
            span: DUMMY_SP,
            arg: Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None }))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &await_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::AwaitNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_ts_type_assertion_panic() {
        /*
            js code:
            <string>value
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_assertion_expr = Expr::TsTypeAssertion(TsTypeAssertion {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: DUMMY_SP,
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_assertion_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }


    #[test]
    fn test_parse_expr_ts_const_assertion_panic() {
        /*
            js code:
            value as const
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_const_assertion_expr = Expr::TsConstAssertion(TsConstAssertion {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_const_assertion_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_ts_non_null_panic() {
        /*
            js code:
            value!
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_non_null_expr = Expr::TsNonNull(TsNonNullExpr {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_non_null_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_ts_as_panic() {
        /*
            js code:
            value as string
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_as_expr = Expr::TsAs(TsAsExpr {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: DUMMY_SP,
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_as_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_ts_instantiation_panic() {
        /*
            js code:
            value<string>
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_instantiation_expr = Expr::TsInstantiation(TsInstantiation {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
            type_args: Box::new(TsTypeParamInstantiation {
                span: DUMMY_SP,
                params: vec![],
            }),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_instantiation_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_ts_satisfies_panic() {
        /*
            js code:
            value satisfies string
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let ts_satisfies_expr = Expr::TsSatisfies(TsSatisfiesExpr {
            span: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident::new_no_ctxt("value".into(), DUMMY_SP))),
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: DUMMY_SP,
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &ts_satisfies_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_private_name_panic() {
        /*
            js code:
            #private
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let private_name_expr = Expr::PrivateName(PrivateName {
            span: DUMMY_SP,
            name: Atom::from("private"),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &private_name_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_expr_opt_chain_panic() {
        /*
            js code:
            obj?.prop
            // Should panic with TsNotAllowed
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let opt_chain_expr = Expr::OptChain(OptChainExpr {
            span: DUMMY_SP,
            optional: false,
            base: Box::from(OptChainBase::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(Ident::new_no_ctxt("obj".into(), DUMMY_SP))),
                prop: MemberProp::Ident(IdentName::new("prop".into(), DUMMY_SP)),
            })),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &opt_chain_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::TsNotAllowed,
        );
    }
    #[test]
    fn test_parse_expr_invalid_panic() {
        /*
            js code:
            (invalid AST node)
            // Should panic with InvalidExpression
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let invalid_expr = Expr::Invalid(Invalid { span: DUMMY_SP });

        assert_compiler_panic(
            || {
                parse_expr(
                    &invalid_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::InvalidExpression,
        );
    }

    #[test]
    #[ignore = "JSX not implemented yet"]
    fn test_parse_expr_jsx_member_panic() {
        /*
            js code:
            <JSXMember />
            // Not implemented yet
        */
        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        let jsx_member_expr = Expr::JSXMember(JSXMemberExpr {
            span: DUMMY_SP,
            obj: JSXObject::Ident(Ident::new_no_ctxt("JSX".into(), DUMMY_SP)),
            prop: IdentName::new("Member".into(), DUMMY_SP),
        });

        assert_compiler_panic(
            || {
                parse_expr(
                    &jsx_member_expr,
                    &test_context,
                    &test_global_ctx,
                    &test_ast_context,
                );
            },
            ExprPanic::InvalidExpression, // временно, потому что todo!() паникует
        );
    }
}


#[cfg(test)]
mod parse_stmt_tests{
    use crate::unit_tests::assert_compiler_panic;
    use swc_common::{SyntaxContext, DUMMY_SP};
    use swc_ecma_ast::{BindingIdent, BlockStmt, Bool, BreakStmt, CatchClause, Class, ClassDecl, ContinueStmt, DebuggerStmt, Decl, DoWhileStmt, EmptyStmt, Expr, ExprStmt, FnDecl, ForHead, ForInStmt, ForOfStmt, ForStmt, Function, Ident, IfStmt, LabeledStmt, Lit, NewExpr, Null, Number, Pat, ReturnStmt, Stmt, ThrowStmt, TryStmt, TsEnumDecl, TsInterfaceBody, TsInterfaceDecl, TsKeywordType, TsKeywordTypeKind, TsModuleDecl, TsModuleName, TsType, TsTypeAliasDecl, UsingDecl, VarDecl, VarDeclKind, VarDeclarator, WhileStmt, WithStmt};
    use crate::scopy_ir::{parse_decl, parse_stmt, StmtPanic};
    use crate::semantic::{AstGlobalContext, CurrentContext, GlobalContext, LiteralValue, ObjectDataValue, Statement};

    #[test]
    fn test_parse_block_stmt() {
        /*
            js code:
            {
                const x = null;
            }
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Создаём AST для блока с одним стейтментом: const x = null;
        let decl_stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Const,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                }
            ],
            declare: false,
        })));

        let block_stmt = Stmt::Block(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: vec![decl_stmt],
        });

        let result = parse_stmt(
            &block_stmt,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        // Ожидаем: Statement::Scope со scope, содержащим один Statement::Object
        match result {
            Statement::Scope(scope) => {
                assert_eq!(scope.statements.len(), 1, "Scope should contain one statement");
                match &scope.statements[0] {
                    Statement::Object(obj_data) => {
                        assert_eq!(obj_data.name, "x");
                        assert_eq!(obj_data.value, ObjectDataValue::Literal(LiteralValue::Null));
                    }
                    _ => panic!("Expected Object statement inside block"),
                }
            }
            _ => panic!("Expected Scope statement"),
        }
    }

    #[test]
    fn test_parse_if_stmt() {
        /*
            js code:
            if (null) {
                const a = 1;
            } else {
                const b = 2;
            }
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Создаём тело if: { const a = 1; }
        let then_block = Stmt::Block(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: vec![
                Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    kind: VarDeclKind::Const,
                    decls: vec![
                        VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(BindingIdent {
                                id: Ident::new_no_ctxt("a".into(), DUMMY_SP),
                                type_ann: None,
                            }),
                            init: Some(Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None })))),
                            definite: false,
                        }
                    ],
                    declare: false,
                }))),
            ],
        });

        // Создаём тело else: { const b = 2; }
        let else_block = Stmt::Block(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: vec![
                Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    kind: VarDeclKind::Const,
                    decls: vec![
                        VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(BindingIdent {
                                id: Ident::new_no_ctxt("b".into(), DUMMY_SP),
                                type_ann: None,
                            }),
                            init: Some(Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 2.0, raw: None })))),
                            definite: false,
                        }
                    ],
                    declare: false,
                }))),
            ],
        });

        let if_stmt = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
            cons: Box::new(then_block),
            alt: Some(Box::new(else_block)),
        });

        let result = parse_stmt(
            &if_stmt,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        // Ожидаем: Statement::Conditional с правильной структурой
        match result {
            Statement::Conditional(cond) => {
                // Проверяем условие
                assert_eq!(cond.cond, ObjectDataValue::Literal(LiteralValue::Null));
                // Проверяем true-ветку (scope должен содержать объявление a)
                let scope = &*cond.true_scope;
                assert_eq!(scope.statements.len(), 1);
                match &scope.statements[0] {
                    Statement::Object(obj) => {
                        assert_eq!(obj.name, "a");
                        assert!(matches!(obj.value, ObjectDataValue::Literal(LiteralValue::Num(_))));
                    }
                    _ => panic!("Expected Object statement in true branch"),
                }
                // Проверяем false-ветку (scope должен содержать объявление b)

                match &cond.false_scope {
                    Some(false_scope) => {
                        let scope = &**false_scope;
                        assert_eq!(scope.statements.len(), 1);
                        match &scope.statements[0] {
                        Statement::Object(obj) => {
                            assert_eq!(obj.name, "b");
                            assert!(matches!(obj.value, ObjectDataValue::Literal(LiteralValue::Num(_))));
                        }
                        _ => panic!("Expected Object statement in false branch"),
                    }
                    }
                    None => panic!("Expected false branch to exist"),
                }
            }
            _ => panic!("Expected Conditional statement"),
        }
    }



    #[test]
    fn test_parse_while_stmt() {
        /*
            js code:
            while (null) {
                const x = null;
            }
        */

        let test_context = CurrentContext::default();
        let test_global_ctx = GlobalContext::default();
        let test_ast_context = AstGlobalContext::default();

        // Тело цикла: { const x = null; }
        let body_block = Stmt::Block(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: vec![
                Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    kind: VarDeclKind::Const,
                    decls: vec![
                        VarDeclarator {
                            span: DUMMY_SP,
                            name: Pat::Ident(BindingIdent {
                                id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                                type_ann: None,
                            }),
                            init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                            definite: false,
                        }
                    ],
                    declare: false,
                }))),
            ],
        });

        let while_stmt = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
            body: Box::new(body_block),
        });

        let result = parse_stmt(
            &while_stmt,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

        // Ожидаем: Statement::Loop с правильным условием и телом
        match result {
            Statement::Loop(loop_stmt) => {
                // Проверяем условие
                assert_eq!(loop_stmt.cond, ObjectDataValue::Literal(LiteralValue::Null));
                // Проверяем тело
                let scope = &*loop_stmt.loop_scope;
                assert_eq!(scope.statements.len(), 1);
                match &scope.statements[0] {
                    Statement::Object(obj) => {
                        assert_eq!(obj.name, "x");
                        assert_eq!(obj.value, ObjectDataValue::Literal(LiteralValue::Null));
                    }
                    _ => panic!("Expected Object statement in loop body"),
                }
            }
            _ => panic!("Expected Loop statement"),
        }
    }

    #[test]
    fn test_parse_stmt_while_object_not_scope_panic() {
        /*
            js code:
            while (null)
                const x = null;
        */

        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let body = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Const,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(
                        Expr::Lit(Lit::Null(Null {
                            span: DUMMY_SP,
                        }))
                    )),
                    definite: false,
                }
            ],
            declare: false,
        })));

        let stmt = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            body: Box::new(body),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ObjectNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_while_object_value_not_scope_panic() {
        /*
            js code:
            while (null)
                null;
        */

        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let body = Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(
                Expr::Lit(Lit::Null(Null {
                    span: DUMMY_SP,
                }))
            ),
        });

        let stmt = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            body: Box::new(body),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ObjectValueNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_while_conditional_not_scope_panic() {
        /*
            js code:
            while (null)
                if (true) {}
        */

        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let body = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(
                Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: true,
                }))
            ),
            cons: Box::new(
                Stmt::Block(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                })
            ),
            alt: None,
        });

        let stmt = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            body: Box::new(body),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ConditionalNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_while_loop_not_scope_panic() {
        /*
            js code:
            while (null)
                while (null) {}
        */

        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let inner_while = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            body: Box::new(
                Stmt::Block(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                })
            ),
        });

        let stmt = Stmt::While(WhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP,
            }))),
            body: Box::new(inner_while),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::LoopNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_if_object_not_scope_panic() {
        /*
            js code:
            if (true)
                const x = 1;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            }))),
            cons: Box::new(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Num(Number {
                        span: DUMMY_SP,
                        value: 1.0,
                        raw: None,
                    })))),
                    definite: false,
                }],
            })))),
            alt: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ObjectNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_if_object_value_not_scope_panic() {
        /*
            js code:
            if (true)
                true;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            }))),
            cons: Box::new(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: true,
                }))),
            })),
            alt: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ObjectValueNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_if_conditional_not_scope_panic() {
        /*
            js code:
            if (true)
                if (false) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            }))),
            cons: Box::new(Stmt::If(IfStmt {
                span: DUMMY_SP,
                test: Box::new(Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: false,
                }))),
                cons: Box::new(Stmt::Block(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                })),
                alt: None,
            })),
            alt: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ConditionalNotScope,
        );
    }

    #[test]
    fn test_parse_stmt_if_loop_not_scope_panic() {
        /*
            js code:
            if (true)
                while (true) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::If(IfStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            }))),
            cons: Box::new(Stmt::While(WhileStmt {
                span: DUMMY_SP,
                test: Box::new(Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value: true,
                }))),
                body: Box::new(Stmt::Block(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                })),
            })),
            alt: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::LoopNotScope,
        );
    }


    #[test]
    fn test_parse_stmt_empty_panic() {
        /*
            js code:
            ;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Empty(EmptyStmt { span: DUMMY_SP });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::EmptyStatementNotAllowed,
        );
    }


    #[test]
    fn test_parse_stmt_debugger_panic() {
        /*
            js code:
            debugger;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Debugger(DebuggerStmt { span: DUMMY_SP });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::DebuggerNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_with_panic() {
        /*
            js code:
            with (obj) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::With(WithStmt {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new_no_ctxt("obj".into(), DUMMY_SP))),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::WithNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_return_panic() {
        /*
            js code:
            return null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Return(ReturnStmt {
            span: DUMMY_SP,
            arg: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ReturnNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_labeled_panic() {
        /*
            js code:
            label: {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Labeled(LabeledStmt {
            span: DUMMY_SP,
            label: Ident::new_no_ctxt("label".into(), DUMMY_SP),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::LabeledNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_break_panic() {
        /*
            js code:
            break;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Break(BreakStmt {
            span: DUMMY_SP,
            label: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::BreakNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_continue_panic() {
        /*
            js code:
            continue;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Continue(ContinueStmt {
            span: DUMMY_SP,
            label: None,
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ContinueNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_throw_panic() {
        /*
            js code:
            throw new Error();
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Throw(ThrowStmt {
            span: DUMMY_SP,
            arg: Box::new(Expr::New(NewExpr {
                span: DUMMY_SP,
                ctxt: Default::default(),
                callee: Box::new(Expr::Ident(Ident::new_no_ctxt("Error".into(), DUMMY_SP))),
                args: Some(vec![]),
                type_args: None,
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ThrowNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_try_panic() {
        /*
            js code:
            try {} catch (e) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Try(Box::new(TryStmt {
            span: DUMMY_SP,
            block: BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            },
            handler: Some(CatchClause {
                span: DUMMY_SP,
                param: Some(Pat::Ident(BindingIdent {
                    id: Ident::new_no_ctxt("e".into(), DUMMY_SP),
                    type_ann: None,
                })),
                body: BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                },
            }),
            finalizer: None,
        }));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::TryNotAllowed,
        );
    }


    #[test]
    fn test_parse_stmt_do_while_panic() {
        /*
            js code:
            do {} while (x);
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::DoWhile(DoWhileStmt {
            span: DUMMY_SP,
            test: Box::new(Expr::Ident(Ident::new_no_ctxt("x".into(), DUMMY_SP))),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::DoWhileNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_for_panic() {
        /*
            js code:
            for (;;) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::For(ForStmt {
            span: DUMMY_SP,
            init: None,
            test: None,
            update: None,
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ForNotAllowed,
        );
    }


    #[test]
    fn test_parse_stmt_for_in_panic() {
        /*
            js code:
            for (var x in obj) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::ForIn(ForInStmt {
            span: DUMMY_SP,
            left: ForHead::VarDecl(Box::new(VarDecl {
                span: DUMMY_SP,
                ctxt: Default::default(),
                kind: VarDeclKind::Var,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: None,
                    definite: false,
                }],
                declare: false,
            })),
            right: Box::new(Expr::Ident(Ident::new_no_ctxt("obj".into(), DUMMY_SP))),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ForInNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_for_of_panic() {
        /*
            js code:
            for (var x of obj) {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::ForOf(ForOfStmt {
            span: DUMMY_SP,
            is_await: false,
            left: ForHead::VarDecl(Box::new(VarDecl {
                span: DUMMY_SP,
                ctxt: Default::default(),
                kind: VarDeclKind::Var,
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: None,
                    definite: false,
                }],
                declare: false,
            })),
            right: Box::new(Expr::Ident(Ident::new_no_ctxt("obj".into(), DUMMY_SP))),
            body: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![],
            })),
        });

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ForOfNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_class_decl_panic() {
        /*
            js code:
            class C {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::Class(ClassDecl {
            ident: Ident::new_no_ctxt("C".into(), DUMMY_SP),
            class: Box::new(Class {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                decorators: vec![],
                body: vec![],
                super_class: None,
                super_type_params: None,
                type_params: None,
                implements: vec![],
                is_abstract: false,
            }),
            declare: false,
        }));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::ClassDeclarationNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_fn_decl_panic() {
        /*
            js code:
            function f() {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::Fn(FnDecl {
            ident: Ident::new_no_ctxt("f".into(), DUMMY_SP),
            declare: false,
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![],
                }),
                is_generator: false,
                is_async: false,
                type_params: None,
                return_type: None,
            }),
        }));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::FunctionDeclarationNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_using_decl_panic() {
        /*
            js code:
            using x = null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::Using(Box::new(UsingDecl {
            span: DUMMY_SP,
            decls: vec![VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent {
                    id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                definite: false,
            }],
            is_await: false,
        })));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::UsingNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_ts_interface_panic() {
        /*
            js code:
            interface I {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::TsInterface(Box::new(TsInterfaceDecl {
            span: DUMMY_SP,
            id: Ident::new_no_ctxt("I".into(), DUMMY_SP),
            type_params: None,
            extends: vec![],
            body: TsInterfaceBody {
                span: DUMMY_SP,
                body: vec![],
            },
            declare: false,
        })));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_ts_type_alias_panic() {
        /*
            js code:
            type T = string;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::TsTypeAlias(Box::new(TsTypeAliasDecl {
            span: DUMMY_SP,
            id: Ident::new_no_ctxt("T".into(), DUMMY_SP),
            type_params: None,
            type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
                span: DUMMY_SP,
                kind: TsKeywordTypeKind::TsStringKeyword,
            })),
            declare: false,
        })));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_ts_enum_panic() {
        /*
            js code:
            enum E { A, B }
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::TsEnum(Box::new(TsEnumDecl {
            span: DUMMY_SP,
            id: Ident::new_no_ctxt("E".into(), DUMMY_SP),
            members: vec![],
            declare: false,
            is_const: false,
        })));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_ts_module_panic() {
        /*
            js code:
            module M {}
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Decl(Decl::TsModule(Box::new(TsModuleDecl {
            span: DUMMY_SP,
            id: TsModuleName::Ident(Ident::new_no_ctxt("M".into(), DUMMY_SP)),
            body: None,
            global: false,
            declare: false,
            namespace: false,
        })));

        assert_compiler_panic(
            || parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::TsNotAllowed,
        );
    }

    #[test]
    fn test_parse_stmt_expr_success() {
        /*
            js code:
            null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let stmt = Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
        });

        let result = parse_stmt(&stmt, &ctx, &glob_ctx, &ast_ctx);

        match result {
            Statement::ObjectValue(val) => {
                assert_eq!(val, ObjectDataValue::Literal(LiteralValue::Null));
            }
            _ => assert!(false, "Expected ObjectValue statement"),
        }
    }


    // ======== Тесты для parse_decl ========

    #[test]
    fn test_parse_decl_no_declarations_panic() {
        /*
            js code:
            const ; // невалидно
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let var_decl = VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Const,
            decls: vec![],
            declare: false,
        };

        assert_compiler_panic(
            || parse_decl(&var_decl, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::NoDeclarations,
        );
    }

    #[test]
    fn test_parse_decl_multiple_declarations_panic() {
        /*
            js code:
            const x = null, y = null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let var_decl = VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Const,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                },
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("y".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                },
            ],
            declare: false,
        };

        assert_compiler_panic(
            || parse_decl(&var_decl, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::MultipleDeclarationsNotAllowed,
        );
    }

    #[test]
    fn test_parse_decl_success_const() {
        /*
            js code:
            const x = null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let var_decl = VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Const,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                },
            ],
            declare: false,
        };

        let result = parse_decl(&var_decl, &ctx, &glob_ctx, &ast_ctx);

        // Проверяем поля ObjectData
        assert_eq!(result.name, "x");
        assert_eq!(result.is_mutable, false);
        assert!(result.attrs.is_empty());
        assert_eq!(result.value, ObjectDataValue::Literal(LiteralValue::Null));
    }

    #[test]
    fn test_parse_decl_var_panic() {
        /*
            js code:
            var x = null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let var_decl = VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Var,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                },
            ],
            declare: false,
        };

        assert_compiler_panic(
            || parse_decl(&var_decl, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::VarNotAllowed,
        );
    }

    #[test]
    fn test_parse_decl_let_panic() {
        /*
            js code:
            let x = null;
        */
        let ctx = CurrentContext::default();
        let glob_ctx = GlobalContext::default();
        let ast_ctx = AstGlobalContext::default();

        let var_decl = VarDecl {
            span: DUMMY_SP,
            ctxt: Default::default(),
            kind: VarDeclKind::Let,
            decls: vec![
                VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
                        type_ann: None,
                    }),
                    init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
                    definite: false,
                },
            ],
            declare: false,
        };

        assert_compiler_panic(
            || parse_decl(&var_decl, &ctx, &glob_ctx, &ast_ctx),
            StmtPanic::LetNotAllowed,
        );
    }
}




// #[cfg(test)]
// mod parse_import_export{
//     // ======== Test on modules ========
//
//
//     // ======== Тесты на ошибки модульных деклараций ========
//
//     use std::collections::HashMap;
//     use swc_common::{SyntaxContext, DUMMY_SP};
//     use swc_ecma_ast::{BindingIdent, BlockStmt, Class, ClassDecl, ClassExpr, Decl, DefaultDecl, ExportAll, ExportDecl, ExportDefaultDecl, ExportDefaultExpr, Expr, ExprStmt, FnDecl, Function, Ident, ImportDecl, ImportNamedSpecifier, ImportPhase, ImportSpecifier, Lit, ModuleDecl, ModuleItem, Null, Number, Pat, Stmt, Str, TsEnumDecl, TsExportAssignment, TsExternalModuleRef, TsImportEqualsDecl, TsInterfaceBody, TsInterfaceDecl, TsKeywordType, TsKeywordTypeKind, TsModuleDecl, TsModuleName, TsModuleRef, TsNamespaceExportDecl, TsType, TsTypeAliasDecl, UsingDecl, VarDeclarator};
//     use crate::scopy_ir::{parse_module_item, ModuleDeclPanic};
//     use crate::semantic::{AstGlobalContext, CurrentContext, CurrentContextType, GlobalContext, LiteralValue, ModuleContext, ObjectData, ObjectDataValue, ObjectKind, ObjectValue, ScopyModule, Statement};
//     use crate::unit_tests::assert_compiler_panic;
//
//     #[test]
//     fn test_parse_module_decl_export_default_decl_panic() {
//         /*
//             js code:
//             export default class {}
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
//             span: DUMMY_SP,
//             decl: DefaultDecl::Class(ClassExpr {
//                 ident: None,
//                 class: Box::new(Class {
//                     span: DUMMY_SP,
//                     ctxt: SyntaxContext::empty(),
//                     decorators: vec![],
//                     body: vec![],
//                     super_class: None,
//                     super_type_params: None,
//                     type_params: None,
//                     implements: vec![],
//                     is_abstract: false,
//                 }),
//             }),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::DefaultExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_decl_export_default_expr_panic() {
//         /*
//             js code:
//             export default 1;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
//             span: DUMMY_SP,
//             expr: Box::new(Expr::Lit(Lit::Num(Number { span: DUMMY_SP, value: 1.0, raw: None }))),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::DefaultExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_decl_export_all_panic() {
//         /*
//             js code:
//             export * from "mod";
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportAll(ExportAll {
//             span: DUMMY_SP,
//             src: Box::new(Str { span: DUMMY_SP, value: "mod".into(), raw: None }),
//             type_only: false,
//             with: None,
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::ExportAllNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_decl_ts_import_equals_panic() {
//         /*
//             js code:
//             import x = require("mod");
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::TsImportEquals(Box::new(
//             TsImportEqualsDecl {
//                 span: DUMMY_SP,
//                 id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
//                 module_ref: TsModuleRef::TsExternalModuleRef(TsExternalModuleRef {
//                     span: DUMMY_SP,
//                     expr: Str { span: DUMMY_SP, value: "mod".into(), raw: None },
//                 }),
//                 is_export: false,
//                 is_type_only: false,
//             }
//         )));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsImportEqualsNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_decl_ts_export_assignment_panic() {
//         /*
//             js code:
//             export = x;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::TsExportAssignment(TsExportAssignment {
//             span: DUMMY_SP,
//             expr: Box::new(Expr::Ident(Ident::new_no_ctxt("x".into(), DUMMY_SP))),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsExportAssignmentNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_decl_ts_namespace_export_panic() {
//         /*
//             js code:
//             export as namespace N;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::TsNamespaceExport(TsNamespaceExportDecl {
//             span: DUMMY_SP,
//             id: Ident::new_no_ctxt("N".into(), DUMMY_SP),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsNamespaceExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_stmt_success() {
//         /*
//             js code (module-level):
//             null;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
//             span: DUMMY_SP,
//             expr: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
//         }));
//
//         let result = parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx);
//
//         match result {
//             Statement::ObjectValue(val) => {
//                 assert_eq!(val, ObjectDataValue::Literal(LiteralValue::Null));
//             }
//             _ => assert!(false, "Expected ObjectValue statement from module item"),
//         }
//     }
//
//
//     // ======== Тесты для export declaration через parse_module_item ========
//
//     #[test]
//     fn test_parse_module_item_export_class_panic() {
//         /*
//             js code:
//             export class C {}
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::Class(ClassDecl {
//                 ident: Ident::new_no_ctxt("C".into(), DUMMY_SP),
//                 class: Box::from(Class {
//                     span: DUMMY_SP,
//                     ctxt: SyntaxContext::empty(),
//                     decorators: vec![],
//                     body: vec![],
//                     super_class: None,
//                     super_type_params: None,
//                     type_params: None,
//                     implements: vec![],
//                     is_abstract: false,
//                 }),
//                 declare: false,
//             }),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::ClassExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_using_panic() {
//         /*
//             js code:
//             export using x = null;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::Using(Box::from(UsingDecl {
//                 span: DUMMY_SP,
//                 decls: vec![VarDeclarator {
//                     span: DUMMY_SP,
//                     name: Pat::Ident(BindingIdent {
//                         id: Ident::new_no_ctxt("x".into(), DUMMY_SP),
//                         type_ann: None,
//                     }),
//                     init: Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
//                     definite: false,
//                 }],
//                 is_await: false,
//             })),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::UsingExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_fn_panic() {
//         /*
//             js code:
//             export function f() {}
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::Fn(FnDecl {
//                 ident: Ident::new_no_ctxt("f".into(), DUMMY_SP),
//                 declare: false,
//                 function: Box::new(Function {
//                     params: vec![],
//                     decorators: vec![],
//                     span: DUMMY_SP,
//                     ctxt: SyntaxContext::empty(),
//                     body: Some(BlockStmt {
//                         span: DUMMY_SP,
//                         ctxt: SyntaxContext::empty(),
//                         stmts: vec![],
//                     }),
//                     is_generator: false,
//                     is_async: false,
//                     type_params: None,
//                     return_type: None,
//                 }),
//             }),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::FnExportNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_ts_interface_panic() {
//         /*
//             js code:
//             export interface I {}
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::TsInterface(Box::new(
//                 TsInterfaceDecl {
//                     span: DUMMY_SP,
//                     id: Ident::new_no_ctxt("I".into(), DUMMY_SP),
//                     type_params: None,
//                     extends: vec![],
//                     body: TsInterfaceBody {
//                         span: DUMMY_SP,
//                         body: vec![],
//                     },
//                     declare: false,
//                 }
//             )),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_ts_type_alias_panic() {
//         /*
//             js code:
//             export type T = string;
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::TsTypeAlias(Box::new(TsTypeAliasDecl {
//                 span: DUMMY_SP,
//                 id: Ident::new_no_ctxt("T".into(), DUMMY_SP),
//                 type_params: None,
//                 type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
//                     span: DUMMY_SP,
//                     kind: TsKeywordTypeKind::TsStringKeyword,
//                 })),
//                 declare: false,
//             })),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_ts_enum_panic() {
//         /*
//             js code:
//             export enum E { A, B }
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::TsEnum(Box::new(TsEnumDecl {
//                 span: DUMMY_SP,
//                 id: Ident::new_no_ctxt("E".into(), DUMMY_SP),
//                 members: vec![],
//                 declare: false,
//                 is_const: false,
//             })),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_export_ts_module_panic() {
//         /*
//             js code:
//             export module M {}
//         */
//         let ctx = CurrentContext::default();
//         let glob_ctx = GlobalContext::default();
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
//             span: DUMMY_SP,
//             decl: Decl::TsModule(Box::new(
//                 TsModuleDecl {
//                     span: DUMMY_SP,
//                     id: TsModuleName::Ident(Ident::new_no_ctxt("M".into(), DUMMY_SP)),
//                     body: None,
//                     global: false,
//                     declare: false,
//                     namespace: false,
//                 }
//             )),
//         }));
//
//         assert_compiler_panic(
//             || parse_module_item(&module_item, &ctx, &glob_ctx, &ast_ctx),
//             ModuleDeclPanic::TsNotAllowed,
//         );
//     }
//
//     #[test]
//     fn test_parse_module_item_import_success() {
//         /*
//             js code:
//             import {string} from "default.js";
//         */
//
//         let ctx = CurrentContext {
//             context_type: CurrentContextType::ModuleContext(ModuleContext {
//                 name: "main.js".to_string(),
//             }),
//             current_module_name: "main.js".to_string(),
//             prev: None,
//             current_statements: vec![],
//         };
//
//         let default_module = ScopyModule {
//             name: "default.js".to_string(),
//             statements: vec![
//                 Statement::Object(ObjectData {
//                     name: "string".to_string(),
//                     is_mutable: false,
//                     attrs: vec![],
//                     value: ObjectDataValue::Object(ObjectValue {
//                         props: vec![],
//                     }),
//                     kind: ObjectKind::Inner
//                 }),
//             ],
//         };
//
//         let glob_ctx = GlobalContext {
//             parsed_modules: HashMap::from([
//                 ("default.js".to_string(), default_module),
//             ]),
//         };
//
//         let ast_ctx = AstGlobalContext::default();
//
//         let module_item = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
//             span: DUMMY_SP,
//             specifiers: vec![
//                 ImportSpecifier::Named(ImportNamedSpecifier {
//                     span: DUMMY_SP,
//                     local: Ident::new_no_ctxt("string".into(), DUMMY_SP),
//                     imported: None,
//                     is_type_only: false,
//                 }),
//             ],
//             src: Box::new(Str {
//                 span: DUMMY_SP,
//                 value: "default.js".into(),
//                 raw: None,
//             }),
//             type_only: false,
//             with: None,
//             phase: ImportPhase::Evaluation,
//         }));
//
//         let result = parse_module_item(
//             &module_item,
//             &ctx,
//             &glob_ctx,
//             &ast_ctx,
//         );
//
//         match result {
//             Statement::Object(import_obj) => {
//                 assert_eq!(import_obj.name, "default.js");
//                 assert_eq!(import_obj.is_mutable, false);
//                 assert!(import_obj.attrs.is_empty());
//
//                 match import_obj.value {
//                     ObjectDataValue::Object(object) => {
//                         assert_eq!(object.props.len(), 1);
//
//                         let imported = &object.props[0];
//
//                         assert_eq!(imported.name, "string");
//                         assert_eq!(imported.is_mutable, false);
//                         assert_eq!(imported.attrs, vec![]);
//
//                         assert_eq!(
//                             imported.value,
//                             ObjectDataValue::Object(ObjectValue {
//                                 props: vec![],
//                             })
//                         );
//                     }
//
//                     _ => assert!(false, "Expected Object value"),
//                 }
//             }
//
//             _ => assert!(false, "Expected imported module object, got {:?}", result),
//         }
//     }
//
//
//
//
// }