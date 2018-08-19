extern crate rocket;

#[get("/version")]
pub mod handlers {
    pub fn version() -> &'static str {
        "{\"versions\":[\"1\"]}"
    }
}
