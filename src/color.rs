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

/// The color of a property.
///
/// This is represented internally as 32-bit RGBA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color([u8; 4]);

impl Color {
    /// Create a new color from its four channels.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    /// Parse a color from its hex representation.
    pub(crate) const fn hex(name: &str) -> Self {
        let name = name.as_bytes();
        let first = [name[1], name[2]];
        let second = [name[3], name[4]];
        let third = [name[5], name[6]];

        Self::new(
            const_parse_hex(&first),
            const_parse_hex(&second),
            const_parse_hex(&third),
            255,
        )
    }

    /// Darken a color by a factor.
    pub(crate) const fn darken(self, percent: u8) -> Self {
        macro_rules! t {
            ($e:expr) => {{
                (($e as u16 * percent as u16) / 100) as u8
            }};
        }

        let [r, g, b, a] = self.0;
        Self::new(t!(r), t!(g), t!(b), a)
    }

    /// Mix two colors by a factor.
    pub(crate) const fn mix(self, other: Self, percent: u8) -> Self {
        macro_rules! t {
            ($e:expr, $o:expr) => {{
                let e = $e as u16;
                let o = $o as u16;
                let p = percent as u16;
                let result = e + ((o - e) * p) / 100;
                result as u8
            }};
        }

        let [r, g, b, a] = self.0;
        let [or, og, ob, oa] = other.0;
        Self::new(t!(r, or), t!(g, og), t!(b, ob), t!(a, oa))
    }

    /// Convert into a 4-tuple.
    pub fn into_tuple(self) -> (u8, u8, u8, u8) {
        let [r, g, b, a] = self.0;
        (r, g, b, a)
    }

    /// Convert into an array.
    pub fn into_array(self) -> [u8; 4] {
        self.0
    }

    /// Get the red component.
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// Get the green component.
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// Get the blue component.
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// Get the alpha component.
    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

/// Parse a hex string to a `u8` at compile time.
///
/// Takes the bytes of a hex string and returns the value of the hex string.
const fn const_parse_hex(hex: &[u8]) -> u8 {
    let mut result = 0;
    let mut index = 0;

    while index < hex.len() {
        let byte = hex[index];
        let value = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => panic!("Invalid hex string"),
        };

        result = result * 16 + value;
        index += 1;
    }

    result
}
