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

//! Get UI and theme information for Windows.
//!
//! Uses the `GetTheme*` functions to query for theme information.

use std::future::Future;
use std::io;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Once;

use crate::{LoadThemeError, ShadePreference, Theme};

// TODO: wintheme

pub(super) fn load_theme_blocking(
    name: Option<&str>,
    shade: ShadePreference,
) -> Result<Theme, LoadThemeError> {
    todo!()
}

pub(super) fn load_theme(
    name: Option<&str>,
    shade: ShadePreference,
) -> impl Future<Output = Result<Theme, LoadThemeError>> + Send {
    // load_theme_blocking reads from files, so we need to unblock it.
    let name = name.map(|s| s.to_owned());
    blocking::unblock(move || load_theme_blocking(name.as_deref(), shade))
}
