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

mod border;
mod color;
mod default_theme;
mod fill;
mod margin;
mod properties;
mod shadow;
mod text;
mod util;
mod widget;

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
use core::hash::Hash;

use alloc::string::String;

pub use border::Border;
pub use color::Color;
pub use fill::Fill;
pub use margin::Margin;
pub use properties::WidgetProperties;
pub use shadow::Shadow;
pub use text::{FontFamily, TextAlignment, TextStyle};
pub use widget::{Widget, WidgetState};

use util::{HashMap, HashMapExt};
use widget::{WIDGETS, WIDGET_STATES};

/// The theme data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Theme {
    /// The name of the theme.
    name: String,

    /// Widgets, their states and their properties.
    properties: HashMap<Key, WidgetProperties>,
}

type Key = (Widget, WidgetState);

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
                let mut map = HashMap::with_capacity(WIDGETS.len() * WIDGET_STATES.len());

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
