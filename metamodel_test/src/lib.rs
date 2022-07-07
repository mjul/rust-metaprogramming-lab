extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Expr, FieldDeclaration, Name, RecordDeclaration};
use metamodel_macros::{generate_data_structures, generate_model_from_tuple};

#[cfg(test)]
mod tests {
    mod generate_data_structures_tests {
        use metamodel::FieldDeclaration;

        use super::super::*;

        #[test]
        fn must_emit_data_structure_for_record_declaration_expr() {
            let no_fields: Vec<FieldDeclaration> = Vec::new();
            let foo_model: Expr = Expr::RecordDeclarationExpr(RecordDeclaration::new(
                Name::Literal(String::from("Foo")),
                Documentation::new(
                    "Foo Record",
                    "A Foo is a very important entity with just and ID.",
                ),
                no_fields,
            ));
            generate_data_structures!(foo_model);

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

        #[test]
        fn must_emit_data_structure_for_record_declaration_with_no_fields() {
            generate_model_from_tuple!((
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
            ));

            // If this compiles, we the struct has been generated
            let _actual = Foo {};

            assert_eq!(true, true);
        }

/*
        #[test]
        fn must_emit_data_structure_for_record_declaration_with_one_field() {
            generate_model_from_tuple!((
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
                [
                    ("name", "id"),
                    (
                        "documentation",
                        [
                            ("label", "ID"),
                            ("description", "The unique Bar entity ID.")
                        ]
                    )
                ]
            ));

            // If this compiles, we the struct has been generated
            let _actual = Bar {id:1};

            assert_eq!(1, actual.id);
        }
*/
    }
}
