// Copyright 2022 The peniko authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

use super::{Blob, Extend};

/// Defines the pixel format of an image.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Format {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8,
}

impl Format {
    /// Returns the required size in bytes for an image in this format
    /// of the given dimensions.
    ///
    /// A result of `None` indicates an overflow in the size calculation.
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
    /// Creates a new image with the given data, format and dimensions.
    pub fn new(data: Blob<u8>, format: Format, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
            extend: Extend::Pad,
        }
    }

    /// Builder method for setting the image extend mode.
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }
}
