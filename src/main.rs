use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    let home = warp::path::end().map(|| {
        let mut body = HashMap::new();

        body.insert("name", "username");
        body.insert("content", "something");

        warp::reply::json(&body)
    });

    warp::serve(home).run(([127, 0, 0, 1], 8080)).await
}
