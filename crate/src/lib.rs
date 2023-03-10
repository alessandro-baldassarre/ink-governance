#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// Traits definition and wrappers used in ink_governance contracts
pub mod traits;

mod governance;
mod token;

#[cfg(feature = "governor")]
pub use governance::governor;

#[cfg(feature = "governor_settings")]
pub use governance::extensions::governor_settings;

#[cfg(feature = "governor_counting_simple")]
pub use governance::modules::governor_counting_simple;

#[cfg(feature = "governor_voting_group")]
pub use governance::modules::governor_voting_group;

#[cfg(feature = "psp22_votes")]
pub use token::psp22::extensions::psp22_votes;
