#![warn(clippy::all, rust_2018_idioms)]
#![allow(mixed_script_confusables)]
#![allow(clippy::needless_return)]

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub(crate) const REPOSITORY: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

const GRID_OFFSET: f32 = -5.;

const GAP: f32 = 3.;
const UI_WIDTH: f32 = 170.;
const PADDING: f32 = 5.;
const CELL_SIZE: f32 = 18.;

// #[allow(dead_code)] // TODO: remove when time comes
mod app;
mod node;
mod components;

pub use app::GridPathFinder;
