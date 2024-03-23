/// Represents every primitive value
/// that a model field can hold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'a> {
    // Strings
    String(&'a str),

    // Unsigned Ints
    U8(&'a u8),
    U16(&'a u16),
    U32(&'a u32),
    U64(&'a u64),

    // Signed Ints
    I8(&'a i8),
    I16(&'a i16),
    I32(&'a i32),
    I64(&'a i64),

    // Floats
    F32(&'a f32),
    F64(&'a f64),

    // Booleans
    Bool(&'a bool),
}

/// Represents every field in a model that
/// maps to a database column.
#[derive(Debug, Clone, Copy)]
pub struct Field<'a> {
    /// Name of the database column to store data
    pub name: &'a str,
    /// Data to store in the database column
    pub value: Value<'a>,
}

/// General iterator to walk over every field in
/// a model.
pub struct FieldIterator<'a, const SIZE: usize> {
    position: usize,
    fields: [Field<'a>; SIZE],
}

impl <'a, const SIZE: usize> FieldIterator<'a, SIZE> {
    pub fn new(fields: [Field<'a>; SIZE]) -> Self {
        Self {
            position: 0,
            fields,
        }
    }
}

impl <'a, const SIZE: usize> Iterator for FieldIterator<'a, SIZE> {
    type Item = Field<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.fields.len() {
            return None;
        }

        let item = Some(self.fields[self.position]);
        self.position += 1;
        item
    }
}

macro_rules! impl_value_from {
    ($variant:expr, $type:ty) => {
        impl<'a> From<&'a $type> for Value<'a> {
            fn from(value: &'a $type) -> Self {
                $variant(value)
            }
        }
    };
}

impl_value_from!(Value::String, str);
impl_value_from!(Value::U8, u8);
impl_value_from!(Value::U16, u16);
impl_value_from!(Value::U32, u32);
impl_value_from!(Value::U64, u64);
impl_value_from!(Value::I8, i8);
impl_value_from!(Value::I16, i16);
impl_value_from!(Value::I32, i32);
impl_value_from!(Value::I64, i64);
impl_value_from!(Value::F32, f32);
impl_value_from!(Value::F64, f64);
impl_value_from!(Value::Bool, bool);

impl <'a> From<&'a String> for Value<'a> {
    fn from(value: &'a String) -> Self {
        Self::String(value.as_str())
    }
}
