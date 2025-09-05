use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowExW, ShowWindow};

use windows_core::w;

pub enum WindowState {
    VISIBLE,
    HIDDEN,
}

pub unsafe fn update_window_state(show: WindowState, window: HWND) {
    let state = match show {
        WindowState::VISIBLE => 9,
        WindowState::HIDDEN => 6,
    };

    let _ = unsafe {
        ShowWindow(
            window,
            windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD(state),
        )
    };
}

pub unsafe fn fetch_window() -> HWND {
    unsafe {
            let mut window = HWND::default();

            let mut hwnd_found = HWND::default();

            while hwnd_found == HWND::default() {
                hwnd_found = FindWindowExW(
                    HWND::default(),
                    hwnd_found,
                    w!("XamlExplorerHostIslandWindow"),
                    w!(""),
                )
                .unwrap();
            }

            while window == HWND::default() {
                window =
                    FindWindowExW(
                        hwnd_found,
                        HWND::default(),
                        w!("Windows.UI.Composition.DesktopWindowContentBridge"),
                        w!("DesktopWindowXamlSource"),
                    )
                    .unwrap();
            }

            return window
        }
}

