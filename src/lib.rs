// Copyright 2022 The peniko authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A Rust 2D graphics type library
//!
//! The `peniko` library builds on top of [`kurbo`] and provides a set of generic types that define
//! styles for rendering and composition.
//!
//! The name "peniko" is Esperanto for "brush" which is one family of types that the library
//! contains.
//!
//! [`kurbo`]: https://crates.io/crates/kurbo

#![warn(clippy::doc_markdown)]

mod blend;
mod blob;
mod brush;
mod color;
mod font;
mod gradient;
mod image;
mod style;

/// Re-export of the kurbo 2D curve library.
pub use kurbo;

pub use blend::{BlendMode, Compose, Mix};
pub use blob::{Blob, WeakBlob};
pub use brush::{Brush, BrushRef, Extend};
pub use color::Color;
pub use font::Font;
pub use gradient::{ColorStop, ColorStops, ColorStopsSource, Gradient, GradientKind};
pub use image::{Format, Image};
pub use style::{Fill, Style, StyleRef};
