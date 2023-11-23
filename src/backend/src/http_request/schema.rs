use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
struct GithubEvent {
    action: String,
    issue: Issue,
    repository: Repository,
    sender: User,
}

#[derive(Debug, CandidType, Deserialize)]
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

#[derive(Debug, CandidType, Deserialize)]
struct Repository {
    id: u64,
    name: String,
    full_name: String,
    // Add more fields as needed
}

#[derive(Debug, CandidType, Deserialize)]
struct User {
    login: String,
    id: u64,
    // Add more fields as needed
}
