use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_stable_structures::{ storable::Bound, Storable };
use std::{borrow::Cow};

// pub type Blob = Vec<u8>;
const MAX_VALUE_SIZE: u32 = 500;
// u32 is 32 bits and 4 bytes
// for struct you combine all the fields

#[derive(Clone, CandidType, Deserialize)]
pub struct Member {
    pub principal: Principal,
    pub username: String,
	pub xp: u64,
	pub level: u64,
	// pub token_balance: u64,
}

#[derive(Clone, CandidType, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct StablePrincipal(pub Principal);

impl Storable for StablePrincipal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
  }

impl Storable for Member {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}