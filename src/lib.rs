//! A basic library to scrape [MangaUpdates](http://www.mangaupdates.com) pages
//!
//! >Right now only some of MangaUpdates pages have been implemented
//!
//! ## Usage
//!
//! In the simplest form you can get series info like this
//!
//! ```no_run
//! # use mu_rs::{MuClient, SeriesId};
//! let client = MuClient::new();
//! let series = client.series(SeriesId(15));
//! ```
//!
//! See [SeriesId] for information on getting the series id from a page on mangaupdates

mod author;
mod client;




mod group;
mod publisher;
mod series;

pub use author::AuthorId;
pub use group::GroupId;
pub use publisher::PublisherId;
pub use series::{Series, SeriesId, SeriesSearchResult};

pub use client::MuClient;

fn get_id<T: AsRef<str>>(url: Option<T>) -> Option<usize> {
    let url = url?;
    let url = url.as_ref();
    let start = url.find("id=")? + 3;

    let id = url[start..].parse().ok()?;

    Some(id)
}
