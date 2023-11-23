use crate::LOGS;
use candid::{define_function, CandidType};
use ic_cdk::api;
use ic_cdk::query;
use ic_cdk_macros::update;
use serde::Deserialize;
use serde_json;
use std::str;

mod schema;

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Token {}

define_function!(pub CallbackFunc : () -> () query);

#[derive(Clone, Debug, CandidType, Deserialize)]
enum StreamingStrategy {
    Callback {
        callback: CallbackFunc,
        token: Token,
    },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
    streaming_strategy: Option<StreamingStrategy>,
}

#[update]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    LOGS.with_borrow_mut(|logs| {
        logs.push(format!(
            "we received a message. method: {}, body len: {}, headers len: {}",
            req.method,
            req.body.len(),
            req.headers.len(),
        ))
    });

    // let body = String::from_utf8(req.body).unwrap();
    // LOGS.with_borrow_mut(|logs| logs.push(body));

    // Check if it's a POST request and from GitHub
    if req.method == "POST" {
        // LOGS.with_borrow_mut(|logs| logs.push("last log was PUT request".to_string()));

        // TODO: extract the data we need
        // if let Some(issue_data) = extract_issue_data(github_event) {
        //     ISSUES.with(|issues| {
        //         issues.borrow_mut().push(issue_data);
        //     });
        // }

        return HttpResponse {
            status_code: 200,
            headers: vec![],
            body: vec![],
            streaming_strategy: None,
        };
    }

    HttpResponse {
        status_code: 400, // Bad Request
        headers: vec![],
        body: vec![],
        streaming_strategy: None,
    }
}

// Helper function to parse the GitHub webhook JSON payload
// fn parse_github_webhook(body: &Vec<u8>) -> Result<GithubEvent, serde_json::Error> {
//     let body_str = str::from_utf8(body).expect("Body is not valid UTF-8");
//     serde_json::from_str(body_str)
// }

// fn extract_issue_data(event: GithubEvent) -> Option<IssueData> {
//     if event.action == "opened" {
//         // or other actions like "closed", "edited", etc.
//         Some(IssueData {
//             number: event.issue.number,
//             title: event.issue.title,
//             body: event.issue.body.unwrap_or_default(),
//             state: event.issue.state,
//             // Add more fields as needed
//         })
//     } else {
//         None
//     }
// }

// #[query]
// pub fn get_all_issues() -> Vec<IssueData> {
//     ISSUES.with(|issues| issues.borrow().clone())
// }

#[query]
pub fn get_logs() -> Vec<String> {
    LOGS.with(|logs| logs.borrow().clone())
}
