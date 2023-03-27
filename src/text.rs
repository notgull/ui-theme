// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `ui-theme`.
//
// `ui-theme` is free software: you can redistribute it and/or modify it under the terms of
// either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
//
// `ui-theme` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License or the Mozilla Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `ui-theme`. If not, see <https://www.gnu.org/licenses/> or
// <https://www.mozilla.org/en-US/MPL/2.0/>.

use crate::color::Color;

/// The text style of a widget.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    /// The font family.
    family: FontFamily,

    /// The size in pixels.
    size: f32,

    /// Text orientation, in radians.
    orientation: f32,

    /// Font weight.
    weight: u16,

    /// The font is italic.
    italic: bool,

    /// The font is underlined.
    underline: bool,

    /// The font is strikethrough.
    strikethrough: bool,

    /// The color of the text.
    color: Color,

    /// The horizontal alignment of the text.
    halignment: TextAlignment,

    /// The vertical alignment of the text.
    valignment: TextAlignment,
}

impl TextStyle {
    /// Create a new text style from its size and font family.
    pub fn new(size: f32, family: impl Into<FontFamily>) -> Self {
        Self {
            family: family.into(),
            size,
            orientation: 0.0,
            weight: 400,
            italic: false,
            underline: false,
            strikethrough: false,
            color: Color::new(0, 0, 0, 0xFF),
            halignment: TextAlignment::Left,
            valignment: TextAlignment::Center,
        }
    }

    /// Get the font family.
    pub fn family(&self) -> &FontFamily {
        &self.family
    }

    /// Set the font family.
    pub fn set_family(&mut self, family: impl Into<FontFamily>) -> &mut Self {
        self.family = family.into();
        self
    }

    /// Get the font size.
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Set the font size.
    pub fn set_size(&mut self, size: f32) -> &mut Self {
        self.size = size;
        self
    }

    /// Get the font orientation.
    pub fn orientation(&self) -> f32 {
        self.orientation
    }

    /// Set the font orientation.
    pub fn set_orientation(&mut self, orientation: f32) -> &mut Self {
        self.orientation = orientation;
        self
    }

    /// Get the font weight.
    pub fn weight(&self) -> u16 {
        self.weight
    }

    /// Set the font weight.
    pub fn set_weight(&mut self, weight: u16) -> &mut Self {
        self.weight = weight;
        self
    }

    /// Get the italic flag.
    pub fn italic(&self) -> bool {
        self.italic
    }

    /// Set the italic flag.
    pub fn set_italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }

    /// Get the underline flag.
    pub fn underline(&self) -> bool {
        self.underline
    }

    /// Set the underline flag.
    pub fn set_underline(&mut self, underline: bool) -> &mut Self {
        self.underline = underline;
        self
    }

    /// Get the strikethrough flag.
    pub fn strikethrough(&self) -> bool {
        self.strikethrough
    }

    /// Set the strikethrough flag.
    pub fn set_strikethrough(&mut self, strikethrough: bool) -> &mut Self {
        self.strikethrough = strikethrough;
        self
    }

    /// Get the color.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color.
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Get the horizontal alignment.
    pub fn halignment(&self) -> TextAlignment {
        self.halignment
    }

    /// Set the horizontal alignment.
    pub fn set_halignment(&mut self, halignment: TextAlignment) -> &mut Self {
        self.halignment = halignment;
        self
    }

    /// Get the vertical alignment.
    pub fn valignment(&self) -> TextAlignment {
        self.valignment
    }

    /// Set the vertical alignment.
    pub fn set_valignment(&mut self, valignment: TextAlignment) -> &mut Self {
        self.valignment = valignment;
        self
    }
}

/// The font family of a widget.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFamily {
    /// Mono spaced font.
    Monospace,

    /// Sans serif font.
    SansSerif,

    /// Serif font.
    Serif,

    /// A custom font.
    Custom(String),
}

impl From<String> for FontFamily {
    fn from(family: String) -> Self {
        Self::Custom(family)
    }
}

impl From<&str> for FontFamily {
    fn from(family: &str) -> Self {
        Self::Custom(family.to_string())
    }
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum TextAlignment {
    /// Align to the left.
    Left,

    /// Align to the center.
    Center,

    /// Align to the right.
    Right,
}
