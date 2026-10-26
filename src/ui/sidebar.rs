use gtk4::prelude::*;
use gtk4::{Box, Label};

pub fn build_sidebar() -> Box {
    let sidebar = Box::new(gtk4::Orientation::Vertical, 10);

    let title = Label::new(Some("📁 tmdgFiles"));
    let home = Label::new(Some("Home"));
    let docs = Label::new(Some("Documents"));
    let dl = Label::new(Some("Téléchargements"));
    let fav = Label::new(Some("Favoris"));

    sidebar.append(&title);
    sidebar.append(&home);
    sidebar.append(&docs);
    sidebar.append(&dl);
    sidebar.append(&fav);

    sidebar
}
