// Copyright 2026 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Color channels for displacement mapping and channel selection.
///
/// Specifies which color channel to use for operations that need to
/// extract or reference individual channels from an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorChannel {
    /// Red color channel (R component).
    Red,
    /// Green color channel (G component).
    Green,
    /// Blue color channel (B component).
    Blue,
    /// Alpha channel (transparency/opacity).
    Alpha,
}
