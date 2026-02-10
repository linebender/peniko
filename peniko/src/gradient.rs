// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::Extend;

use color::{
    AlphaColor, ColorSpace, ColorSpaceTag, DynamicColor, HueDirection, OpaqueColor,
    cache_key::{BitEq, BitHash},
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

/// Parameters that define the position of a linear gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradientPosition {
    /// Starting point.
    pub start: Point,
    /// Ending point.
    pub end: Point,
}

impl LinearGradientPosition {
    /// Creates a new linear gradient position for the specified start and end points.
    pub fn new(start: impl Into<Point>, end: impl Into<Point>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}

/// Parameters that define the position of a radial gradient.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradientPosition {
    /// Center of start circle.
    pub start_center: Point,
    /// Radius of start circle.
    pub start_radius: f32,
    /// Center of end circle.
    pub end_center: Point,
    /// Radius of end circle.
    pub end_radius: f32,
}

impl RadialGradientPosition {
    /// Creates a new radial gradient position for the specified center point and radius.
    pub fn new(center: impl Into<Point>, radius: f32) -> Self {
        let center = center.into();
        Self {
            start_center: center,
            start_radius: 0.0,
            end_center: center,
            end_radius: radius,
        }
    }
    /// Creates a new two point radial gradient position for the specified center points and radii.
    pub fn new_two_point(
        start_center: impl Into<Point>,
        start_radius: f32,
        end_center: impl Into<Point>,
        end_radius: f32,
    ) -> Self {
        Self {
            start_center: start_center.into(),
            start_radius,
            end_center: end_center.into(),
            end_radius,
        }
    }
}

/// Parameters that define the position of a sweep gradient.
///
/// Conventionally, a positive increase in one of the sweep angles is a clockwise rotation in a
/// Y-down, X-right coordinate system (as is common for graphics). More generally, the convention
/// for rotations is that a positive angle rotates a positive X direction into positive Y.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SweepGradientPosition {
    /// Center point.
    pub center: Point,
    /// Start angle of the sweep in radians, measuring from the positive X-axis.
    ///
    /// Clockwise in a Y-down coordinate system.
    pub start_angle: f32,
    /// End angle of the sweep in radians, measuring from the positive X-axis.
    ///
    /// Clockwise in a Y-down coordinate system.
    pub end_angle: f32,
}

impl SweepGradientPosition {
    /// Creates a new sweep gradient for the specified center point, start and end angles.
    pub fn new(center: impl Into<Point>, start_angle: f32, end_angle: f32) -> Self {
        Self {
            center: center.into(),
            start_angle,
            end_angle,
        }
    }
}

/// Defines how color channels should be handled when interpolating
/// between transparent colors.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterpolationAlphaSpace {
    /// Colors are interpolated with their color channels premultiplied by the alpha
    /// channel. This is almost always what you want.
    ///
    /// Used when interpolating colors in the premultiplied alpha space, which allows
    /// for correct interpolation when colors are transparent. This matches behavior
    /// described in [CSS Color Module Level 4 § 12.3].
    ///
    /// Following the convention of CSS Color Module Level 4, in cylindrical color
    /// spaces the hue channel is not premultiplied. If it were, interpolation would
    /// give undesirable results. See also [`color::PremulColor`].
    ///
    /// [CSS Color Module Level 4 § 12.3]: https://drafts.csswg.org/css-color/#interpolation-alpha
    #[default]
    Premultiplied = 0,
    /// Colors are interpolated without premultiplying their color channels by the alpha channel.
    ///
    /// This causes color information to leak out of transparent colors. For example, when
    /// interpolating from a fully transparent red to a fully opaque blue in sRGB, this
    /// method will go through an intermediate purple.
    ///
    /// Used when interpolating colors in the unpremultiplied (straight) alpha space.
    /// This matches behavior of gradients in the HTML `canvas` element.
    /// See [The 2D rendering context § Fill and stroke styles].
    ///
    /// [The 2D rendering context § Fill and stroke styles]: https://html.spec.whatwg.org/multipage/#interpolation
    Unpremultiplied = 1,
}

/// Properties for the supported [gradient](Gradient) types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientKind {
    /// Gradient that transitions between two or more colors along a line.
    Linear(LinearGradientPosition),
    /// Gradient that transitions between two or more colors that radiate from an origin.
    Radial(RadialGradientPosition),
    /// Gradient that transitions between two or more colors that rotate around a center
    /// point.
    Sweep(SweepGradientPosition),
}

impl From<LinearGradientPosition> for GradientKind {
    #[inline(always)]
    fn from(value: LinearGradientPosition) -> Self {
        Self::Linear(value)
    }
}
impl From<RadialGradientPosition> for GradientKind {
    #[inline(always)]
    fn from(value: RadialGradientPosition) -> Self {
        Self::Radial(value)
    }
}
impl From<SweepGradientPosition> for GradientKind {
    #[inline(always)]
    fn from(value: SweepGradientPosition) -> Self {
        Self::Sweep(value)
    }
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
    /// The gradient's color ramps will be interpolated linearly in this color space between the
    /// specified color stops.
    ///
    /// This defaults to [sRGB](ColorSpaceTag::Srgb).
    pub interpolation_cs: ColorSpaceTag,
    /// When interpolating within a cylindrical color space, the direction for the hue.
    ///
    /// This is interpreted as described in [CSS Color Module Level 4 § 12.4].
    ///
    /// [CSS Color Module Level 4 § 12.4]: https://drafts.csswg.org/css-color/#hue-interpolation
    pub hue_direction: HueDirection,
    /// Alpha space to be used for interpolation
    pub interpolation_alpha_space: InterpolationAlphaSpace,
    /// Color stop collection.
    pub stops: ColorStops,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: LinearGradientPosition {
                start: Point::default(),
                end: Point::default(),
            }
            .into(),
            extend: Extend::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: HueDirection::default(),
            interpolation_alpha_space: InterpolationAlphaSpace::default(),
            stops: ColorStops::default(),
        }
    }
}

impl Gradient {
    /// Creates a new linear gradient for the specified start and end points.
    pub fn new_linear(start: impl Into<Point>, end: impl Into<Point>) -> Self {
        Self {
            kind: LinearGradientPosition::new(start, end).into(),
            extend: Extend::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: HueDirection::default(),
            interpolation_alpha_space: InterpolationAlphaSpace::default(),
            stops: ColorStops::default(),
        }
    }

    /// Creates a new radial gradient for the specified center point and radius.
    pub fn new_radial(center: impl Into<Point>, radius: f32) -> Self {
        let center = center.into();
        Self {
            kind: RadialGradientPosition::new(center, radius).into(),
            extend: Extend::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: HueDirection::default(),
            interpolation_alpha_space: InterpolationAlphaSpace::default(),
            stops: ColorStops::default(),
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
            kind: RadialGradientPosition::new_two_point(
                start_center,
                start_radius,
                end_center,
                end_radius,
            )
            .into(),
            extend: Extend::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: HueDirection::default(),
            interpolation_alpha_space: InterpolationAlphaSpace::default(),
            stops: ColorStops::default(),
        }
    }

    /// Creates a new sweep gradient for the specified center point, start and
    /// end angles.
    pub fn new_sweep(center: impl Into<Point>, start_angle: f32, end_angle: f32) -> Self {
        Self {
            kind: SweepGradientPosition::new(center, start_angle, end_angle).into(),
            extend: Extend::default(),
            interpolation_cs: DEFAULT_GRADIENT_COLOR_SPACE,
            hue_direction: HueDirection::default(),
            interpolation_alpha_space: InterpolationAlphaSpace::default(),
            stops: ColorStops::default(),
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

    /// Builder method for setting the interpolation alpha space.
    #[must_use]
    pub const fn with_interpolation_alpha_space(
        mut self,
        interpolation_alpha_space: InterpolationAlphaSpace,
    ) -> Self {
        self.interpolation_alpha_space = interpolation_alpha_space;
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
    /// Append the stops represented within `self` into `stops`.
    fn collect_stops(self, stops: &mut ColorStops);
}

impl<T> ColorStopsSource for &'_ [T]
where
    T: Into<ColorStop> + Copy,
{
    fn collect_stops(self, stops: &mut ColorStops) {
        for &stop in self {
            stops.push(stop.into());
        }
    }
}

impl<T, const N: usize> ColorStopsSource for [T; N]
where
    T: Into<ColorStop>,
{
    fn collect_stops(self, stops: &mut ColorStops) {
        for stop in self.into_iter() {
            stops.push(stop.into());
        }
    }
}

impl<CS: ColorSpace> ColorStopsSource for &'_ [AlphaColor<CS>] {
    fn collect_stops(self, stops: &mut ColorStops) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            stops.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: DynamicColor::from_alpha_color(*c),
            }));
        }
    }
}

impl ColorStopsSource for &'_ [DynamicColor] {
    fn collect_stops(self, stops: &mut ColorStops) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            stops.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: (*c),
            }));
        }
    }
}

impl<CS: ColorSpace> ColorStopsSource for &'_ [OpaqueColor<CS>] {
    fn collect_stops(self, stops: &mut ColorStops) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            stops.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: DynamicColor::from_alpha_color((*c).with_alpha(1.)),
            }));
        }
    }
}

impl<const N: usize, CS: ColorSpace> ColorStopsSource for [AlphaColor<CS>; N] {
    fn collect_stops(self, stops: &mut ColorStops) {
        (&self[..]).collect_stops(stops);
    }
}
impl<const N: usize> ColorStopsSource for [DynamicColor; N] {
    fn collect_stops(self, stops: &mut ColorStops) {
        (&self[..]).collect_stops(stops);
    }
}
impl<const N: usize, CS: ColorSpace> ColorStopsSource for [OpaqueColor<CS>; N] {
    fn collect_stops(self, stops: &mut ColorStops) {
        (&self[..]).collect_stops(stops);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;
    use super::Gradient;
    use alloc::vec;
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
