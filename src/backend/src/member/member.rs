
use ic_cdk::{query, update, caller};
use crate::{types::{Member, StablePrincipal}, enums::MemberError};
use crate::{MEMBERS, roles};
use candid::{Principal};

#[update]
fn create_member(
    username: String,
    xp: u64,
    level: u64,
    roles: u64
    // token_balance: u64
) -> Result<Option<Member>, MemberError> {
    let caller: Principal = caller();

    let principal_exists = MEMBERS.with(|members| members.borrow().contains_key(&StablePrincipal(caller)));
    if principal_exists {
        return Err(MemberError::PrincipalAlreadyRegistered);}

    let username_exists = MEMBERS.with(|members| {
        members.borrow().iter().any(|(_key, member)| member.username == username)
    });
    
    if username_exists {
        return Err(MemberError::UsernameExists);}

    let new_member = Member {
        principal: caller,
        username,
        xp,
        level,
        roles
        // token_balance,
    };
    insert_member(caller, new_member).map_or(Ok(None), |d| Ok(Some(d)))
}

fn insert_member(key: Principal, value: Member) -> Option<Member> {
    let stable_key = StablePrincipal(key);
    MEMBERS.with(|p| p.borrow_mut().insert(stable_key, value))
}

#[query]
fn get_all_members() -> Vec<Member> {
    MEMBERS.with(|map| {
        map.borrow()
            .iter() 
            .map(|(_key, value)| value.clone())
            .collect()
    })
}
