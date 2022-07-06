use std::clone::Clone;

pub enum PrimitiveType {
    Id,
    LocalDate
}

pub enum Type {
    Primitive(PrimitiveType)
}

pub enum Name {
    Literal(String)
}

/// Documentation
pub struct Documentation {
    label: String,
    description: String,
}

impl Documentation {
    pub fn new(label: &str, description: &str) -> Self { Self {label: label.to_string(), description: description.to_string() } }
}


pub enum Expr {
    FieldDeclarationExpr(Name, Documentation, Type),
    RecordDeclarationExpr(Name, Documentation)
}

