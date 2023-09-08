//#![windows_subsystem = "windows"]
use std::{task::Waker, time::Duration};

use once_cell::sync::OnceCell;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
mod model;
mod system;
use i_slint_backend_winit::WinitWindowAccessor;
use system::system_window;
slint::include_modules!();

static STATUS: OnceCell<i8> = OnceCell::new();

#[tokio::main]
async fn main() {
    STATUS.set(0).expect("set failed!!!");

    let _ = slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()));

    let main = Main::new().expect("Form initialization failed!");

    let main_weak = main.as_weak();
    main_weak
        .unwrap()
        .window()
        .with_winit_window(|winit_window: &winit::window::Window| {
            winit_window.set_decorations(false);
            winit_window.set_transparent(true);
        });

    let icon = Icon::from_path(
        "E:/rust/slint-template-custom/ui/assets/tray/show.ico",
        Some((64, 64)),
    )
    .unwrap();

    let quit = MenuItem::new("退出", true, None);
    let show = MenuItem::new("显示", true, None);

    let tray_menu = Menu::new();
    let _ = tray_menu.append(&show);
    let _ = tray_menu.append(&quit);

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("demo menu")
        .with_icon(icon)
        .build()
        .unwrap();

    tray_icon.set_title(Some("title"));

    let event_handle_weak = main.as_weak();

    let _event_handle = tokio::spawn(async move {
        let weak = event_handle_weak;
        loop {
            if let Ok(event) = MenuEvent::receiver().recv_timeout(Duration::from_millis(200)) {
                println!("event:{:?}", event);
                if event.id.0 == "1000" {
                    break;
                } else if event.id.0 == "1001" {
                    let event_handle_weak = weak.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        event_handle_weak.unwrap().window()
                            .with_winit_window(|winit_window: &winit::window::Window| {
                                winit_window.set_visible(true);
                            });
                    });
                }
            }
        }

        let event_handle_weak = weak.clone();
        let _ = slint::invoke_from_event_loop(move || {
            let weak = event_handle_weak.unwrap();
            let _ = weak.hide();
        });

        println!("Program exit!")
    });

    system_window(main_weak);

    i_slint_backend_winit::with_event_loop_window_target(|event| {
        println!("event:{:?}", event);
    });

    main.run().expect("fail to start!");
}
