//! Core requests handler

use crate::common::{ListData, RequestFailed};

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::error::Error;

const API_URL: &'static str = "https://swapi.dev/api";

/// Custom result type
pub type Output<T> = std::result::Result<T, Box<dyn Error>>;

/// Url path for Star Wars Object
pub trait UrlPath {
    const URL_PATH: &'static str;
}

/// Get object handler
pub trait QueryDetail: UrlPath {
    fn get(id: usize) -> Output<Box<Self>>
    where
        Self: DeserializeOwned,
    {
        match reqwest::blocking::get(API_URL.to_owned() + Self::URL_PATH + &id.to_string()) {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(res.json::<Self>().unwrap().into()),
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}

/// List object handler
pub trait QueryList: UrlPath {
    fn list(page: Option<u8>) -> Output<Box<ListData<Self>>>
    where
        Self: DeserializeOwned,
    {
        let page_query = match page {
            Some(x) => "?page=".to_owned() + &x.to_string(),
            None => String::from("?page=1"),
        };

        match reqwest::blocking::get(API_URL.to_owned() + Self::URL_PATH + &page_query) {
            Ok(res) => match res.status() {
                StatusCode::OK => {
                    let mut list_data = res.json::<ListData<Self>>().unwrap();
                    list_data.current_page = page;
                    Ok(list_data.into())
                }
                _ => Err(RequestFailed(res).into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}
/// Main request handler for query starwars objects
pub trait RequestHandler: QueryDetail + QueryList {
    fn list(page: Option<u8>) -> Output<Box<ListData<Self>>>
    where
        Self: DeserializeOwned,
    {
        QueryList::list(page)
    }

    fn get(id: usize) -> Output<Box<Self>>
    where
        Self: DeserializeOwned,
    {
        QueryDetail::get(id)
    }
}

impl<T> UrlPath for ListData<T>
where
    T: UrlPath,
{
    const URL_PATH: &'static str = T::URL_PATH;
}

impl<T> QueryList for ListData<T> where T: UrlPath {}

impl<T: QueryList + DeserializeOwned> ListData<T> {
    fn get_page_value(&self) -> u8 {
        match self.current_page {
            Some(x) => x,
            None => 1,
        }
    }

    pub fn next(&self) -> Option<Box<Self>> {
        match self.has_next() {
            true => Some(T::list(Some(self.get_page_value() + 1)).unwrap()),
            false => None,
        }
    }

    pub fn prev(&self) -> Option<Box<Self>> {
        match self.has_prev() {
            true => Some(T::list(Some(self.get_page_value() - 1)).unwrap()),
            false => None,
        }
    }
}
