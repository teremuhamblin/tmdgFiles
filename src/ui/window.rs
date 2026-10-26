use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box};

pub fn build_main_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("tmdgFiles")
        .default_width(900)
        .default_height(600)
        .build();

    let root = Box::new(gtk4::Orientation::Vertical, 0);

    window.set_child(Some(&root));
    window.show();
}
