use axum::{routing::post, Router};
// use flexi_logger::{FileSpec, Logger};
use std::{net::SocketAddr, process::Command};

const CANISTER_ID: &'static str = "3c7jb-myaaa-aaaab-qacoa-cai";
const METHOD_NAME: &'static str = "add_to_log";

#[tokio::main]
async fn main() {
    // Logger::try_with_str("info")
    //     .unwrap()
    //     .log_to_file(FileSpec::default().directory("logs").basename("log"))
    //     .print_message()
    //     .start()
    //     .unwrap();

    let routes = Router::new().route("/", post(root_post));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("starting server");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn root_post(body: String) {
    println!("sending {body} to the canister");
    let body = body.replace("\"", "\\\"");
    let body = format!("(\"{body}\")");
    println!("{body}");
    let status = Command::new(format!("dfx"))
        .args([
            "canister",
            "call",
            CANISTER_ID,
            METHOD_NAME,
            &body,
            "--network",
            "ic",
        ])
        .status()
        .unwrap();
    println!("done {status}");
}