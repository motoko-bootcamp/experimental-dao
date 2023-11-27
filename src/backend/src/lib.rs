use ic_cdk_macros::{export_candid, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableVec};
use std::cell::RefCell;
use types::{Bounty, Proposal};

use crate::{
    enums::MemberError,
    types::{Member, StablePrincipal},
};

mod enums;
mod http_request;
mod member;
mod roles;
mod types;
use http_request::{HttpRequest, HttpResponse};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static MEMBERS: RefCell<StableBTreeMap<StablePrincipal, Member, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    pub static PROPOSALS: RefCell<StableVec<Proposal, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    pub static BOUNTIES: RefCell<StableVec<Bounty, Memory>> = RefCell::new(
        StableVec::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        ).unwrap()
    );

    // pub static ISSUES: RefCell<Vec<IssueData>> = RefCell::new(Vec::new());
    pub static LOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[update]
fn add_to_log(log: String) {
    LOGS.with_borrow_mut(|logs| logs.push(log));
}

export_candid!();
