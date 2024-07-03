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
    /// Returns the brush with the alpha component multiplied by the specified
    /// factor.
    #[must_use]
    pub fn with_alpha_factor(&self, alpha: f32) -> Self {
        let result: BrushRef = self.into();
        result.with_alpha_factor(alpha)
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

impl<'a> BrushRef<'a> {
    /// Converts the reference to an owned brush.
    #[must_use]
    pub fn to_owned(&self) -> Brush {
        match self {
            Self::Solid(color) => Brush::Solid(*color),
            Self::Gradient(gradient) => Brush::Gradient((*gradient).clone()),
            Self::Image(image) => Brush::Image((*image).clone()),
        }
    }

    /// Returns the brush with the alpha component multiplied by the specified
    /// factor.
    #[must_use]
    pub fn with_alpha_factor(&self, alpha: f32) -> Brush {
        if alpha == 1.0 {
            self.to_owned()
        } else {
            match *self {
                BrushRef::Solid(color) => color.with_alpha_factor(alpha).into(),
                BrushRef::Gradient(gradient) => Brush::Gradient(Gradient {
                    kind: gradient.kind,
                    extend: gradient.extend,
                    stops: gradient
                        .stops
                        .iter()
                        .map(|stop| stop.with_alpha_factor(alpha))
                        .collect(),
                }),
                BrushRef::Image(image) => image.clone().with_alpha_factor(alpha).into(),
            }
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
