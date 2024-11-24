
#![warn(clippy::all, rust_2018_idioms)]
#![allow(mixed_script_confusables)]

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub(crate) const REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

mod app;
mod node;
pub use app::GridPathFinder;
pub use rand::Rng;
