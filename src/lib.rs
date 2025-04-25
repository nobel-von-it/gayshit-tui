pub mod app;
pub mod ui;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
