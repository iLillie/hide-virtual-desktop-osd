#![windows_subsystem = "windows"]
use std::time::Duration;

use debounce::EventDebouncer;
use tray_icon::{TrayIconEvent, menu::MenuEvent};
use winit::event_loop::EventLoop;
use winvd::{DesktopEvent, listen_desktop_events};

mod application;
mod window;

use crate::{
    application::{Application, UserEvent},
    window::{WindowState, fetch_window, update_window_state},
};

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |_| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent());
    }));
    let proxy = event_loop.create_proxy();

    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let mut app = Application::new();

    let _menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();

    let (tx, rx) = std::sync::mpsc::channel::<DesktopEvent>();
    let (tx2, rx2) = std::sync::mpsc::channel::<bool>();

    let _notifications_thread = listen_desktop_events(tx);

    std::thread::spawn(|| {
        let window = unsafe { fetch_window() };
        let _ = tx2.send(true);

        let debouncer = EventDebouncer::new(Duration::from_millis(2500), move |_| {
            let _ = tx2.send(true);
        });

        for item in rx {
            match item {
                DesktopEvent::DesktopChanged { new: _, old: _ } => {
                    unsafe { update_window_state(WindowState::HIDDEN, window) };
                    debouncer.put(String::from("DesktopChanged"));
                }
                _ => {}
            }
        }
    });

    std::thread::spawn(|| {
        let window = unsafe { fetch_window() };

        for item in rx2 {
            let state = match item {
                true => WindowState::VISIBLE,
                false => WindowState::HIDDEN,
            };

            unsafe { update_window_state(state, window) };
        }
    });

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {err:?}");
    }

    let window = unsafe { fetch_window() };
    // Edge case, where it might quit during those 2.5 seconds.
    unsafe { update_window_state(WindowState::VISIBLE, window) };
}
