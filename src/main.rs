use std::time::Duration;

use debounce::EventDebouncer;
use tray_icon::{
    TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::{application::ApplicationHandler, event_loop::EventLoop};
use winvd::{DesktopEvent, listen_desktop_events};

mod window;

use crate::window::{WindowState, initialize_window, update_window_state};

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(),
    MenuEvent(),
}

struct Application {
    tray_icon: Option<TrayIcon>,
}

impl Application {
    fn new() -> Application {
        Application { tray_icon: None }
    }

    fn new_tray_icon() -> TrayIcon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
        let icon: tray_icon::Icon = load_icon(std::path::Path::new(path));

        TrayIconBuilder::new()
            .with_menu(Box::new(Self::new_tray_menu()))
            .with_tooltip("Hides virtual desktop on-screen display")
            .with_title("HideVirtualDesktopOSD")
            .with_icon(icon)
            .build()
            .unwrap()
    }

    fn new_tray_menu() -> Menu {
        let menu = Menu::new();
        let item1 = MenuItem::new("item1", true, None);
        if let Err(err) = menu.append(&item1) {
            println!("{err:?}");
        }
        menu
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {

        if winit::event::StartCause::Init == cause {
            self.tray_icon = Some(Self::new_tray_icon());

            unsafe {
                initialize_window();
                update_window_state(WindowState::VISIBLE);
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, _event: UserEvent) {
    }
}

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    // set a tray event handler that forwards the event and wakes up the event loop
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |_| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent());
    }));
    let proxy = event_loop.create_proxy();

    MenuEvent::set_event_handler(Some(move |_| {
        let _ = proxy.send_event(UserEvent::MenuEvent());
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
                    debouncer.put(String::from("Event"));
                }
                _ => {}
            }
        }
    });

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {err:?}");
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}