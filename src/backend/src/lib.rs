use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_cdk_macros::export_candid;

use crate::{types::{Member, StablePrincipal}, enums::MemberError};

mod types;
mod enums;
mod member { mod member; }
mod roles;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static MEMBERS: RefCell<StableBTreeMap<StablePrincipal, Member, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

export_candid!();

// fn main() {
//     let roy: u64 = roles::CODING_TEAM_MEMBER | roles::CODING_TEAM_ADMIN;

//     let is_roy_a_coding_member = roy & roles::CODING_TEAM_MEMBER != 0;
//     assert_eq!(is_roy_a_coding_member, true);

//     let is_roy_a_marketing_member = roy & roles::MARKETING_TEAM_MEMBER != 0;
//     assert_eq!(is_roy_a_marketing_member, false);
// }