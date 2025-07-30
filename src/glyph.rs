// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Normalized variation coordinate in 2.14 fixed point format.
pub type NormalizedCoord = i16;

/// A glyph positioned in 2D space
#[derive(Copy, Clone, Debug)]
pub struct PositionedGlyph {
    /// The ID of the glyph within it's font
    pub id: u32,
    /// The x position of the glyph
    pub x: f32,
    /// The y position of the glyph
    pub y: f32,
}
