//! Catalog core for Panaseeya.
//!
//! One local catalog of every game a person owns, regardless of where it came
//! from. Modules are added here as the catalog grows.

pub mod catalog;
pub mod error;
pub mod model;
pub mod store;
pub mod title;

pub use catalog::Catalog;
pub use error::{Error, Result};
pub use model::{Medium, NewOwnership, Origin};
pub use store::Store;
pub use title::normalize_title;
