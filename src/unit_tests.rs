
#[cfg(test)]
mod get_attributes_tests {
    use crate::test_unit_helpers::{create_test_comment_with_attribute, create_test_fake_span};
    use crate::{get_attributes, AstGlobalContext};
    use swc_common::comments::Comments;


    //TODO: make others use cases with attributes

    #[test]
    fn test_get_attributes(){
        let attribute = r#"@testAttribute("test", test2)"#.to_string();
        let ctx = create_global_ctx_with_comments(attribute);
        let span = create_test_fake_span();
        let attributes = get_attributes(&span, &ctx);
        let res = attributes.iter().find(|attr|{attr.name == "testAttribute"});
        assert!(res.is_some(), "Cannot find attribute 'testAttribute'");
    }

    fn create_global_ctx_with_comments(attributes: String) -> AstGlobalContext{
        let ast_global_ctx = AstGlobalContext::default();

        let comment_test = create_test_comment_with_attribute(attributes);
        ast_global_ctx.comments.add_leading(create_test_fake_span().lo, comment_test);


        ast_global_ctx

    }
}


#[cfg(test)]
mod parse_pat_tests {
    use crate::parse_pat_as_ident;
    use crate::test_unit_helpers::create_test_pat_as_ident;


    #[test]
    fn test_parse_pat_as_ident(){

        let test_name = "test_name".to_string();

        let pat = create_test_pat_as_ident(test_name.clone());

        let name = parse_pat_as_ident(&pat);

        assert_eq!(name, test_name.clone(), "wrong name of pat ident");
    }
}


#[cfg(test)]
mod parse_ident_tests {
    use crate::parse_ident;
    use crate::test_unit_helpers::create_test_ident;

    #[test]
    fn test_parse_ident_as_ident(){

        let test_name = "test_name".to_string();

        let ident = create_test_ident(test_name.clone());

        let name = parse_ident(&ident);

        assert_eq!(name, test_name.clone(), "cannot read test_name in ident");
    }

}


#[cfg(test)]
mod parse_expr_tests {
    use swc_atoms::Wtf8Atom;
    use crate::test_unit_helpers::{create_test_ast_context, create_test_current_context, create_test_fake_span, create_test_global_context, create_test_object_lit, create_test_prop};
    use crate::{parse_expr, LiteralValue, ObjectDataValue};
    use swc_ecma_ast::{Expr, Lit, Str};

    #[test]
    fn parse_object_lit_test(){
        let prop_name = "test1".to_string();
        let ident_prop_val = "test2".to_string();

        let expr_obj_lit_props = create_test_object_lit_with_props_of_idents(prop_name.clone(), ident_prop_val.clone());

        let test_context = create_test_current_context();
        let test_global_ctx = create_test_global_context();
        let test_ast_context = create_test_ast_context();

        let prop_res = parse_expr(&expr_obj_lit_props, &test_context, &test_global_ctx, &test_ast_context);


        match prop_res
        {
            ObjectDataValue::Object(obj_val) => {
                let el = obj_val.props.iter().find(|p|  {
                    if p.name == prop_name{
                        match &p.value
                        {
                            ObjectDataValue::Literal(lit_val) => {
                                match lit_val {
                                    LiteralValue::Str(val) => {
                                        val.val == ident_prop_val
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    }
                    else {
                        false
                    }
                });
                assert!(el.is_some(), "Cannot find object with prop_name {} with val {}", prop_name, ident_prop_val);
            },
            _ => {
                assert!(false, "Failed to parse object literal. it's not an Object in ObjectDataValue");
            }
        }
    }




    fn create_test_object_lit_with_props_of_idents(prop_name: String, value: String) -> Expr{
        create_test_object_lit(vec![
            create_test_prop(prop_name, Expr::Lit(Lit::Str(Str{
                span: create_test_fake_span(),
                value: Wtf8Atom::from(value),
                raw: None,
            }))),
        ])
    }





}


