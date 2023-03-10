mod governor;

pub use governor::*;

/// Traits implementation of modules to use combined with governor.
pub mod modules {
    pub mod counting_simple;
    pub mod voting_group;
    pub mod voter;
    pub mod counter;
}

/// Traits implementation of extensions of governor base contracts.
pub mod extensions {
    pub mod settings;
}

/// Traits implementation of utils to extend governor base contracts.
pub mod utils {
    pub mod votes;
}
