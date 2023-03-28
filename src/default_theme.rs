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
    Border, Color, FontFamily, LoadThemeError, Margin, ShadePreference, TextAlignment, TextStyle,
    Theme, Widget, WidgetState, WIDGETS, WIDGET_STATES,
};

use alloc::format;

macro_rules! choose {
    ($T:ident,$light:expr,$dark:expr) => {{
        if $T::IS_LIGHT {
            $light
        } else {
            $dark
        }
    }};
}

const BLACK: Color = Color::new(0, 0, 0, 255);
const WHITE: Color = Color::new(255, 255, 255, 255);

trait ThemeType {
    const IS_LIGHT: bool;

    const TEXT_COLOR: Color = choose!(Self, BLACK, WHITE);
    const BASE_COLOR: Color = choose!(Self, WHITE, BLACK);
    const BG_COLOR: Color = choose!(Self, Color::hex("#f6f5f4"), Color::hex("#3d3846"));
    const FG_COLOR: Color = choose!(Self, Color::hex("#2e3436"), Color::hex("#eeeeec"));

    const SELECTED_FG_COLOR: Color = WHITE;
    const SELECTED_BG_COLOR: Color = choose!(
        Self,
        Color::hex("#3584e4"),
        Color::hex("#3584e3").darken(20)
    );
    const SELECTED_BORDERS_COLOR: Color = choose!(
        Self,
        Self::SELECTED_BG_COLOR.darken(15),
        Self::SELECTED_BG_COLOR.darken(30)
    );

    const BORDERS_COLOR: Color =
        choose!(Self, Self::BG_COLOR.darken(18), Self::BG_COLOR.darken(10));
    const ALT_BORDERS_COLOR: Color =
        choose!(Self, Self::BG_COLOR.darken(24), Self::BG_COLOR.darken(18));
    const LINK_COLOR: Color = choose!(
        Self,
        Self::SELECTED_BG_COLOR.darken(10),
        Self::SELECTED_BG_COLOR.darken(20)
    );
    const SELECTED_LINK_COLOR: Color = choose!(
        Self,
        Self::SELECTED_BG_COLOR.darken(20),
        Self::SELECTED_BG_COLOR.darken(10)
    );

    const SCROLLBAR_BG_COLOR: Color = choose!(
        Self,
        Self::BG_COLOR.mix(Self::FG_COLOR, 80),
        Self::BASE_COLOR.mix(Self::BG_COLOR, 50)
    );
    const SCROLLBAR_SLIDER_COLOR: Color = Self::FG_COLOR.mix(Self::BG_COLOR, 60);

    const DISABLED_FG_COLOR: Color = Self::FG_COLOR.mix(Self::BG_COLOR, 50);
    const DISABLED_BG_COLOR: Color = Self::BG_COLOR.mix(Self::BASE_COLOR, 60);
    const DISABLED_BORDERS_COLOR: Color = Self::BORDERS_COLOR.mix(Self::BG_COLOR, 80);
}

struct Light;
impl ThemeType for Light {
    const IS_LIGHT: bool = true;
}

struct Dark;
impl ThemeType for Dark {
    const IS_LIGHT: bool = false;
}

#[inline]
fn default_theme_inner<T: ThemeType>(theme: &mut Theme) {
    for widget in WIDGETS {
        for state in WIDGET_STATES {
            let props = theme.get_mut(*widget, *state);

            // Set the background color.
            let bg_color = match *state {
                WidgetState::Disabled => T::DISABLED_BG_COLOR,
                _ => T::BG_COLOR,
            };

            props.set_background(bg_color);

            // Set the foreground text color.
            let fg_color = match *state {
                WidgetState::Disabled => T::DISABLED_FG_COLOR,
                _ => T::FG_COLOR,
            };

            let mut text_style = TextStyle::new(12.0, FontFamily::SansSerif);
            text_style
                .set_color(fg_color)
                .set_halignment(TextAlignment::Center)
                .set_valignment(TextAlignment::Center);
            props.set_text_style(text_style);

            // Figure out if we need to set a border.
            let border_color = match *state {
                WidgetState::Disabled => T::DISABLED_BORDERS_COLOR,
                WidgetState::Selected => T::SELECTED_BORDERS_COLOR,
                _ => T::BORDERS_COLOR,
            };
            let border_data = match *widget {
                Widget::Button => Some((1.0, border_color)),
                _ => None,
            };

            if let Some((radius, color)) = border_data {
                let mut border = Border::new(2.0, color);
                border.set_radius(radius);
                props.set_border(border);
            }

            let margin = Margin::new(2.0, 2.0, 2.0, 2.0);
            props.set_margin(margin).set_padding(margin);

            // Set default sizes.
            let height = if border_data.is_some() { 24 } else { 20 };

            props.set_default_size((height, height));

            let bar_size_long = 16;
            let bar_size_short = 8;

            props.set_menu_bar_size((bar_size_long, bar_size_short));
            props.set_scroll_bar_size((bar_size_short, bar_size_long));
        }
    }
}

pub(crate) fn default_theme(shade: ShadePreference) -> Theme {
    let mut theme = Theme::empty(format!("Default_{:?}", shade));
    match shade {
        ShadePreference::Light => default_theme_inner::<Light>(&mut theme),
        ShadePreference::Dark => default_theme_inner::<Dark>(&mut theme),
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
