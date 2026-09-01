use crate::{ExprPanic};



use std::panic::{catch_unwind, AssertUnwindSafe};
#[allow(unused)]
fn assert_compiler_panic<F, R>(f: F, expected: ExprPanic)
where
    F: FnOnce() -> R, R: std::fmt::Debug
{
    let result = catch_unwind(AssertUnwindSafe(f));

    let panic = result.expect_err("Expected compiler panic");

    let reason = panic
        .downcast_ref::<ExprPanic>()
        .expect("Expected ExprPanic");

    assert_eq!(reason, &expected);
}

#[cfg(test)]
mod get_attributes_tests {
    use swc_atoms::Atom;
    use crate::{get_attributes, parse_attributes, parse_attributes_from_comments, AstGlobalContext};
    use swc_common::comments::{Comment, CommentKind, Comments};
    use swc_common::DUMMY_SP;


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
    use swc_atoms::Atom;
    use swc_common::{SyntaxContext, DUMMY_SP};
    use swc_ecma_ast::{ArrayPat, AssignPat, Expr, Ident, Invalid, Lit, Null, ObjectPat, RestPat};
    use crate::{parse_ident, ExprPanic};
    use swc_ecma_ast::{BindingIdent, Pat};
    use crate::parse_pat_as_ident;
    use crate::unit_tests::assert_compiler_panic;

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

        let name = parse_pat_as_ident(&pat);

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


        assert_compiler_panic(|| parse_pat_as_ident(&pat), ExprPanic::ArrayPatternNotSupported);
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
            || parse_pat_as_ident(&pat),
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
            || parse_pat_as_ident(&pat),
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
            || parse_pat_as_ident(&pat),
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
            || parse_pat_as_ident(&pat),
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
            || parse_pat_as_ident(&pat),
            ExprPanic::ExpressionPatternNotSupported,
        );
    }


}


#[cfg(test)]
mod parse_expr_tests {
    use swc_atoms::{Atom, Wtf8Atom};
    use swc_common::{SyntaxContext, DUMMY_SP};
    use crate::{parse_expr, AnotherObjectValue, AnotherObjectValuePath, AstGlobalContext, BinaryOpType, CurrentContext, ExprPanic, FunctionValue, GlobalContext, LitValueBool, LitValueNum, LitValueString, LiteralValue, ObjectData, ObjectDataValue, ObjectValue, Scope, Statement};
    use swc_ecma_ast::{ArrowExpr, AssignPat, AssignProp, BigInt, BinExpr, BinaryOp, BindingIdent, BlockStmt, BlockStmtOrExpr, Bool, CallExpr, Callee, ComputedPropName, Expr, Function, GetterProp, Ident, IdentName, Import, KeyValueProp, Lit, MemberExpr, MemberProp, MethodProp, Null, Number, ObjectLit, ParenExpr, Pat, PrivateName, Prop, PropName, PropOrSpread, Regex, SetterProp, SpreadElement, Str, Super};
    use crate::unit_tests::assert_compiler_panic;

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
                    ObjectData{
                        value: ObjectDataValue::Object(ObjectValue{
                            props: vec![
                                ObjectData{
                                    is_mutable: false,
                                    name: prop_name.clone(),
                                    attrs: vec![],
                                    value: ObjectDataValue::Literal(LiteralValue::Null)
                                }
                            ]
                        }),
                        name: ident_name.clone(),
                        is_mutable: false,
                        attrs: vec![]
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


        let obj_parsed_value = parse_expr(&obj_lit, &test_context, &test_global_ctx, &test_ast_context);



        if let ObjectDataValue::Object(obj_parsed) = obj_parsed_value {
            let d = obj_parsed.props.iter().find(|p| p.name == prop_name.clone()).is_some();
            assert!(d, "There is no value for {:?}", prop_name)
        } else {
            assert!(false, "Wrong object value")
        }
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
            ExprPanic::ObjectValueCannotBeSpread,
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
            ExprPanic::FunctionValueCannotBeSpread,
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
            ExprPanic::BinaryValueCannotBeSpread,
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
                    ObjectData{
                        is_mutable: false,
                        name: obj_name.clone().to_string(),
                        value: ObjectDataValue::Object(ObjectValue{
                            props: vec![
                                ObjectData{
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

        // assert!(false, "{}", format!("{:?}", res));

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
                Statement::Object(ObjectData{
                    is_mutable: false,
                    name: func_name.clone().into(),
                    value: ObjectDataValue::Function(FunctionValue{
                        scope: Box::new(Scope{
                            statements: vec![],
                        }),
                        params: vec![],
                        result: Box::new(ObjectData {
                            is_mutable: false,
                            name: "result".to_string(),
                            value: ObjectDataValue::Object(ObjectValue{
                                props: vec![
                                    ObjectData{
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


        let _ = parse_expr(
            &expr,
            &test_context,
            &test_global_ctx,
            &test_ast_context,
        );

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
                Statement::Object(ObjectData{
                    is_mutable: false,
                    name: func_name.clone(),
                    value: ObjectDataValue::Function(FunctionValue{
                        scope: Box::new(Scope {
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
}





