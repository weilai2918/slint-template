#![windows_subsystem = "windows"]

mod model;
mod system;
use system::system_window;
slint::include_modules!();

fn main() {

    let _ = slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()));

    let main = Main::new().expect("Form initialization failed!");

    let main_weak = main.as_weak();
    system_window(main_weak);

    main.run().expect("fail to start!");
    println!("Program exit!");
}
