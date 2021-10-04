use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use analyze::{analyze, Model};
use codegen::codegen;
use lower::{lower, Ir};
use parse::{parse, Ast};

mod analyze;
mod codegen;
mod lower;
mod parse;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn contracts(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse(args.into(), item.into());
    let model = analyze(ast);
    let ir = lower(model);
    let rust = codegen(ir);
    rust.into()
}
