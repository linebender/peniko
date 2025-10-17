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
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the `bytemuck::Contiguous` impl.
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
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the `bytemuck::Contiguous` impl.
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
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the `bytemuck::Contiguous` impl.
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

/// Parameters which specify how to sample an image during rendering.
///
/// When a renderer is drawing an image, they will (in most cases) not directly
/// copy the bytes from the source image to their render target; instead, they will
/// sample from the image.
/// This involves determining from which part of the source image to read, and how to
/// handle cases where the source image's pixels are not aligned with the render target
/// exactly, in any combination of scale, position or rotation.
/// They might also perform an alpha multiplication, as done here.
/// This struct contains the parameters used by sampling.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageSampler {
    /// Extend mode in the horizontal direction.
    pub x_extend: Extend,
    /// Extend mode in the vertical direction.
    pub y_extend: Extend,
    /// Hint for desired rendering quality.
    pub quality: ImageQuality,
    /// An additional alpha multiplier to use with the image.
    pub alpha: f32,
}

impl Default for ImageSampler {
    fn default() -> Self {
        Self {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Medium,
            alpha: 1., // Opaque
        }
    }
}

impl ImageSampler {
    /// Creates a new `ImageSampler` with default values
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
/// This type is generic over the storage used for the image data.
/// By default, the generic parameter is [`ImageData`], which is a shared image with dynamic lifetime.
/// However, different renderers can use different types here, such as a pre-registered id.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageBrush<D = ImageData> {
    /// The image to render.
    pub image: D,
    /// Parameters which specify how to sample from the image during rendering.
    pub sampler: ImageSampler,
}

impl<D> ImageBrush<D> {
    /// Builder method for setting the image [extend mode](Extend) in both
    /// directions.
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.sampler.x_extend = mode;
        self.sampler.y_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// horizontal direction.
    #[must_use]
    pub fn with_x_extend(mut self, mode: Extend) -> Self {
        self.sampler.x_extend = mode;
        self
    }

    /// Builder method for setting the image [extend mode](Extend) in the
    /// vertical direction.
    #[must_use]
    pub fn with_y_extend(mut self, mode: Extend) -> Self {
        self.sampler.y_extend = mode;
        self
    }

    /// Builder method for setting a hint for the desired image [quality](ImageQuality)
    /// when rendering.
    #[must_use]
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.sampler.quality = quality;
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
        self.sampler.alpha = alpha;
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
        self.sampler.alpha *= alpha;
        self
    }
}

impl ImageBrush {
    /// Creates a new `ImageBrush` for the specified `ImageData` with default `ImageSampler`.
    #[must_use]
    pub fn new(image: ImageData) -> Self {
        Self {
            image,
            sampler: ImageSampler::default(),
        }
    }

    /// Converts an owned `ImageBrush` into a borrowed `ImageBrushRef`.
    #[must_use]
    pub fn as_ref(&'_ self) -> ImageBrushRef<'_> {
        ImageBrush {
            image: &self.image,
            sampler: self.sampler,
        }
    }
}

impl From<ImageData> for ImageBrush {
    fn from(image: ImageData) -> Self {
        Self::new(image)
    }
}

/// Borrowed version of [`ImageBrush`] for avoiding reference counting overhead.
///
/// This is useful for methods that would like to accept image brushes by reference.
/// Defining the type as `impl Into<ImageBrushRef>` is the most general useful argument
/// type, as it also allows `&ImageBrush`.
pub type ImageBrushRef<'a> = ImageBrush<&'a ImageData>;

impl ImageBrushRef<'_> {
    /// Converts the `ImageBrushRef` to an owned `ImageBrush`.
    #[must_use]
    pub fn to_owned(&self) -> ImageBrush {
        ImageBrush {
            image: (*self.image).clone(),
            sampler: self.sampler,
        }
    }
}

impl<'a> From<&'a ImageBrush> for ImageBrushRef<'a> {
    fn from(value: &'a ImageBrush) -> Self {
        value.as_ref()
    }
}

impl<'a> From<&'a ImageData> for ImageBrushRef<'a> {
    fn from(image: &'a ImageData) -> Self {
        Self {
            image,
            sampler: ImageSampler::default(),
        }
    }
}
