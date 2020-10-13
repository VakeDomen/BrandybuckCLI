use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize)]
pub enum DbType {
    SQLITE
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DbField {
    SqliteField(SqliteTypes)
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SqliteTypes {
    INT,
    INTEGER,
    TINYINT,
    SMALLINT,
    MEDIUMINT,
    BIGINT,
    INT2,
    INT8,
    CHARACTER,
    VARCHAR,
    NCHAR,
    NVARCHAR,
    TEXT,
    CLOB,
    BLOB,
    REAL,
    DOUBLE,
    FLOAT,
    NUMERIC,
    DECIMAL,
    BOOLEAN,
    DATE,
    DATETIME,
}

impl fmt::Display for DbField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let field_type = match self { 
            DbField::SqliteField(ft) => ft 
        };
        write!(f, "{:?}", field_type)
    }
}

pub fn map_field_to_ts_type(field: &DbField) -> String {
  match field {
    DbField::SqliteField(field_type) => map_sqlite_type(field_type)
  }
}

#[allow(non_snake_case)]
fn map_sqlite_type(sqlite_type: &SqliteTypes) -> String {
    match sqlite_type {
        SqliteTypes::INT => String::from("number"),
        SqliteTypes::INTEGER => String::from("number"),
        SqliteTypes::TINYINT => String::from("number"),
        SqliteTypes::SMALLINT => String::from("number"),
        SqliteTypes::MEDIUMINT => String::from("number"),
        SqliteTypes::BIGINT => String::from("number"),
        SqliteTypes::INT2 => String::from("number"),
        SqliteTypes::INT8 => String::from("number"),
        SqliteTypes::CHARACTER => String::from("char"),
        SqliteTypes::VARCHAR => String::from("string"),
        SqliteTypes::NCHAR => String::from("string"),
        SqliteTypes::NVARCHAR => String::from("string"),
        SqliteTypes::TEXT => String::from("string"),
        SqliteTypes::CLOB => String::from("string"),
        SqliteTypes::REAL => String::from("number"),
        SqliteTypes::DOUBLE => String::from("number"),
        SqliteTypes::FLOAT => String::from("number"),
        SqliteTypes::NUMERIC => String::from("number"),
        SqliteTypes::DECIMAL => String::from("number"),
        SqliteTypes::BOOLEAN => String::from("number"),
        SqliteTypes::DATE => String::from("Date"),
        SqliteTypes::DATETIME => String::from("Date"),
        SqliteTypes::BLOB => String::from("string"),
    }
}