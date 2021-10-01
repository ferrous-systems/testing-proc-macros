#[contracts::contracts]
#[precondition(input)]
fn f(input: bool) {}

#[test]
fn pass() {
    f(true)
}

#[test]
#[should_panic = "violation of precondition `input`"]
fn fail() {
    f(false)
}
