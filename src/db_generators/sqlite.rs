use crate::helpers::file_helper::write_file;
use crate::models::config_file::ConfigFile;
use crate::models::models_file::{ModelFile, Model};

pub fn generate_sqlite_migation_files(config_file: &ConfigFile, model_file: &ModelFile) -> () {
    let table_models_up = generate_tables_up(config_file, model_file);
    let table_models_down = generate_tables_down(config_file, model_file);
    let mut migration_string = String::from("-- Up\n") + &table_models_up + "\n-- Down\n" + &table_models_down;
    write_file(&mut migration_string, "app/migrations/001-inital-schema.sql".to_string());
}

fn generate_tables_up(config_file: &ConfigFile, model_file: &ModelFile) -> String {
    let mut tables: Vec<String> = Vec::new();
    for model in model_file.models.iter() {
        tables.push(generate_model_table(model));
    }
    tables.join("\n")
}

fn generate_tables_down(config_file: &ConfigFile, model_file: &ModelFile) -> String {
    let mut tables: Vec<String> = Vec::new();
    for model in model_file.models.iter() {
        tables.push(String::from("DROP TABLE ") + &model.name + ";");
    }
    tables.join("\n")
}

fn generate_model_table(model: &Model) -> String {
    let mut table_rows = Vec::new();
    table_rows.push(String::from("CREATE TABLE ") + &model.name + " (");
    table_rows.push(String::from("\tid VARCHAR PRIMARY KEY"));
    for col in model.fields.iter() {
        let mut column = String::from("\t") + &col.name + " " + &col.data_type + " ";
        if col.null {
            column = column + "NULL,";
        } else {
            column = column + "NOT NULL,";
        }
        table_rows.push(column)
    }
    let row_len = &table_rows.len() - 1;
    let str_len = &table_rows[row_len.clone()].len() - 1;
    table_rows[row_len].remove(str_len);
    table_rows.push(String::from(");\n"));
    table_rows.join("\n")
}