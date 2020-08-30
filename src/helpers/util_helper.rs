use crate::db_generators::db_types::DbType;

pub fn first_letter_to_uppper_case (s1: &String) -> String {
    let mut c = s1.chars();
    match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn map_db_type_to_ts(db_filed_type: &String, db_type: &DbType) -> String {
  match db_type {
    DbType::SQLITE => {
      match db_filed_type {
        INT => String::from("number"),
        INTEGER => String::from("number"),
        TINYINT => String::from("number"),
        SMALLINT => String::from("number"),
        MEDIUMINT => String::from("number"),
        BIGINT => String::from("number"),
        INT2 => String::from("number"),
        INT8 => String::from("number"),
        CHARACTER => String::from("char"),
        VARCHAR => String::from("string"),
        NCHAR => String::from("string"),
        NVARCHAR => String::from("string"),
        TEXT => String::from("string"),
        CLOB => String::from("string"),
        REAL => String::from("number"),
        DOUBLE => String::from("number"),
        FLOAT => String::from("number"),
        NUMERIC => String::from("number"),
        DECIMAL => String::from("number"),
        BOOLEAN => String::from("number"),
        DATE => String::from("Date"),
        DATETIME => String::from("Date"),
      }
    }
  }
}