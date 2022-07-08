#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
extern crate serde_aux;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

mod account;
mod auth;
mod book;
mod client;
mod ledger;
mod orders;
mod ticker;
mod trades;

pub mod api;
pub mod candles;
pub mod currency;
pub mod errors;
pub mod events;
pub mod funding;
pub mod pairs;
pub mod precision;
pub mod websockets;
