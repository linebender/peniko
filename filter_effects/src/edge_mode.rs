// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Edge mode for filter operations.
///
/// Determines how to extend the input image when filter operations require sampling
/// beyond the original image boundaries. This is particularly important for blur and
/// convolution operations near edges.
///
/// See: <https://drafts.fxtf.org/filter-effects/#element-attrdef-filter-primitive-edgemode>
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EdgeMode {
    /// Extend by duplicating edge pixels (clamp to edge).
    ///
    /// The input image is extended along each border by replicating the color values
    /// at the given edge of the input image. This prevents dark halos around edges.
    Duplicate,
    /// Extend by wrapping to the opposite edge (repeat/tile).
    ///
    /// The input image is extended by taking color values from the opposite edge,
    /// creating a tiling effect.
    Wrap,
    /// Extend by mirroring across the edge.
    ///
    /// The input image is extended by taking color values mirrored across the edge.
    /// This creates seamless continuation at boundaries.
    Mirror,
    /// Extend with transparent black (zeros).
    ///
    /// The input image is extended with pixel values of zero for R, G, B and A.
    /// This is the default and most common mode, creating natural fade-to-transparent edges.
    #[default]
    None,
}
