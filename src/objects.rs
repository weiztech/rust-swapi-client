//! Star Wars Objects

use crate::requests::{QueryDetail, QueryList, RequestHandler, UrlPath};

use serde::Deserialize;

/// Planet resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]

pub struct Planet {
    pub climate: String,
    pub diameter: String,
    pub gravity: String,
    pub name: String,
    pub orbital_period: String,
    pub population: String,
    pub residents: Vec<String>,
    pub rotation_period: String,
    pub surface_water: String,
    pub terrain: String,
    pub url: String,
}

/// People resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct People {
    pub birth_year: String,
    pub eye_color: String,
    pub films: Vec<String>,
    pub gender: String,
    pub hair_color: String,
    pub height: String,
    pub homeworld: String,
    pub mass: String,
    pub name: String,
    pub skin_color: String,
    pub created: String,
    pub edited: String,
    pub species: Vec<String>,
    pub starships: Vec<String>,
    pub url: String,
    pub vehicles: Vec<String>,
}

/// Film resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Film {
    characters: Vec<String>,
    created: String,
    director: String,
    edited: String,
    episode_id: usize,
    opening_crawl: String,
    planets: Vec<String>,
    producer: String,
    release_date: String,
    species: Vec<String>,
    starships: Vec<String>,
    title: String,
    url: String,
    vehicles: Vec<String>,
}

/// Starship resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Starship {
    #[serde(alias = "MGLT")]
    mglt: String,

    cargo_capacity: String,
    consumables: String,
    cost_in_credits: String,
    created: String,
    crew: String,
    edited: String,
    hyperdrive_rating: String,
    length: String,
    manufacturer: String,
    max_atmosphering_speed: String,
    model: String,
    name: String,
    passengers: String,
    films: Vec<String>,
    pilots: Vec<String>,
    starship_class: String,
    url: String,
}

/// Vehicle resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Vehicle {
    cargo_capacity: String,
    consumables: String,
    cost_in_credits: String,
    created: String,
    crew: String,
    edited: String,
    length: String,
    manufacturer: String,
    max_atmosphering_speed: String,
    model: String,
    name: String,
    passengers: String,
    pilots: Vec<String>,
    films: Vec<String>,
    url: String,
    vehicle_class: String,
}

/// Species resource within the Star Wars universe.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Species {
    average_height: String,
    average_lifespan: String,
    classification: String,
    created: String,
    designation: String,
    edited: String,
    eye_colors: String,
    hair_colors: String,
    homeworld: Option<String>,
    language: String,
    name: String,
    people: Vec<String>,
    films: Vec<String>,
    skin_colors: String,
    url: String,
}

impl UrlPath for Planet {
    const URL_PATH: &'static str = "/planets/";
}

impl QueryList for Planet {}

impl QueryDetail for Planet {}

impl RequestHandler for Planet {}

impl UrlPath for People {
    const URL_PATH: &'static str = "/people/";
}

impl QueryList for People {}

impl QueryDetail for People {}

impl UrlPath for Film {
    const URL_PATH: &'static str = "/films/";
}

impl QueryList for Film {}

impl QueryDetail for Film {}

impl UrlPath for Starship {
    const URL_PATH: &'static str = "/starships/";
}

impl QueryList for Starship {}

impl QueryDetail for Starship {}

impl UrlPath for Vehicle {
    const URL_PATH: &'static str = "/vehicles/";
}

impl QueryList for Vehicle {}

impl QueryDetail for Vehicle {}

impl UrlPath for Species {
    const URL_PATH: &'static str = "/species/";
}

impl QueryList for Species {}

impl QueryDetail for Species {}
