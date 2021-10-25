//! Shared libs

use std::error::Error;
use std::fmt;

#[derive(Debug, serde::Deserialize, Default, Clone)]
pub struct ListData<T> {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

impl<T> ListData<T> {
    pub fn has_next(&self) -> bool {
        match &self.next {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn has_prev(&self) -> bool {
        match &self.previous {
            Some(_) => true,
            _ => false,
        }
    }
}

impl<T> IntoIterator for ListData<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

#[derive(Debug)]
pub struct RequestFailed(pub reqwest::blocking::Response);

impl fmt::Display for RequestFailed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request failed")
    }
}

impl Error for RequestFailed {}
