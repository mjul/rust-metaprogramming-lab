extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Expr, Name};
use metamodel_macros::{generate_data_structures, generate_model_from_tuple};

#[cfg(test)]
mod tests {
    mod generate_data_structures_tests {
        use super::super::*;

        #[test]
        fn must_emit_data_structure_for_record_declaration_expr() {
            let foo_model: Expr = Expr::RecordDeclarationExpr(
                Name::Literal(String::from("Foo")),
                Documentation::new(
                    "Foo Record",
                    "A Foo is a very important entity with just and ID.",
                ),
            );
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
        fn must_emit_data_structure_for_record_declaration() {
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
                                "A Foo is a very important entity with just and ID."
                            ),
                        ],
                    ),
                ],
            ));

            // If this compiles, we the struct has been generated
            let _actual = Foo {};

            assert_eq!(true, true);
        }
    }
}
