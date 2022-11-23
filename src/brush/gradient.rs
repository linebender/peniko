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

use super::{Color, Extend};
use core::hash::{Hash, Hasher};
use kurbo::Point;
use smallvec::SmallVec;

/// Offset and color of a transition point in a gradient.
#[derive(Copy, Clone, PartialOrd, Default, Debug)]
pub struct ColorStop {
    /// Normalized offset of the stop.
    pub offset: f32,
    /// Color at the specified offset.
    pub color: Color,
}

impl Hash for ColorStop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.to_bits().hash(state);
        self.color.hash(state);
    }
}

// Override PartialEq to use to_bits for the offset to match with the Hash impl
impl std::cmp::PartialEq for ColorStop {
    fn eq(&self, other: &Self) -> bool {
        self.offset.to_bits() == other.offset.to_bits() && self.color == other.color
    }
}

impl std::cmp::Eq for ColorStop {}

impl From<(f32, Color)> for ColorStop {
    fn from(pair: (f32, Color)) -> Self {
        Self {
            offset: pair.0,
            color: pair.1,
        }
    }
}

/// Collection of color stops.
pub type ColorStops = SmallVec<[ColorStop; 4]>;

/// Definition of a gradient that transitions between two or more colors along
/// a line.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct LinearGradient {
    pub start: Point,
    pub end: Point,
    pub extend: Extend,
    pub stops: ColorStops,
}

impl LinearGradient {
    /// Creates a new linear gradient for the specified start and end points.
    pub fn new(start: impl Into<Point>, end: impl Into<Point>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Builder method for setting the gradient extend mode.
    pub fn extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Builder method for setting the color stop collection.
    pub fn stops(mut self, stops: impl ColorStopsSource) -> Self {
        self.stops.clear();
        stops.collect_stops(&mut self.stops);
        self
    }
}

/// Definition of a gradient that transitions between two or more colors that
/// radiate from an origin.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct RadialGradient {
    /// Center of start circle.
    pub start_center: Point,
    /// Radius of start circle.
    pub start_radius: f32,
    /// Center of end circle.
    pub end_center: Point,
    /// Radius of end circle.
    pub end_radius: f32,
    /// Extend mode.
    pub extend: Extend,
    /// Color stop collection.
    pub stops: ColorStops,
}

impl RadialGradient {
    /// Creates a new radial gradient for the specified center point and radius.
    pub fn new(center: impl Into<Point>, radius: f32) -> Self {
        let center = center.into();
        Self {
            start_center: center,
            start_radius: 0.0,
            end_center: center,
            end_radius: radius,
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Builder method for setting the center and radius of the start circle.
    pub fn start_circle(mut self, center: impl Into<Point>, radius: f32) -> Self {
        self.start_center = center.into();
        self.start_radius = radius;
        self
    }

    /// Builder method for setting the gradient extend mode.
    pub fn extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Builder method for setting the color stop collection.
    pub fn stops(mut self, stops: impl ColorStopsSource) -> Self {
        self.stops.clear();
        stops.collect_stops(&mut self.stops);
        self
    }
}

/// Definition gradient that transitions between two or more colors that rotate
/// around a center point.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct SweepGradient {
    /// Center point.
    pub center: Point,
    /// Start angle of the sweep, counter-clockwise of the x-axis.
    pub start_angle: f32,
    /// End angle of the sweep, counter-clockwise of the x-axis.
    pub end_angle: f32,
    /// Extend mode.
    pub extend: Extend,
    /// Color stop collection.
    pub stops: ColorStops,
}

impl SweepGradient {
    /// Creates a new sweep gradient for the specified center point, start and
    /// end angles.
    pub fn new(center: impl Into<Point>, start_angle: f32, end_angle: f32) -> Self {
        Self {
            center: center.into(),
            start_angle,
            end_angle,
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Builder method for setting the gradient extend mode.
    pub fn extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Builder method for setting the color stop collection.
    pub fn stops(mut self, stops: impl ColorStopsSource) -> Self {
        self.stops.clear();
        stops.collect_stops(&mut self.stops);
        self
    }
}

/// Trait for types that represent a source of color stops.
pub trait ColorStopsSource {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>);
}

impl<T> ColorStopsSource for &'_ [T]
where
    T: Into<ColorStop> + Copy,
{
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        for &stop in *self {
            vec.push(stop.into());
        }
    }
}

impl<T, const N: usize> ColorStopsSource for [T; N]
where
    T: Into<ColorStop> + Copy,
{
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        for stop in *self {
            vec.push(stop.into());
        }
    }
}

impl ColorStopsSource for &'_ [Color] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            vec.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: *c,
            }));
        }
    }
}

impl<const N: usize> ColorStopsSource for [Color; N] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        (&self[..]).collect_stops(vec);
    }
}
