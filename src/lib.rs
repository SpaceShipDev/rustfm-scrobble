//! # rustfm-scrobble
//!
//! Client for the Last.fm Scrobble API v2.0.

extern crate reqwest;
extern crate crypto;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod scrobbler;
mod client;
mod auth;
mod dto;

pub use scrobbler::Scrobbler;
