//! Rust client for Star Wars API

pub mod common;
pub mod objects;
mod requests;

pub use crate::objects::{Film, People, Planet, Species, Starship, Vehicle};
pub use crate::requests::RequestHandler;
