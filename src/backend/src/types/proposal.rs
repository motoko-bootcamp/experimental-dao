use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use derive_getters::Getters;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::github::GitHubIssue;

#[derive(Debug, Clone, Copy)]
pub enum ProposalError {
    TitleTooLong,
    DescriptionTooLong,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum ProposalStatus {
    Open,
    Accepted,
    Rejected,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum ProposalType {
    GitHubIssue(GitHubIssue),
    // TODO: GitHubPullRequest, General
}

#[derive(Clone, CandidType, Deserialize Getters)]
pub struct Proposal {
    id: u64,
    author: Principal,
    created_at: u64,
    last_update: u64,
    title: String,
    description: String,
    status: ProposalStatus,
    proposal_type: ProposalType,
}

impl Proposal {
    const MAX_TITLE_LENGTH: u32 = 100;
    const MAX_DESCRIPTION_LENGTH: u32 = 1000;

    pub fn new(
        id: u64,
        author: Principal,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<Self, ProposalError> {
        if title.as_bytes().len() > Self::MAX_TITLE_LENGTH as usize {
            return Err(ProposalError::TitleTooLong);
        }

        if title.as_bytes().len() > Self::MAX_DESCRIPTION_LENGTH as usize {
            return Err(ProposalError::DescriptionTooLong);
        }

        let time = ic_cdk::api::time();

        Ok(Self {
            id,
            author,
            created_at: time,
            last_update: time,
            title,
            description,
            status: ProposalStatus::Open,
            proposal_type,
        })
    }

    fn update_status(&mut self, status: ProposalStatus) {
        self.last_update = ic_cdk::api::time();
        self.status = status;
    }

    pub fn accept(&mut self) {
        self.update_status(ProposalStatus::Accepted)
    }

    pub fn rejected(&mut self) {
        self.update_status(ProposalStatus::Rejected)
    }
}

impl Storable for Proposal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 115 + Self::MAX_TITLE_LENGTH + Self::MAX_DESCRIPTION_LENGTH,
        is_fixed_size: false,
    };
}
