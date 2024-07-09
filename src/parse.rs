use proc_macro2::{Span, TokenStream};
use syn::{spanned::Spanned, Expr, Item, ItemFn};

use crate::error;

pub type Ast = ItemFn;

pub fn parse(args: TokenStream, item: TokenStream) -> syn::Result<Ast> {
    const ERROR: &str = "this attribute takes no arguments";
    const HELP: &str = "use `#[contracts]`";

    if !args.is_empty() {
        let span = if let Ok(expr) = syn::parse2::<Expr>(args) {
            // ../tests/ui/has-expr-argument.rs
            expr.span()
        } else {
            // ../tests/ui/has-arguments.rs
            Span::call_site()
        };
        return Err(syn::Error::new(span, error::message(ERROR, HELP)));
    }

    match syn::parse2::<Item>(item) {
        Ok(Item::Fn(item)) => Ok(item),
        Ok(item) => {
            // ../tests/ui/item-is-not-a-function.rs
            Err(syn::Error::new_spanned(
                item,
                error::message(
                    "item is not a function",
                    "`#[contracts]` can only be used on functions",
                ),
            ))
        }
        Err(_) => unreachable!(), // ?
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn valid_syntax() {
        parse(
            quote!(),
            quote!(
                #[inline]
                #[precondition(x % 2 == 0)]
                fn even_to_odd(x: u32) -> u32 {
                    x + 1
                }
            ),
        )
        .unwrap();
    }
}
