use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item, ItemFn};

pub type Ast = ItemFn;

pub fn parse(args: TokenStream, item: TokenStream) -> Ast {
    const ERROR: &str = "this attribute takes no arguments";
    const HELP: &str = "use `#[contracts]`";

    if !args.is_empty() {
        if let Ok(expr) = syn::parse2::<Expr>(args) {
            // ../tests/ui/has-expr-argument.rs
            abort!(expr, ERROR; help = HELP)
        } else {
            // ../tests/ui/has-arguments.rs
            abort_call_site!(ERROR; help = HELP)
        }
    }

    match syn::parse2::<Item>(item) {
        Ok(Item::Fn(item)) => item,
        Ok(item) => {
            // ../tests/ui/item-is-not-a-function.rs
            abort!(
                item,
                "item is not a function";
                help = "`#[contracts]` can only be used on functions"
            )
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
        );
    }
}
