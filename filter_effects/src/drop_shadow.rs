// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use peniko::color::{AlphaColor, Srgb};

use crate::EdgeMode;

/// Drop shadow effect (compound primitive).
///
/// Creates a drop shadow by blurring the input's alpha channel, offsetting it,
/// and compositing it with the original. This is a compound operation that
/// combines multiple primitive operations into one.
///
/// See [`FilterPrimitive::DropShadow`](crate::FilterPrimitive::DropShadow).
#[derive(Debug, Clone, PartialEq)]
pub struct DropShadow {
    /// Horizontal offset of the shadow in pixels. Positive values shift right.
    pub dx: f32,
    /// Vertical offset of the shadow in pixels. Positive values shift down.
    pub dy: f32,
    /// Blur standard deviation for the shadow. Larger values create softer shadows.
    pub std_deviation: f32,
    /// Shadow color with alpha channel. Alpha controls shadow opacity.
    pub color: AlphaColor<Srgb>,
    /// Edge mode for handling boundaries during blur operation.
    /// Default is `EdgeMode::None` per SVG spec.
    pub edge_mode: EdgeMode,
}
