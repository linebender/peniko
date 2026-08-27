// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Generate Perlin noise/turbulence patterns.
///
/// Creates procedural noise patterns useful for textures, clouds,
/// marble effects, and other organic-looking randomness.
///
/// See [`FilterPrimitive::Turbulence`](crate::FilterPrimitive::Turbulence).
#[derive(Debug, Clone, PartialEq)]
pub struct Turbulence {
    /// Base frequency for noise generation. Higher values create finer detail.
    pub base_frequency: f32,
    /// Number of octaves for fractal noise. More octaves add finer detail.
    pub num_octaves: u32,
    /// Random seed for reproducible noise generation.
    pub seed: u32,
    /// Type of noise: smooth fractal or more chaotic turbulence.
    pub turbulence_type: TurbulenceType,
}

/// Types of turbulence noise generation.
///
/// Determines the algorithm used for generating procedural noise patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbulenceType {
    /// Fractal noise (smooth, natural-looking Perlin noise).
    ///
    /// Creates smooth, continuous patterns suitable for natural textures
    /// like clouds, marble, wood grain, or terrain.
    FractalNoise,
    /// Turbulence noise (more chaotic and energetic).
    ///
    /// Creates more chaotic patterns with sharper transitions,
    /// suitable for fire, smoke, or turbulent effects.
    Turbulence,
}
