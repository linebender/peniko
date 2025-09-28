// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A Rust 2D graphics type library
//!
//! The `peniko` library builds on top of [`kurbo`] and [`color`] and provides a set of
//! generic types that define styles for rendering and composition.
//!
//! The name "peniko" is Esperanto for "brush" which is one family of types that the library
//! contains.
//!
//! [`kurbo`]: https://crates.io/crates/kurbo
//! [`color`]: https://crates.io/crates/color

// LINEBENDER LINT SET - lib.rs - v1
// See https://linebender.org/wiki/canonical-lints/
// These lints aren't included in Cargo.toml because they
// shouldn't apply to examples and tests
#![warn(unused_crate_dependencies)]
#![warn(clippy::print_stdout, clippy::print_stderr)]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![expect(
    clippy::exhaustive_enums,
    reason = "Most of the enums are correctly exhaustive as this is a vocabulary crate."
)]

mod blend;
mod brush;
mod gradient;
mod image;
mod style;

#[cfg(feature = "bytemuck")]
mod impl_bytemuck;

/// Re-export of the color library.
pub use color;

/// Re-export of the kurbo 2D curve library.
pub use kurbo;

/// Re-export of the linebender resource handle library types.
pub use linebender_resource_handle::{self, Blob, FontData, WeakBlob};

pub use blend::{BlendMode, Compose, Mix};
pub use brush::{Brush, BrushRef, Extend};
pub use gradient::{
    ColorStop, ColorStops, ColorStopsSource, Gradient, GradientKind, InterpolationAlphaSpace,
    LinearGradientPosition, RadialGradientPosition, SweepGradientPosition,
};
pub use image::{
    ImageAlphaType, ImageBrush, ImageBrushRef, ImageData, ImageFormat, ImageQuality, ImageSampler,
};
pub use style::{Fill, Style, StyleRef};

/// A convenient alias for the color type used for [`Brush`].
pub type Color = color::AlphaColor<color::Srgb>;
