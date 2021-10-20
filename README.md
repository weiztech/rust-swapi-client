# Rust Swapi Client

[![Crate](https://img.shields.io/crates/v/swapi-client)](https://crates.io/crates/swapi-client)

Rust client for Star Wars API (https://swapi.co/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
swapi_client = "0.0.2"
```

Query Star Wars objects:

```
use swapi_client::RequestHandler;
use swapi_client::{Film, People, Planet, Species, Starship, Vehicle};

println!("API Planet GET: {:?}", Planet::get(10));
let planets = Planet::list(Some(2));
println!("API Planet LIST: {:?}", planets);

println!("\nAPI Film GET {:?}", People::get(10));
println!("API Film LIST {:?}", Film::list(None));

println!("\nAPI People GET {:?}", People::get(1));
println!("API People LIST {:?}", People::list(None));

println!("\nAPI Species GET {:?}", Species::get(1));
println!("API Species LIST {:?}", Species::list(None));

println!("\nAPI Starship GET {:?}", Starship::get(2));
println!("API Starship LIST {:?}", Starship::list(None));

println!("\nAPI Vehicle GET {:?}", Vehicle::get(7));
println!("API Vehicle LIST {:?}", Vehicle::list(None));


// Iterate from list data
for nplanet in planets.unwrap().into_iter() {
  println!("Planet: {}", nplanet.name);
}

```
