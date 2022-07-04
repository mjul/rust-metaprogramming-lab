use std::clone::Clone;

/// A documentation string
#[derive(Debug, Clone)]
struct DocString {
    value: String,
}
impl DocString {
    pub fn from(value: &str) -> Self {
        DocString {
            value: value.to_string(),
        }
    }
}

enum PrimitiveType {
    Id,
    String,
    LocalDate,
}

enum FieldType {
    Primitive(PrimitiveType),
}

/// Named represents a named entity
trait Named {
    fn get_name(&self) -> String;
}

/// Documented represents something with documentation, a docstring.
trait Documented {
    fn get_docstring(&self) -> DocString;
}

/// Declarations are named and documented
trait Declaration: Named + Documented {}

struct FieldDeclaration {
    name: String,
    field_type: FieldType,
    doc_str: DocString,
    sort_index: usize,
}

impl FieldDeclaration {
    fn new(name: String, field_type: FieldType, doc_str: DocString, sort_index: usize) -> Self {
        FieldDeclaration {
            name,
            field_type,
            doc_str,
            sort_index,
        }
    }
}

impl Named for FieldDeclaration {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Documented for FieldDeclaration {
    fn get_docstring(&self) -> DocString {
        self.doc_string.clone()
    }
}

impl Declaration for FieldDeclaration {}

/// Declaration for an event
struct EventDeclaration {
    name: String,
    doc_str: DocString,
    fields: Vec<FieldDeclaration>,
}

impl EventDeclaration {
    // Create a new event
    fn new(name: String, doc_str: DocString, fields: Vec<FieldDeclaration>) -> Self {
        EventDeclaration {
            name,
            doc_str,
            fields,
        }
    }
}

impl Named for EventDeclaration {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Documented for EventDeclaration {
    fn get_docstring(&self) -> DocString {
        self.doc_str.clone()
    }
}

impl Declaration for EventDeclaration {}

/// The MetaModel is the main entry-point into the meta-model
pub struct MetaModel {
    events: Vec<EventDeclaration>,
}

impl MetaModel {
    /// Construct a new MetaModel
    fn new(events: Vec<EventDeclaration>) -> Self {
        MetaModel { events }
    }
}

pub fn new_example_model() -> MetaModel {
    MetaModel::new(vec![EventDeclaration::new(
        String::from("MonthEnded"),
        DocString::from("This event signals that a calendar month has ended."),
        vec![
            FieldDeclaration::new(
                String::from("id"),
                FieldType::Primitive(PrimitiveType::Id),
                DocString::from("Globally unique event ID"),
                1,
            ),
            FieldDeclaration::new(
                String::from("description"),
                FieldType::Primitive(PrimitiveType::Id),
                DocString::from("Textual description of this event."),
                2,
            ),
            FieldDeclaration::new(
                String::from("first_day_in_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                DocString::from("The first date in the month that ended."),
                3,
            ),
            FieldDeclaration::new(
                String::from("last_day_in_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                DocString::from("The last date in the month that ended."),
                4,
            ),
            FieldDeclaration::new(
                String::from("first_day_in_next_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                DocString::from("The first date in the following month."),
                5,
            ),
        ],
    )])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_example_model_must_return_model() -> Result<(), String> {
        let actual = new_example_model();
        match actual.events.is_empty() {
            false => Ok(()),
            true => Err(String::from("Expected some event declarations")),
        }
    }
}
