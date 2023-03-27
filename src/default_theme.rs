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

//! A default theme similar to Adwaita.

use crate::{
    Color, FontFamily, LoadThemeError, ShadePreference, TextStyle, Theme, WIDGETS, WIDGET_STATES,
};

use alloc::format;

const FG_LIGHT_COLOR: Color = Color::hex("#2e3436");
const BG_LIGHT_COLOR: Color = Color::hex("#f6f5f4");
const FG_DARK_COLOR: Color = Color::hex("#eeeeec");
const BG_DARK_COLOR: Color = Color::hex("#3d3846");

macro_rules! choose {
    ($shade:expr,$light:expr,$dark:expr) => {{
        match $shade {
            ShadePreference::Light => $light,
            ShadePreference::Dark => $dark,
        }
    }};
}

pub(crate) fn default_theme(shade: ShadePreference) -> Theme {
    let mut theme = Theme::empty(format!("Default_{:?}", shade));

    let fg_color = choose!(shade, FG_LIGHT_COLOR, FG_DARK_COLOR);
    let bg_color = choose!(shade, BG_LIGHT_COLOR, BG_DARK_COLOR);

    for widget in WIDGETS {
        for state in WIDGET_STATES {
            // Get the widget state.
            let props = theme.get_mut(*widget, *state);

            // Set the background color.
            props.set_background(bg_color);

            // Set the default text style.
            props.set_text_style({
                let mut style = TextStyle::new(24.0, FontFamily::SansSerif);
                style.set_color(fg_color);
                style
            });
        }
    }

    theme
}

#[allow(unused)]
pub(super) fn load_theme_blocking(
    _name: Option<&str>,
    shade: ShadePreference,
) -> Result<Theme, LoadThemeError> {
    Ok(default_theme(shade))
}

pub(super) async fn load_theme(
    _name: Option<&str>,
    shade: ShadePreference,
) -> Result<Theme, LoadThemeError> {
    Ok(default_theme(shade))
}
