//! CPU information.
//!
//! This module is enabled with the `**cpu**` feature flag (enabled by default).

#![allow(stable_features)]
#![feature(futures_api)]

pub mod os;
mod sys;

mod freq;
mod stats;
mod times;
mod units;

pub use self::freq::*;
pub use self::stats::*;
pub use self::times::*;
pub use self::units::*;
