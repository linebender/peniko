// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::Extend;

use color::{
    cache_key::{BitEq, BitHash},
    AlphaColor, ColorSpace, ColorSpaceTag, DynamicColor, HueDirection, OpaqueColor,
};
use kurbo::Point;
use smallvec::SmallVec;

use core::{
    hash::Hasher,
    ops::{Deref, DerefMut},
};

/// The default for `Gradient::interpolation_cs`.
// This is intentionally not `pub` and is here in case we change it
// in the future.
const DEFAULT_GRADIENT_COLOR_SPACE: ColorSpaceTag = ColorSpaceTag::Srgb;

/// Offset and color of a transition point in a [gradient](Gradient).
///
/// Color stops are compatible with use as a cache key.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorStop {
    /// Normalized offset of the stop.
    pub offset: f32,
    /// Color at the specified offset.
    pub color: DynamicColor,
}

impl BitHash for ColorStop {
    fn bit_hash<H: Hasher>(&self, state: &mut H) {
        self.offset.bit_hash(state);
        self.color.bit_hash(state);
    }
}

impl BitEq for ColorStop {
    fn bit_eq(&self, other: &Self) -> bool {
        self.offset.bit_eq(&other.offset) && self.color.bit_eq(&other.color)
    }
}

impl ColorStop {
    /// Returns the color stop with the alpha component set to `alpha`.
    #[must_use]
    pub const fn with_alpha(self, alpha: f32) -> Self {
        Self {
            offset: self.offset,
            color: self.color.with_alpha(alpha),
        }
    }

    /// Returns the color stop with the alpha component multiplied by `alpha`.
    /// The behaviour of this transformation is undefined if `alpha` is negative.
    ///
    /// If any resulting alphas would overflow, these currently saturate (to opaque).
    #[must_use]
    pub const fn multiply_alpha(self, alpha: f32) -> Self {
        Self {
            offset: self.offset,
            color: self.color.multiply_alpha(alpha),
        }
    }
}

impl<CS: ColorSpace> From<(f32, AlphaColor<CS>)> for ColorStop {
    fn from(pair: (f32, AlphaColor<CS>)) -> Self {
        Self {
            offset: pair.0,
            color: DynamicColor::from_alpha_color(pair.1),
        }
    }
}

impl From<(f32, DynamicColor)> for ColorStop {
    fn from(pair: (f32, DynamicColor)) -> Self {
        Self {
            offset: pair.0,
            color: pair.1,
        }
    }
}

impl<CS: ColorSpace> From<(f32, OpaqueColor<CS>)> for ColorStop {
    fn from(pair: (f32, OpaqueColor<CS>)) -> Self {
        Self {
            offset: pair.0,
            color: DynamicColor::from_alpha_color(pair.1.with_alpha(1.)),
        }
    }
}

/// Collection of color stops.
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorStops(pub SmallVec<[ColorStop; 4]>);

impl Deref for ColorStops {
    type Target = SmallVec<[ColorStop; 4]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorStops {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ColorStops {
    /// Construct an empty collection of stops.
    pub fn new() -> Self {
        Self::default()
    }
}

impl BitEq for ColorStops {
    fn bit_eq(&self, other: &Self) -> bool {
        self.as_slice().bit_eq(other.as_slice())
    }
}

impl BitHash for ColorStops {
    fn bit_hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().bit_hash(state);
    }
}

impl From<&[ColorStop]> for ColorStops {
    fn from(slice: &[ColorStop]) -> Self {
        Self(slice.into())
    }
}

/// Properties for the supported [gradient](Gradient) types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientKind {
    /// Gradient that transitions between two or more colors along a line.
    Linear {
        /// Starting point.
        start: Point,
        /// Ending point.
        end: Point,
    },
    /// Gradient that transitions between two or more colors that radiate from an origin.
    Radial {
        /// Center of start circle.
        start_center: Point,
        /// Radius of start circle.
        start_radius: f32,
        /// Center of end circle.
        end_center: Point,
        /// Radius of end circle.
        end_radius: f32,
    },
    /// Gradient that transitions between two or more colors that rotate around a center
    /// point.
    Sweep {
        /// Center point.
        center: Point,
        /// Start angle of the sweep, counter-clockwise of the x-axis.
        start_angle: f32,
        /// End angle of the sweep, counter-clockwise of the x-axis.
        end_angle: f32,
    },
}

/// Definition of a gradient that transitions between two or more colors.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gradient {
    /// Kind and properties of the gradient.
    pub kind: GradientKind,
    /// Extend mode.
    pub extend: Extend,
    /// The color space to be used for interpolation.
    ///
    /// The colors in the color stops will be converted to this color space.
    ///
    /// This defaults to [sRGB](ColorSpaceTag::Srgb).
    pub interpolation_cs: ColorSpaceTag,
    /// When interpolating within a cylindrical color space, the direction for the hue.
    ///
    /// This is interpreted as described in [CSS Color Module Level 4 § 12.4].
    ///
    /// [CSS Color Module Level 4 § 12.4]: https://drafts.csswg.org/css-color/#hue-interpolation
    pub hue_direction: HueDirection,
    /// Color stop collection.
    pub stops: ColorStops,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: GradientKind::Linear {
                start: Point::default(),
                end: Point::default(),
            },
            extend: Default::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: Default::default(),
            stops: Default::default(),
        }
    }
}

impl Gradient {
    /// Creates a new linear gradient for the specified start and end points.
    pub fn new_linear(start: impl Into<Point>, end: impl Into<Point>) -> Self {
        Self {
            kind: GradientKind::Linear {
                start: start.into(),
                end: end.into(),
            },
            extend: Default::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new radial gradient for the specified center point and radius.
    pub fn new_radial(center: impl Into<Point>, radius: f32) -> Self {
        let center = center.into();
        Self {
            kind: GradientKind::Radial {
                start_center: center,
                start_radius: 0.0,
                end_center: center,
                end_radius: radius,
            },
            extend: Default::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new two point radial gradient for the specified center points and radii.
    pub fn new_two_point_radial(
        start_center: impl Into<Point>,
        start_radius: f32,
        end_center: impl Into<Point>,
        end_radius: f32,
    ) -> Self {
        Self {
            kind: GradientKind::Radial {
                start_center: start_center.into(),
                start_radius,
                end_center: end_center.into(),
                end_radius,
            },
            extend: Default::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new sweep gradient for the specified center point, start and
    /// end angles.
    pub fn new_sweep(center: impl Into<Point>, start_angle: f32, end_angle: f32) -> Self {
        Self {
            kind: GradientKind::Sweep {
                center: center.into(),
                start_angle,
                end_angle,
            },
            extend: Default::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: Default::default(),
            stops: Default::default(),
        }
    }

    /// Builder method for setting the gradient extend mode.
    #[must_use]
    pub const fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Builder method for setting the interpolation color space.
    #[must_use]
    pub const fn with_interpolation_cs(mut self, interpolation_cs: ColorSpaceTag) -> Self {
        self.interpolation_cs = interpolation_cs;
        self
    }

    /// Builder method for setting the hue direction when interpolating within a cylindrical color space.
    #[must_use]
    pub const fn with_hue_direction(mut self, hue_direction: HueDirection) -> Self {
        self.hue_direction = hue_direction;
        self
    }

    /// Builder method for setting the color stop collection.
    #[must_use]
    pub fn with_stops(mut self, stops: impl ColorStopsSource) -> Self {
        self.stops.clear();
        stops.collect_stops(&mut self.stops);
        self
    }

    /// Returns the gradient with the alpha component for all color stops set to `alpha`.
    #[must_use]
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.stops
            .iter_mut()
            .for_each(|stop| *stop = stop.with_alpha(alpha));
        self
    }

    /// Returns the gradient with the alpha component for all color stops
    /// multiplied by `alpha`.
    #[must_use]
    pub fn multiply_alpha(mut self, alpha: f32) -> Self {
        self.stops
            .iter_mut()
            .for_each(|stop| *stop = stop.multiply_alpha(alpha));
        self
    }
}

/// Trait for types that represent a source of color stops.
pub trait ColorStopsSource {
    /// Append the stops represented within `self` into `vec`.
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

impl<CS: ColorSpace> ColorStopsSource for &'_ [AlphaColor<CS>] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            vec.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: DynamicColor::from_alpha_color(*c),
            }));
        }
    }
}

impl ColorStopsSource for &'_ [DynamicColor] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            vec.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: (*c),
            }));
        }
    }
}

impl<CS: ColorSpace> ColorStopsSource for &'_ [OpaqueColor<CS>] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            vec.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: DynamicColor::from_alpha_color((*c).with_alpha(1.)),
            }));
        }
    }
}

impl<const N: usize, CS: ColorSpace> ColorStopsSource for [AlphaColor<CS>; N] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        (&self[..]).collect_stops(vec);
    }
}
impl<const N: usize> ColorStopsSource for [DynamicColor; N] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        (&self[..]).collect_stops(vec);
    }
}
impl<const N: usize, CS: ColorSpace> ColorStopsSource for [OpaqueColor<CS>; N] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        (&self[..]).collect_stops(vec);
    }
}

#[cfg(test)]
mod tests {
    use super::Gradient;
    use color::{cache_key::CacheKey, palette, parse_color};
    use std::collections::HashSet;

    #[test]
    fn color_stops_cache() {
        let mut set = HashSet::new();
        let stops = Gradient::default()
            .with_stops([palette::css::RED, palette::css::LIME, palette::css::BLUE])
            .stops;
        let stops_clone = stops.clone();
        let parsed_gradient = Gradient::default().with_stops(
            vec![
                parse_color("red").unwrap(),
                parse_color("lime").unwrap(),
                parse_color("blue").unwrap(),
            ]
            .as_slice(),
        );
        let parsed_stops = parsed_gradient.stops.clone();
        set.insert(CacheKey(stops));
        // TODO: Ideally this wouldn't need to turn more_stops into a `CacheKey`;
        assert!(set.contains(&CacheKey(stops_clone)));
        set.insert(CacheKey(parsed_stops));
        let new_grad = parsed_gradient.clone();
        assert!(set.contains(&CacheKey(new_grad.stops)));
    }
}
