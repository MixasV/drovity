// Simplified proxy module for drovity
// Core proxy server implementation extracted from DroidGravity

pub mod server;
pub mod config;

pub use server::start_server;
pub use config::ProxyConfig;
