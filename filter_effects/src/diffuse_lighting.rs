// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::LightSource;

/// Diffuse lighting simulation.
///
/// Creates a lighting effect by treating the input's alpha channel as a height map
/// and calculating diffuse (matte) reflection from a light source.
///
/// See [`FilterPrimitive::DiffuseLighting`](crate::FilterPrimitive::DiffuseLighting).
#[derive(Debug, Clone, PartialEq)]
pub struct DiffuseLighting {
    /// Surface scale factor for converting alpha values to heights.
    pub surface_scale: f32,
    /// Diffuse reflection constant (kd). Controls lighting intensity.
    pub diffuse_constant: f32,
    /// Kernel unit length for gradient calculations in user space.
    pub kernel_unit_length: f32,
    /// Configuration of the light source (point, distant, or spot).
    pub light_source: LightSource,
}
