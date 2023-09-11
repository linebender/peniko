// Copyright 2022 The peniko authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

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

/// Defines the connection between two segments of a [stroke](Stroke).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Join {
    /// A straight line connecting the segments.
    Bevel,
    /// The segments are extended to their natural intersection point.
    Miter,
    /// An arc between the segments.
    Round,
}

/// Defines the shape to be drawn at the ends of a [stroke](Stroke).
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
    #[must_use]
    pub fn new(width: f32) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    /// Builder method for setting the join style.
    #[must_use]
    pub fn with_join(mut self, join: Join) -> Self {
        self.join = join;
        self
    }

    /// Builder method for setting the limit for miter joins.
    #[must_use]
    pub fn with_miter_limit(mut self, limit: f32) -> Self {
        self.miter_limit = limit;
        self
    }

    /// Builder method for setting the cap style for the start of the stroke.
    #[must_use]
    pub fn with_start_cap(mut self, cap: Cap) -> Self {
        self.start_cap = cap;
        self
    }

    /// Builder method for setting the cap style for the end of the stroke.
    #[must_use]
    pub fn with_end_cap(mut self, cap: Cap) -> Self {
        self.end_cap = cap;
        self
    }

    /// Builder method for setting the cap style.
    #[must_use]
    pub fn with_caps(mut self, cap: Cap) -> Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    /// Builder method for setting the dashing parameters.
    #[must_use]
    pub fn with_dashes<P>(mut self, offset: f32, pattern: P) -> Self
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
    #[must_use]
    pub fn with_scale(mut self, yes: bool) -> Self {
        self.scale = yes;
        self
    }
}

/// Collection of values representing lengths in a dash pattern.
pub type Dashes = SmallVec<[f32; 4]>;

/// Describes draw style-- either a [fill](Fill) or [stroke](Stroke).
///
/// See also [`StyleRef`] which can be used to avoid allocations.
#[derive(Clone, Debug)]
pub enum Style {
    /// Filled draw operation.
    Fill(Fill),
    /// Stroked draw operation.
    Stroke(Stroke),
}

impl From<Fill> for Style {
    fn from(fill: Fill) -> Self {
        Self::Fill(fill)
    }
}

impl From<Stroke> for Style {
    fn from(stroke: Stroke) -> Self {
        Self::Stroke(stroke)
    }
}

/// Reference to a [draw style](Style).
///
/// This is useful for methods that would like to accept draw styles by reference. Defining
/// the type as `impl<Into<DrawRef>>` allows accepting types like `&Stroke` or `Fill`
/// directly without cloning or allocating.
pub enum StyleRef<'a> {
    /// Filled draw operation.
    Fill(Fill),
    /// Stroked draw operation.
    Stroke(&'a Stroke),
}

impl<'a> StyleRef<'a> {
    /// Converts the reference to an owned draw.
    #[must_use]
    pub fn to_owned(&self) -> Style {
        match self {
            Self::Fill(fill) => Style::Fill(*fill),
            Self::Stroke(stroke) => Style::Stroke((*stroke).clone()),
        }
    }
}

impl From<Fill> for StyleRef<'_> {
    fn from(fill: Fill) -> Self {
        Self::Fill(fill)
    }
}

impl<'a> From<&'a Stroke> for StyleRef<'a> {
    fn from(stroke: &'a Stroke) -> Self {
        Self::Stroke(stroke)
    }
}

impl<'a> From<&'a Style> for StyleRef<'a> {
    fn from(draw: &'a Style) -> Self {
        match draw {
            Style::Fill(fill) => Self::Fill(*fill),
            Style::Stroke(stroke) => Self::Stroke(stroke),
        }
    }
}
