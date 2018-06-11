extern crate rocket;

use rocket::http::RawStr;
pub mod api_router {
    #[get("/test")]
    pub fn api_get() -> &'static str {
        "Get!"
    }

    #[post("/test/<index>")]
    pub fn api_post(index : i32) -> &'static str {
        let mut input_text = String::new();
        print!("{}");
    }
}