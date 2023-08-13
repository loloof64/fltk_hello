#![windows_subsystem = "windows"]

use fltk::{prelude::*, window::Window, button::Button, group, app, dialog};

fn main() {
    let a = app::App::default();
    
    let mut wind = Window::default().with_size(300, 180).center_screen().with_label("Fltk Hello");
    
    let flex = group::Flex::default().with_size(300, 180).column().center_of_parent();
    let mut button = Button::default().with_label("Say hello !");
    flex.end();

    button.set_callback(|_btn| {
        dialog::message_default("Hello, World !");
    });
    
    wind.end();
    wind.show();

    a.run().unwrap();
}
