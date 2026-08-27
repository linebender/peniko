// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::vec::Vec;

/// Convolution kernel for custom filtering operations.
///
/// Defines a square matrix of weights used for convolution-based image processing.
/// The kernel is applied to each pixel by multiplying surrounding pixels by the weights,
/// summing the results, dividing by the divisor, and adding the bias.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvolutionKernel {
    /// Kernel size (e.g., 3 for a 3×3 kernel, 5 for 5×5).
    /// The kernel must be square, so this defines both width and height.
    pub size: u32,
    /// Kernel weight values in row-major order.
    /// Length must equal size × size. Center of kernel is typically at (size/2, size/2).
    pub values: Vec<f32>,
    /// Normalization divisor applied to the convolution result.
    /// Common practice is to use the sum of all weights for averaging, or 1.0 otherwise.
    pub divisor: f32,
    /// Bias value added to the result after normalization.
    /// Useful for edge detection or emboss effects to shift the result range.
    pub bias: f32,
    /// Whether to preserve the alpha channel unchanged.
    /// If true, convolution only applies to RGB; if false, it applies to RGBA.
    pub preserve_alpha: bool,
}
