#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub mod handlers {
    pub mod version;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/zv/", routes![handlers::])
        .launch();
}
