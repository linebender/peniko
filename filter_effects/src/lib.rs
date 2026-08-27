// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Definitions for SVG filter effects.
//!
//! The `filter_effects` library builds on top of [`peniko`] and provides
//! the [`FilterPrimitive`] vocabulary type to specify transformations as part of SVG filter graphs.
//!
//! This library doesn't include any code for computing these effects on the CPU or on the GPU,
//! and doesn't include a definition of a filter graph.
//! It only defines the effects themselves, and lets consumers decide how to compose
//! and apply them.
//!
//! [`peniko`]: https://crates.io/crates/peniko

// LINEBENDER LINT SET - lib.rs - v4
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

extern crate alloc;

mod color_channel;
mod composite_operator;
mod convolution_kernel;
mod diffuse_lighting;
mod displacement_map;
mod drop_shadow;
mod edge_mode;
mod filter_primitive;
mod gaussian_blur;
mod light_source;
mod morphology;
mod offset;
mod specular_lighting;
mod turbulence;

pub use color_channel::*;
pub use composite_operator::*;
pub use convolution_kernel::*;
pub use diffuse_lighting::*;
pub use displacement_map::*;
pub use drop_shadow::*;
pub use edge_mode::*;
pub use filter_primitive::*;
pub use gaussian_blur::*;
pub use light_source::*;
pub use morphology::*;
pub use offset::*;
pub use specular_lighting::*;
pub use turbulence::*;

/// Re-export of the peniko library.
pub use peniko;
