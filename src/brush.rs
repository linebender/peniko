// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Color, Gradient, Image};

/// Describes the color content of a filled or stroked shape.
///
/// See also [`BrushRef`] which can be used to avoid allocations.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Brush {
    /// Solid color brush.
    Solid(Color),
    /// Gradient brush.
    Gradient(Gradient),
    /// Image brush.
    Image(Image),
}

impl From<Color> for Brush {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

impl From<Gradient> for Brush {
    fn from(g: Gradient) -> Self {
        Self::Gradient(g)
    }
}

impl From<Image> for Brush {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::Solid(Color::default())
    }
}

impl Brush {
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
                Self::Gradient(mut gradient) => {
                    gradient
                        .stops
                        .iter_mut()
                        .for_each(|stop| *stop = stop.multiply_alpha(alpha));
                    gradient.into()
                }
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BrushRef<'a> {
    /// Solid color brush.
    Solid(Color),
    /// Gradient brush.
    Gradient(&'a Gradient),
    /// Image brush.
    Image(&'a Image),
}

impl BrushRef<'_> {
    /// Converts the reference to an owned brush.
    #[must_use]
    pub fn to_owned(&self) -> Brush {
        match self {
            Self::Solid(color) => Brush::Solid(*color),
            Self::Gradient(gradient) => Brush::Gradient((*gradient).clone()),
            Self::Image(image) => Brush::Image((*image).clone()),
        }
    }
}

impl From<Color> for BrushRef<'_> {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl<'a> From<&'a Color> for BrushRef<'_> {
    fn from(color: &'a Color) -> Self {
        Self::Solid(*color)
    }
}

impl<'a> From<&'a Gradient> for BrushRef<'a> {
    fn from(gradient: &'a Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl<'a> From<&'a Image> for BrushRef<'a> {
    fn from(image: &'a Image) -> Self {
        Self::Image(image)
    }
}

impl<'a> From<&'a Brush> for BrushRef<'a> {
    fn from(brush: &'a Brush) -> Self {
        match brush {
            Brush::Solid(color) => Self::Solid(*color),
            Brush::Gradient(gradient) => Self::Gradient(gradient),
            Brush::Image(image) => Self::Image(image),
        }
    }
}

/// Defines how a brush is extended when the content does not
/// fill a shape.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Extend {
    /// Extends the image by repeating the edge color of the brush.
    #[default]
    Pad,
    /// Extends the image by repeating the brush.
    Repeat,
    /// Extends the image by reflecting the brush.
    Reflect,
}
