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
#[derive(Clone, Debug)]
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
