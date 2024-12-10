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
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![expect(
    clippy::exhaustive_enums,
    reason = "Most of the enums are correctly exhaustive as this is a vocabulary crate."
)]

mod blend;
mod blob;
mod brush;
mod font;
mod gradient;
mod image;
mod style;

#[cfg(feature = "bytemuck")]
mod impl_bytemuck;

/// Re-export of the color library.
pub use color;

/// Re-export of the kurbo 2D curve library.
pub use kurbo;

pub use blend::{BlendMode, Compose, Mix};
pub use blob::{Blob, WeakBlob};
pub use brush::{Brush, BrushRef, Extend};
pub use font::Font;
pub use gradient::{ColorStop, ColorStops, ColorStopsSource, Gradient, GradientKind};
pub use image::{Format, Image};
pub use style::{Fill, Style, StyleRef};

/// A convenient alias for the color type used for [`Brush`].
pub type Color = color::AlphaColor<color::Srgb>;

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
