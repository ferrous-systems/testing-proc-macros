use proc_macro2::Span;
use quote::ToTokens;

pub(crate) fn abort<T>(span: Span, msg: &str, help: &str) -> syn::Result<T> {
    Err(syn::Error::new(span, message(msg, help)))
}

pub(crate) fn abort_call_side<T>(msg: &str, help: &str) -> syn::Result<T> {
    abort(Span::call_site(), msg, help)
}

pub(crate) fn abort_spanned<T>(tokens: impl ToTokens, msg: &str, help: &str) -> syn::Result<T> {
    Err(syn::Error::new_spanned(tokens, message(msg, help)))
}

fn message(msg: &str, help: &str) -> String {
    format!("{msg}\n\n  = help: {help}\n\n")
}
