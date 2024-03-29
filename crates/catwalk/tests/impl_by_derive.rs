#![allow(dead_code)]

#[derive(Debug, Clone, catwalk::Catwalk)]
#[catwalk(tablename = "people")]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
}

#[test]
fn it_works() {
    use catwalk::Model;

    let person = Person {
        id: 42,
        first_name: "Ben".to_owned(),
        last_name: "Falk".to_owned(),
    };
    assert_eq!(person.tablename(), "people");
}
