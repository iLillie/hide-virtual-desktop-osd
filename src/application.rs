//#![windows_subsystem = "windows"]
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::application::ApplicationHandler;

#[derive(Debug)]
pub enum UserEvent {
    TrayIconEvent(),
    MenuEvent(MenuEvent),
}

pub struct Application {
    tray_icon: Option<TrayIcon>,
}

impl Application {
    pub fn new() -> Application {
        Application { tray_icon: None }
    }

    fn new_tray_icon() -> TrayIcon {
        let bytes = include_bytes!("../assets/icon.png");

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(bytes)
                .expect("Failed to load image from memory")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            (rgba, width, height)
        };
        let icon = tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("Failed to open icon");

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
        let item1 = MenuItem::new("Quit", true, None);
        if let Err(err) = menu.append(&item1) {
            println!("{err:?}");
        }
        menu
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

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
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent() => {}
            UserEvent::MenuEvent(menu_event) => {
                if menu_event.id() == "1001" {
                    event_loop.exit()
                }
            }
        }
    }
}
