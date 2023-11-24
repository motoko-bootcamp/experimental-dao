use ic_cdk_macros::{export_candid, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

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

    pub static PROPOSALS: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

    static NEXT_PROPOSAL_ID: RefCell<StableCell<u64, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            0
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

// fn main() {
//     let roy: u64 = roles::CODING_TEAM_MEMBER | roles::CODING_TEAM_ADMIN;

//     let is_roy_a_coding_member = roy & roles::CODING_TEAM_MEMBER != 0;
//     assert_eq!(is_roy_a_coding_member, true);

//     let is_roy_a_marketing_member = roy & roles::MARKETING_TEAM_MEMBER != 0;
//     assert_eq!(is_roy_a_marketing_member, false);
// }
