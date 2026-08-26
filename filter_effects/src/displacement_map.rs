// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::ColorChannel;

/// Displace pixels using a displacement map.
///
/// Uses the color values from a second input to spatially displace pixels
/// in the primary input, creating warping and distortion effects.
///
/// See [`FilterPrimitive::DisplacementMap`](crate::FilterPrimitive::DisplacementMap).
#[derive(Debug, Clone, PartialEq)]
pub struct DisplacementMap {
    /// Scale factor controlling the displacement intensity.
    pub scale: f32,
    /// Color channel from the displacement map used for X-axis displacement.
    pub x_channel: ColorChannel,
    /// Color channel from the displacement map used for Y-axis displacement.
    pub y_channel: ColorChannel,
}
