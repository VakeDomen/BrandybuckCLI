use crate::db_generators::db_types::{DbField, SqliteTypes};

#[derive(Serialize, Deserialize)]
pub struct ModelFile {
    pub models: Vec<Model>
}


#[derive(Serialize, Deserialize)]
#[serde(default = "default_model")]
pub struct Model {
    pub name: String,
    pub crud: Crud,
    pub fields: Vec<Field>
}

#[derive(Serialize, Deserialize)]
#[serde(default = "default_crud")]
pub struct Crud {
    pub create: bool,
    pub create_auth: bool,
    pub read: bool,
    pub read_auth: bool,
    pub update: bool,
    pub update_auth: bool,
    pub delete: bool,
    pub delete_auth: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(default = "default_field")]
pub struct Field {
    pub name: String,
    pub data_type: DbField,
    pub null: bool
}

impl ModelFile {
    pub fn new () -> ModelFile {
        ModelFile {
            models: vec!(default_model())
        }
    }
}

fn default_model() -> Model {
    Model {
        name: String::from("table"),
        crud: default_crud(),
        fields: vec!(
            default_named_field(String::from("col1")),
            default_named_field(String::from("col2"))
        )
    }
}

fn default_crud() -> Crud {
    Crud {
        create: true,
        create_auth: false,
        read: true,
        read_auth: false,
        update: true,
        update_auth: false,
        delete: true,
        delete_auth: false
    }
}

fn default_field() -> Field {
    Field {
        name: String::from("col"),
        data_type: DbField::SqliteField(SqliteTypes::VARCHAR),
        null: false
    }
}
fn default_named_field(name: String) -> Field {
    Field {
        name: name,
        data_type: DbField::SqliteField(SqliteTypes::VARCHAR),
        null: false
    }
}