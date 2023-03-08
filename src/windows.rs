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

use std::io;
use std::sync::Once;
use std::sync::atomic::{AtomicIsize, Ordering};

use windows_sys::w;

use windows_sys::Win32::Foundation::{HWND, HINSTANCE};

use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;

use windows_sys::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows_sys::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CloseWindow, DefWindowProcW, RegisterClassExW};

use windows_sys::Win32::UI::Controls::HTHEME;
use windows_sys::Win32::UI::Controls::{OpenThemeData, CloseThemeData};

/// A theme handle.
struct Theme(HTHEME);

impl Drop for Theme {
    fn drop(&mut self) {
        unsafe {
            CloseThemeData(self.0);
        }
    }
}

/// A handle to a window.
/// 
/// Windows are needed to retrieve theme data.
struct Window(HWND);

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            CloseWindow(self.0);
        }
    }
}

impl Theme {
    /// Get a new theme from its class name.
    fn new(window: &Window, name: &str) -> io::Result<Self> {


        todo!()
    }
}

impl Window {
    /// Create a new, dummy window.
    fn dummy() -> io::Result<Self> {
        const WINDOW_CLASS: *const u16 = w!("notgull::ui_theme::DummyWindow");
        const WINDOW_NAME: *const u16 = w!("");
        const INIT_CLASS: Once = Once::new();

        // Register a window class if we haven't already.
        INIT_CLASS.call_once(|| {
            let class = WNDCLASSEXW {
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: instance().expect("Failed to load hInstance"),
                lpszClassName: WINDOW_CLASS,
                ..unsafe { std::mem::zeroed() }
            };

            let result = unsafe {
                RegisterClassExW(&class)
            };

            if result == 0 {
                panic!("Failed to register dummy window class");
            }
        });

        // Create a window.

        todo!()
    }
}

fn instance() -> io::Result<HINSTANCE> {
    static INSTANCE: AtomicIsize = AtomicIsize::new(0);

    let instance = INSTANCE.load(Ordering::Relaxed);
    if instance != 0 {
        return Ok(instance);
    }

    // Load the current instance handle.
    let handle = unsafe {
        GetModuleHandleW(std::ptr::null())
    };

    if handle == 0 {
        return Err(io::Error::last_os_error());
    }

    // Install it in our cached variable.
    let instance = INSTANCE.compare_exchange(instance, handle, Ordering::SeqCst, Ordering::SeqCst).unwrap_or_else(|x| x);

    Ok(instance)
}
