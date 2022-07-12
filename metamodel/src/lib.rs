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
#[derive(Debug)]
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
    pub fn new(name: Name, documentation: Documentation, field_type: Type) -> Self { Self { name, documentation, field_type } }
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
        Self { name, documentation, fields }
    }
}



#[derive(Debug)]
pub enum Expr {
    FieldDeclarationExpr(FieldDeclaration),
    RecordDeclarationExpr(RecordDeclaration),
}



/// A displayable value
#[derive(Debug)]
pub enum DisplayableValue {
    String(String),
    LocalDate(String),
    Id(i32),
}


// A displayable record
#[derive(Debug)]
pub struct Displayable {
    pub documentation: Documentation,
    pub values : Vec<(DisplayableValue, Documentation)>
}

