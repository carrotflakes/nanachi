//! # Nanachi
//! Nanachi is a 2D graphics library for Rust.
//!
//! ## Features
//! - path filling and stroking
//! - linear gradients, radial gradients and tiled images coloring
//! - 24 color-composition modes
//! - anti-aliasing (or not use)
//! - affine transformation
//! - κ-curve implementation

#[cfg(feature = "image-crate")]
pub extern crate image;

pub mod buffer;
pub mod compositor;
pub mod context;
pub mod draw_image;
pub mod fill_color;
pub mod fill_rule;
pub(crate) mod geometry;
#[cfg(feature = "image-crate")]
pub mod image_crate_adapter;
pub mod interpolation;
pub mod k_curve;
#[cfg(feature = "image-crate")]
pub mod legacy_draw;
pub mod legacy_path;
pub mod legacy_primitives;
pub mod matrix;
pub(crate) mod models;
pub mod path;
pub mod path_builder;
#[cfg(feature = "path-data-notation")]
pub mod path_data_notation;
pub mod path_flatten;
pub mod path_outline;
pub mod path_transform;
pub mod pixel;
pub mod point;
pub mod rasterizer;
pub mod writer;
