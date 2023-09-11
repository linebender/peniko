// Copyright 2022 The peniko authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::Blob;

/// Owned shareable font resource.
#[derive(Clone, PartialEq, Debug)]
pub struct Font {
    /// Blob containing the content of the font file.
    pub data: Blob<u8>,
    /// Index of the font in a collection, or 0 for a single font.
    pub index: u32,
}

impl Font {
    /// Creates a new font with the given data and collection index.
    #[must_use]
    pub fn new(data: Blob<u8>, index: u32) -> Self {
        Self { data, index }
    }
}
