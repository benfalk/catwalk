mod field;
pub use field::*;
pub use catwalk_derive::Catwalk;

/// All of the core functionality every model will need
/// in order to interface with the database.
pub trait Model {
    /// Database tablename where a model's data is
    fn tablename(&self) -> &str;
}

/// Any model which can be uniquely identified via a unique key
pub trait PrimaryKey {
    /// Field which uniquely identifies a row in the database
    fn primary_key(&self) -> Field<'_>;
}

/// All fields of a model excluding the primary key
pub trait Fields {
    /// Iterator over every field in a model **except**
    /// the primary key in a database row.
    fn fields(&self) -> impl Iterator<Item = Field<'_>>;
}

/// Tagged trait to identify a model can be deleted
pub trait Deleteable: Model + PrimaryKey {}

/// Tagged trait to identify a model can be inserted
pub trait Insertable: Model + Fields {}

/// Tagged trait to identify a model can be updated
pub trait Updateable: Model + PrimaryKey + Fields {}
