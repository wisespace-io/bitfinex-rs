#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces)]

#[macro_use] 
extern crate error_chain;

extern crate hex;
extern crate ring;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use] 
extern crate serde_derive;

mod book;
mod ticker;
mod trades;
mod orders;
mod account;
mod ledger;
mod auth;
mod client;

pub mod model;
pub mod candles;
pub mod api;
pub mod pairs;
pub mod currency;
pub mod precision;
pub mod websockets;
pub mod events;
pub mod errors;
