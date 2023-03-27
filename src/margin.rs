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

/// The margin of a widget.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Margin {
    /// The left margin.
    left: f32,

    /// The right margin.
    right: f32,

    /// The top margin.
    top: f32,

    /// The bottom margin.
    bottom: f32,
}

impl Margin {
    /// Create a new margin.
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Get the left margin.
    pub fn left(&self) -> f32 {
        self.left
    }

    /// Set the left margin.
    pub fn set_left(&mut self, left: f32) -> &mut Self {
        self.left = left;
        self
    }

    /// Get the right margin.
    pub fn right(&self) -> f32 {
        self.right
    }

    /// Set the right margin.
    pub fn set_right(&mut self, right: f32) -> &mut Self {
        self.right = right;
        self
    }

    /// Get the top margin.
    pub fn top(&self) -> f32 {
        self.top
    }

    /// Set the top margin.
    pub fn set_top(&mut self, top: f32) -> &mut Self {
        self.top = top;
        self
    }

    /// Get the bottom margin.
    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    /// Set the bottom margin.
    pub fn set_bottom(&mut self, bottom: f32) -> &mut Self {
        self.bottom = bottom;
        self
    }
}
