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

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Attribute};

    use super::*;

    #[test]
    fn can_extract_precondition() {
        let model = analyze(parse_quote!(
            #[precondition(x)]
            fn f(x: bool) {}
        ));

        let expected: &[Expr] = &[parse_quote!(x)];
        assert_eq!(expected, model.preconditions);

        assert!(model.item.attrs.is_empty());
    }

    #[test]
    fn non_dsl_attributes_are_preserved() {
        let model = analyze(parse_quote!(
            #[a]
            #[precondition(x)]
            #[b]
            fn f(x: bool) {}
        ));

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];
        assert_eq!(expected, model.item.attrs);
    }
}
