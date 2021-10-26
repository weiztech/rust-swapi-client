# Rust Swapi Client

[![Crate](https://img.shields.io/crates/v/swapi-client)](https://crates.io/crates/swapi-client)

Rust client for Star Wars API (https://swapi.co/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
swapi_client = "0.0.5"
```

Example Usage:

```
use swapi_client::RequestHandler;
use swapi_client::{Film, People, Planet, Species, Starship, Vehicle};

// Get Planet object
Planet::get(10);

// Get Planets
let planet_list = Planet::list(Some(2)).unwrap();

// Check list data has prev or next pagination
planet_list.has_next();
planet_list.has_prev();

// List data query next and prev
planet_list.next();
planet_list.prev();

// Iterate from list data
for planet in planet_list.into_iter() {
  println!("Planet: {}", planet.name);
}
```
