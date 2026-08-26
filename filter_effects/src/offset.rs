// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Geometric offset/translation.
///
/// Shifts the input image by the specified offset. Useful for creating
/// shadow effects or positioning elements in a filter graph.
///
/// See [`FilterPrimitive::Offset`](crate::FilterPrimitive::Offset).
#[derive(Debug, Clone, PartialEq)]
pub struct Offset {
    /// Horizontal offset in pixels. Positive values shift right.
    pub dx: f32,
    /// Vertical offset in pixels. Positive values shift down.
    pub dy: f32,
}

impl Offset {
    /// Create offset with given values.
    pub const fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }
}

impl From<(f32, f32)> for Offset {
    fn from(value: (f32, f32)) -> Self {
        let (dx, dy) = value;
        Self { dx, dy }
    }
}
