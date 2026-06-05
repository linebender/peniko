// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::color::{AlphaColor, Srgb};
use crate::kurbo::Rect;

/// A blurred, rounded rectangle.
///
/// May be used in renderer APIs, as a fast path to render shadows for rounded rectangles.
#[derive(Debug)]
pub struct BlurredRoundedRectangle {
    /// The base rectangle to use for the blur effect.
    pub rect: Rect,
    /// The color of the blurred rectangle.
    pub color: AlphaColor<Srgb>,
    /// The radius of the rounded rectangle's corners.
    pub radius: f32,
    /// The standard deviation of the blur effect.
    pub std_dev: f32,
}
