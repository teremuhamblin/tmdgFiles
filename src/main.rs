mod ui;
mod core;
mod config;

use gtk4::prelude::*;
use gtk4::{Application};

fn main() {
    let app = Application::new(
        Some("com.tmdg.files"),
        Default::default(),
    );

    app.connect_activate(|app| {
        ui::window::build_main_window(app);
    });

    app.run();
}
