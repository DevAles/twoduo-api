pub mod routes;

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let url = (IP, PORT);

    let routes = routes::load();

    warp::serve(routes).run(url).await
}
