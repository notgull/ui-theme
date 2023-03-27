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

use crate::border::Border;
use crate::fill::Fill;
use crate::margin::Margin;
use crate::shadow::Shadow;
use crate::text::TextStyle;

/// Properties of a widget.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetProperties {
    /// The border of this widget.
    border: Option<Border>,

    /// The background color of this widget.
    background: Option<Fill>,

    /// The text style of this widget.
    text: Option<TextStyle>,

    /// The menu text style of this widget.
    menu_text: Option<TextStyle>,

    /// Text shadow.
    text_shadow: Option<Shadow>,

    /// Box shadow.
    box_shadow: Option<Shadow>,

    /// Margin.
    margin: Option<Margin>,

    /// Padding.
    padding: Option<Margin>,

    /// Default size.
    default_size: Option<(u32, u32)>,

    /// Size of the menubar.
    menu_bar_size: Option<(u32, u32)>,

    /// Size of the scroll bar.
    scroll_bar_size: Option<(u32, u32)>,
}

impl WidgetProperties {
    /// Get the border of the widget.
    pub fn border(&self) -> Option<&Border> {
        self.border.as_ref()
    }

    /// Set the border of the widget.
    pub fn set_border(&mut self, border: impl Into<Border>) -> &mut Self {
        self.border = Some(border.into());
        self
    }

    /// Get the background of the widget.
    pub fn background(&self) -> Option<&Fill> {
        self.background.as_ref()
    }

    /// Set the background of the widget.
    pub fn set_background(&mut self, background: impl Into<Fill>) -> &mut Self {
        self.background = Some(background.into());
        self
    }

    /// Get the text style of the widget.
    pub fn text_style(&self) -> Option<&TextStyle> {
        self.text.as_ref()
    }

    /// Set the text style of the widget.
    pub fn set_text_style(&mut self, text_style: impl Into<TextStyle>) -> &mut Self {
        self.text = Some(text_style.into());
        self
    }

    /// Get the menu text style of the widget.
    pub fn menu_text_style(&self) -> Option<&TextStyle> {
        self.menu_text.as_ref()
    }

    /// Set the menu text style of the widget.
    pub fn set_menu_text_style(&mut self, text_style: impl Into<TextStyle>) -> &mut Self {
        self.menu_text = Some(text_style.into());
        self
    }

    /// Get the text shadow of the widget.
    pub fn text_shadow(&self) -> Option<&Shadow> {
        self.text_shadow.as_ref()
    }

    /// Set the text shadow of the widget.
    pub fn set_text_shadow(&mut self, text_shadow: impl Into<Shadow>) -> &mut Self {
        self.text_shadow = Some(text_shadow.into());
        self
    }

    /// Get the box shadow of the widget.
    pub fn box_shadow(&self) -> Option<&Shadow> {
        self.box_shadow.as_ref()
    }

    /// Set the box shadow of the widget.
    pub fn set_box_shadow(&mut self, box_shadow: impl Into<Shadow>) -> &mut Self {
        self.box_shadow = Some(box_shadow.into());
        self
    }

    /// Get the margin of the widget.
    pub fn margin(&self) -> Option<&Margin> {
        self.margin.as_ref()
    }

    /// Set the margin of the widget.
    pub fn set_margin(&mut self, margin: impl Into<Margin>) -> &mut Self {
        self.margin = Some(margin.into());
        self
    }

    /// Get the padding of the widget.
    pub fn padding(&self) -> Option<&Margin> {
        self.padding.as_ref()
    }

    /// Set the padding of the widget.
    pub fn set_padding(&mut self, padding: impl Into<Margin>) -> &mut Self {
        self.padding = Some(padding.into());
        self
    }

    /// Get the default size of the widget.
    pub fn default_size(&self) -> Option<(u32, u32)> {
        self.default_size
    }

    /// Set the default size of the widget.
    pub fn set_default_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.default_size = Some(size);
        self
    }

    /// Get the size of the menu bar.
    pub fn menu_bar_size(&self) -> Option<(u32, u32)> {
        self.menu_bar_size
    }

    /// Set the size of the menu bar.
    pub fn set_menu_bar_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.menu_bar_size = Some(size);
        self
    }

    /// Get the size of the scroll bar.
    pub fn scroll_bar_size(&self) -> Option<(u32, u32)> {
        self.scroll_bar_size
    }

    /// Set the size of the scroll bar.
    pub fn set_scroll_bar_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.scroll_bar_size = Some(size);
        self
    }
}
