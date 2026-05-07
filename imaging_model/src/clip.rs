// Copyright 2026 the Imaging Model Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo::{Affine, BezPath, Rect, RoundedRect, Shape as _, Stroke};
use peniko::Fill;

/// Geometry payload used by [`Clip`] operations.
///
/// This enum captures the shape before any operation-specific interpretation.
/// For [`Clip::Fill`], the geometry interior is used directly. For
/// [`Clip::Stroke`], the geometry is first stroked and the resulting outline is
/// used as the clip region.
#[derive(Clone, Debug, PartialEq)]
pub enum ClipGeometry {
    /// Axis-aligned rectangle.
    Rect(Rect),
    /// Axis-aligned rounded rectangle.
    RoundedRect(RoundedRect),
    /// General path.
    Path(BezPath),
}

impl ClipGeometry {
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

    /// Convert a geometry payload into a path.
    pub fn to_path(&self, tolerance: f64) -> BezPath {
        match self {
            Self::Rect(r) => r.to_path(tolerance),
            Self::RoundedRect(rr) => rr.to_path(tolerance),
            Self::Path(p) => p.clone(),
        }
    }

    /// Conservative axis-aligned bounds of this geometry.
    ///
    /// Returns `None` if the geometry bounds are non-finite.
    pub fn bounds(&self) -> Option<Rect> {
        let bounds = match self {
            Self::Rect(r) => *r,
            Self::RoundedRect(rr) => rr.bounding_box(),
            Self::Path(p) => p.bounding_box(),
        };
        bounds.is_finite().then_some(bounds)
    }
}

/// A clip operation pushed onto the non-isolated clip stack.
#[derive(Clone, Debug, PartialEq)]
pub enum Clip {
    /// Clip to the fill region of a shape.
    Fill {
        /// Transform applied to the clip shape.
        ///
        /// This does not affect subsequent draws; it only affects how the clip shape is interpreted.
        transform: Affine,
        /// Shape used to define the clip region.
        shape: ClipGeometry,
        /// Fill rule used to determine the interior for path clips.
        fill_rule: Fill,
    },
    /// Clip to the filled outline of a stroked shape.
    Stroke {
        /// Transform applied to the clip shape.
        ///
        /// This does not affect subsequent draws; it only affects how the clip shape is interpreted.
        transform: Affine,
        /// Shape whose stroked outline defines the clip region.
        shape: ClipGeometry,
        /// Stroke style used to compute the outline (including dashes).
        stroke: Stroke,
    },
}

impl Clip {
    /// Conservative axis-aligned bounds of this clip after applying transform.
    ///
    /// Returns `None` if any inputs needed for bounds are non-finite.
    ///
    /// ```
    /// use imaging_model::{Clip, ClipGeometry, Fill};
    /// use kurbo::{Affine, Rect};
    ///
    /// let clip = Clip::Fill {
    ///     transform: Affine::translate((5.0, -2.0)),
    ///     shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 2.0, 3.0)),
    ///     fill_rule: Fill::NonZero,
    /// };
    ///
    /// assert_eq!(clip.bounds(), Some(Rect::new(5.0, -2.0, 7.0, 1.0)));
    /// ```
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            Self::Fill {
                transform, shape, ..
            } => shape
                .bounds()
                .map(|bounds| transform.transform_rect_bbox(bounds))
                .filter(|bounds| bounds.is_finite()),
            Self::Stroke {
                transform,
                shape,
                stroke,
            } => shape
                .bounds()
                .and_then(|bounds| stroke_bounds(bounds, stroke))
                .map(|bounds| transform.transform_rect_bbox(bounds))
                .filter(|bounds| bounds.is_finite()),
        }
    }

    /// Is this clip finite?
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Fill {
                transform, shape, ..
            } => transform.is_finite() && shape.is_finite(),
            Self::Stroke {
                transform,
                shape,
                stroke,
            } => transform.is_finite() && shape.is_finite() && stroke_is_finite(stroke),
        }
    }

    /// Is this clip NaN?
    pub fn is_nan(&self) -> bool {
        match self {
            Self::Fill {
                transform, shape, ..
            } => transform.is_nan() || shape.is_nan(),
            Self::Stroke {
                transform,
                shape,
                stroke,
            } => transform.is_nan() || shape.is_nan() || stroke_is_nan(stroke),
        }
    }
}

fn stroke_is_finite(stroke: &Stroke) -> bool {
    // TODO(kurbo#545): Replace this local helper with `kurbo::Stroke::is_finite`.
    stroke.width.is_finite()
        && stroke.miter_limit.is_finite()
        && stroke.dash_offset.is_finite()
        && stroke.dash_pattern.iter().all(|dash| dash.is_finite())
}

fn stroke_is_nan(stroke: &Stroke) -> bool {
    // TODO(kurbo#545): Replace this local helper with `kurbo::Stroke::is_nan`.
    stroke.width.is_nan()
        || stroke.miter_limit.is_nan()
        || stroke.dash_offset.is_nan()
        || stroke.dash_pattern.iter().any(|dash| dash.is_nan())
}

fn stroke_bounds(bounds: Rect, stroke: &Stroke) -> Option<Rect> {
    if !stroke_is_finite(stroke) {
        return None;
    }

    // Conservative outset to account for width, caps, and miter joins.
    let outset = 0.5 * stroke.width.abs() * stroke.miter_limit.abs().max(1.0);
    let expanded = bounds.inflate(outset, outset);
    expanded.is_finite().then_some(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_is_finite_depends_on_transform_and_shape() {
        let mut non_finite_path = BezPath::new();
        non_finite_path.move_to((0.0, 0.0));
        non_finite_path.line_to((f64::INFINITY, 1.0));

        let finite = Clip::Fill {
            transform: Affine::translate((3.0, -2.0)),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 10.0, 4.0)),
            fill_rule: Fill::NonZero,
        };
        let non_finite_transform = Clip::Fill {
            transform: Affine::new([f64::NAN, 0.0, 0.0, 1.0, 0.0, 0.0]),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 10.0, 4.0)),
            fill_rule: Fill::NonZero,
        };
        let non_finite_shape = Clip::Fill {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Path(non_finite_path),
            fill_rule: Fill::NonZero,
        };

        assert!(finite.is_finite());
        assert!(!non_finite_transform.is_finite());
        assert!(!non_finite_shape.is_finite());
    }

    #[test]
    fn fill_is_nan_depends_on_transform_and_shape() {
        let finite = Clip::Fill {
            transform: Affine::translate((3.0, -2.0)),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 10.0, 4.0)),
            fill_rule: Fill::NonZero,
        };
        let nan_transform = Clip::Fill {
            transform: Affine::new([f64::NAN, 0.0, 0.0, 1.0, 0.0, 0.0]),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 10.0, 4.0)),
            fill_rule: Fill::NonZero,
        };
        let nan_shape = Clip::Fill {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, f64::NAN, 1.0)),
            fill_rule: Fill::NonZero,
        };
        let infinite_transform = Clip::Fill {
            transform: Affine::new([f64::INFINITY, 0.0, 0.0, 1.0, 0.0, 0.0]),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 10.0, 4.0)),
            fill_rule: Fill::NonZero,
        };

        assert!(!finite.is_nan());
        assert!(nan_transform.is_nan());
        assert!(nan_shape.is_nan());
        assert!(!infinite_transform.is_nan());
    }

    #[test]
    fn stroke_is_finite_depends_on_transform_and_shape() {
        let finite = Clip::Stroke {
            transform: Affine::scale(2.0),
            shape: ClipGeometry::RoundedRect(RoundedRect::from_rect(
                Rect::new(-1.0, -1.0, 1.0, 1.0),
                0.25,
            )),
            stroke: Stroke::new(3.0),
        };
        let non_finite_transform = Clip::Stroke {
            transform: Affine::new([1.0, 0.0, 0.0, f64::NEG_INFINITY, 0.0, 0.0]),
            shape: ClipGeometry::RoundedRect(RoundedRect::from_rect(
                Rect::new(-1.0, -1.0, 1.0, 1.0),
                0.25,
            )),
            stroke: Stroke::new(3.0),
        };
        let non_finite_shape = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, f64::NAN, 1.0)),
            stroke: Stroke::new(3.0),
        };
        let non_finite_stroke = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
            stroke: Stroke::new(f64::INFINITY),
        };
        let non_finite_dash_pattern = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
            stroke: Stroke::new(2.0).with_dashes(0.0, [1.0, f64::NAN]),
        };

        assert!(finite.is_finite());
        assert!(!non_finite_transform.is_finite());
        assert!(!non_finite_shape.is_finite());
        assert!(!non_finite_stroke.is_finite());
        assert!(!non_finite_dash_pattern.is_finite());
    }

    #[test]
    fn stroke_is_nan_depends_on_transform_shape_and_stroke() {
        let finite = Clip::Stroke {
            transform: Affine::scale(2.0),
            shape: ClipGeometry::RoundedRect(RoundedRect::from_rect(
                Rect::new(-1.0, -1.0, 1.0, 1.0),
                0.25,
            )),
            stroke: Stroke::new(3.0),
        };
        let nan_transform = Clip::Stroke {
            transform: Affine::new([1.0, 0.0, 0.0, f64::NAN, 0.0, 0.0]),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
            stroke: Stroke::new(3.0),
        };
        let nan_shape = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, f64::NAN, 1.0)),
            stroke: Stroke::new(3.0),
        };
        let nan_stroke = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
            stroke: Stroke::new(2.0).with_dashes(0.0, [1.0, f64::NAN]),
        };
        let infinite_stroke = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 1.0, 1.0)),
            stroke: Stroke::new(f64::INFINITY),
        };

        assert!(!finite.is_nan());
        assert!(nan_transform.is_nan());
        assert!(nan_shape.is_nan());
        assert!(nan_stroke.is_nan());
        assert!(!infinite_stroke.is_nan());
    }

    #[test]
    fn clip_geometry_bounds() {
        let rect = ClipGeometry::Rect(Rect::new(1.0, 2.0, 4.0, 8.0));
        assert_eq!(rect.bounds(), Some(Rect::new(1.0, 2.0, 4.0, 8.0)));

        let rounded =
            ClipGeometry::RoundedRect(RoundedRect::from_rect(Rect::new(-1.0, -2.0, 3.0, 4.0), 0.5));
        assert_eq!(rounded.bounds(), Some(Rect::new(-1.0, -2.0, 3.0, 4.0)));
    }

    #[test]
    fn clip_bounds_for_fill_and_stroke() {
        let fill = Clip::Fill {
            transform: Affine::translate((10.0, -4.0)),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 2.0, 3.0)),
            fill_rule: Fill::NonZero,
        };
        assert_eq!(fill.bounds(), Some(Rect::new(10.0, -4.0, 12.0, -1.0)));

        let stroke = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 2.0, 3.0)),
            stroke: Stroke::new(2.0).with_miter_limit(4.0),
        };
        assert_eq!(stroke.bounds(), Some(Rect::new(-4.0, -4.0, 6.0, 7.0)));
    }

    #[test]
    fn clip_bounds_returns_none_for_non_finite_inputs() {
        let non_finite_fill = Clip::Fill {
            transform: Affine::new([1.0, 0.0, 0.0, f64::NAN, 0.0, 0.0]),
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 2.0, 3.0)),
            fill_rule: Fill::NonZero,
        };
        assert_eq!(non_finite_fill.bounds(), None);

        let non_finite_stroke = Clip::Stroke {
            transform: Affine::IDENTITY,
            shape: ClipGeometry::Rect(Rect::new(0.0, 0.0, 2.0, 3.0)),
            stroke: Stroke::new(1.0).with_dashes(0.0, [1.0, f64::NAN]),
        };
        assert_eq!(non_finite_stroke.bounds(), None);
    }
}
