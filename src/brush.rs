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

mod color;
mod gradient;
mod image;

pub use color::Color;
pub use gradient::{
    ColorStop, ColorStops, ColorStopsSource, LinearGradient, RadialGradient, SweepGradient,
};
pub use image::Format;

/// Describes the color content of a filled or stroked shape.
#[derive(Clone, PartialEq, Debug)]
pub enum Brush {
    /// Solid color brush.
    Solid(Color),
    /// Linear gradient brush.
    LinearGradient(LinearGradient),
    /// Radial gradient brush.
    RadialGradient(RadialGradient),
    /// Sweep gradient brush.
    SweepGradient(SweepGradient),
}

impl From<Color> for Brush {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

impl From<LinearGradient> for Brush {
    fn from(g: LinearGradient) -> Self {
        Self::LinearGradient(g)
    }
}

impl From<RadialGradient> for Brush {
    fn from(g: RadialGradient) -> Self {
        Self::RadialGradient(g)
    }
}

impl From<SweepGradient> for Brush {
    fn from(g: SweepGradient) -> Self {
        Self::SweepGradient(g)
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
    /// Linear gradient brush.
    LinearGradient(&'a LinearGradient),
    /// Radial gradient brush.
    RadialGradient(&'a RadialGradient),
    /// Sweep gradient brush.
    SweepGradient(&'a SweepGradient),
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

impl<'a> From<&'a LinearGradient> for BrushRef<'a> {
    fn from(gradient: &'a LinearGradient) -> Self {
        Self::LinearGradient(gradient)
    }
}

impl<'a> From<&'a RadialGradient> for BrushRef<'a> {
    fn from(gradient: &'a RadialGradient) -> Self {
        Self::RadialGradient(gradient)
    }
}

impl<'a> From<&'a SweepGradient> for BrushRef<'a> {
    fn from(gradient: &'a SweepGradient) -> Self {
        Self::SweepGradient(gradient)
    }
}

impl<'a> From<&'a Brush> for BrushRef<'a> {
    fn from(brush: &'a Brush) -> Self {
        match brush {
            Brush::Solid(color) => Self::Solid(*color),
            Brush::LinearGradient(gradient) => Self::LinearGradient(gradient),
            Brush::RadialGradient(gradient) => Self::RadialGradient(gradient),
            Brush::SweepGradient(gradient) => Self::SweepGradient(gradient),
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
