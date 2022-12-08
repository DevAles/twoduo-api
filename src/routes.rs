use warp::{reply::Json, Filter};

pub fn load() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(get_json)
}

async fn get_json() -> Result<Json, warp::Rejection> {
    todo!()
}
