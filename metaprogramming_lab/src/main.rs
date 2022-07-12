extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Name};

use fltk::{app, prelude::*, window::Window, group::Group};

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
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    let disp : metamodel::Displayable = datum.into();
    let mut group = Group::default().with_label(&disp.documentation.label.as_str());

    wind.end();
    wind.show();
    app.run().unwrap();


}
