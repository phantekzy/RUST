//Packages and Crates
//
// A crate is a binary or library . The crate root is  a source file that thhe Rust compiler
// starts from and makes up the root module of our crate .
//
// A Package contains a Cargo.toml file that describes how to build those crates .
//
// Several rules determine what a package can contain . A package must contain zero or one library
// crates , and no more . It can contain as many binary crates as we'd like , but it must contain at
// least one crate (either library or binary).
//
// What happens when we create a package :
// cargo new phantekzy
//   Created binary (application)  `phantekzy` package
//   ls phantekzy
//   Cargo.toml
//   src
//   ls phantekzy/src
//   main.rs
//
// When we entered the command , Cargo created a Cargo.toml file , giving us a package .
// Looking at the contents of Cargo.toml , there is no mention of src/main.rs because
// Cargo follows a convention that src/main.rs is the crate root of a binary crate with
// the same name as the package .
// Likewise , Cargo knows that if the package directory contains src/lib.rs , the package contains a
// library crate with the same name as the package , and src/lib.rs is its crate root .
// Cargo passes the crate root files to rustc to build the library or binary.
// Main function
fn main() {}
