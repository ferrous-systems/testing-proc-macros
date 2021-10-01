use proc_macro_error::{abort, abort_call_site};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, ItemFn,
};

use crate::Ast;

pub fn analyze(ast: Ast) -> Model {
    let mut preconditions = vec![];

    let mut item = ast;
    let attrs = &mut item.attrs;
    for index in (0..attrs.len()).rev() {
        if let Some(ident) = attrs[index].path.get_ident() {
            if ident.to_string().as_str() == "precondition" {
                let attr = attrs.remove(index);
                let span = attr.tokens.span();

                if let Ok(arg) = syn::parse2::<AttributeArgument>(attr.tokens) {
                    preconditions.push(arg.expr);
                } else {
                    // ../tests/ui/precondition-is-not-an-expression.rs
                    abort!(
                        span,
                        "expected an expression as argument";
                        help = "example syntax: `#[precondition(argument % 2 == 0)]`")
                }
            }
        }
    }

    if preconditions.is_empty() {
        // ../tests/ui/zero-contracts.rs
        abort_call_site!(
            "no contracts were specified";
            help = "add a `#[precondition]`"
        )
    }

    Model {
        preconditions,
        item,
    }
}

struct AttributeArgument {
    expr: Expr,
}

impl Parse for AttributeArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _parenthesis = parenthesized!(content in input);

        Ok(AttributeArgument {
            expr: content.parse()?,
        })
    }
}

pub struct Model {
    pub preconditions: Vec<Expr>,
    pub item: ItemFn,
}

impl Model {
    #[cfg(test)]
    pub fn stub() -> Self {
        use syn::parse_quote;

        Self {
            preconditions: vec![],
            item: parse_quote!(
                fn f() {}
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn can_extract_precondition() {
        let model = analyze(parse_quote!(
            #[precondition(x)]
            fn f() {}
        ));

        assert_eq!(1, model.preconditions.len());
        let expr = &model.preconditions[0];
        assert_eq!("x", quote!(#expr).to_string());
    }

    #[test]
    fn non_dsl_attributes_are_kept() {
        let model = analyze(parse_quote!(
            #[a]
            #[precondition(x)]
            #[b]
            fn f() {}
        ));

        assert_eq!(2, model.item.attrs.len());
        let first = &model.item.attrs[0].path;
        assert_eq!("a", quote!(#first).to_string());

        let second = &model.item.attrs[1].path;
        assert_eq!("b", quote!(#second).to_string());
    }
}
