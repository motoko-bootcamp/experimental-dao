use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use derive_getters::Getters;
use ic_stable_structures::{storable::Bound, Storable};

#[derive(Clone, CandidType, Deserialize)]
pub enum BountyStatus {
    Open,
    Completed { completed_by: Principal },
}

#[derive(Clone, CandidType, Deserialize, Getter)]
pub struct Bounty {
    id: u64,
    author: Principal,
    proposal_id: u64,
    tokens: u64,
    created_at: u64,
    expiration: u64,
    last_update: u64,
    status: BountyStatus,
}

// TODO: make a small utility library for this
// fn add_week(weeks: u64) -> u64 {
//     ic_cdk::api::time() + weeks * 7 * 24 * 60 * 60 * 1000 * 1000
// }

impl Bounty {
    pub fn new(
        id: u64,
        author: Principal,
        proposal_id: u64,
        tokens: u64,
        expiration: Option<u64>,
    ) -> Self {
        let time = ic_cdk::api::time();
        Self {
            id,
            author,
            proposal_id,
            tokens,
            created_at: time,
            expiration: expiration.unwrap_or(u64::MAX),
            last_update: time,
            status: BountyStatus::Open,
        }
    }

    pub fn remaining_time(&self) -> u64 {
        let time = ic_cdk::api::time();
        if time <= self.expiration {
            self.expiration - time
        } else {
            0
        }
    }
}

impl Storable for Bounty {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: true,
    };
}
