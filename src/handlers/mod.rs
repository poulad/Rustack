extern crate rocket;

#[get("/version")]
pub fn version() -> &'static str {
    "{\"versions\":[\"1\"]}"
}

#[post("/graphql")]
fn graphql() -> &'static str {
    "Hello, World"
}
