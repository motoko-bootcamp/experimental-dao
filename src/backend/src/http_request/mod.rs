use candid::{define_function, CandidType};
use ic_cdk::{query};
use ic_cdk::api;
use serde::Deserialize;
use serde_json;
use std::str;
use crate::{ISSUES, LOGS};

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

#[derive(Debug, Deserialize)]
struct GithubEvent {
    action: String,
    issue: Issue,
    repository: Repository,
    sender: User,
}

#[derive(Debug, Deserialize)]
struct Issue {
    number: u64,
    title: String,
    body: Option<String>,
    state: String,
    // Add more fields as needed
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct IssueData {
    number: u64,
    title: String,
    body: String,
    state: String,
    // Add more fields as needed
}


#[derive(Debug, Deserialize)]
struct Repository {
    id: u64,
    name: String,
    full_name: String,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: u64,
    // Add more fields as needed
}

#[query]
pub fn http_request(req: HttpRequest) -> HttpResponse {

    LOGS.with(|logs| logs.borrow_mut().push(req));

    // Check if it's a POST request and from GitHub
    if req.method == "POST" && is_github_webhook(&req.headers) {
        // Parse the JSON body to extract issue data
        if let Ok(github_event) = parse_github_webhook(&req.body) {
            // Check if it's an issue event and process it
            if let Some(issue_data) = extract_issue_data(github_event) {
                ISSUES.with(|issues| {
                    issues.borrow_mut().push(issue_data);
                });
            }
        }

        // Respond with a success status code
        HttpResponse {
            status_code: 200,
            headers: vec![],
            body: vec![],
            streaming_strategy: None,
        }
    } else {
        // Handle other requests or invalid webhook calls
        HttpResponse {
            status_code: 400, // Bad Request
            headers: vec![],
            body: vec![],
            streaming_strategy: None,
        }
    }
}

// Helper function to determine if the request is from GitHub
fn is_github_webhook(headers: &Vec<HeaderField>) -> bool {
    // Implement logic to validate GitHub headers
    // For example, check for specific GitHub signature headers
    true // Placeholder return value
}

// Helper function to parse the GitHub webhook JSON payload
fn parse_github_webhook(body: &Vec<u8>) -> Result<GithubEvent, serde_json::Error> {
    let body_str = str::from_utf8(body).expect("Body is not valid UTF-8");
    serde_json::from_str(body_str)
}

fn extract_issue_data(event: GithubEvent) -> Option<IssueData> {
    if event.action == "opened" { // or other actions like "closed", "edited", etc.
        Some(IssueData {
            number: event.issue.number,
            title: event.issue.title,
            body: event.issue.body.unwrap_or_default(),
            state: event.issue.state,
            // Add more fields as needed
        })
    } else {
        None
    }
}

#[query]
pub fn get_all_issues() -> Vec<IssueData> {
    ISSUES.with(|issues| issues.borrow().clone())
}

#[query]
pub fn get_logs() -> Vec<String> {
    LOGS.with(|logs| logs.borrow().clone())
}


