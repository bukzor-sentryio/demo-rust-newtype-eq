```console
$ cargo run
   Compiling demo-rust-newtype-eq v0.1.0 (./demo-rust-newtype-eq)
error[E0369]: binary operation `==` cannot be applied to type `demo_rust_newtype_eq::foreign_crate::Old`
 --> src/main.rs:8:24
  |
8 |     println!("{}", old == new);
  |                    --- ^^ --- New
  |                    |
  |                    demo_rust_newtype_eq::foreign_crate::Old

For more information about this error, try `rustc --explain E0369`.
error: could not compile `demo-rust-newtype-eq` due to previous error
exit code: 101
```
