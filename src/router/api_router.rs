extern crate rocket;

use rocket::http::RawStr;
pub mod api_router {
    #[get("/test")]
    pub fn api_get() -> &'static str {
        "Get!"
    }

    // #[post("/test/<index>")]
    // pub fn api_post(index : i32) -> String{
    //     format!("index : {}",index);

    // }

    // #[post("/test/test", format = "application/json", data ="<data>")]
    // pub fn api_body_parse_test(data : &RawStr) -> String{
    //     data.to_string();
    // }
}