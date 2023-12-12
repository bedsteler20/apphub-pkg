mod client;
mod proxy;
mod transaction;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub use client::Client;
pub use gio;
pub use glib;
pub use transaction::*;
