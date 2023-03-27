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

use alloc::vec::Vec;

/// The border of a widget.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Border {
    /// Thickness of the border.
    thickness: f32,

    /// The color of the border.
    color: Color,

    /// If the border is dashed, this is the length of the dashes.
    dashes: Option<Vec<f32>>,

    /// The rounding radius of the border.
    radius: f32,
}

impl Border {
    /// Create a new border with the given thickness and color.
    pub fn new(thickness: f32, color: Color) -> Self {
        Self {
            thickness,
            color,
            dashes: None,
            radius: 0.0,
        }
    }

    /// Get the tickness.
    pub fn thickness(&self) -> f32 {
        self.thickness
    }

    /// Set the thickness.
    pub fn set_thickness(&mut self, thickness: f32) -> &mut Self {
        self.thickness = thickness;
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

    /// Get the dashes.
    pub fn dashes(&self) -> Option<&[f32]> {
        self.dashes.as_deref()
    }

    /// Set the dashes.
    pub fn set_dashes(&mut self, dashes: impl Into<Vec<f32>>) -> &mut Self {
        self.dashes = Some(dashes.into());
        self
    }

    /// Get the radius of the border's corners.
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Set the radius of the border's corners.
    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }
}
