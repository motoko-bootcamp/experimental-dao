use axum::{routing::get, Router};
use flexi_logger::{FileSpec, Logger};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs").basename("log"))
        .print_message()
        .start()
        .unwrap();

    let routes = Router::new().route("/", get(root_get).post(root_post));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("starting server");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn root_get() {
    log::info!("get requets to root");
}

async fn root_post(body: String) {
    log::info!("post requets to root. Body:\n{body}");
}