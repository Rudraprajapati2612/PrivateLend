// programs/collateral-vault/src/instructions/mod.rs
pub mod initialize;
pub mod lock;
pub mod release;
pub mod liquidate;
pub mod stats;

pub use initialize::*;
pub use lock::*;
pub use release::*;
pub use liquidate::*;
pub use stats::*;