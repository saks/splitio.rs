mod client;
mod condition;
mod errors;
mod matcher;
mod split;
mod splitter;
pub mod storage;
mod treatment;

pub use client::Client;
pub use treatment::Treatment;

pub struct Factory;

impl Factory {
    // fn with_storage()
}
