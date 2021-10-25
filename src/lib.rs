//! Rust client for Star Wars API
pub mod common;
mod objects;
mod requests;

pub use objects::{Film, People, Planet, Species, Starship, Vehicle};
pub use requests::{Output, QueryDetail, QueryList, RequestHandler};
