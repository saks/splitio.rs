mod client;
mod condition;
mod split;
mod storage;
mod treatment;
mod errors;

pub use client::Client;
pub use treatment::Treatment;

pub struct Factory;

impl Factory {
    // fn with_storage()
}
