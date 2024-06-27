#![doc = include_str!("../README.md")]

pub use crate::array::{ArrayValue, H3Converter};
pub use crate::axis::AxisOrder;
pub use crate::coverage::CellCoverage;
pub use crate::error::Error;
pub use crate::resolution::ResolutionSearchMode;

mod array;
mod axis;
mod coverage;
mod error;
mod resolution;
pub mod sphere;
pub mod transform;
mod util;
