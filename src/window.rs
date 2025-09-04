use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowExW, ShowWindow};

use windows_core::w;

// I don't like this, but not bothered to find another option
static mut WINDOW: Option<HWND> = None;

pub enum WindowState {
    VISIBLE,
    HIDDEN,
}

pub unsafe fn update_window_state(show: WindowState) {
    let state = match show {
        WindowState::VISIBLE => 9,
        WindowState::HIDDEN => 6,
    };

    let _ = unsafe {
        ShowWindow(
            WINDOW.unwrap(),
            windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD(state),
        )
    };
}

pub unsafe fn initialize_window() {
    unsafe {
        WINDOW = Some(HWND::default());

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

        while WINDOW.unwrap() == HWND::default() {
            WINDOW = Some(
                FindWindowExW(
                    hwnd_found,
                    HWND::default(),
                    w!("Windows.UI.Composition.DesktopWindowContentBridge"),
                    w!("DesktopWindowXamlSource"),
                )
                .unwrap(),
            );
        }
    }
}
