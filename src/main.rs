pub mod routes;

use warp::Filter;

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let url = (IP, PORT);

    let home = warp::path::end();
    let routes = home.map(routes::load);

    warp::serve(routes).run(url).await
}
