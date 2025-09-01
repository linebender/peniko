// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Defines the color mixing function for a [blend operation](BlendMode).
///
/// See [W3C's *Compositing and Blending Level 1* draft](https://www.w3.org/TR/compositing-1/) for more details.
/// Illustrations fall under the [W3C open license](https://www.w3.org/copyright/software-license/).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Mix {
    /// Default attribute which specifies no blending. The blending formula simply selects the source color.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/normal.png)
    Normal = 0,
    /// Source color is multiplied by the destination color and replaces the destination.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/multiply.png)
    Multiply = 1,
    /// Multiplies the complements of the backdrop and source color values, then complements the result.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/screen.png)
    Screen = 2,
    /// Multiplies or screens the colors, depending on the backdrop color value.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/overlay.png)
    Overlay = 3,
    /// Selects the darker of the backdrop and source colors.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/darken.png)
    Darken = 4,
    /// Selects the lighter of the backdrop and source colors.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/lighten.png)
    Lighten = 5,
    /// Brightens the backdrop color to reflect the source color. Painting with black produces no
    /// change.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/colordodge.png)
    ColorDodge = 6,
    /// Darkens the backdrop color to reflect the source color. Painting with white produces no
    /// change.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/colorburn.png)
    ColorBurn = 7,
    /// Multiplies or screens the colors, depending on the source color value. The effect is
    /// similar to shining a harsh spotlight on the backdrop.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/hardlight.png)
    HardLight = 8,
    /// Darkens or lightens the colors, depending on the source color value. The effect is similar
    /// to shining a diffused spotlight on the backdrop.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/softlight.png)
    SoftLight = 9,
    /// Subtracts the darker of the two constituent colors from the lighter color.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/difference.png)
    Difference = 10,
    /// Produces an effect similar to that of the `Difference` mode but lower in contrast. Painting
    /// with white inverts the backdrop color; painting with black produces no change.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/exclusion.png)
    Exclusion = 11,
    /// Creates a color with the hue of the source color and the saturation and luminosity of the
    /// backdrop color.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/hue.png)
    Hue = 12,
    /// Creates a color with the saturation of the source color and the hue and luminosity of the
    /// backdrop color. Painting with this mode in an area of the backdrop that is a pure gray
    /// (no saturation) produces no change.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/saturation.png)
    Saturation = 13,
    /// Creates a color with the hue and saturation of the source color and the luminosity of the
    /// backdrop color. This preserves the gray levels of the backdrop and is useful for coloring
    /// monochrome images or tinting color images.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/color.png)
    Color = 14,
    /// Creates a color with the luminosity of the source color and the hue and saturation of the
    /// backdrop color. This produces an inverse effect to that of the `Color` mode.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/luminosity.png)
    Luminosity = 15,
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the `bytemuck::Contiguous` impl.
}

/// Defines the layer composition function for a [blend operation](BlendMode).
///
/// See [W3C's *Compositing and Blending Level 1* draft](https://www.w3.org/TR/compositing-1/) for more details.
/// Illustrations fall under the [W3C open license](https://www.w3.org/copyright/software-license/).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum Compose {
    /// No regions are enabled.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_clr.svg)
    Clear = 0,
    /// Only the source will be present.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_src.svg)
    Copy = 1,
    /// Only the destination will be present.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_dst.svg)
    Dest = 2,
    /// The source is placed over the destination.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_src-over.svg)
    SrcOver = 3,
    /// The destination is placed over the source.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_dst-over.svg)
    DestOver = 4,
    /// The parts of the source that overlap with the destination are placed.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_src-in.svg)
    SrcIn = 5,
    /// The parts of the destination that overlap with the source are placed.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_dst-in.svg)
    DestIn = 6,
    /// The parts of the source that fall outside of the destination are placed.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_src-out.svg)
    SrcOut = 7,
    /// The parts of the destination that fall outside of the source are placed.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_dst-out.svg)
    DestOut = 8,
    /// The parts of the source which overlap the destination replace the destination. The
    /// destination is placed everywhere else.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_src-atop.svg)
    SrcAtop = 9,
    /// The parts of the destination which overlaps the source replace the source. The source is
    /// placed everywhere else.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_dst-atop.svg)
    DestAtop = 10,
    /// The non-overlapping regions of source and destination are combined.
    ///
    /// ![](https://www.w3.org/TR/compositing-1/examples/PD_xor.svg)
    Xor = 11,
    /// The sum of the source image and destination image is displayed.
    Plus = 12,
    /// Allows two elements to cross fade by changing their opacities from 0 to 1 on one
    /// element and 1 to 0 on the other element.
    PlusLighter = 13,
    // NOTICE: If a new value is added, be sure to modify `MAX_VALUE` in the `bytemuck::Contiguous` impl.
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
            mix: Mix::Normal,
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
