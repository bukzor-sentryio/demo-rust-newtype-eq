use demo_rust_newtype_eq::{New, Old};

fn main() {
    let new = New::Bool(true);
    let old = Old::Bool(true);

    println!("{}", new == old);
    println!("{}", old == new);
}
