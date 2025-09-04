//#![windows_subsystem = "windows"]
use std::time::Duration;

use debounce::EventDebouncer;
use tray_icon::{TrayIconEvent, menu::MenuEvent};
use winit::event_loop::EventLoop;
use winvd::{DesktopEvent, listen_desktop_events};

mod application;
mod window;

use crate::{
    application::{Application, UserEvent},
    window::{WindowState, update_window_state},
};

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    // set a tray event handler that forwards the event and wakes up the event loop
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

    let _notifications_thread = listen_desktop_events(tx);

    std::thread::spawn(|| {
        let debouncer = EventDebouncer::new(Duration::from_millis(2500), move |_| {
            // After 2.5 seconds of last DesktopChanged event, then update visiblity
            unsafe { update_window_state(WindowState::VISIBLE) };
        });

        for item in rx {
            match item {
                DesktopEvent::DesktopChanged { new: _, old: _ } => {
                    unsafe { update_window_state(WindowState::HIDDEN) };
                    debouncer.put(String::from("DesktopChanged"));
                }
                _ => {}
            }
        }
    });

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {err:?}");
    }

    // Edge case, where it might quit during those 2.5 seconds.
    unsafe { update_window_state(WindowState::VISIBLE) };
}
