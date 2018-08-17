#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[post("/graphql")]
fn graphql() -> &'static str {
    "Hello, World"
}
