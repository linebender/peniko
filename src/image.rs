// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(all(not(feature = "std"), feature = "libm"))]
#[allow(unused_imports)]
use kurbo::common::FloatFuncs as _;

use super::{Blob, Extend};

/// Defines the pixel format of an [image](Image).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Format {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8,
}

impl Format {
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

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// Blob containing the image data.
    pub data: Blob<u8>,
    /// Pixel format of the image.
    pub format: Format,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
    /// Extend mode.
    pub extend: Extend,
    /// An additional alpha multiplier to use with the image.
    pub alpha: u8,
}

impl Image {
    /// Creates a new image with the given data, [format](Format) and dimensions.
    #[must_use]
    pub fn new(data: Blob<u8>, format: Format, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
            extend: Extend::Pad,
            // Opaque
            alpha: u8::MAX,
        }
    }

    /// Builder method for setting the image [extend mode](Extend).
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Returns the image with the alpha multiplier multiplied again by `alpha`.
    /// The behaviour of this transformation is undefined if `alpha` is negative.
    ///
    /// If any resulting alphas would overflow, these currently saturate (to opaque).
    #[must_use]
    #[track_caller]
    pub fn multiply_alpha(mut self, alpha: f32) -> Self {
        debug_assert!(
            alpha.is_finite() && alpha >= 0.0,
            "A non-finite or negative alpha ({alpha}) is meaningless."
        );
        self.alpha = ((self.alpha as f32) * alpha).round() as u8;
        self
    }
}
