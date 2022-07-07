#[derive(Debug)]
pub enum PrimitiveType {
    Id,
    LocalDate,
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
#[derive(Debug)]
pub struct Documentation {
    label: String,
    description: String,
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
pub enum Expr {
    FieldDeclarationExpr(Name, Documentation, Type),
    RecordDeclarationExpr(Name, Documentation),
}
