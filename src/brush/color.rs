// Copyright 2022 The peniko authors and piet authors.
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

// Borrows code heavily from the piet (https://github.com/linebender/piet/) Color
// type.

use crate::ParseError;

/// 32-bit RGBA color.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Color {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha component.
    pub a: u8,
}

impl Color {
    /// Creates a new RGB color with 255 alpha.
    pub const fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new RGBA color.
    pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from three floating point values, each in the range 0.0 to 1.0.
    ///
    /// The interpretation is the same as rgb8, and no greater precision is
    /// (currently) assumed.
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Create a color from four floating point values, each in the range 0.0 to 1.0.
    ///
    /// The interpretation is the same as rgba32, and no greater precision is
    /// (currently) assumed.
    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        let r = (r.max(0.0).min(1.0) * 255.0).round() as u8;
        let g = (g.max(0.0).min(1.0) * 255.0).round() as u8;
        let b = (b.max(0.0).min(1.0) * 255.0).round() as u8;
        let a = (a.max(0.0).min(1.0) * 255.0).round() as u8;
        Self { r, g, b, a }
    }

    /// Create a color from a CIEL\*a\*b\* polar (also known as CIE HCL)
    /// specification.
    ///
    /// The `h` parameter is an angle in degrees, with 0 roughly magenta, 90
    /// roughly yellow, 180 roughly cyan, and 270 roughly blue. The `l`
    /// parameter is perceptual luminance, with 0 black and 100 white.
    /// The `c` parameter is a chrominance concentration, with 0 grayscale
    /// and a nominal maximum of 127 (in the future, higher values might
    /// be useful, for high gamut contexts).
    ///
    /// Currently this is just converted into sRGB, but in the future as we
    /// support high-gamut colorspaces, it can be used to specify more colors
    /// or existing colors with a higher accuracy.
    ///
    /// Currently out-of-gamut values are clipped to the nearest sRGB color,
    /// which is perhaps not ideal (the clipping might change the hue). See
    /// <https://github.com/d3/d3-color/issues/33> for discussion.
    pub fn hlc(h: f64, l: f64, c: f64) -> Self {
        Self::hlca(h, l, c, 1.0)
    }

    /// Create a color from a CIEL\*a\*b\* polar specification and alpha.
    ///
    /// The `a` value represents alpha in the range 0.0 to 1.0.
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::unreadable_literal)]
    pub fn hlca(h: f64, l: f64, c: f64, a: f64) -> Color {
        let alpha = a;
        // The reverse transformation from Lab to XYZ, see
        // https://en.wikipedia.org/wiki/CIELAB_color_space
        fn f_inv(t: f64) -> f64 {
            let d = 6. / 29.;
            if t > d {
                t.powi(3)
            } else {
                3. * d * d * (t - 4. / 29.)
            }
        }
        let th = h * (std::f64::consts::PI / 180.);
        let a = c * th.cos();
        let b = c * th.sin();
        let ll = (l + 16.) * (1. / 116.);
        // Produce raw XYZ values
        let X = f_inv(ll + a * (1. / 500.));
        let Y = f_inv(ll);
        let Z = f_inv(ll - b * (1. / 200.));
        // This matrix is the concatenation of three sources.
        // First, the white point is taken to be ICC standard D50, so
        // the diagonal matrix of [0.9642, 1, 0.8249]. Note that there
        // is some controversy around this value. However, it matches
        // the other matrices, thus minimizing chroma error.
        //
        // Second, an adaption matrix from D50 to D65. This is the
        // inverse of the recommended D50 to D65 adaptation matrix
        // from the W3C sRGB spec:
        // https://www.w3.org/Graphics/Color/srgb
        //
        // Finally, the conversion from XYZ to linear sRGB values,
        // also taken from the W3C sRGB spec.
        let r_lin = 3.02172918 * X - 1.61692294 * Y - 0.40480625 * Z;
        let g_lin = -0.94339358 * X + 1.91584267 * Y + 0.02755094 * Z;
        let b_lin = 0.06945666 * X - 0.22903204 * Y + 1.15957526 * Z;
        fn gamma(u: f64) -> f64 {
            if u <= 0.0031308 {
                12.92 * u
            } else {
                1.055 * u.powf(1. / 2.4) - 0.055
            }
        }
        Color::rgba(gamma(r_lin), gamma(g_lin), gamma(b_lin), alpha)
    }

    /// Parses a color from a string.
    ///
    /// Currently accepts CSS style hexidecimal colors of the forms #RGB, #RGBA,
    /// #RRGGBB, #RRGGBBAA or the name of an SVG color such as "aliceblue".
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        parse_color(s)
    }

    /// Premultiplies the color by the alpha component.
    pub fn to_premul(self) -> Self {
        let a = self.a as f64 * (1.0 / 255.0);
        let r = (self.r as f64 * a).round() as u8;
        let g = (self.g as f64 * a).round() as u8;
        let b = (self.b as f64 * a).round() as u8;
        Self { r, g, b, a: self.a }
    }

    /// Un-premultiplies the color by the alpha component.
    pub fn to_separate(self) -> Self {
        let a = 1.0 / (self.a as f64 * (1.0 / 255.0) + f64::EPSILON);
        let r = (self.r as f64 * a).round() as u8;
        let g = (self.g as f64 * a).round() as u8;
        let b = (self.b as f64 * a).round() as u8;
        Self { r, g, b, a: self.a }
    }

    /// Returns the color as a packed premultiplied value.
    pub fn to_premul_u32(self) -> u32 {
        let Self { r, g, b, a } = self.to_premul();
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        (r << 24) | (g << 16) | (b << 8) | a as u32
    }
}

/// Named SVG colors.
impl Color {
    /// Alice blue (240, 248, 255, 255)
    pub const ALICE_BLUE: Color = Color::rgba8(240, 248, 255, 255);
    /// Antique white (250, 235, 215, 255)
    pub const ANTIQUE_WHITE: Color = Color::rgba8(250, 235, 215, 255);
    /// Aqua (0, 255, 255, 255)
    pub const AQUA: Color = Color::rgba8(0, 255, 255, 255);
    /// Aquamarine (127, 255, 212, 255)
    pub const AQUAMARINE: Color = Color::rgba8(127, 255, 212, 255);
    /// Azure (240, 255, 255, 255)
    pub const AZURE: Color = Color::rgba8(240, 255, 255, 255);
    /// Beige (245, 245, 220, 255)
    pub const BEIGE: Color = Color::rgba8(245, 245, 220, 255);
    /// Bisque (255, 228, 196, 255)
    pub const BISQUE: Color = Color::rgba8(255, 228, 196, 255);
    /// Black (0, 0, 0, 255)
    pub const BLACK: Color = Color::rgba8(0, 0, 0, 255);
    /// Blanched almond (255, 235, 205, 255)
    pub const BLANCHED_ALMOND: Color = Color::rgba8(255, 235, 205, 255);
    /// Blue (0, 0, 255, 255)
    pub const BLUE: Color = Color::rgba8(0, 0, 255, 255);
    /// Blue violet (138, 43, 226, 255)
    pub const BLUE_VIOLET: Color = Color::rgba8(138, 43, 226, 255);
    /// Brown (165, 42, 42, 255)
    pub const BROWN: Color = Color::rgba8(165, 42, 42, 255);
    /// Burlywood (222, 184, 135, 255)
    pub const BURLYWOOD: Color = Color::rgba8(222, 184, 135, 255);
    /// Cadet blue (95, 158, 160, 255)
    pub const CADET_BLUE: Color = Color::rgba8(95, 158, 160, 255);
    /// Chartreuse (127, 255, 0, 255)
    pub const CHARTREUSE: Color = Color::rgba8(127, 255, 0, 255);
    /// Chocolate (210, 105, 30, 255)
    pub const CHOCOLATE: Color = Color::rgba8(210, 105, 30, 255);
    /// Coral (255, 127, 80, 255)
    pub const CORAL: Color = Color::rgba8(255, 127, 80, 255);
    /// Cornflower blue (100, 149, 237, 255)
    pub const CORNFLOWER_BLUE: Color = Color::rgba8(100, 149, 237, 255);
    /// Cornsilk (255, 248, 220, 255)
    pub const CORNSILK: Color = Color::rgba8(255, 248, 220, 255);
    /// Crimson (220, 20, 60, 255)
    pub const CRIMSON: Color = Color::rgba8(220, 20, 60, 255);
    /// Cyan (0, 255, 255, 255)
    pub const CYAN: Color = Color::rgba8(0, 255, 255, 255);
    /// Dark blue (0, 0, 139, 255)
    pub const DARK_BLUE: Color = Color::rgba8(0, 0, 139, 255);
    /// Dark cyan (0, 139, 139, 255)
    pub const DARK_CYAN: Color = Color::rgba8(0, 139, 139, 255);
    /// Dark goldenrod (184, 134, 11, 255)
    pub const DARK_GOLDENROD: Color = Color::rgba8(184, 134, 11, 255);
    /// Dark gray (169, 169, 169, 255)
    pub const DARK_GRAY: Color = Color::rgba8(169, 169, 169, 255);
    /// Dark green (0, 100, 0, 255)
    pub const DARK_GREEN: Color = Color::rgba8(0, 100, 0, 255);
    /// Dark khaki (189, 183, 107, 255)
    pub const DARK_KHAKI: Color = Color::rgba8(189, 183, 107, 255);
    /// Dark magenta (139, 0, 139, 255)
    pub const DARK_MAGENTA: Color = Color::rgba8(139, 0, 139, 255);
    /// Dark olive green (85, 107, 47, 255)
    pub const DARK_OLIVE_GREEN: Color = Color::rgba8(85, 107, 47, 255);
    /// Dark orange (255, 140, 0, 255)
    pub const DARK_ORANGE: Color = Color::rgba8(255, 140, 0, 255);
    /// Dark orchid (153, 50, 204, 255)
    pub const DARK_ORCHID: Color = Color::rgba8(153, 50, 204, 255);
    /// Dark red (139, 0, 0, 255)
    pub const DARK_RED: Color = Color::rgba8(139, 0, 0, 255);
    /// Dark salmon (233, 150, 122, 255)
    pub const DARK_SALMON: Color = Color::rgba8(233, 150, 122, 255);
    /// Dark sea green (143, 188, 143, 255)
    pub const DARK_SEA_GREEN: Color = Color::rgba8(143, 188, 143, 255);
    /// Dark slate blue (72, 61, 139, 255)
    pub const DARK_SLATE_BLUE: Color = Color::rgba8(72, 61, 139, 255);
    /// Dark slate gray (47, 79, 79, 255)
    pub const DARK_SLATE_GRAY: Color = Color::rgba8(47, 79, 79, 255);
    /// Dark turquoise (0, 206, 209, 255)
    pub const DARK_TURQUOISE: Color = Color::rgba8(0, 206, 209, 255);
    /// Dark violet (148, 0, 211, 255)
    pub const DARK_VIOLET: Color = Color::rgba8(148, 0, 211, 255);
    /// Deep pink (255, 20, 147, 255)
    pub const DEEP_PINK: Color = Color::rgba8(255, 20, 147, 255);
    /// Deep sky blue (0, 191, 255, 255)
    pub const DEEP_SKY_BLUE: Color = Color::rgba8(0, 191, 255, 255);
    /// Dim gray (105, 105, 105, 255)
    pub const DIM_GRAY: Color = Color::rgba8(105, 105, 105, 255);
    /// Dodger blue (30, 144, 255, 255)
    pub const DODGER_BLUE: Color = Color::rgba8(30, 144, 255, 255);
    /// Firebrick (178, 34, 34, 255)
    pub const FIREBRICK: Color = Color::rgba8(178, 34, 34, 255);
    /// Floral white (255, 250, 240, 255)
    pub const FLORAL_WHITE: Color = Color::rgba8(255, 250, 240, 255);
    /// Forest green (34, 139, 34, 255)
    pub const FOREST_GREEN: Color = Color::rgba8(34, 139, 34, 255);
    /// Fuchsia (255, 0, 255, 255)
    pub const FUCHSIA: Color = Color::rgba8(255, 0, 255, 255);
    /// Gainsboro (220, 220, 220, 255)
    pub const GAINSBORO: Color = Color::rgba8(220, 220, 220, 255);
    /// Ghost white (248, 248, 255, 255)
    pub const GHOST_WHITE: Color = Color::rgba8(248, 248, 255, 255);
    /// Gold (255, 215, 0, 255)
    pub const GOLD: Color = Color::rgba8(255, 215, 0, 255);
    /// Goldenrod (218, 165, 32, 255)
    pub const GOLDENROD: Color = Color::rgba8(218, 165, 32, 255);
    /// Gray (128, 128, 128, 255)
    pub const GRAY: Color = Color::rgba8(128, 128, 128, 255);
    /// Green (0, 128, 0, 255)
    pub const GREEN: Color = Color::rgba8(0, 128, 0, 255);
    /// Green yellow (173, 255, 47, 255)
    pub const GREEN_YELLOW: Color = Color::rgba8(173, 255, 47, 255);
    /// Honeydew (240, 255, 240, 255)
    pub const HONEYDEW: Color = Color::rgba8(240, 255, 240, 255);
    /// Hot pink (255, 105, 180, 255)
    pub const HOT_PINK: Color = Color::rgba8(255, 105, 180, 255);
    /// Indian red (205, 92, 92, 255)
    pub const INDIAN_RED: Color = Color::rgba8(205, 92, 92, 255);
    /// Indigo (75, 0, 130, 255)
    pub const INDIGO: Color = Color::rgba8(75, 0, 130, 255);
    /// Ivory (255, 255, 240, 255)
    pub const IVORY: Color = Color::rgba8(255, 255, 240, 255);
    /// Khaki (240, 230, 140, 255)
    pub const KHAKI: Color = Color::rgba8(240, 230, 140, 255);
    /// Lavender (230, 230, 250, 255)
    pub const LAVENDER: Color = Color::rgba8(230, 230, 250, 255);
    /// Lavender blush (255, 240, 245, 255)
    pub const LAVENDER_BLUSH: Color = Color::rgba8(255, 240, 245, 255);
    /// Lawn green (124, 252, 0, 255)
    pub const LAWN_GREEN: Color = Color::rgba8(124, 252, 0, 255);
    /// Lemon chiffon (255, 250, 205, 255)
    pub const LEMON_CHIFFON: Color = Color::rgba8(255, 250, 205, 255);
    /// Light blue (173, 216, 230, 255)
    pub const LIGHT_BLUE: Color = Color::rgba8(173, 216, 230, 255);
    /// Light coral (240, 128, 128, 255)
    pub const LIGHT_CORAL: Color = Color::rgba8(240, 128, 128, 255);
    /// Light cyan (224, 255, 255, 255)
    pub const LIGHT_CYAN: Color = Color::rgba8(224, 255, 255, 255);
    /// Light goldenrod yellow (250, 250, 210, 255)
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::rgba8(250, 250, 210, 255);
    /// Light gray (211, 211, 211, 255)
    pub const LIGHT_GRAY: Color = Color::rgba8(211, 211, 211, 255);
    /// Light green (144, 238, 144, 255)
    pub const LIGHT_GREEN: Color = Color::rgba8(144, 238, 144, 255);
    /// Light pink (255, 182, 193, 255)
    pub const LIGHT_PINK: Color = Color::rgba8(255, 182, 193, 255);
    /// Light salmon (255, 160, 122, 255)
    pub const LIGHT_SALMON: Color = Color::rgba8(255, 160, 122, 255);
    /// Light sea green (32, 178, 170, 255)
    pub const LIGHT_SEA_GREEN: Color = Color::rgba8(32, 178, 170, 255);
    /// Light sky blue (135, 206, 250, 255)
    pub const LIGHT_SKY_BLUE: Color = Color::rgba8(135, 206, 250, 255);
    /// Light slate gray (119, 136, 153, 255)
    pub const LIGHT_SLATE_GRAY: Color = Color::rgba8(119, 136, 153, 255);
    /// Light steel blue (176, 196, 222, 255)
    pub const LIGHT_STEEL_BLUE: Color = Color::rgba8(176, 196, 222, 255);
    /// Light yellow (255, 255, 224, 255)
    pub const LIGHT_YELLOW: Color = Color::rgba8(255, 255, 224, 255);
    /// Lime (0, 255, 0, 255)
    pub const LIME: Color = Color::rgba8(0, 255, 0, 255);
    /// Lime green (50, 205, 50, 255)
    pub const LIME_GREEN: Color = Color::rgba8(50, 205, 50, 255);
    /// Linen (250, 240, 230, 255)
    pub const LINEN: Color = Color::rgba8(250, 240, 230, 255);
    /// Magenta (255, 0, 255, 255)
    pub const MAGENTA: Color = Color::rgba8(255, 0, 255, 255);
    /// Maroon (128, 0, 0, 255)
    pub const MAROON: Color = Color::rgba8(128, 0, 0, 255);
    /// Medium aquamarine (102, 205, 170, 255)
    pub const MEDIUM_AQUAMARINE: Color = Color::rgba8(102, 205, 170, 255);
    /// Medium blue (0, 0, 205, 255)
    pub const MEDIUM_BLUE: Color = Color::rgba8(0, 0, 205, 255);
    /// Medium orchid (186, 85, 211, 255)
    pub const MEDIUM_ORCHID: Color = Color::rgba8(186, 85, 211, 255);
    /// Medium purple (147, 112, 219, 255)
    pub const MEDIUM_PURPLE: Color = Color::rgba8(147, 112, 219, 255);
    /// Medium sea green (60, 179, 113, 255)
    pub const MEDIUM_SEA_GREEN: Color = Color::rgba8(60, 179, 113, 255);
    /// Medium slate blue (123, 104, 238, 255)
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgba8(123, 104, 238, 255);
    /// Medium spring green (0, 250, 154, 255)
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgba8(0, 250, 154, 255);
    /// Medium turquoise (72, 209, 204, 255)
    pub const MEDIUM_TURQUOISE: Color = Color::rgba8(72, 209, 204, 255);
    /// Medium violet red (199, 21, 133, 255)
    pub const MEDIUM_VIOLET_RED: Color = Color::rgba8(199, 21, 133, 255);
    /// Midnight blue (25, 25, 112, 255)
    pub const MIDNIGHT_BLUE: Color = Color::rgba8(25, 25, 112, 255);
    /// Mint cream (245, 255, 250, 255)
    pub const MINT_CREAM: Color = Color::rgba8(245, 255, 250, 255);
    /// Misty rose (255, 228, 225, 255)
    pub const MISTY_ROSE: Color = Color::rgba8(255, 228, 225, 255);
    /// Moccasin (255, 228, 181, 255)
    pub const MOCCASIN: Color = Color::rgba8(255, 228, 181, 255);
    /// Navajo white (255, 222, 173, 255)
    pub const NAVAJO_WHITE: Color = Color::rgba8(255, 222, 173, 255);
    /// Navy (0, 0, 128, 255)
    pub const NAVY: Color = Color::rgba8(0, 0, 128, 255);
    /// Old lace (253, 245, 230, 255)
    pub const OLD_LACE: Color = Color::rgba8(253, 245, 230, 255);
    /// Olive (128, 128, 0, 255)
    pub const OLIVE: Color = Color::rgba8(128, 128, 0, 255);
    /// Olive drab (107, 142, 35, 255)
    pub const OLIVE_DRAB: Color = Color::rgba8(107, 142, 35, 255);
    /// Orange (255, 165, 0, 255)
    pub const ORANGE: Color = Color::rgba8(255, 165, 0, 255);
    /// Orange red (255, 69, 0, 255)
    pub const ORANGE_RED: Color = Color::rgba8(255, 69, 0, 255);
    /// Orchid (218, 112, 214, 255)
    pub const ORCHID: Color = Color::rgba8(218, 112, 214, 255);
    /// Pale goldenrod (238, 232, 170, 255)
    pub const PALE_GOLDENROD: Color = Color::rgba8(238, 232, 170, 255);
    /// Pale green (152, 251, 152, 255)
    pub const PALE_GREEN: Color = Color::rgba8(152, 251, 152, 255);
    /// Pale turquoise (175, 238, 238, 255)
    pub const PALE_TURQUOISE: Color = Color::rgba8(175, 238, 238, 255);
    /// Pale violet red (219, 112, 147, 255)
    pub const PALE_VIOLET_RED: Color = Color::rgba8(219, 112, 147, 255);
    /// Papaya whip (255, 239, 213, 255)
    pub const PAPAYA_WHIP: Color = Color::rgba8(255, 239, 213, 255);
    /// Peach puff (255, 218, 185, 255)
    pub const PEACH_PUFF: Color = Color::rgba8(255, 218, 185, 255);
    /// Peru (205, 133, 63, 255)
    pub const PERU: Color = Color::rgba8(205, 133, 63, 255);
    /// Pink (255, 192, 203, 255)
    pub const PINK: Color = Color::rgba8(255, 192, 203, 255);
    /// Plum (221, 160, 221, 255)
    pub const PLUM: Color = Color::rgba8(221, 160, 221, 255);
    /// Powder blue (176, 224, 230, 255)
    pub const POWDER_BLUE: Color = Color::rgba8(176, 224, 230, 255);
    /// Purple (128, 0, 128, 255)
    pub const PURPLE: Color = Color::rgba8(128, 0, 128, 255);
    /// Rebecca purple (102, 51, 153, 255)
    pub const REBECCA_PURPLE: Color = Color::rgba8(102, 51, 153, 255);
    /// Red (255, 0, 0, 255)
    pub const RED: Color = Color::rgba8(255, 0, 0, 255);
    /// Rosy brown (188, 143, 143, 255)
    pub const ROSY_BROWN: Color = Color::rgba8(188, 143, 143, 255);
    /// Royal blue (65, 105, 225, 255)
    pub const ROYAL_BLUE: Color = Color::rgba8(65, 105, 225, 255);
    /// Saddle brown (139, 69, 19, 255)
    pub const SADDLE_BROWN: Color = Color::rgba8(139, 69, 19, 255);
    /// Salmon (250, 128, 114, 255)
    pub const SALMON: Color = Color::rgba8(250, 128, 114, 255);
    /// Sandy brown (244, 164, 96, 255)
    pub const SANDY_BROWN: Color = Color::rgba8(244, 164, 96, 255);
    /// Sea green (46, 139, 87, 255)
    pub const SEA_GREEN: Color = Color::rgba8(46, 139, 87, 255);
    /// Seashell (255, 245, 238, 255)
    pub const SEASHELL: Color = Color::rgba8(255, 245, 238, 255);
    /// Sienna (160, 82, 45, 255)
    pub const SIENNA: Color = Color::rgba8(160, 82, 45, 255);
    /// Silver (192, 192, 192, 255)
    pub const SILVER: Color = Color::rgba8(192, 192, 192, 255);
    /// Sky blue (135, 206, 235, 255)
    pub const SKY_BLUE: Color = Color::rgba8(135, 206, 235, 255);
    /// Slate blue (106, 90, 205, 255)
    pub const SLATE_BLUE: Color = Color::rgba8(106, 90, 205, 255);
    /// Slate gray (112, 128, 144, 255)
    pub const SLATE_GRAY: Color = Color::rgba8(112, 128, 144, 255);
    /// Snow (255, 250, 250, 255)
    pub const SNOW: Color = Color::rgba8(255, 250, 250, 255);
    /// Spring green (0, 255, 127, 255)
    pub const SPRING_GREEN: Color = Color::rgba8(0, 255, 127, 255);
    /// Steel blue (70, 130, 180, 255)
    pub const STEEL_BLUE: Color = Color::rgba8(70, 130, 180, 255);
    /// Tan (210, 180, 140, 255)
    pub const TAN: Color = Color::rgba8(210, 180, 140, 255);
    /// Teal (0, 128, 128, 255)
    pub const TEAL: Color = Color::rgba8(0, 128, 128, 255);
    /// Thistle (216, 191, 216, 255)
    pub const THISTLE: Color = Color::rgba8(216, 191, 216, 255);
    /// Tomato (255, 99, 71, 255)
    pub const TOMATO: Color = Color::rgba8(255, 99, 71, 255);
    /// Transparent (0, 0, 0, 0)
    pub const TRANSPARENT: Color = Color::rgba8(0, 0, 0, 0);
    /// Turquoise (64, 224, 208, 255)
    pub const TURQUOISE: Color = Color::rgba8(64, 224, 208, 255);
    /// Violet (238, 130, 238, 255)
    pub const VIOLET: Color = Color::rgba8(238, 130, 238, 255);
    /// Wheat (245, 222, 179, 255)
    pub const WHEAT: Color = Color::rgba8(245, 222, 179, 255);
    /// White (255, 255, 255, 255)
    pub const WHITE: Color = Color::rgba8(255, 255, 255, 255);
    /// White smoke (245, 245, 245, 255)
    pub const WHITE_SMOKE: Color = Color::rgba8(245, 245, 245, 255);
    /// Yellow (255, 255, 0, 255)
    pub const YELLOW: Color = Color::rgba8(255, 255, 0, 255);
    /// Yellow green (154, 205, 50, 255)
    pub const YELLOW_GREEN: Color = Color::rgba8(154, 205, 50, 255);
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        Self::rgb8(rgb[0], rgb[1], rgb[2])
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self::rgba8(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

fn parse_color(s: &str) -> Result<Color, ParseError> {
    let s = s.trim();
    if let Some(stripped) = s.strip_prefix('#') {
        match get_4bit_hex_channels(stripped) {
            Ok(channels) => Ok(color_from_4bit_hex(channels)),
            Err(e) => Err(e),
        }
    } else {
        Ok(match s {
            "aliceblue" => Color::ALICE_BLUE,
            "antiquewhite" => Color::ANTIQUE_WHITE,
            "aqua" => Color::AQUA,
            "aquamarine" => Color::AQUAMARINE,
            "azure" => Color::AZURE,
            "beige" => Color::BEIGE,
            "bisque" => Color::BISQUE,
            "black" => Color::BLACK,
            "blanchedalmond" => Color::BLANCHED_ALMOND,
            "blue" => Color::BLUE,
            "blueviolet" => Color::BLUE_VIOLET,
            "brown" => Color::BROWN,
            "burlywood" => Color::BURLYWOOD,
            "cadetblue" => Color::CADET_BLUE,
            "chartreuse" => Color::CHARTREUSE,
            "chocolate" => Color::CHOCOLATE,
            "coral" => Color::CORAL,
            "cornflowerblue" => Color::CORNFLOWER_BLUE,
            "cornsilk" => Color::CORNSILK,
            "crimson" => Color::CRIMSON,
            "cyan" => Color::CYAN,
            "darkblue" => Color::DARK_BLUE,
            "darkcyan" => Color::DARK_CYAN,
            "darkgoldenrod" => Color::DARK_GOLDENROD,
            "darkgray" => Color::DARK_GRAY,
            "darkgreen" => Color::DARK_GREEN,
            "darkkhaki" => Color::DARK_KHAKI,
            "darkmagenta" => Color::DARK_MAGENTA,
            "darkolivegreen" => Color::DARK_OLIVE_GREEN,
            "darkorange" => Color::DARK_ORANGE,
            "darkorchid" => Color::DARK_ORCHID,
            "darkred" => Color::DARK_RED,
            "darksalmon" => Color::DARK_SALMON,
            "darkseagreen" => Color::DARK_SEA_GREEN,
            "darkslateblue" => Color::DARK_SLATE_BLUE,
            "darkslategray" => Color::DARK_SLATE_GRAY,
            "darkturquoise" => Color::DARK_TURQUOISE,
            "darkviolet" => Color::DARK_VIOLET,
            "deeppink" => Color::DEEP_PINK,
            "deepskyblue" => Color::DEEP_SKY_BLUE,
            "dimgray" => Color::DIM_GRAY,
            "dodgerblue" => Color::DODGER_BLUE,
            "firebrick" => Color::FIREBRICK,
            "floralwhite" => Color::FLORAL_WHITE,
            "forestgreen" => Color::FOREST_GREEN,
            "fuchsia" => Color::FUCHSIA,
            "gainsboro" => Color::GAINSBORO,
            "ghostwhite" => Color::GHOST_WHITE,
            "gold" => Color::GOLD,
            "goldenrod" => Color::GOLDENROD,
            "gray" => Color::GRAY,
            "green" => Color::GREEN,
            "greenyellow" => Color::GREEN_YELLOW,
            "honeydew" => Color::HONEYDEW,
            "hotpink" => Color::HOT_PINK,
            "indianred" => Color::INDIAN_RED,
            "indigo" => Color::INDIGO,
            "ivory" => Color::IVORY,
            "khaki" => Color::KHAKI,
            "lavender" => Color::LAVENDER,
            "lavenderblush" => Color::LAVENDER_BLUSH,
            "lawngreen" => Color::LAWN_GREEN,
            "lemonchiffon" => Color::LEMON_CHIFFON,
            "lightblue" => Color::LIGHT_BLUE,
            "lightcoral" => Color::LIGHT_CORAL,
            "lightcyan" => Color::LIGHT_CYAN,
            "lightgoldenrodyellow" => Color::LIGHT_GOLDENROD_YELLOW,
            "lightgray" => Color::LIGHT_GRAY,
            "lightgreen" => Color::LIGHT_GREEN,
            "lightpink" => Color::LIGHT_PINK,
            "lightsalmon" => Color::LIGHT_SALMON,
            "lightseagreen" => Color::LIGHT_SEA_GREEN,
            "lightskyblue" => Color::LIGHT_SKY_BLUE,
            "lightslategray" => Color::LIGHT_SLATE_GRAY,
            "lightsteelblue" => Color::LIGHT_STEEL_BLUE,
            "lightyellow" => Color::LIGHT_YELLOW,
            "lime" => Color::LIME,
            "limegreen" => Color::LIME_GREEN,
            "linen" => Color::LINEN,
            "magenta" => Color::MAGENTA,
            "maroon" => Color::MAROON,
            "mediumaquamarine" => Color::MEDIUM_AQUAMARINE,
            "mediumblue" => Color::MEDIUM_BLUE,
            "mediumorchid" => Color::MEDIUM_ORCHID,
            "mediumpurple" => Color::MEDIUM_PURPLE,
            "mediumseagreen" => Color::MEDIUM_SEA_GREEN,
            "mediumslateblue" => Color::MEDIUM_SLATE_BLUE,
            "mediumspringgreen" => Color::MEDIUM_SPRING_GREEN,
            "mediumturquoise" => Color::MEDIUM_TURQUOISE,
            "mediumvioletred" => Color::MEDIUM_VIOLET_RED,
            "midnightblue" => Color::MIDNIGHT_BLUE,
            "mintcream" => Color::MINT_CREAM,
            "mistyrose" => Color::MISTY_ROSE,
            "moccasin" => Color::MOCCASIN,
            "navajowhite" => Color::NAVAJO_WHITE,
            "navy" => Color::NAVY,
            "oldlace" => Color::OLD_LACE,
            "olive" => Color::OLIVE,
            "olivedrab" => Color::OLIVE_DRAB,
            "orange" => Color::ORANGE,
            "orangered" => Color::ORANGE_RED,
            "orchid" => Color::ORCHID,
            "palegoldenrod" => Color::PALE_GOLDENROD,
            "palegreen" => Color::PALE_GREEN,
            "paleturquoise" => Color::PALE_TURQUOISE,
            "palevioletred" => Color::PALE_VIOLET_RED,
            "papayawhip" => Color::PAPAYA_WHIP,
            "peachpuff" => Color::PEACH_PUFF,
            "peru" => Color::PERU,
            "pink" => Color::PINK,
            "plum" => Color::PLUM,
            "powderblue" => Color::POWDER_BLUE,
            "purple" => Color::PURPLE,
            "rebeccapurple" => Color::REBECCA_PURPLE,
            "red" => Color::RED,
            "rosybrown" => Color::ROSY_BROWN,
            "royalblue" => Color::ROYAL_BLUE,
            "saddlebrown" => Color::SADDLE_BROWN,
            "salmon" => Color::SALMON,
            "sandybrown" => Color::SANDY_BROWN,
            "seagreen" => Color::SEA_GREEN,
            "seashell" => Color::SEASHELL,
            "sienna" => Color::SIENNA,
            "silver" => Color::SILVER,
            "skyblue" => Color::SKY_BLUE,
            "slateblue" => Color::SLATE_BLUE,
            "slategray" => Color::SLATE_GRAY,
            "snow" => Color::SNOW,
            "springgreen" => Color::SPRING_GREEN,
            "steelblue" => Color::STEEL_BLUE,
            "tan" => Color::TAN,
            "teal" => Color::TEAL,
            "thistle" => Color::THISTLE,
            "tomato" => Color::TOMATO,
            "transparent" => Color::TRANSPARENT,
            "turquoise" => Color::TURQUOISE,
            "violet" => Color::VIOLET,
            "wheat" => Color::WHEAT,
            "white" => Color::WHITE,
            "whitesmoke" => Color::WHITE_SMOKE,
            "yellow" => Color::YELLOW,
            "yellowgreen" => Color::YELLOW_GREEN,
            _ => return Err(ParseError::InvalidColorName),
        })
    }
}

// The following hex color parsing code taken from piet:

const fn get_4bit_hex_channels(hex_str: &str) -> Result<[u8; 8], ParseError> {
    let mut four_bit_channels = match hex_str.as_bytes() {
        &[b'#', r, g, b] | &[r, g, b] => [r, r, g, g, b, b, b'f', b'f'],
        &[b'#', r, g, b, a] | &[r, g, b, a] => [r, r, g, g, b, b, a, a],
        &[b'#', r0, r1, g0, g1, b0, b1] | &[r0, r1, g0, g1, b0, b1] => {
            [r0, r1, g0, g1, b0, b1, b'f', b'f']
        }
        &[b'#', r0, r1, g0, g1, b0, b1, a0, a1] | &[r0, r1, g0, g1, b0, b1, a0, a1] => {
            [r0, r1, g0, g1, b0, b1, a0, a1]
        }
        _ => return Err(ParseError::InvalidLength),
    };

    // convert to hex in-place
    // this is written without a for loop to satisfy `const`
    let mut i = 0;
    while i < four_bit_channels.len() {
        let ascii = four_bit_channels[i];
        let as_hex = match hex_from_ascii_byte(ascii) {
            Ok(hex) => hex,
            Err(byte) => {
                return Err(ParseError::InvalidHexDigit {
                    ch: byte as _,
                    index: i,
                })
            }
        };
        four_bit_channels[i] = as_hex;
        i += 1;
    }
    Ok(four_bit_channels)
}

const fn color_from_4bit_hex(components: [u8; 8]) -> Color {
    let [r0, r1, g0, g1, b0, b1, a0, a1] = components;
    Color::rgba8(r0 << 4 | r1, g0 << 4 | g1, b0 << 4 | b1, a0 << 4 | a1)
}

const fn hex_from_ascii_byte(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        _ => Err(b),
    }
}
