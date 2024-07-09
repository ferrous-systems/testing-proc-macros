pub(crate) fn message(msg: &str, help: &str) -> String {
    format!("{msg}\n\n  = help: {help}\n\n")
}
