#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod traits;

mod governance;

#[cfg(feature = "governor")]
pub use governance::governor;

#[cfg(feature = "governor_counting_simple")]
pub use governor::governor_counting_simple;

#[cfg(feature = "governor_votes_members")]
pub use governor::governor_votes_members;
