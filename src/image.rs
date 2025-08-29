// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Blob, Extend};

/// Defines the pixel format of an [image](ImageData).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[repr(u8)]
pub enum ImageFormat {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8 = 0,
    /// 32-bit BGRA with 8-bit channels.
    Bgra8 = 1,
    // NOTICE: If a new value is added, be sure to update the bytemuck CheckedBitPattern impl.
}

impl ImageFormat {
    /// Returns the required size in bytes for an image in this format
    /// of the given dimensions.
    ///
    /// A result of `None` indicates an overflow in the size calculation.
    #[must_use]
    pub fn size_in_bytes(self, width: u32, height: u32) -> Option<usize> {
        match self {
            Self::Rgba8 | Self::Bgra8 => 4_usize
                .checked_mul(width as usize)
                .and_then(|x| x.checked_mul(height as usize)),
        }
    }
}

/// Handling of alpha channel.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ImageAlphaType {
    /// Image has separate alpha channel (also called straight/unpremultiplied alpha).
    Alpha = 0,
    /// Image has colors with premultiplied alpha.
    AlphaPremultiplied = 1,
}

/// Defines the desired quality for sampling an image.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ImageQuality {
    /// Lowest quality with best performance characteristics.
    ///
    /// This is typically nearest neighbor sampling.
    Low = 0,
    /// Medium quality with reasonable performance characteristics.
    ///
    /// This is typically bilinear sampling.
    #[default]
    Medium = 1,
    /// Highest quality with worst performance characteristics.
    ///
    /// This is typically bicubic sampling.
    High = 2,
    // NOTICE: If a new value is added, be sure to update the bytemuck CheckedBitPattern impl.
}

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageData {
    /// Blob containing the image data.
    pub data: Blob<u8>,
    /// Pixel format of the image.
    pub format: ImageFormat,
    /// Encoding of alpha in the image pixels.
    pub alpha_type: ImageAlphaType,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
}

/// Parameters which specify how to render an image.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageRenderParams {
    /// Extend mode in the horizontal direction.
    pub x_extend: Extend,
    /// Extend mode in the vertical direction.
    pub y_extend: Extend,
    /// Hint for desired rendering quality.
    pub quality: ImageQuality,
    /// An additional alpha multiplier to use with the image.
    pub alpha: f32,
}

impl Default for ImageRenderParams {
    fn default() -> Self {
        Self {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Medium,
            alpha: 1., // Opaque
        }
    }
}

impl ImageRenderParams {
    /// Creates a new `ImageRenderParams` with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

/// Describes the image content of a filled or stroked shape.
///
/// See also [`ImageBrushRef`] which can be used to avoid reference counting overhead.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageBrush {
    /// The image to render.
    pub image: ImageData,
    /// Parameters which specify how to render the image.
    pub params: ImageRenderParams,
}

impl ImageBrush {
    /// Creates a new `ImageBrush` for the specified `ImageData` with default `ImageRenderParams`.
    #[must_use]
    pub fn new(image: ImageData) -> Self {
        Self {
            image,
            params: ImageRenderParams::default(),
        }
    }

    /// Converts an owned `ImageBrush` into a borrowed `ImageBrushRef`.
    #[must_use]
    pub fn as_ref(&self) -> ImageBrushRef<'_> {
        ImageBrushRef {
            image: &self.image,
            params: self.params,
        }
    }

    /// Builder method for setting the image [extend mode](Extend) in both
    /// directions.
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.params.x_extend = mode;
        self.params.y_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// horizontal direction.
    #[must_use]
    pub fn with_x_extend(mut self, mode: Extend) -> Self {
        self.params.x_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// vertical direction.
    #[must_use]
    pub fn with_y_extend(mut self, mode: Extend) -> Self {
        self.params.y_extend = mode;
        self
    }

    /// Builder method for setting a hint for the desired image [quality](ImageQuality)
    /// when rendering.
    #[must_use]
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.params.quality = quality;
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
        self.params.alpha = alpha;
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
        self.params.alpha *= alpha;
        self
    }
}

/// Borrowed version of [`ImageBrush`] for avoiding reference counting overhead.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ImageBrushRef<'a> {
    /// The image to render.
    pub image: &'a ImageData,
    /// Parameters which specify how to render the image.
    pub params: ImageRenderParams,
}

impl ImageBrushRef<'_> {
    /// Creates a new image with the given data, [format](ImageFormat) and dimensions.
    #[must_use]
    pub fn new<'a>(image: &'a ImageData) -> ImageBrushRef<'a> {
        ImageBrushRef {
            image,
            params: ImageRenderParams::default(),
        }
    }

    /// Converts the `ImageBrushRef` to an owned `ImageBrush`.
    #[must_use]
    pub fn to_owned(&self) -> ImageBrush {
        ImageBrush {
            image: (*self.image).clone(),
            params: self.params,
        }
    }

    /// Builder method for setting the image [extend mode](Extend) in both
    /// directions.
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.params.x_extend = mode;
        self.params.y_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// horizontal direction.
    #[must_use]
    pub fn with_x_extend(mut self, mode: Extend) -> Self {
        self.params.x_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// vertical direction.
    #[must_use]
    pub fn with_y_extend(mut self, mode: Extend) -> Self {
        self.params.y_extend = mode;
        self
    }

    /// Builder method for setting a hint for the desired image [quality](ImageQuality)
    /// when rendering.
    #[must_use]
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.params.quality = quality;
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
        self.params.alpha = alpha;
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
        self.params.alpha *= alpha;
        self
    }
}
