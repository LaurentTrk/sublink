#![cfg_attr(not(feature = "std"), no_std)]

#[doc(inline)]
pub use ink_prelude::{format, vec::Vec};
pub use log::Level;

use ink_env::{DefaultEnvironment, Environment};
use ink_lang as ink;
use scale::{
    Decode,
    Encode,
};
use scale_info::TypeInfo;


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SubLinkEnvironment {}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Default, TypeInfo, Copy)]
pub struct RoundData {
    pub started_at: <DefaultEnvironment as Environment>::BlockNumber,
    pub answer: u128,
    pub updated_at: <DefaultEnvironment as Environment>::BlockNumber,
    pub answered_in_round: u32,
}


#[ink::chain_extension]
pub trait SubLinkExtension {
    type ErrorCode = SubLinkErr;

    #[ink(extension = 70930000, returns_result = false)]
    fn latest_round_data(feed_id: u32) -> RoundData;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SubLinkErr {
    FailGetFeed,
}

impl ink_env::chain_extension::FromStatusCode for SubLinkErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetFeed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl Environment for SubLinkEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

    type ChainExtension = SubLinkExtension;
}