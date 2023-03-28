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

    const BORDERS_COLOR: Color = choose!(
        Self,
        Self::BG_COLOR.darken(18),
        Self::BG_COLOR.darken(10)
    );
    const ALT_BORDERS_COLOR: Color = choose!(
        Self,
        Self::BG_COLOR.darken(24),
        Self::BG_COLOR.darken(18)
    );
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
}

struct Light;
impl ThemeType for Light {
    const IS_LIGHT: bool = true;
}

struct Dark;
impl ThemeType for Dark {
    const IS_LIGHT: bool = false;
}

fn default_theme_inner<T: ThemeType>(theme: &mut Theme) {

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
