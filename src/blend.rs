// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Defines the color mixing function for a [blend operation](BlendMode).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Mix {
    /// Default attribute which specifies no blending. The blending formula simply selects the source color.
    Normal = 0,
    /// Source color is multiplied by the destination color and replaces the destination.
    Multiply = 1,
    /// Multiplies the complements of the backdrop and source color values, then complements the result.
    Screen = 2,
    /// Multiplies or screens the colors, depending on the backdrop color value.
    Overlay = 3,
    /// Selects the darker of the backdrop and source colors.
    Darken = 4,
    /// Selects the lighter of the backdrop and source colors.
    Lighten = 5,
    /// Brightens the backdrop color to reflect the source color. Painting with black produces no
    /// change.
    ColorDodge = 6,
    /// Darkens the backdrop color to reflect the source color. Painting with white produces no
    /// change.
    ColorBurn = 7,
    /// Multiplies or screens the colors, depending on the source color value. The effect is
    /// similar to shining a harsh spotlight on the backdrop.
    HardLight = 8,
    /// Darkens or lightens the colors, depending on the source color value. The effect is similar
    /// to shining a diffused spotlight on the backdrop.
    SoftLight = 9,
    /// Subtracts the darker of the two constituent colors from the lighter color.
    Difference = 10,
    /// Produces an effect similar to that of the `Difference` mode but lower in contrast. Painting
    /// with white inverts the backdrop color; painting with black produces no change.
    Exclusion = 11,
    /// Creates a color with the hue of the source color and the saturation and luminosity of the
    /// backdrop color.
    Hue = 12,
    /// Creates a color with the saturation of the source color and the hue and luminosity of the
    /// backdrop color. Painting with this mode in an area of the backdrop that is a pure gray
    /// (no saturation) produces no change.
    Saturation = 13,
    /// Creates a color with the hue and saturation of the source color and the luminosity of the
    /// backdrop color. This preserves the gray levels of the backdrop and is useful for coloring
    /// monochrome images or tinting color images.
    Color = 14,
    /// Creates a color with the luminosity of the source color and the hue and saturation of the
    /// backdrop color. This produces an inverse effect to that of the `Color` mode.
    Luminosity = 15,
    /// `Clip` is the same as `Normal`, but the latter always creates an isolated blend group and the
    /// former can optimize that out.
    Clip = 128,
    // NOTICE: If a new value is added, be sure to update the bytemuck CheckedBitPattern impl.
}

/// Defines the layer composition function for a [blend operation](BlendMode).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Compose {
    /// No regions are enabled.
    Clear = 0,
    /// Only the source will be present.
    Copy = 1,
    /// Only the destination will be present.
    Dest = 2,
    /// The source is placed over the destination.
    SrcOver = 3,
    /// The destination is placed over the source.
    DestOver = 4,
    /// The parts of the source that overlap with the destination are placed.
    SrcIn = 5,
    /// The parts of the destination that overlap with the source are placed.
    DestIn = 6,
    /// The parts of the source that fall outside of the destination are placed.
    SrcOut = 7,
    /// The parts of the destination that fall outside of the source are placed.
    DestOut = 8,
    /// The parts of the source which overlap the destination replace the destination. The
    /// destination is placed everywhere else.
    SrcAtop = 9,
    /// The parts of the destination which overlaps the source replace the source. The source is
    /// placed everywhere else.
    DestAtop = 10,
    /// The non-overlapping regions of source and destination are combined.
    Xor = 11,
    /// The sum of the source image and destination image is displayed.
    Plus = 12,
    /// Allows two elements to cross fade by changing their opacities from 0 to 1 on one
    /// element and 1 to 0 on the other element.
    PlusLighter = 13,
    /// Allows two elements to cross fade by changing their opacities from 1 to 0 on one
    /// element and 0 to 1 on the other element.
    PlusDarker = 14,
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the bytemuck impl.
}

/// Blend mode consisting of [color mixing](Mix) and [composition functions](Compose).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlendMode {
    /// The color mixing function.
    pub mix: Mix,
    /// The layer composition function.
    pub compose: Compose,
}

impl BlendMode {
    /// Creates a new blend mode from color mixing and layer composition
    /// functions.
    #[must_use]
    pub const fn new(mix: Mix, compose: Compose) -> Self {
        Self { mix, compose }
    }
}

impl Default for BlendMode {
    fn default() -> Self {
        Self {
            mix: Mix::Clip,
            compose: Compose::SrcOver,
        }
    }
}

impl From<Mix> for BlendMode {
    fn from(mix: Mix) -> Self {
        Self {
            mix,
            compose: Compose::SrcOver,
        }
    }
}

impl From<Compose> for BlendMode {
    fn from(compose: Compose) -> Self {
        Self {
            mix: Mix::Normal,
            compose,
        }
    }
}
