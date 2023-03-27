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

/// Properties of a shadow.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shadow {
    /// The color of the shadow.
    color: Color,

    /// The offset of the shadow.
    offset: (f32, f32),

    /// The blur radius of the shadow.
    blur: f32,
}

impl Shadow {
    /// Create a new shadow with the provided color.
    pub fn new(color: Color) -> Self {
        Self {
            color,
            offset: (0.0, 0.0),
            blur: 0.0,
        }
    }

    /// Get the color of the shadow.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color of the shadow.
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Get the offset of the shadow.
    pub fn offset(&self) -> (f32, f32) {
        self.offset
    }

    /// Set the offset of the shadow.
    pub fn set_offset(&mut self, offset: impl Into<(f32, f32)>) -> &mut Self {
        self.offset = offset.into();
        self
    }

    /// Get the blur radius of the shadow.
    pub fn blur(&self) -> f32 {
        self.blur
    }

    /// Set the blur radius of the shadow.
    pub fn set_blur(&mut self, blur: f32) -> &mut Self {
        self.blur = blur;
        self
    }
}
