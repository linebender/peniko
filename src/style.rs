// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo::Stroke;

/// Describes the rule that determines the interior portion of a shape.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Fill {
    /// Non-zero fill rule.
    NonZero = 0,
    /// Even-odd fill rule.
    EvenOdd = 1,
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the bytemuck impl.
}

/// Describes draw style-- either a [fill](Fill) or [stroke](Stroke).
///
/// See also [`StyleRef`] which can be used to avoid allocations.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[expect(
    variant_size_differences,
    reason = "We don't expect this enum to be operated on in bulk."
)]
#[derive(Debug, Copy, Clone)]
pub enum StyleRef<'a> {
    /// Filled draw operation.
    Fill(Fill),
    /// Stroked draw operation.
    Stroke(&'a Stroke),
}

impl StyleRef<'_> {
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
