#[contracts::contracts]
#[precondition(input)]
fn f(input: bool) -> i32 {
    0
}

#[test]
fn pass() {
    f(true);
}

#[test]
#[should_panic = "violation of precondition `input`"]
fn fail() {
    f(false);
}
