use crate::auth;

use std::collections::HashMap;

use serde_json::json;
use warp::{reply::Json, Filter};

pub fn load() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::form())
        .and_then(|credentials: HashMap<String, String>| {
            let username = credentials.get("username").unwrap();
            let password = credentials.get("password").unwrap();

            auth::verify(username, password);
            get_json()
        })
        .with(warp::cors().allow_any_origin())
}

async fn get_json() -> Result<Json, warp::Rejection> {
    let tasks = vec!["two", "duo"];
    let json = json!({ "tasks": tasks });

    let response = warp::reply::json(&json);

    Ok(response)
}
