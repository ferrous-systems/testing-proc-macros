use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemFn, Stmt};

use crate::{lower::Assertion, Ir};

pub type Rust = TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir { assertions, item } = ir;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;

    quote!(
        #(#attrs)*
        #vis #sig {
            #(#assertions)*
            #block
        }
    )
}

impl ToTokens for Assertion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Assertion { expr, message } = self;
        let stmt: Stmt = parse_quote!(assert!(#expr, #message););
        stmt.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_function_item() {
        let ir = Ir {
            assertions: vec![Assertion {
                expr: parse_quote!(x),
                message: "message".to_string(),
            }],
            item: parse_quote!(
                fn f() {}
            ),
        };
        let rust = codegen(ir);

        assert!(syn::parse2::<ItemFn>(rust).is_ok());
    }
}
