use quote::quote;
use syn::{Expr, ItemFn};

use crate::Model;

pub fn lower(model: Model) -> Ir {
    let Model {
        preconditions,
        item,
    } = model;

    let assertions = preconditions
        .into_iter()
        .map(|expr| Assertion {
            message: format!("violation of precondition `{}`", quote!(#expr)),
            expr,
        })
        .collect();

    Ir { assertions, item }
}

pub struct Ir {
    pub assertions: Vec<Assertion>,
    pub item: ItemFn,
}

pub struct Assertion {
    pub expr: Expr,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    impl Model {
        fn stub() -> Self {
            Self {
                preconditions: vec![],
                item: parse_quote!(
                    fn f() {}
                ),
            }
        }
    }

    #[test]
    fn produces_assertion_for_precondition() {
        let mut model = Model::stub();
        model.preconditions.push(parse_quote!(x));

        let ir = lower(model);

        assert_eq!(1, ir.assertions.len());

        let assertion = &ir.assertions[0];
        let expected: Expr = parse_quote!(x);
        assert_eq!(expected, assertion.expr);
        assert_eq!("violation of precondition `x`", assertion.message);
    }
}
