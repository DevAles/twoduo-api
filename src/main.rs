pub mod routes;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let url = (IP, PORT);

    let routes = routes::load();

    warp::serve(routes).run(url).await
}
