mod governor;

pub use governor::*;

/// Traits definition of modules to use combined with governor.
pub mod modules {
    /// Trait definition that a "counter" sub-module must implement
    pub mod counter;
    /// Trait definition of counting simple "counter" sub-module
    pub mod counting_simple;
    /// Trait definition that a "voter" sub-module must implement
    pub mod voter;
    /// Trait definition of voting group "voter" sub-module
    pub mod voting_group;
}

/// Traits definition of extensions of governor base contracts.
pub mod extensions {
    pub mod settings;
}

/// Traits definition of utils to extend governor base contracts.
pub mod utils {
    pub mod votes;
}
