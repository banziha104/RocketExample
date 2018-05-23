extern crate rocket;

pub mod api_router {
    #[get("/api")]
    pub fn api_get() -> &'static str {
        "Get!"
    }

    #[get("/api/<index>")]
    pub fn api_post(index : i32) -> String{
        format!("index : {}",index)
    }
}