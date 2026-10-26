use gtk4::prelude::*;
use gtk4::{Box, Label};

pub fn build_file_list() -> Box {
    let list = Box::new(gtk4::Orientation::Vertical, 5);

    let placeholder = Label::new(Some("Liste des fichiers…"));
    list.append(&placeholder);

    list
}
