// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Blob, Extend};

/// Defines the pixel format of an [image](Image).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ImageFormat {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8,
}

impl ImageFormat {
    /// Returns the required size in bytes for an image in this format
    /// of the given dimensions.
    ///
    /// A result of `None` indicates an overflow in the size calculation.
    #[must_use]
    pub fn size_in_bytes(self, width: u32, height: u32) -> Option<usize> {
        match self {
            Self::Rgba8 => 4_usize
                .checked_mul(width as usize)
                .and_then(|x| x.checked_mul(height as usize)),
        }
    }
}

/// Defines the desired quality for sampling an [image](Image).
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImageQuality {
    /// Lowest quality with best performance characteristics.
    ///
    /// This is typically nearest neighbor sampling.
    Low,
    /// Medium quality with reasonable performance characteristics.
    ///
    /// This is typically bilinear sampling.
    #[default]
    Medium,
    /// Highest quality with worst performance characteristics.
    ///
    /// This is typically bicubic sampling.
    High,
}

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// Blob containing the image data.
    pub data: Blob<u8>,
    /// Pixel format of the image.
    pub format: ImageFormat,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
    /// Extend mode in the horizontal direction.
    pub x_extend: Extend,
    /// Extend mode in the vertical direction.
    pub y_extend: Extend,
    /// Hint for desired rendering quality.
    pub quality: ImageQuality,
    /// An additional alpha multiplier to use with the image.
    pub alpha: f32,
}

impl Image {
    /// Creates a new image with the given data, [format](ImageFormat) and dimensions.
    #[must_use]
    pub fn new(data: Blob<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Medium,
            // Opaque
            alpha: 1.,
        }
    }

    /// Builder method for setting the image [extend mode](Extend) in both
    /// directions.
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.x_extend = mode;
        self.y_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// horizontal direction.
    #[must_use]
    pub fn with_x_extend(mut self, mode: Extend) -> Self {
        self.x_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// vertical direction.
    #[must_use]
    pub fn with_y_extend(mut self, mode: Extend) -> Self {
        self.y_extend = mode;
        self
    }

    /// Builder method for setting a hint for the desired image [quality](ImageQuality)
    /// when rendering.
    #[must_use]
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Returns the image with the alpha multiplier set to `alpha`.
    #[must_use]
    #[track_caller]
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        debug_assert!(
            alpha.is_finite() && alpha >= 0.0,
            "A non-finite or negative alpha ({alpha}) is meaningless."
        );
        self.alpha = alpha;
        self
    }

    /// Returns the image with the alpha multiplier multiplied again by `alpha`.
    /// The behaviour of this transformation is undefined if `alpha` is negative.
    #[must_use]
    #[track_caller]
    pub fn multiply_alpha(mut self, alpha: f32) -> Self {
        debug_assert!(
            alpha.is_finite() && alpha >= 0.0,
            "A non-finite or negative alpha ({alpha}) is meaningless."
        );
        self.alpha *= alpha;
        self
    }
}
