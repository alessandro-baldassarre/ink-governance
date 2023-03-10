/// Implementation of counting simple "counter" sub-module.
#[cfg(feature = "governor_counting_simple")]
pub mod governor_counting_simple;

/// Implementation of voting group "voter" sub-module.
#[cfg(feature = "governor_voting_group")]
pub mod governor_voting_group;

/// Implementation of basic "counter" sub-module.
#[cfg(feature = "governor")]
pub mod counter;

/// Implementation of basic "voter" sub-module.
#[cfg(feature = "governor")]
pub mod voter;
