use crate::common::{ListData, RequestFailed};

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::error::Error;

const API_URL: &'static str = "https://swapi.dev/api";

pub type Output<T> = std::result::Result<T, Box<dyn Error>>;

// Get object handler
pub trait QueryDetail {
    fn get(id: usize, path: &'static str) -> Output<Box<Self>>
    where
        Self: DeserializeOwned,
    {
        match reqwest::blocking::get(API_URL.to_owned() + path + &id.to_string()) {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(res.json::<Self>().unwrap().into()),
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}

// List object handler
pub trait QueryList {
    fn list(page: Option<usize>, path: &'static str) -> Output<Box<ListData<Self>>>
    where
        Self: DeserializeOwned,
    {
        let page = match page {
            Some(x) => "?page=".to_owned() + &x.to_string(),
            None => String::from("?page=1"),
        };

        match reqwest::blocking::get(API_URL.to_owned() + path + &page) {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(res.json::<ListData<Self>>().unwrap().into()),
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}
/// Main request handler for query starwars objects
pub trait RequestHandler: QueryDetail + QueryList {
    const URL_PATH: &'static str;
    fn list(page: Option<usize>) -> Output<Box<crate::common::ListData<Self>>>
    where
        Self: serde::de::DeserializeOwned,
    {
        QueryList::list(page, Self::URL_PATH)
    }

    fn get(id: usize) -> Output<Box<Self>>
    where
        Self: serde::de::DeserializeOwned,
    {
        QueryDetail::get(id, Self::URL_PATH)
    }
}
