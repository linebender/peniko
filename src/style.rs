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

use core::borrow::Borrow;
use smallvec::SmallVec;

/// Describes the rule that determines the interior portion of a shape.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Fill {
    /// Non-zero fill rule.
    NonZero,
    /// Even-odd fill rule.
    EvenOdd,
}

/// Defines the connection between two segments of a stroke.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Join {
    /// A straight line connecting the segments.
    Bevel,
    /// The segments are extended to their natural intersection point.
    Miter,
    /// An arc between the segments.
    Round,
}

/// Defines the shape to be drawn at the ends of a stroke.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cap {
    /// Flat cap.
    Butt,
    /// Square cap with dimensions equal to half the stroke width.
    Square,
    /// Rounded cap with radius equal to half the stroke width.
    Round,
}

/// Describes the visual style of a stroke.
#[derive(Clone, Debug)]
pub struct Stroke {
    /// Width of the stroke.
    pub width: f32,
    /// Style for connecting segments of the stroke.
    pub join: Join,
    /// Limit for miter joins.
    pub miter_limit: f32,
    /// Style for capping the beginning of an open subpath.
    pub start_cap: Cap,
    /// Style for capping the end of an open subpath.
    pub end_cap: Cap,
    /// Lengths of dashes in alternating on/off order.
    pub dash_pattern: Dashes,
    /// Offset of the first dash.
    pub dash_offset: f32,
    /// True if the stroke width should be affected by the scale of a
    /// transform.
    pub scale: bool,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            width: 1.0,
            join: Join::Round,
            miter_limit: 4.0,
            start_cap: Cap::Round,
            end_cap: Cap::Round,
            dash_pattern: Default::default(),
            dash_offset: 0.0,
            scale: true,
        }
    }
}

impl Stroke {
    /// Creates a new stroke with the specified width.
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    /// Builder method for setting the join style.
    pub fn join(mut self, join: Join) -> Self {
        self.join = join;
        self
    }

    /// Builder method for setting the limit for miter joins.
    pub fn miter_limit(mut self, limit: f32) -> Self {
        self.miter_limit = limit;
        self
    }

    /// Builder method for setting the cap style for the start of the stroke.
    pub fn start_cap(mut self, cap: Cap) -> Self {
        self.start_cap = cap;
        self
    }

    /// Builder method for setting the cap style for the end of the stroke.
    pub fn end_cap(mut self, cap: Cap) -> Self {
        self.end_cap = cap;
        self
    }

    /// Builder method for setting the cap style.
    pub fn caps(mut self, cap: Cap) -> Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    /// Builder method for setting the dashing parameters.
    pub fn dash<P>(mut self, offset: f32, pattern: P) -> Self
    where
        P: IntoIterator,
        P::Item: Borrow<f32>,
    {
        self.dash_offset = offset;
        self.dash_pattern.clear();
        self.dash_pattern
            .extend(pattern.into_iter().map(|dash| *dash.borrow()));
        self
    }

    /// Builder method for setting whether or not the stroke should be affected
    /// by the scale of any applied transform.
    pub fn scale(mut self, yes: bool) -> Self {
        self.scale = yes;
        self
    }
}

/// Collection of values representing lengths in a dash pattern.
pub type Dashes = SmallVec<[f32; 4]>;
