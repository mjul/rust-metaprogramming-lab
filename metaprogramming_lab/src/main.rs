extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Name};
use time::Date;

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

    let datum = Birth::new(1, String::from("Haskell Curry"), time::macros::date!(1900-09-12));
    let disp : metamodel::Displayable = datum.into();

    let app = app::App::default();

    let mut wind = Window::default().with_size(640, 440).with_label("Meta-Programming Lab");

    let mut flex = Flex::default().size_of_parent().column();

    let mut headline = Frame::default().with_label(&disp.documentation.label);
    headline.set_label_size(42);
    let mut description = Frame::default().with_label(&disp.documentation.description);

    for (val, doc) in disp.values.iter() {
        let mut row = Pack::default().with_type(PackType::Horizontal);
        row.set_spacing(20);
        let mut row_label = Frame::default().with_size(100, 30).with_label(&doc.label);
        let v = match &val {
                    metamodel::DisplayableValue::String(s) => s.clone(),
                    metamodel::DisplayableValue::LocalDate(s) => s.to_string(),
                    metamodel::DisplayableValue::Id(n) => n.to_string(),
        };
        let mut row_value = Output::default().with_size(300, 30).set_value(v.as_str());
        let mut row_desc = Frame::default().with_label(&doc.description).with_align(Align::Right);
        row.end();
    }

    flex.end();
    wind.end();
    wind.show();
    app.run().unwrap();

}
