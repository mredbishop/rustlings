// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut vector = fill_vec();

    assert_eq!(vector, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    let mut vector = vec![22, 44, 66];

    vector.push(88);

    vector
}