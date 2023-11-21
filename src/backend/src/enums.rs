
use candid::{CandidType};

#[derive(Debug, CandidType)]
pub enum MemberError {
    PrincipalAlreadyRegistered,
    UsernameExists,
}
