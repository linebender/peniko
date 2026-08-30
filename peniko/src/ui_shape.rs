// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo::{BezPath, PathEl, Rect, RoundedRect, Shape};

/// Shape that can be an arbitrary [`BezPath`], but is usually an axis-aligned rectangle.
///
/// This is useful for APIs that want to accept arbitrary [`Shape`]s,
/// but have a fast path for [`Rect`] or [`RoundedRect`].
#[derive(Clone, Debug, PartialEq)]
pub enum UiShape {
    /// Axis-aligned rectangle.
    Rect(Rect),
    /// Axis-aligned rounded rectangle.
    RoundedRect(RoundedRect),
    /// General path.
    Path(BezPath),
}

#[doc(hidden)]
#[expect(missing_debug_implementations, reason = "Internal type")]
#[expect(unnameable_types, reason = "Internal type")]
#[expect(clippy::large_enum_variant, reason = "Type will likely be inlined")]
pub enum UiShapePathIter<'i> {
    Rect(<Rect as Shape>::PathElementsIter<'i>),
    RoundedRect(<RoundedRect as Shape>::PathElementsIter<'i>),
    Path(<BezPath as Shape>::PathElementsIter<'i>),
}

impl UiShape {
    /// Is this clip geometry finite?
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Rect(r) => r.is_finite(),
            Self::RoundedRect(rr) => rr.is_finite(),
            Self::Path(p) => p.is_finite(),
        }
    }

    /// Is this clip geometry NaN?
    pub fn is_nan(&self) -> bool {
        match self {
            Self::Rect(r) => r.is_nan(),
            Self::RoundedRect(rr) => rr.is_nan(),
            Self::Path(p) => p.is_nan(),
        }
    }
}

impl Shape for UiShape {
    type PathElementsIter<'i>
        = UiShapePathIter<'i>
    where
        Self: 'i;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        match self {
            Self::Rect(rect) => UiShapePathIter::Rect(rect.path_elements(tolerance)),
            Self::RoundedRect(rounded_rect) => {
                UiShapePathIter::RoundedRect(rounded_rect.path_elements(tolerance))
            }
            Self::Path(bez_path) => UiShapePathIter::Path(bez_path.path_elements(tolerance)),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Self::Rect(rect) => rect.area(),
            Self::RoundedRect(rounded_rect) => rounded_rect.area(),
            Self::Path(bez_path) => bez_path.area(),
        }
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        match self {
            Self::Rect(rect) => rect.perimeter(accuracy),
            Self::RoundedRect(rounded_rect) => rounded_rect.perimeter(accuracy),
            Self::Path(bez_path) => bez_path.perimeter(accuracy),
        }
    }

    fn winding(&self, pt: kurbo::Point) -> i32 {
        match self {
            Self::Rect(rect) => rect.winding(pt),
            Self::RoundedRect(rounded_rect) => rounded_rect.winding(pt),
            Self::Path(bez_path) => bez_path.winding(pt),
        }
    }

    fn bounding_box(&self) -> Rect {
        match self {
            Self::Rect(rect) => rect.bounding_box(),
            Self::RoundedRect(rounded_rect) => rounded_rect.bounding_box(),
            Self::Path(bez_path) => bez_path.bounding_box(),
        }
    }
}

impl<'i> Iterator for UiShapePathIter<'i> {
    type Item = PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            UiShapePathIter::Rect(iter) => iter.next(),
            UiShapePathIter::RoundedRect(iter) => iter.next(),
            UiShapePathIter::Path(iter) => iter.next(),
        }
    }

    // TODO - Add try_fold impl once Try trait stabilizes.
}
