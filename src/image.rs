// Copyright 2022 The peniko authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Blob, Extend};

/// Defines the pixel format of an [image](Image).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            Self::Rgba8 => 4usize
                .checked_mul(width as usize)
                .and_then(|x| x.checked_mul(height as usize)),
        }
    }
}

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
    /// Extend mode
    pub extend: Extend,
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
        }
    }

    /// Builder method for setting the image [extend mode](Extend).
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }
}
