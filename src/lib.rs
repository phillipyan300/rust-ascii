//! Rust ASCII Image Converter Library
//! 
//! A library for converting images to ASCII art with various resampling methods
//! and output formats including HTML visualization.

pub mod image_processing;
pub mod html_generation;
pub mod cli_utils;

// Re-export main functionality for easy use
pub use image_processing::{image_to_ascii, select_filter, select_ramp};
pub use html_generation::{ascii_to_html, HtmlConfig};
pub use cli_utils::*;
