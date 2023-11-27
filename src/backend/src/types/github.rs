use candid::{CandidType, Deserialize};

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GitHubIssueState {
    Closed,
    Open,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GitHubIssue {
    pub id: u64,
    pub title: String,
    pub number: u64,
    pub user: GitHubUser,
    pub state: GitHubIssueState,
    /// ISO encoded date-time string (for example '2023-11-23T15:56:41Z')
    pub created_at: String,
    pub body: Option<String>,
}
