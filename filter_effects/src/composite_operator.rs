// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Composite operators for combining filter inputs.
///
/// These are the Porter-Duff compositing operators used to combine two images.
/// Each operator defines how the source (input 1) and destination (input 2)
/// are combined based on their color and alpha values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompositeOperator {
    /// Source over destination (standard alpha blending).
    ///
    /// The source is composited over the destination. This is the most common
    /// blending mode where source alpha determines visibility.
    Over,
    /// Source in destination (intersection).
    ///
    /// The source is only visible where the destination is opaque.
    /// Result alpha = `source_alpha` × `dest_alpha`.
    In,
    /// Source out destination (subtract).
    ///
    /// The source is only visible where the destination is transparent.
    /// Useful for masking/cutting out regions.
    Out,
    /// Source atop destination.
    ///
    /// Source is composited over destination, but only where destination is opaque.
    Atop,
    /// Source XOR destination (exclusive or).
    ///
    /// Shows source where destination is transparent and vice versa,
    /// but not where both are opaque.
    Xor,
    /// Arithmetic combination with custom coefficients.
    ///
    /// Custom linear combination: result = k1*src*dst + k2*src + k3*dst + k4.
    /// Allows creating custom compositing operations beyond the standard Porter-Duff set.
    Arithmetic {
        /// Coefficient k1 for the (source * destination) term.
        k1: f32,
        /// Coefficient k2 for the source term.
        k2: f32,
        /// Coefficient k3 for the destination term.
        k3: f32,
        /// Constant offset k4 added to the result.
        k4: f32,
    },
}
