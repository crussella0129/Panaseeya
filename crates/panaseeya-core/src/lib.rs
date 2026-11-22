//! Catalog core for Panaseeya.
//!
//! One local catalog of every game a person owns, regardless of where it came
//! from. Modules are added here as the catalog grows.

pub mod title;

pub use title::normalize_title;
