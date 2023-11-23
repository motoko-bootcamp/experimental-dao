use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, process::Command};

const METHOD_NAME: &'static str = "add_to_log";

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum State {
    Closed,
    Open,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    login: String,
    id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Issue {
    id: u64,
    title: String,
    number: u64,
    user: User,
    // issue.labels: [string] ---- ignore for now,
    state: State,
    created_at: String, // (date-time) (2023-11-23T15:56:41Z),
    body: Option<String>,
}

fn get_local_backend_canister_id() -> String {
    let source = std::fs::read_to_string("../.dfx/local/canister_ids.json").unwrap();
    let value: HashMap<String, HashMap<String, String>> = serde_json::from_str(&source).unwrap();
    value["backend"]["local"].clone()
}

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/", post(root_post));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("starting server");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn root_post(body: String) {
    let mut body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let issue = body["issue"].take();
    let issue: Issue = serde_json::from_value(issue).unwrap();
    send_to_canister(issue)
}

fn send_to_canister(arg: impl Serialize) {
    let body = serde_json::to_string(&arg).unwrap();
    println!("sending {body} to the canister");
    let canister_id = get_local_backend_canister_id();
    let body = body.replace("\"", "\\\"");
    let body = format!("(\"{body}\")");
    let status = Command::new(format!("dfx"))
        .args([
            "canister",
            "call",
            &canister_id,
            METHOD_NAME,
            &body,
            // "--network",
            // "ic",
        ])
        .status()
        .unwrap();
    println!("done {status}");
}
