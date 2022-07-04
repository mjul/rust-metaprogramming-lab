enum PrimitiveType {
    Id,
    String,
    LocalDate,
}

enum FieldType {
    Primitive(PrimitiveType)
}

struct FieldDeclaration {
    name: String,
    field_type: FieldType,
    docstring: String,
    sort_index: usize,
}

impl FieldDeclaration {
    fn new(name: String, tp: FieldType, docstr: String, sort_index: usize) -> Self {
        FieldDeclaration {
            name: name,
            field_type: tp,
            docstring: docstr,
            sort_index: sort_index,
        }
    }
}

/// Declaration for an event
struct EventDeclaration {
    name: String,
    fields: Vec<FieldDeclaration>,
}

impl EventDeclaration {
    // Create a new event
    fn new(name: String, fields: Vec<FieldDeclaration>) -> Self {
        EventDeclaration {
            name: name,
            fields: fields,
        }
    }
}

/// The MetaModel is the main entry-point into the meta-model
pub struct MetaModel {
    events: Vec<EventDeclaration>,
}

impl MetaModel {
    /// Construct a new MetaModel
    fn new(evts: Vec<EventDeclaration>) -> Self {
        MetaModel { events: evts }
    }
}

pub fn new_example_model() -> MetaModel {
    MetaModel::new(vec![EventDeclaration::new(
        String::from("MonthEnded"),
        vec![
            FieldDeclaration::new(
                String::from("id"),
                FieldType::Primitive(PrimitiveType::Id),
                String::from("Globally unique event-id"),
                1,
            ),
            FieldDeclaration::new(
                String::from("description"),
                FieldType::Primitive(PrimitiveType::Id),
                String::from("Textual description of this event."),
                2,
            ),
            FieldDeclaration::new(
                String::from("first_day_in_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                String::from("The first date in the month that ended."),
                3,
            ),
            FieldDeclaration::new(
                String::from("last_day_in_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                String::from("The last date in the month that ended."),
                4,
            ),
            FieldDeclaration::new(
                String::from("first_day_in_next_month"),
                FieldType::Primitive(PrimitiveType::LocalDate),
                String::from("The first date in the following month."),
                5,
            ),
        ],
    )])
}
