#[derive(Debug)]
pub enum PrimitiveType {
    Id,
    LocalDate,
    String,
}

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
}

#[derive(Debug)]
pub enum Name {
    Literal(String),
}

/// Documentation
#[derive(Debug, Eq, PartialEq)]
pub struct Documentation {
    pub label: String,
    pub description: String,
}

impl Documentation {
    pub fn new(label: &str, description: &str) -> Self {
        Self {
            label: label.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: Name,
    pub documentation: Documentation,
    pub field_type: Type,
}

impl FieldDeclaration {
    pub fn new(name: Name, documentation: Documentation, field_type: Type) -> Self {
        Self {
            name,
            documentation,
            field_type,
        }
    }
}

#[derive(Debug)]
pub struct RecordDeclaration {
    pub name: Name,
    pub documentation: Documentation,
    pub fields: Vec<FieldDeclaration>,
}

impl RecordDeclaration {
    pub fn new(name: Name, documentation: Documentation, fields: Vec<FieldDeclaration>) -> Self {
        // TODO: protect against unique duplicate field names
        // TODO: validate field names
        Self {
            name,
            documentation,
            fields,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    FieldDeclarationExpr(FieldDeclaration),
    RecordDeclarationExpr(RecordDeclaration),
}

/// A displayable value
#[derive(Debug, Eq, PartialEq)]
pub enum DisplayableValue {
    String(String),
    LocalDate(time::Date),
    Id(u64),
}

/// Get a string representation of a Displayable Value for displaying.
impl std::fmt::Display for DisplayableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayableValue::String(s) => write!(f, "{}", s),
            DisplayableValue::LocalDate(d) => {
                write!(f, "{}-{:02}-{:02}", d.year(), d.month() as i32, d.day())
            }
            DisplayableValue::Id(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    mod displayable_tests {
        use super::super::*;

        #[test]
        fn displayable_value_implements_fmt_display_trait() {
            let s = DisplayableValue::String(String::from("foo"));
            assert_eq!("foo", format!("{s}"));

            let d = DisplayableValue::LocalDate(time::macros::date!(2022 - 07 - 13));
            assert_eq!("2022-07-13", format!("{d}"));

            let i = DisplayableValue::Id(42);
            assert_eq!("42", format!("{i}"));
        }
    }
}

// A displayable record
#[derive(Debug)]
pub struct Displayable {
    pub documentation: Documentation,
    pub values: Vec<(DisplayableValue, Documentation)>,
}
