use crate::common::{ListData, RequestFailed};

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::error::Error;

const API_URL: &'static str = "https://swapi.dev/api";

type Output<T> = std::result::Result<T, Box<dyn Error>>;

/// Main request handler for query starwars objects
pub trait RequestHandler {
    const URL_PATH: &'static str;

    fn get(id: usize) -> Output<Box<Self>>
    where
        Self: DeserializeOwned,
    {
        match reqwest::blocking::get(API_URL.to_owned() + Self::URL_PATH + &id.to_string()) {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(Box::new(res.json::<Self>().unwrap())),
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }

    fn list(page: Option<usize>) -> Output<Box<ListData<Self>>>
    where
        Self: DeserializeOwned,
    {
        let page = match page {
            Some(x) => "?page=".to_owned() + &x.to_string(),
            None => String::from("?page=1"),
        };

        match reqwest::blocking::get(API_URL.to_owned() + Self::URL_PATH + &page) {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(Box::new(res.json::<ListData<Self>>().unwrap())),
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}
