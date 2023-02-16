#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod traits;

mod governance;

#[cfg(feature = "governor")]
pub use governance::governor;
