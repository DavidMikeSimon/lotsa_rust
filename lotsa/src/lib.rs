#![feature(test)]
#![warn(unused)]
#![warn(future_incompatible)]
#![warn(clippy::all)]

#[cfg(feature = "client")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(clippy::useless_attribute, unused)]
#[macro_use]
extern crate maplit;

#[allow(clippy::useless_attribute, unused)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_big_array;

#[cfg(test)]
extern crate test;

pub mod block;
pub mod chunk;
pub mod chunk_pos;
pub mod debug;
pub mod dirtiness_tracker;
pub mod life;
pub mod query;
pub mod relative_pos;
pub mod sim;
pub mod unique_descrip;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;
