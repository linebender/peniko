// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Morphological operations (dilate/erode).
///
/// Expands (dilate) or contracts (erode) the shapes in the input image.
/// Useful for creating outline effects or cleaning up edges.
///
/// See [`FilterPrimitive::Morphology`](crate::FilterPrimitive::Morphology).
#[derive(Debug, Clone, PartialEq)]
pub struct Morphology {
    /// Morphological operator determining whether to erode or dilate.
    pub operator: MorphologyOperator,
    /// Operation radius in pixels. Larger values create stronger effects.
    pub radius: f32,
}

/// Morphological operators for dilate/erode operations.
///
/// These operators modify the shape of objects by expanding or contracting them.
/// They work by examining neighborhoods of pixels and applying min/max operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphologyOperator {
    /// Erode operation (shrink/thin shapes).
    ///
    /// Makes objects smaller by removing pixels at the edges. Takes the minimum
    /// value in the neighborhood. Useful for removing noise or separating touching objects.
    Erode,
    /// Dilate operation (expand/thicken shapes).
    ///
    /// Makes objects larger by adding pixels at the edges. Takes the maximum
    /// value in the neighborhood. Useful for filling holes or connecting nearby objects.
    Dilate,
}
