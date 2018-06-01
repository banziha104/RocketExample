# 요청

- path 파싱 

```rust
#[get("/test/<index>")]
    pub fn api_post(index : i32) -> String{
        format!("index : {}",index)
        
    }
```
