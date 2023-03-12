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

//! User interface themes for Rust GUI.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod default_theme;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        // Fall back to the default theme.
        use default_theme as platform;
    } else if #[cfg(free_unix)] {
        mod free_unix;
        use free_unix as platform;
    } else if #[cfg(win32)] {
        mod windows;
        use windows as platform;
    } else {
        use default_theme as platform;
    }
}

use core::fmt;
use core::hash::{BuildHasher, Hash};

use alloc::string::String;
use alloc::vec::Vec;

use ahash::RandomState;
use hashbrown::HashMap;

#[derive(Clone)]
struct PsuedoRandom(RandomState);

impl Default for PsuedoRandom {
    fn default() -> Self {
        #[cfg(feature = "std")]
        let rng = fastrand::Rng::new();

        #[cfg(not(feature = "std"))]
        let rng = fastrand::Rng::with_seed(0x4816152342);

        Self(RandomState::with_seeds(
            rng.u64(..),
            rng.u64(..),
            rng.u64(..),
            rng.u64(..),
        ))
    }
}

impl BuildHasher for PsuedoRandom {
    type Hasher = ahash::AHasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}

/// The theme data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Theme {
    /// The name of the theme.
    name: String,

    /// Widgets, their states and their properties.
    properties: HashMap<Key, WidgetProperties, PsuedoRandom>,
}

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

type Key = (Widget, WidgetState);

/// Widgets that can be styled.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Widget {
    /// A normal button.
    Button,

    /// A checkbox.
    Checkbox,

    /// A radio button.
    RadioButton,

    /// A combo box.
    ComboBox,

    /// The combo box's drop-down button.
    ComboBoxButton,

    /// A date-time picker.
    DateTimePicker,

    /// A text editor.
    Editor,

    /// List view.
    ListView,

    /// List view item.
    ListViewItem,

    /// List view expand button.
    ListViewExpandButton,

    /// Menu bar.
    MenuBar,

    /// Menu bar item.
    MenuBarItem,

    /// Pop-up menu.
    PopupMenu,

    /// Pop-up menu item.
    PopupMenuItem,

    /// Menu bar separator.
    MenuSeparator,

    /// Back button.
    NavigationBack,

    /// Forward button.
    NavigationForward,

    /// Menu button.
    NavigationMenu,

    /// Page down.
    NavigationPageDown,

    /// Page up.
    NavigationPageUp,

    /// Progress bar.
    ProgressBar,

    /// Progress bar chunk.
    ProgressBarChunk,

    /// Scroll bar arrow button.
    ScrollBarArrow,

    /// Scroll bar handle.
    ScrollBarHandle,

    /// Down spinner.
    SpinnerDown,

    /// Up spinner.
    SpinnerUp,

    /// Tab body.
    TabBody,

    /// Tab pane.
    TabPane,

    /// Tab item.
    TabItem,

    /// Taskbar.
    Taskbar,

    /// Text body style.
    TextBody,

    /// Text title.
    TextTitle,

    /// Text hyperlink.
    TextHyperlink,

    /// Text label.
    TextLabel,

    /// Toolbar button.
    ToolbarButton,

    /// Toolbar dropdown button.
    ToolbarDropdownButton,

    /// Toolbar separator.
    ToolbarSeparator,

    /// Tooltip balloon.
    TooltipBalloon,

    /// Tooltip balloon stem.
    TooltipBalloonStem,
    // TODO: Add more widgets.
}

const WIDGETS: &[Widget] = &[
    Widget::Button,
    Widget::Checkbox,
    Widget::RadioButton,
    Widget::ComboBox,
    Widget::ComboBoxButton,
    Widget::DateTimePicker,
    Widget::Editor,
    Widget::ListView,
    Widget::ListViewItem,
    Widget::ListViewExpandButton,
    Widget::MenuBar,
    Widget::MenuBarItem,
    Widget::PopupMenu,
    Widget::PopupMenuItem,
    Widget::MenuSeparator,
    Widget::NavigationBack,
    Widget::NavigationForward,
    Widget::NavigationMenu,
    Widget::NavigationPageDown,
    Widget::NavigationPageUp,
    Widget::ProgressBar,
    Widget::ProgressBarChunk,
    Widget::ScrollBarArrow,
    Widget::ScrollBarHandle,
    Widget::SpinnerDown,
    Widget::SpinnerUp,
    Widget::TabBody,
    Widget::TabPane,
    Widget::TabItem,
    Widget::Taskbar,
    Widget::TextBody,
    Widget::TextTitle,
    Widget::TextHyperlink,
    Widget::TextLabel,
    Widget::ToolbarButton,
    Widget::ToolbarDropdownButton,
    Widget::ToolbarSeparator,
    Widget::TooltipBalloon,
    Widget::TooltipBalloonStem,
];

/// Widget states.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum WidgetState {
    /// The widget is disabled.
    Disabled,

    /// The widget is enabled.
    Enabled,

    /// The widget is focused.
    Focused,

    /// The widget is selected.
    Selected,

    /// The widget is hovered.
    Hovered,

    /// The widget is pressed.
    Pressed,

    /// The widget is checked.
    Checked,
}

const WIDGET_STATES: &[WidgetState] = &[
    WidgetState::Disabled,
    WidgetState::Enabled,
    WidgetState::Focused,
    WidgetState::Selected,
    WidgetState::Hovered,
    WidgetState::Pressed,
    WidgetState::Checked,
];

impl Default for WidgetState {
    fn default() -> Self {
        Self::Enabled
    }
}

impl Default for Theme {
    fn default() -> Self {
        default_theme::default_theme(ShadePreference::Light)
    }
}

impl Theme {
    /// Load a theme from the system.
    pub async fn load(
        name: impl Into<Option<&str>>,
        shade: ShadePreference,
    ) -> Result<Self, LoadThemeError> {
        platform::load_theme(name.into(), shade).await
    }

    /// Load a theme from the system using the blocking API.
    pub fn load_blocking<'a>(
        name: impl Into<Option<&'a str>>,
        shade: ShadePreference,
    ) -> Result<Self, LoadThemeError> {
        platform::load_theme_blocking(name.into(), shade)
    }

    /// Load the default theme.
    pub fn default_theme(shade: ShadePreference) -> Self {
        default_theme::default_theme(shade)
    }

    fn empty(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            properties: {
                let mut map = HashMap::with_hasher(PsuedoRandom::default());

                for widget in WIDGETS {
                    map.insert(
                        (*widget, WidgetState::default()),
                        WidgetProperties::default(),
                    );
                }

                map
            },
        }
    }

    /// Get the name of the theme.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name of the theme.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Get the properties of a widget.
    pub fn get(&self, widget: Widget, state: WidgetState) -> &WidgetProperties {
        // First, try with the state.
        if let Some(props) = self.properties.get(&(widget, state)) {
            return props;
        }

        // Then, try with the default state.
        if let Some(props) = self.properties.get(&(widget, WidgetState::default())) {
            return props;
        }

        panic!("No properties for widget {:?} in state {:?}", widget, state);
    }

    /// Get a mutable reference to widget properties.
    ///
    /// This will insert the properties if they don't exist.
    pub fn get_mut(&mut self, widget: Widget, state: WidgetState) -> &mut WidgetProperties {
        self.properties
            .entry((widget, state))
            .or_insert_with(WidgetProperties::default)
    }
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

/// The background fill of a widget.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Fill {
    /// This is a solid color.
    Color(Color),
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

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
pub enum TextAlignment {
    /// Align to the left.
    Left,

    /// Align to the center.
    Center,

    /// Align to the right.
    Right,
}

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

/// The color of a property.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color([u8; 4]);

const fn const_parse_hex(hex: &[u8]) -> u8 {
    let mut result = 0;
    let mut index = (hex.len() - 1) as i32;

    while index >= 0 {
        let c = hex[index as usize];
        result *= 16;
        result += match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'a' | b'A' => 10,
            b'b' | b'B' => 11,
            b'c' | b'C' => 12,
            b'd' | b'D' => 13,
            b'e' | b'E' => 14,
            b'f' | b'F' => 15,
            _ => panic!("Invalid hex character"),
        };

        index -= 1;
    }
    result
}

impl Color {
    /// Create a new color.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    const fn hex(name: &str) -> Self {
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

    /// Convert into a 4-tuple.
    pub fn into_tuple(self) -> (u8, u8, u8, u8) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
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

/// Whether or not to prefer dark themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShadePreference {
    /// Prefer light themes.
    Light,

    /// Prefer dark themes.
    Dark,
}

/// The error associated with loading a theme.
pub struct LoadThemeError(ErrorImpl);

#[cfg(feature = "std")]
type ErrorImpl = std::io::Error;

#[cfg(not(feature = "std"))]
type ErrorImpl = core::convert::Infallible;

impl fmt::Debug for LoadThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for LoadThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl LoadThemeError {
    /// Get a reference to the inner I/O error.
    #[cfg(feature = "std")]
    pub fn io_error(&self) -> Option<&std::io::Error> {
        Some(&self.0)
    }

    /// Convert the error into the inner I/O error.
    #[cfg(feature = "std")]
    pub fn into_io_error(self) -> Result<std::io::Error, Self> {
        Ok(self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoadThemeError {}
