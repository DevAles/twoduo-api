use std::collections::HashMap;
use warp::reply::Json;

pub fn load() -> Json {
    let mut body = HashMap::new();

    body.insert("name", "username");
    body.insert("content", "something");

    warp::reply::json(&body)
}
