#[cfg(feature = "governor_counting_simple")]
pub mod governor_counting_simple;

#[cfg(feature = "governor_voting_group")]
pub mod governor_voting_group;

#[cfg(feature = "governor")]
pub mod counter;

#[cfg(feature = "governor")]
pub mod voter;
