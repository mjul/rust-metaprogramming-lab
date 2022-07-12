extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Name};

use fltk::{app, prelude::*, window::Window, group::{Group,Flex, Pack, PackType}, frame::Frame, button::Button, enums::{Align, Font, LabelType}, output::Output, text::TextDisplay};

metamodel_macros::generate_model_from_tuple!((
            "record",
            [
                ("name", "Birth"),
                (
                    "documentation",
                    [
                        ("label", "Birth Information"),
                        ("description", "This holds information about a birth."),
                    ],
                ),
            ],
            (
                "fields",
                [
                    [
                        ("name", "id"),
                        (
                            "documentation",
                            [("label", "ID"), ("description", "The unique entity ID.")]
                        ),
                        ("type", "ID")
                    ],
                    [
                        ("name", "full_name"),
                        (
                            "documentation",
                            [
                                ("label", "Full Name"),
                                ("description", "The full name of the person")
                            ]
                        ),
                        ("type", "String")
                    ],
                    [
                        ("name", "birthday"),
                        (
                            "documentation",
                            [
                                ("label", "Birthday"),
                                ("description", "The birthday itself.")
                            ]
                        ),
                        ("type", "LocalDate")
                    ]
                ]
            )
        ));



fn main() {

    let datum = Birth::new(1, String::from("Haskell Curry"), String::from("1900-09-12"));
    let disp : metamodel::Displayable = datum.into();

    let app = app::App::default();

    let mut wind = Window::default().with_size(640, 440).with_label("Meta-Programming Lab");

    let mut flex = Flex::default().size_of_parent().column();

    let mut headline = Frame::default().with_label(&disp.documentation.label);
    headline.set_label_size(42);
    let mut description = Frame::default().with_label(&disp.documentation.description);

    let mut name_row = Pack::default().with_type(PackType::Horizontal);
    name_row.set_spacing(20);
    let mut name_frame = Frame::default().with_size(100, 30).with_label("Name frame:");
    let mut name_value = Output::default().with_size(500, 30).set_value("Name Here");
    name_row.end();

    flex.end();
    wind.end();
    wind.show();
    app.run().unwrap();

}