#![allow(dead_code)]

use catwalk::{Fields, Model, PrimaryKey};

#[derive(Debug, Clone)]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
}

impl catwalk::Model for Person {
    fn tablename(&self) -> &str {
        "people"
    }
}

impl catwalk::PrimaryKey for Person {
    fn primary_key(&self) -> catwalk::Field<'_> {
        catwalk::Field {
            name: "person_id",
            value: catwalk::Value::from(&self.id),
        }
    }
}

impl catwalk::Fields for Person {
    fn fields(&self) -> impl Iterator<Item = catwalk::Field<'_>> {
        catwalk::FieldIterator::new([
            catwalk::Field {
                name: "fname",
                value: catwalk::Value::from(&self.first_name),
            },
            catwalk::Field {
                name: "lname",
                value: catwalk::Value::from(&self.last_name),
            }
        ])
    }
}

#[test]
fn it_works() {
    let person = Person {
        id: 42,
        first_name: "Ben".to_owned(),
        last_name: "Falk".to_owned(),
    };
    assert_eq!(person.tablename(), "people");
    assert_eq!(person.primary_key().name, "person_id");
    assert_eq!(person.primary_key().value, catwalk::Value::U32(&42));
    let mut fields = person.fields();
    let first_name = fields.next().unwrap();
    let last_name = fields.next().unwrap();
    assert_eq!(first_name.name, "fname");
    assert_eq!(first_name.value, catwalk::Value::String("Ben"));
    assert_eq!(last_name.name, "lname");
    assert_eq!(last_name.value, catwalk::Value::String("Falk"));
    assert!(fields.next().is_none());
}
