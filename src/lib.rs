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
mod client;
mod ticker;
mod trades;
mod orders;
mod candles;
mod account;

pub mod api;
pub mod pairs;
pub mod currency;
pub mod precision;
pub mod websockets;
pub mod events;
pub mod errors;