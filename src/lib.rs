//! # octopust
//!
//! A high-performance, idiomatic Rust client for the Octopus Energy API.
//!
//! ## Example
//!
//! ```no_run
//! use octopust::{Client, models::ProductQuery};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("API_KEY");
//!     let products = client.list_products(ProductQuery { ..Default::default() }).await.unwrap();
//!     println!("{:?}", products);
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod api;

pub use client::Client;
pub use error::OctopustError;