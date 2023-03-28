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

//! Get themes for free Unix.
//!
//! First, we try to find the desktop environment from the `XDG_CURRENT_DESKTOP` environment
//! variable. From there, we take one of two paths:
//!
//! - If the desktop environment is GNOME or Gtk-like, we try to find the theme CSS file in the
//!   typial directories. If it's there, we parse it and return the theme.
//! - If the desktop environment is KDE, we open the theme directory and try reading the color schemes
//!   and properties.
//!
//! If none of the above steps work, we use `zbus` to get the desired color scheme from the
//! `org.freedesktop.portal.Settings` service, and use that to choose between the light and dark
//! variants of the default theme.

mod gtk_theme;

use crate::{LoadThemeError, ShadePreference, Theme};

use futures_lite::future;

use std::io;

/// Get the theme.
pub(super) async fn load_theme(
    mut name: Option<&str>,
    mut shade: ShadePreference,
) -> Result<Theme, LoadThemeError> {
    let dconf_key: String;

    // Take the current theme type.
    match ThemeType::get() {
        ThemeType::GtkTheme(key) => {
            if name.is_none() {
                if let Ok(key) = dconf_string(key).await {
                    dconf_key = key;
                    name = Some(dconf_key.as_str());
                }
            }

            if let Some(name) = name {
                if let Some(gtk_theme) = gtk_theme::load_theme(name, shade).ok().flatten() {
                    return Ok(gtk_theme);
                }
            }
        }

        ThemeType::KdeTheme => {
            // TODO
        }

        _ => {}
    }

    // Modify the shade preference if necessary.
    if let Some(user_shade) = user_shade_preference().await.ok().flatten() {
        shade = user_shade;
    }

    // Load the default value.
    crate::default_theme::load_theme(name, shade).await
}

/// Get the theme in a blocking fashion.
pub(super) fn load_theme_blocking(
    name: Option<&str>,
    shade: ShadePreference,
) -> Result<Theme, LoadThemeError> {
    // TODO: Only use block_on where enecessary
    future::block_on(load_theme(name, shade))
}

/// The type of theme to load.
enum ThemeType {
    /// We are loading a GTK theme using the provided Dconf entry.
    GtkTheme(&'static str),

    /// We are loading a KDE theme.
    KdeTheme,

    /// We are loading neither.
    None,
}

impl ThemeType {
    fn get() -> Self {
        use detect_desktop_environment::DesktopEnvironment as DE;

        match DE::detect() {
            DE::Kde => Self::KdeTheme,
            DE::Cinnamon => Self::GtkTheme("/org/cinnamon/desktop/interface/gtk-theme"),
            DE::Gnome | DE::Unity => Self::GtkTheme("/org/gnome/desktop/interface/gtk-theme"),
            DE::Mate => Self::GtkTheme("/org/mate/desktop/interface/gtk-theme"),
            _ => Self::None,
        }
    }
}

/// Get the user's light/dark mode preference, if any.
async fn user_shade_preference() -> io::Result<Option<ShadePreference>> {
    const LIGHT_MODE: u32 = 1;
    const DARK_MODE: u32 = 2;

    // Open a ZBus connection, but make sure we poll the executor ourselves.
    let conn = zbus::ConnectionBuilder::session()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .internal_executor(false)
        .build()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Poll the executor as we continue.
    let poll_executor = {
        let executor = conn.executor();
        async move {
            loop {
                executor.tick().await;
            }
        }
    };

    // On another future, get the theme info.
    let get_theme_info = {
        let conn = &conn;
        async move {
            // Call the `org.freedesktop.portal.Settings` service.
            let reply = conn
                .call_method(
                    Some("org.freedesktop.portal.Desktop"),
                    "/org/freedesktop/portal/desktop",
                    Some("org.freedesktop.portal.Settings"),
                    "Read",
                    &("org.freedesktop.appearance", "color-scheme"),
                )
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Get the value.
            let theme = reply
                .body::<zvariant::Value>()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .downcast::<u32>()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Failed to downcast theme info")
                })?;

            // Figure out if it's light or dark mode.
            match theme {
                LIGHT_MODE => io::Result::Ok(Some(ShadePreference::Light)),
                DARK_MODE => Ok(Some(ShadePreference::Dark)),
                _ => Ok(None),
            }
        }
    };

    // Zip them together.
    future::or(get_theme_info, poll_executor).await
}

/// Get a string value from a key through `dconf`.
async fn dconf_string(key: &str) -> io::Result<String> {
    let mut stdout = String::from_utf8(
        async_process::Command::new("dconf")
            .args(["read", key])
            .output()
            .await?
            .stdout,
    )
    .map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to convert dconf output to string: {}", e),
        )
    })?;

    stdout.retain(|c| " '".contains(c));

    Ok(stdout)
}
