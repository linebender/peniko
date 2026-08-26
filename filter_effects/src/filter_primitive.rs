// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::vec::Vec;

use peniko::Mix;
use peniko::color::{AlphaColor, Srgb};

use crate::{
    CompositeOperator, ConvolutionKernel, DiffuseLighting, DisplacementMap, DropShadow,
    GaussianBlur, Morphology, Offset, SpecularLighting, Turbulence,
};

/// Low-level definition of a visual effect that should be applied to zero, one or more input images.
///
/// This only stores the definition of the transformation to be applied, not inputs,
/// outputs, texture resolution, or any other kind of filter graph data.
///
/// Most filters expect one input image.
/// Some filters expect two inputs.
/// [`Self::Flood`] and [`Self::Image`] are essentially sources expect no input.
/// [`Self::Merge`] expects an arbitrary number of inputs.
///
/// Filter definitions match SVG filter primitives.
/// See [Filter Effects Module Level 1 § 9.1](https://drafts.csswg.org/filter-effects/#FilterPrimitivesOverviewIntro) for more info.
#[derive(Debug, Clone, PartialEq)]
pub enum FilterPrimitive {
    /// Blend two inputs using an imaging software blending mode.
    ///
    /// The [`BlendMode`](peniko::BlendMode) consists of the given color [`Mix`]
    /// applied with the [`Compose::SrcOver`](peniko::Compose::SrcOver) operator.
    ///
    /// See [Filter Effects Module Level 1 § 9.5](https://drafts.csswg.org/filter-effects/#feBlendElement).
    Blend(Mix),

    /// Apply a matrix transformation on the RGBA values of every inpux pixel.
    ///
    /// See [Filter Effects Module Level 1 § 9.6](https://drafts.csswg.org/filter-effects/#feColorMatrixElement).
    ColorMatrix {
        // TODO - Replace with color_operations::ColorMatrix
        // once https://github.com/linebender/color/pull/227 is merged
        /// 4x5 color transformation matrix: 4 rows (R,G,B,A) × 5 columns (R,G,B,A,offset).
        /// Each output channel is computed as a linear combination of input channels plus offset.
        matrix: [f32; 20],
    },

    /// Perform per-channel remapping on the RGBA values of every inpux pixel.
    ///
    /// See [Filter Effects Module Level 1 § 9.7](https://drafts.csswg.org/filter-effects/#feComponentTransferElement).
    ComponentTransfer {
        // TODO - Replace with color_operations::ComponentTransfer
        // once https://github.com/linebender/color/pull/227 is merged
        /// Transfer function applied to the red channel (None = identity).
        red_function: Option<TransferFunction>,
        /// Transfer function applied to the green channel (None = identity).
        green_function: Option<TransferFunction>,
        /// Transfer function applied to the blue channel (None = identity).
        blue_function: Option<TransferFunction>,
        /// Transfer function applied to the alpha channel (None = identity).
        alpha_function: Option<TransferFunction>,
    },

    /// Combine two inputs using Porter-Duff compositing operations.
    ///
    /// Uses standard operators (over, in, out, atop, xor) or custom arithmetic combination.
    ///
    /// See [Filter Effects Module Level 1 § 9.8](https://drafts.csswg.org/filter-effects/#feCompositeElement).
    Composite(CompositeOperator),

    /// Apply convolution kernel to input image.
    ///
    /// Each output pixel is a result of multiplying the input pixels and its neighbors
    /// by the convolution matrix.
    /// Enables effects like sharpening, edge detection, embossing, and custom filters.
    ///
    /// See [Filter Effects Module Level 1 § 9.9](https://drafts.csswg.org/filter-effects/#feConvolveMatrixElement).
    ConvolveMatrix(ConvolutionKernel),

    /// Light an image using the alpha channel as a bump map.
    ///
    /// Computes diffuse (matte) reflection from a light source on that bump map.
    /// The computed light map can be combined with a texture image using the
    /// [`Composite`](Self::Composite) operation.
    ///
    /// See [Filter Effects Module Level 1 § 9.10](https://drafts.csswg.org/filter-effects/#feDiffuseLightingElement).
    DiffuseLighting(DiffuseLighting),

    /// Displace pixels using a displacement map.
    ///
    /// Uses the color values from a second input as vectors to spatially displace
    /// pixels in the primary input, creating warping and distortion effects.
    ///
    /// See [Filter Effects Module Level 1 § 9.11](https://drafts.csswg.org/filter-effects/#feDisplacementMapElement).
    DisplacementMap(DisplacementMap),

    /// Create a drop shadow of the input image.
    ///
    /// Blurs the input's alpha channel, offsets it, composes it with the original.
    /// This is a compound operation that combines multiple primitive operations into one.
    ///
    /// See [Filter Effects Module Level 1 § 9.12](https://drafts.csswg.org/filter-effects/#feDropShadowElement).
    DropShadow(DropShadow),

    /// Create a rectangle filled with the specified color
    ///
    /// Typically used as input to other filter operations (e.g., for colored shadows).
    ///
    /// See [Filter Effects Module Level 1 § 9.13](https://drafts.csswg.org/filter-effects/#feFloodElement).
    Flood(AlphaColor<Srgb>),

    /// Perform a Gaussian blur on the input image.
    ///
    /// See [Filter Effects Module Level 1 § 9.14](https://drafts.csswg.org/filter-effects/#feGaussianBlurElement).
    GaussianBlur(GaussianBlur),

    /// Reference an external image as filter input.
    ///
    /// Allows using pre-existing images (from an atlas or resource) as
    /// input to filter operations, useful for texturing and overlays.
    ///
    /// The id is an arbitrary integer; how the integer is interpreted is defined
    /// by whichever framework consumes this filter.
    ///
    /// See [Filter Effects Module Level 1 § 9.15](https://drafts.csswg.org/filter-effects/#feImageElement).
    Image(ImageId),

    /// Composite input images on top of each other using the
    /// [`Over`](CompositeOperator::Over) operator.
    ///
    /// See [Filter Effects Module Level 1 § 9.16](https://drafts.csswg.org/filter-effects/#feMergeElement).
    Merge,

    /// Performs "fattening" or "thinning" of input image.
    ///
    /// Useful for creating outline effects or cleaning up edges.
    ///
    /// See [Filter Effects Module Level 1 § 9.17](https://drafts.csswg.org/filter-effects/#feMorphologyElement).
    Morphology(Morphology),

    /// Translate the input image by the specified offset.
    ///
    /// This should be interpreted as translating the input layer's pixels,
    /// not the input layer itself.
    /// A large translation may result in an empty image, as all the pixels are
    /// translated out of bounds.
    ///
    /// See [Filter Effects Module Level 1 § 9.18](https://drafts.csswg.org/filter-effects/#feOffsetElement).
    Offset(Offset),

    /// Light an image using the alpha channel as a bump map.
    ///
    /// Computes specular (shiny) reflection highlights from a light source on that bump map.
    ///
    /// See [Filter Effects Module Level 1 § 9.19](https://drafts.csswg.org/filter-effects/#feSpecularLightingElement).
    SpecularLighting(SpecularLighting),

    /// Tile the input to fill the filter region.
    ///
    /// Repeats the input image to fill the entire filter primitive subregion,
    /// creating a tiling/repeating pattern.
    ///
    /// See [Filter Effects Module Level 1 § 9.20](https://drafts.csswg.org/filter-effects/#feTileElement).
    Tile,

    /// Generate Perlin noise/turbulence patterns.
    ///
    /// Creates procedural noise patterns useful for textures, clouds,
    /// marble effects, and other organic-looking randomness.
    ///
    /// See [Filter Effects Module Level 1 § 9.21](https://drafts.csswg.org/filter-effects/#feTurbulenceElement).
    Turbulence(Turbulence),
}

/// Arbitrary integer representing an index into some imagee storing system.
pub type ImageId = u64;

// ---

// TODO - Remove once https://github.com/linebender/color/pull/227 is merged
/// Transfer functions for component transfer operations.
///
/// These functions map input color channel values to output values,
/// enabling gamma correction, color grading, and custom color curves.
/// Input and output values are typically in the range [0, 1].
#[derive(Debug, Clone, PartialEq)]
pub enum TransferFunction {
    /// Identity function (output = input, no change).
    Identity,
    /// Table lookup with linear interpolation.
    ///
    /// Maps input values using a lookup table with linear interpolation between entries.
    /// Input 0.0 maps to values\[0\], 1.0 maps to values\[n-1\], intermediate values interpolate.
    Table {
        /// Lookup table values defining the transfer curve.
        /// More values provide smoother curves. Minimum 2 values required.
        values: Vec<f32>,
    },
    /// Discrete step function (posterization).
    ///
    /// Maps input to discrete output values without interpolation, creating step/banding effects.
    /// Each segment gets a constant output value from the table.
    Discrete {
        /// Step values for each discrete output level.
        /// Input range is divided into len(values) segments, each mapping to one value.
        values: Vec<f32>,
    },
    /// Linear function: output = slope × input + intercept.
    ///
    /// Simple linear transformation of the input value.
    Linear {
        /// Slope coefficient (rate of change).
        slope: f32,
        /// Intercept offset (constant added to result).
        intercept: f32,
    },
    /// Gamma correction: output = amplitude × input^exponent + offset.
    ///
    /// Applies power-law transformation, commonly used for gamma correction and
    /// adjusting midtone brightness without affecting blacks or whites.
    Gamma {
        /// Amplitude multiplier applied to the result.
        amplitude: f32,
        /// Gamma exponent (< 1 brightens, > 1 darkens midtones).
        exponent: f32,
        /// Offset added to the final result.
        offset: f32,
    },
}
