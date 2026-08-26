// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Light source configurations for lighting effects.
///
/// Defines different types of light sources used in diffuse and specular lighting
/// filter primitives. Each type has different characteristics and use cases.
#[derive(Debug, Clone, PartialEq)]
pub enum LightSource {
    /// Distant light source (infinitely far away, like the sun).
    ///
    /// All rays are parallel, creating uniform lighting across the surface.
    /// Direction is specified using spherical coordinates (azimuth and elevation).
    Distant {
        /// Azimuth angle in degrees (0° = pointing right, 90° = pointing up).
        /// Defines the horizontal direction of the light.
        azimuth: f32,
        /// Elevation angle in degrees (0° = horizon, 90° = directly overhead).
        /// Defines the vertical angle of the light source.
        elevation: f32,
    },
    /// Point light source at a specific 3D position.
    ///
    /// Light radiates uniformly in all directions from a single point.
    /// Intensity decreases with distance. Like a light bulb.
    Point {
        /// Light source X coordinate in user space.
        x: f32,
        /// Light source Y coordinate in user space.
        y: f32,
        /// Light source Z coordinate (height above the surface).
        /// Larger values create softer lighting across larger areas.
        z: f32,
    },
    /// Spot light with position, direction, and cone angle.
    ///
    /// Light emanates from a point in a specific direction with limited spread.
    /// Like a flashlight or stage spotlight with adjustable focus.
    Spot {
        /// Light source X coordinate in user space.
        x: f32,
        /// Light source Y coordinate in user space.
        y: f32,
        /// Light source Z coordinate (height above the surface).
        z: f32,
        /// X coordinate the spotlight is aimed at.
        points_at_x: f32,
        /// Y coordinate the spotlight is aimed at.
        points_at_y: f32,
        /// Z coordinate the spotlight is aimed at.
        points_at_z: f32,
        /// Specular exponent controlling the focus/sharpness of the spotlight beam.
        /// Higher values create tighter, more focused beams.
        specular_exponent: f32,
        /// Optional cone angle in degrees limiting the spotlight spread.
        /// If None, the light spreads based only on the specular exponent.
        limiting_cone_angle: Option<f32>,
    },
}
