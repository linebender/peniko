// Copyright 2022 The peniko authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

use super::{Color, Gradient, Image};

/// Describes the color content of a filled or stroked shape.
#[derive(Clone, PartialEq, Debug)]
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

/// Reference to a brush.
///
/// This is useful for methods that would like to accept brushes by reference. Defining
/// the type as `impl<Into<BrushRef>>` allows accepting types like `&LinearGradient`
/// directly without cloning or allocating.
#[derive(Clone, PartialEq, Debug)]
pub enum BrushRef<'a> {
    /// Solid color brush.
    Solid(Color),
    /// Gradient brush.
    Gradient(&'a Gradient),
    /// Image brush.
    Image(&'a Image),
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
pub enum Extend {
    /// Extends the image by repeating the edge color of the brush.
    #[default]
    Pad,
    /// Extends the image by repeating the brush.
    Repeat,
    /// Extends the image by reflecting the brush.
    Reflect,
}
