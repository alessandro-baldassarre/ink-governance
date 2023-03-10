//! # Ink-Governance
//!
//! Ink-Governance is a library crate to simplify the creation of governance based smart contracts written in ink! <https://use.ink/>.
//! It's supposed to be an extension of the open-brush library <https://openbrush.io/> , so it depends on it.
//!
//! It is a modular system where the resulting smart contract is composed of a core governor module to which a
//! sub-module for counting the votes and one for extracting the weight of the votes must be added.
//! Other optional extensions can be added to costumize the module.
//!
//! See the repo for documentation and usage examples <https://github.com/alessandro-baldassarre/ink-governance>

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
