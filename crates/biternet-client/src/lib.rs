//! SRun campus gateway client.

mod client;
mod config;
mod crypto;
mod models;
mod xencode;

pub use client::Client;
pub use config::{Config, DEFAULT_CAPTIVE_PORTAL_URL, DEFAULT_PORTAL_URL};
pub use models::{LoginState, PortalResponse};
