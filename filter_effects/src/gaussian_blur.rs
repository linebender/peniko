// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::EdgeMode;

/// Gaussian blur filter.
///
/// Applies a Gaussian blur using the specified standard deviation (σ).
/// The effective blur range (distance over which pixels are sampled) is
/// approximately 3 × `std_deviation`, as this captures ~99.7% of the
/// Gaussian distribution.
///
/// See [`FilterPrimitive::GaussianBlur`](crate::FilterPrimitive::GaussianBlur).
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianBlur {
    /// Standard deviation for the blur kernel. Larger values create more blur.
    /// Must be non-negative. A value of 0 means no blur.
    ///
    /// This directly corresponds to the σ (sigma) parameter in the Gaussian
    /// function. The visible blur effect extends approximately 3σ in each direction.
    ///
    /// TODO: Per the W3C specification, this should support separate x and y values.
    /// The spec allows `stdDeviation` to be either one number (applied to both axes)
    /// or two numbers (first for x-axis, second for y-axis). Currently only uniform
    /// blur is supported. Consider changing to `(f32, f32)` or a dedicated type.
    pub std_deviation: f32,
    /// Edge mode determining how pixels beyond the input bounds are handled.
    pub edge_mode: EdgeMode,
}
