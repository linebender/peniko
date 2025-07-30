// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::ImageBrushRef;

use super::{Gradient, ImageBrush};

use color::{AlphaColor, ColorSpace, DynamicColor, OpaqueColor, Srgb};

/// Describes the color content of a filled or stroked shape.
///
/// See also [`BrushRef`] which can be used to avoid allocations.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Brush {
    /// Solid color brush.
    Solid(AlphaColor<Srgb>),
    /// Gradient brush.
    Gradient(Gradient),
    /// Image brush.
    Image(ImageBrush),
}

impl<CS: ColorSpace> From<AlphaColor<CS>> for Brush {
    fn from(c: AlphaColor<CS>) -> Self {
        Self::Solid(c.convert())
    }
}

impl From<DynamicColor> for Brush {
    fn from(c: DynamicColor) -> Self {
        Self::Solid(c.to_alpha_color::<Srgb>())
    }
}

impl<CS: ColorSpace> From<OpaqueColor<CS>> for Brush {
    fn from(c: OpaqueColor<CS>) -> Self {
        Self::Solid(c.with_alpha(1.).convert())
    }
}

impl From<Gradient> for Brush {
    fn from(g: Gradient) -> Self {
        Self::Gradient(g)
    }
}

impl From<ImageBrush> for Brush {
    fn from(value: ImageBrush) -> Self {
        Self::Image(value)
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::Solid(AlphaColor::<Srgb>::TRANSPARENT)
    }
}

impl Brush {
    /// Returns the brush with the alpha component set to `alpha`.
    #[must_use]
    pub fn with_alpha(self, alpha: f32) -> Self {
        match self {
            Self::Solid(color) => color.with_alpha(alpha).into(),
            Self::Gradient(gradient) => gradient.with_alpha(alpha).into(),
            Self::Image(image) => image.with_alpha(alpha).into(),
        }
    }

    /// Returns the brush with the alpha component multiplied by `alpha`.
    /// The behaviour of this transformation is undefined if `alpha` is negative.
    ///
    /// If any resulting alphas would overflow, these currently saturate (to opaque).
    #[must_use]
    #[doc(alias = "with_alpha_factor")]
    #[track_caller]
    pub fn multiply_alpha(self, alpha: f32) -> Self {
        debug_assert!(
            alpha.is_finite() && alpha >= 0.0,
            "A non-finite or negative alpha ({alpha}) is meaningless."
        );
        if alpha == 1.0 {
            self
        } else {
            match self {
                Self::Solid(color) => color.multiply_alpha(alpha).into(),
                Self::Gradient(gradient) => gradient.multiply_alpha(alpha).into(),
                Self::Image(image) => image.multiply_alpha(alpha).into(),
            }
        }
    }
}

/// Reference to a [brush](Brush).
///
/// This is useful for methods that would like to accept brushes by reference. Defining
/// the type as `impl<Into<BrushRef>>` allows accepting types like `&LinearGradient`
/// directly without cloning or allocating.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BrushRef<'a> {
    /// Solid color brush.
    Solid(AlphaColor<Srgb>),
    /// Gradient brush.
    Gradient(&'a Gradient),
    /// Image brush.
    Image(ImageBrushRef<'a>),
}

impl BrushRef<'_> {
    /// Converts the reference to an owned brush.
    #[must_use]
    pub fn to_owned(&self) -> Brush {
        match self {
            Self::Solid(color) => Brush::Solid(*color),
            Self::Gradient(gradient) => Brush::Gradient((*gradient).clone()),
            Self::Image(image) => Brush::Image(image.to_owned()),
        }
    }
}

impl<CS: ColorSpace> From<AlphaColor<CS>> for BrushRef<'_> {
    fn from(color: AlphaColor<CS>) -> Self {
        Self::Solid(color.convert())
    }
}

impl From<DynamicColor> for BrushRef<'_> {
    fn from(color: DynamicColor) -> Self {
        Self::Solid(color.to_alpha_color::<Srgb>())
    }
}

impl<CS: ColorSpace> From<OpaqueColor<CS>> for BrushRef<'_> {
    fn from(color: OpaqueColor<CS>) -> Self {
        Self::Solid(color.with_alpha(1.).convert())
    }
}

impl<'a, CS: ColorSpace> From<&'a AlphaColor<CS>> for BrushRef<'_> {
    fn from(color: &'a AlphaColor<CS>) -> Self {
        Self::Solid((*color).convert())
    }
}

impl<'a> From<&'a DynamicColor> for BrushRef<'_> {
    fn from(color: &'a DynamicColor) -> Self {
        Self::Solid((*color).to_alpha_color::<Srgb>())
    }
}

impl<'a, CS: ColorSpace> From<&'a OpaqueColor<CS>> for BrushRef<'_> {
    fn from(color: &'a OpaqueColor<CS>) -> Self {
        Self::Solid((*color).with_alpha(1.).convert())
    }
}

impl<'a> From<&'a Gradient> for BrushRef<'a> {
    fn from(gradient: &'a Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl<'a> From<ImageBrushRef<'a>> for BrushRef<'a> {
    fn from(image: ImageBrushRef<'a>) -> Self {
        Self::Image(image)
    }
}

impl<'a> From<&'a ImageBrush> for BrushRef<'a> {
    fn from(image: &'a ImageBrush) -> Self {
        Self::Image(image.as_ref())
    }
}

impl<'a> From<&'a Brush> for BrushRef<'a> {
    fn from(brush: &'a Brush) -> Self {
        match brush {
            Brush::Solid(color) => Self::Solid(*color),
            Brush::Gradient(gradient) => Self::Gradient(gradient),
            Brush::Image(image) => Self::Image(image.as_ref()),
        }
    }
}

/// Defines how a brush is extended when the content does not
/// fill a shape.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Extend {
    /// Extends the image by repeating the edge color of the brush.
    #[default]
    Pad = 0,
    /// Extends the image by repeating the brush.
    Repeat = 1,
    /// Extends the image by reflecting the brush.
    Reflect = 2,
}
