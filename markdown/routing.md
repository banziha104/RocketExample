# 라우팅

- function attribute 속성의 get,post 등의 메서드로 패스 지정

```rust
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
```

- route 에 등록

```rust
fn main() {
    rocket::ignite().mount("/",
    routes![
        index,
        api_router::api_get,
        api_router::api_post
        ]).launch();
}
```
