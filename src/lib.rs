pub extern crate image;

pub(crate) mod bezier_area;
pub mod buffer;
pub mod compositor;
pub mod context;
pub mod draw_image;
pub mod fill_color;
pub mod fill_path;
pub mod fill_rule;
pub(crate) mod geometry;
pub mod image_crate_adapter;
pub mod k_curve;
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
pub mod writer;
