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

//! Code for loading a GTK theme.

use crate::ShadePreference;
use crate::{LoadThemeError, Theme};

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use simplecss::StyleSheet;
use tinyvec::array_vec;

/// Load a GTK theme by its name.
///
/// Only supports CSS themes for now.
pub(super) fn load_theme(
    name: &str,
    shade: ShadePreference,
) -> Result<Option<Theme>, LoadThemeError> {
    // Try the user data directory first.
    if let Some(mut user_data) = user_data_dir() {
        user_data.push("themes");
        if let Some(theme) = load_from_dir(&user_data, name, shade)? {
            return Ok(Some(theme));
        }
    }

    // Try the home directory.
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(".themes");
        if let Some(theme) = load_from_dir(&home_dir, name, shade)? {
            return Ok(Some(theme));
        }
    }

    // Try the data directories.
    for data_dir in data_dirs() {
        if let Some(theme) = load_from_dir(&data_dir, name, shade)? {
            return Ok(Some(theme));
        }
    }

    // Try the GTK data prefix.
    if let Some(mut gtk_prefix) = env::var_os("GTK_DATA_PREFIX") {
        gtk_prefix.push("/share/themes");
        if let Some(theme) = load_from_dir(gtk_prefix.as_ref(), name, shade)? {
            return Ok(Some(theme));
        }
    }

    // Out of places to look.
    Ok(None)
}

fn load_from_dir(
    dir: &Path,
    name: &str,
    shade: ShadePreference,
) -> Result<Option<Theme>, LoadThemeError> {
    macro_rules! leap {
        ($e:expr) => {{
            match $e {
                Ok(v) => v,
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => return Ok(None),
                    io::ErrorKind::PermissionDenied => return Ok(None),
                    _ => return Err(LoadThemeError(e)),
                },
            }
        }};
    }

    let mut variants = array_vec![[&'static str; 2] => "gtk.css"];

    if matches!(shade, ShadePreference::Dark) {
        variants.push("gtk-dark.css");
    }

    // Create a directory from the name and dir.
    let mut theme_dir = dir.join(name);

    // List the directories in this directory.
    let mut list_dir = leap!(fs::read_dir(theme_dir));

    for dir in list_dir {
        let dir = leap!(dir);
        let path = dir.path();

        if leap!(dir.file_type()).is_dir() {
            // Check if it contains one of our variants.
            for variant in variants {
                let variant_path = path.join(variant);

                if let Ok(file) = fs::File::open(&variant_path) {
                    let file = io::BufReader::new(file);
                    return Ok(Some(load_file_file(name, file)?));
                }
            }
        }
    }

    Ok(None)
}

fn load_file_file<IO: io::BufRead>(name: &str, mut file: IO) -> Result<Theme, LoadThemeError> {
    let mut theme = Theme::empty(name);

    // Read in the file and parse the CSS.
    let mut css = String::new();
    file.read_to_string(&mut css).map_err(LoadThemeError)?;
    let sheet = StyleSheet::parse(&css);

    // TODO: Read the GTK theme from the stylesheet.

    Ok(theme)
}

fn user_data_dir() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME").map(Into::into).or_else(|| {
        dirs::home_dir().map(|mut p| {
            p.push("local/.share");
            p
        })
    })
}

fn data_dirs() -> impl Iterator<Item = PathBuf> {
    env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".into())
        .split(':')
        .map(Into::into)
        .collect::<Vec<PathBuf>>()
        .into_iter()
}
