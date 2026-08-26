// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::LightSource;

/// Specular lighting simulation.
///
/// Creates a lighting effect by treating the input's alpha channel as a height map
/// and calculating specular (shiny) reflection highlights from a light source.
///
/// See [`FilterPrimitive::SpecularLighting`](crate::FilterPrimitive::SpecularLighting).
#[derive(Debug, Clone, PartialEq)]
pub struct SpecularLighting {
    /// Surface scale factor for converting alpha values to heights.
    pub surface_scale: f32,
    /// Specular reflection constant (ks). Controls highlight intensity.
    pub specular_constant: f32,
    /// Specular reflection exponent. Controls highlight sharpness (higher = sharper).
    pub specular_exponent: f32,
    /// Kernel unit length for gradient calculations in user space.
    pub kernel_unit_length: f32,
    /// Configuration of the light source (point, distant, or spot).
    pub light_source: LightSource,
}
