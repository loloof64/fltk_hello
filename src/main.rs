#![windows_subsystem = "windows"]

use fltk::{
    app,
    button::Button,
    dialog, group, input,
    prelude::{WidgetExt, *},
    window::Window, frame,
};

fn main() {
    let a = app::App::default();

    let mut wind = Window::default()
        .with_size(300, 180)
        .center_screen()
        .with_label("Fltk Hello");

    let flex = group::Flex::default()
        .with_size(300, 180)
        .column()
        .center_of_parent();
    let flex2 = group::Flex::default()
        .with_size(300, 90)
        .row()
        .center_of_parent();
    let _label = frame::Frame::default().set_label("Your firstname : ");
    let input = input::Input::default();
    flex2.end();
    let mut button = Button::default().with_label("Say hello !");
    flex.end();

    button.set_callback(move |_btn| {
        let firstname = input.value();
        dialog::message_default(format!("Hello, {} !", firstname).as_str());
    });

    wind.end();
    wind.show();

    a.run().unwrap();
}
