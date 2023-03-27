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

pub(crate) const WIDGETS: &[Widget] = &[
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

pub(crate) const WIDGET_STATES: &[WidgetState] = &[
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
