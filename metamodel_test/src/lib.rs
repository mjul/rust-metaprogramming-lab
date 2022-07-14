extern crate metamodel;
extern crate metamodel_macros;

#[cfg(test)]
mod tests {
    mod generate_data_structures_tests {
        use super::super::*;
        use metamodel::{Documentation, Expr, FieldDeclaration, Name, RecordDeclaration};

        #[test]
        #[ignore]
        fn must_emit_data_structure_for_record_declaration_expr() {
            let no_fields: Vec<FieldDeclaration> = Vec::new();
            let _foo_model: Expr = Expr::RecordDeclarationExpr(RecordDeclaration::new(
                Name::Literal(String::from("Foo")),
                Documentation::new(
                    "Foo Record",
                    "A Foo is a very important entity with just and ID.",
                ),
                no_fields,
            ));
            metamodel_macros::generate_data_structures!(foo_model);

            /*generate_data_structures!(Expr::RecordDeclarationExpr(
            Name::Literal(String::from("Foo")),
            Documentation::new(
            "Foo Record",
            "A Foo is a very important entity with just and ID."
            )
            ));*/

            // when Foo has been generated from the model above, the following will work
            let actual = Foo::new(1);
            assert_eq!(1, actual.id);
        }
    }

    mod generate_model_from_tuple_tests {
        use super::super::*;
        use metamodel::Documentation;

        #[test]
        fn must_emit_data_structure_for_record_declaration_with_no_fields() {
            metamodel_macros::generate_model_from_tuple!((
                "record",
                [
                    ("name", "Foo"),
                    (
                        "documentation",
                        [
                            ("label", "Foo Record"),
                            (
                                "description",
                                "A Foo is a very important entity with no fields."
                            ),
                        ],
                    ),
                ],
                ("fields", [])
            ));

            // If this compiles, we the struct has been generated
            let _actual = Foo {};

            // If this compiles, we the new constructor has been generated
            let _actual = Foo::new();

            assert_eq!(true, true);
        }

        #[test]
        fn must_emit_data_structure_for_record_declaration_with_one_field() {
            metamodel_macros::generate_model_from_tuple!((
                "record",
                [
                    ("name", "Bar"),
                    (
                        "documentation",
                        [
                            ("label", "Bar Record"),
                            (
                                "description",
                                "A Bar is a very important entity with just and ID."
                            ),
                        ],
                    ),
                ],
                (
                    "fields",
                    [[
                        ("name", "id"),
                        (
                            "documentation",
                            [
                                ("label", "ID"),
                                ("description", "The unique Bar entity ID.")
                            ]
                        ),
                        ("type", "ID")
                    ]]
                )
            ));

            // If this compiles, the struct has been generated
            let actual_raw = Bar { id: 1 };

            // If this compiles, the constructor has been generated
            let actual_via_new = Bar::new(1);

            assert_eq!(1, actual_raw.id);
            assert_eq!(1, actual_via_new.id);
        }

        #[test]
        fn must_emit_data_structure_for_record_declaration_with_two_fields() {
            metamodel_macros::generate_model_from_tuple!((
                "record",
                [
                    ("name", "Baz"),
                    (
                        "documentation",
                        [
                            ("label", "Baz Record"),
                            (
                                "description",
                                "A Baz is a very important entity with two fields."
                            ),
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
                                [
                                    ("label", "ID"),
                                    ("description", "The unique Bar entity ID.")
                                ]
                            ),
                            ("type", "ID")
                        ],
                        [
                            ("name", "birthday"),
                            (
                                "documentation",
                                [
                                    ("label", "Birthday"),
                                    ("description", "The birthday for this Baz.")
                                ]
                            ),
                            ("type", "LocalDate")
                        ]
                    ]
                )
            ));

            // If this compiles, we the struct has been generated
            let actual = Baz {
                id: 1,
                birthday: time::macros::date!(1970 - 01 - 01),
            };

            assert_eq!(1, actual.id);
            assert_eq!(time::macros::date!(1970 - 01 - 01), actual.birthday);

            // If this compiles, we the new constructor been generated
            let actual = Baz::new(1, time::macros::date!(1970 - 01 - 01));

            assert_eq!(1, actual.id);
            assert_eq!(time::macros::date!(1970 - 01 - 01), actual.birthday);
        }

        #[test]
        fn must_emit_data_structure_for_record_declaration_with_all_field_types() {
            metamodel_macros::generate_model_from_tuple!((
                "record",
                [
                    ("name", "AllFieldTypes"),
                    (
                        "documentation",
                        [
                            ("label", "All Field Types"),
                            ("description", "This record has fields of all types."),
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
                                [("label", "ID"), ("description", "The unique Entity ID.")]
                            ),
                            ("type", "ID")
                        ],
                        [
                            ("name", "name"),
                            (
                                "documentation",
                                [
                                    ("label", "Name"),
                                    ("description", "The name of this Entity.")
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
                                    ("description", "The birthday for this Entity.")
                                ]
                            ),
                            ("type", "LocalDate")
                        ],
                    ]
                )
            ));

            // If this compiles, the struct has been generated
            let actual = AllFieldTypes {
                id: 1,
                name: String::from("Unichs Taim"),
                birthday: time::macros::date!(1970 - 01 - 01),
            };

            assert_eq!(1, actual.id);
            assert_eq!("Unichs Taim", actual.name);
            assert_eq!(time::macros::date!(1970 - 01 - 01), actual.birthday);

            // If this compiles, the new constructor been generated
            let actual = AllFieldTypes::new(
                1,
                String::from("Unichs Taim"),
                time::macros::date!(1970 - 01 - 01),
            );

            assert_eq!(1, actual.id);
            assert_eq!("Unichs Taim", actual.name);
            assert_eq!(time::macros::date!(1970 - 01 - 01), actual.birthday);
        }
    }

    mod generate_model_from_expression_stream_tests {
        //use metamodel::FieldDeclaration;
        //use super::super::*;

        #[test]
        #[ignore]
        fn must_emit_data_structure_for_record_declaration_expr() {

            /*
            generate_model_from_expression_stream!(
            metamodel::Expr::RecordDeclarationExpr(
            metamodel::RecordDeclaration::new(
            metamodel::Name::Literal(String::from("Foo")),
            metamodel::Documentation::new(
            "Foo Record",
            "A Foo is a very important entity with just and ID.",
            ),
            vec![],
            )
            )
            );

            // when Foo has been generated from the model above, the following will work
            let actual = Foo::new(1);
            assert_eq!(1, actual.id);

            */
        }
    }

    /*
    // This test using the nested macros fails (
    #[test]
    pub fn bug_report() {
    metamodel_macros::this_fails!(1*2*3*7);
    }
    */

    mod gui_code_generation_tests {
        use super::super::*;
        use metamodel::Documentation;

        // Generate a model in this scope from the macro that works
        // All macros use the same code-generation back-end, so it is not important which one.
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
                                ("description", "The full name of the person.")
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

        #[test]
        fn must_emit_record_with_into_trait_to_displayable() {
            let id = 1;
            let name = "Unichs Taim";
            let bday = time::macros::date!(1970 - 01 - 01);

            let datum = Birth::new(id, String::from(name), bday);

            // if this compiles the Into trait implementation has been generated
            let actual: metamodel::Displayable = datum.into();

            assert_correct_conversion_to_displayable(id, name, &bday, actual);
        }

        fn assert_correct_conversion_to_displayable(
            expected_id: u64,
            expected_name: &str,
            expected_bday: &time::Date,
            actual: metamodel::Displayable,
        ) {
            let Documentation { label, description } = actual.documentation;
            assert_eq!("Birth Information", label);
            assert_eq!("This holds information about a birth.", description);

            assert_eq!(3, actual.values.len());

            let expected_id = (
                metamodel::DisplayableValue::Id(expected_id),
                Documentation::new("ID", "The unique entity ID."),
            );
            assert_eq!(expected_id, actual.values[0]);

            let expected_name = (
                metamodel::DisplayableValue::String(String::from(expected_name)),
                Documentation::new("Full Name", "The full name of the person."),
            );
            assert_eq!(expected_name, actual.values[1]);

            let expected_bday = (
                metamodel::DisplayableValue::LocalDate(*expected_bday),
                Documentation::new("Birthday", "The birthday itself."),
            );
            assert_eq!(expected_bday, actual.values[2]);
        }

        #[test]
        fn must_emit_record_with_from_trait_to_displayable() {
            let id = 1;
            let name = "Unichs Taim";
            let bday = time::macros::date!(1970 - 01 - 01);

            let datum = Birth::new(id, String::from(name), bday);

            // if this compiles the Fromm trait implementation has been generated
            let actual = metamodel::Displayable::from(datum);

            let Documentation { label, description } = actual.documentation;
            assert_eq!("Birth Information", label);
            assert_eq!("This holds information about a birth.", description);

            assert_eq!(3, actual.values.len());

            let expected_id = (
                metamodel::DisplayableValue::Id(id),
                Documentation::new("ID", "The unique entity ID."),
            );
            assert_eq!(expected_id, actual.values[0]);

            let expected_name = (
                metamodel::DisplayableValue::String(String::from(name)),
                Documentation::new("Full Name", "The full name of the person."),
            );
            assert_eq!(expected_name, actual.values[1]);

            let expected_bday = (
                metamodel::DisplayableValue::LocalDate(bday),
                Documentation::new("Birthday", "The birthday itself."),
            );
            assert_eq!(expected_bday, actual.values[2]);
        }
    }
}
