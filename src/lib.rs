use proc_macro::TokenStream;

use analyze::{analyze, Model};
use codegen::codegen;
use lower::{lower, Ir};
use parse::{parse, Ast};

mod analyze;
mod codegen;
mod error;
mod lower;
mod parse;

#[proc_macro_attribute]
pub fn contracts(args: TokenStream, item: TokenStream) -> TokenStream {
    main(args, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn main(args: TokenStream, item: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ast = parse(args.into(), item.into())?;
    let model = analyze(ast)?;
    let ir = lower(model);
    let rust = codegen(ir);
    Ok(rust)
}
