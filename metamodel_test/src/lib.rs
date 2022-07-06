extern crate metamodel;
extern crate metamodel_macros;

use metamodel::{Documentation, Expr, Name};
use metamodel_macros::generate_data_structures;

#[cfg(test)]
mod tests {
    mod generate_data_structures {
        use super::super::*;

        #[test]
        fn must_emit_data_structure_for_record_declaration_expr() {

            let foo_model : Expr = Expr::RecordDeclarationExpr(
                Name::Literal(String::from("Foo")),
                Documentation::new(
                    "Foo Record",
                    "A Foo is a very important entity with just and ID."
                )
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
}
